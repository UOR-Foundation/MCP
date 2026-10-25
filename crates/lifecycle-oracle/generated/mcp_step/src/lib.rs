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
    C0(Str, Option<Str>, Str, Str, Option<u64>, Option<u64>),
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
    C0(Adt87, Adt67),
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
    C15(Adt89),
}

#[derive(Clone)]
pub enum Adt89 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt90 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt91 {
    C0(List<u64>),
    C1(Adt89),
}

#[derive(Clone)]
pub enum Adt92 {
    C0(Adt56),
    C1(Adt85),
    C2(Adt89),
}

#[derive(Clone)]
pub enum Adt93 {
    C0(Adt62),
    C1(Adt85),
    C2(Adt89),
}

#[derive(Clone)]
pub enum Adt94 {
    C0(List<u64>, List<u64>, List<Adt95>, Bytes, u64, List<List<u64>>, Bytes),
}

#[derive(Clone)]
pub enum Adt95 {
    C0(List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt96 {
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
pub enum Adt97 {
    C0(Adt41),
    C1(Adt85),
    C2(Adt86),
    C3(Adt89),
}

#[derive(Clone)]
pub enum Adt98 {
    C0(u64, List<Adt95>, List<List<u64>>, List<u64>),
}

#[derive(Clone)]
pub enum Adt99 {
    C0(Adt53),
    C1(Adt85),
}

#[derive(Clone)]
pub enum Adt100 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt101 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt102 {
    C0,
    C1,
    C2(Adt8),
}

#[derive(Clone)]
pub enum Adt103 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt104 {
    C0(Adt9, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt105 {
    C0,
    C1(List<Adt106>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt106 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt107 {
    C0,
    C1(List<Adt106>, Str),
}

#[derive(Clone)]
pub enum Adt108 {
    C0(Adt9, Adt9, List<Adt106>),
    C1(List<Adt9>, Adt9, List<Adt106>),
    C2(List<Adt9>, Adt9, List<Adt106>, u64),
    C3(List<Adt106>),
    C4(Option<Adt9>, Option<Adt9>, Adt9, List<Adt106>),
    C5(Adt109, Adt9, Adt9, List<Adt106>),
}

#[derive(Clone)]
pub enum Adt109 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt110 {
    C0,
    C1(Adt9),
}

#[derive(Clone)]
pub enum Adt111 {
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
pub enum Adt112 {
    C0,
    C1(Option<Adt103>, Adt113),
}

#[derive(Clone)]
pub enum Adt113 {
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
pub enum Adt114 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt115 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt116 {
    C0(List<Adt15>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt117 {
    C0(Adt107, List<Adt108>),
}

#[derive(Clone)]
pub enum Adt118 {
    C0(Adt103, Str, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt119 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt108>),
}

#[derive(Clone)]
pub enum Adt120 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt121 {
    C0(List<u64>),
    C1(u64, Adt9),
    C2(u64, List<Adt9>),
    C3(u64, List<Adt9>),
    C4(u64, List<(Str, Adt9)>),
    C5(u64, List<(Str, Adt9)>),
}

#[derive(Clone)]
pub enum Adt122 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt123>, List<Adt125>),
}

#[derive(Clone)]
pub enum Adt123 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt124),
    C7(Adt124),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt124 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt125 {
    C0(Str, List<Str>, Adt16, Adt126),
}

#[derive(Clone)]
pub enum Adt126 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt127 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt128 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt129 {
    C0(Adt130, List<Adt134>, u64),
}

#[derive(Clone)]
pub enum Adt130 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt131, List<u64>, bool),
    C7(u64, u64),
    C8(Adt132),
    C9(Adt9),
}

#[derive(Clone)]
pub enum Adt131 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt132 {
    C0(bool, Adt133, i64, u64, i64, bool, i64),
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
    C7,
}

#[derive(Clone)]
pub enum Adt134 {
    C0(List<Adt9>),
    C1(List<(Str, Adt9)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt135 {
    C0(Adt103, Result<Adt136, Adt139>),
}

#[derive(Clone)]
pub enum Adt136 {
    C0(Adt36, Adt137, List<Adt138>),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(Option<Adt4>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(Adt23, Adt54),
}

#[derive(Clone)]
pub enum Adt139 {
    C0(Adt140),
    C1(Str, List<Str>),
    C2(Adt123),
    C3(Str),
    C4(Adt88),
}

#[derive(Clone)]
pub enum Adt140 {
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
pub enum Adt141 {
    C0(Option<Str>, Option<Adt123>),
}

#[derive(Clone)]
pub enum Adt142 {
    C0,
    C1(Adt103),
    C2,
}

#[derive(Clone)]
pub enum Adt143 {
    C0(Adt111),
    C1(Adt129),
}

#[derive(Clone)]
pub enum Adt144 {
    C0(Adt96),
    C1(Adt116),
}

#[derive(Clone)]
pub enum Adt145 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt146 {
    C0(Adt111),
    C1(List<Adt121>, List<u64>),
}

#[derive(Clone)]
pub enum Adt147 {
    C0(Adt103, Adt148, Adt20, Adt8),
}

#[derive(Clone)]
pub enum Adt148 {
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
pub enum Adt149 {
    C0,
    C1,
    C2(Adt9),
}

#[derive(Clone)]
pub enum Adt150 {
    C0,
    C1(Adt148),
}

#[derive(Clone)]
pub enum Adt151 {
    C0(Adt152),
    C1(Adt139),
}

#[derive(Clone)]
pub enum Adt152 {
    C0(Adt103, Str, Adt20, Adt8),
}

#[derive(Clone)]
pub enum Adt153 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt154 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt155 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt156 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt157 {
    C0(Adt9),
    C1(Adt9),
}

#[derive(Clone)]
pub enum Adt158 {
    C0(List<Adt9>, List<Adt9>),
    C1(Str, List<(Str, Adt9)>, List<(Str, Adt9)>),
}

#[derive(Clone)]
pub enum Adt159 {
    C0,
    C1(List<(Adt9, Adt9)>),
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
    C1(Adt22),
}

#[derive(Clone)]
pub enum Adt163 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt164 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt165 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt164, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F4,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt17) -> R<Result<Adt36, Adt88>> {
        match self {
            Fn0::F4 => f4(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F25,
    F62(Fn0, Adt0),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt18) -> R<Result<Adt36, Adt88>> {
        match self {
            Fn1::F25 => f25(p0),
            Fn1::F62(k0, k1) => f62(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F39(List<Adt1>),
    F86,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt25) -> R<Result<Adt53, Adt88>> {
        match self {
            Fn2::F39(k0) => Ok(f39(k0.clone(), p0)),
            Fn2::F86 => f86(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F85,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt25) -> R<Result<Adt99, Adt88>> {
        match self {
            Fn3::F85 => f85(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F114(Fn2, Adt39),
    F118(Fn3, Adt39),
}

impl Fn4 {
    pub fn apply(&self, p0: Adt25) -> R<Result<Adt36, Adt88>> {
        match self {
            Fn4::F114(k0, k1) => f114(k0.clone(), k1.clone(), p0),
            Fn4::F118(k0, k1) => f118(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F146,
}

impl Fn5 {
    pub fn apply(&self, p0: Str) -> R<Option<List<Adt50>>> {
        match self {
            Fn5::F146 => f146(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F158,
}

impl Fn6 {
    pub fn apply(&self, p0: Str, p1: List<(Str, Str)>) -> Adt62 {
        match self {
            Fn6::F158 => f158(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F160,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt28) -> R<Result<List<Str>, Adt88>> {
        match self {
            Fn7::F160 => f160(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F959(Bytes),
    F984(Bytes, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn8::F959(k0) => f959(k0.clone(), p0),
            Fn8::F984(k0, k1) => f984(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F964(Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn9::F964(k0) => f964(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F969(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn10::F969(k0) => f969(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F992(Bytes, u64, u64, u64),
    F999(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt163)) -> R<(u64, Adt163)> {
        match self {
            Fn11::F992(k0, k1, k2, k3) => f992(k0.clone(), *k1, *k2, *k3, p0),
            Fn11::F999(k0, k1, k2) => f999(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F994,
}

impl Fn12 {
    pub fn apply(&self, p0: Adt165) -> R<Adt165> {
        match self {
            Fn12::F994 => f994(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F1024,
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn13::F1024 => f1024(p0),
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

pub fn f4(v0: Adt17) -> R<Result<Adt36, Adt88>> {
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

pub fn f6(v0: Bytes, v1: Option<Adt90>) -> R<Result<Adt0, Adt2>> {
    let m27 = v1;
    match m27 {
        None => {
            Ok(Err::<Adt0, Adt2>(Adt2::C0(0u64)))
        }
        Some(v2) => {
            f26(v0.clone(), v2.clone(), f27(v0, match v2 {
                Adt90::C0(_, h28) => {
                    h28
                }
            })?)
        }
    }
}

pub fn f7(v0: Bytes, v1: u64) -> R<Option<Adt90>> {
    let m29 = f27(v0.clone(), v1)?;
    match m29 {
        None => {
            Ok(None::<Adt90>)
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
                    Ok(None::<Adt90>)
                }
                Some(v3) => {
                    Ok(Some(Adt90::C0(v3, {
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

pub fn f8(v0: Adt91) -> R<List<u64>> {
    let m40 = v0;
    match m40 {
        Adt91::C0(v1) => {
            let a41 = f28(0u64);
            let a42 = v1;
            Ok(append_list(a41, a42))
        }
        Adt91::C1(v2) => {
            let a47 = f28(1u64);
            let a48 = {
                let a45 = f28(match v2.clone() {
                    Adt89::C0(h43, _) => {
                        h43
                    }
                });
                let a46 = f29(match v2 {
                    Adt89::C0(_, h44) => {
                        h44
                    }
                })?;
                append_list(a45, a46)
            };
            Ok(append_list(a47, a48))
        }
    }
}

pub fn f9(v0: Adt3, v1: Fn0, v2: Bytes) -> R<Adt91> {
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
    Adt7::C0(None::<Adt8>, None::<Adt8>, Some(f32()), Some(Adt11::C0(None::<bool>)), Some(Adt12::C0(None::<bool>, None::<bool>)), Some(Adt11::C0(None::<bool>)), None::<Adt8>)
}

pub fn f12() -> Adt13 {
    Adt13::C0(0u64, Adt14::C1)
}

pub fn f13(v0: List<Adt39>) -> R<List<Adt15>> {
    let m51 = f33(v0)?;
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
    let a52 = f34()?;
    let a53 = f35();
    Ok(append_list(a52, a53))
}

pub fn f15() -> u64 {
    100u64
}

pub fn f16(v0: List<Adt1>, v1: Adt18, v2: Adt25) -> R<Result<Adt36, Adt88>> {
    if f36(v2.clone()) {
        f37(f38()?, Fn2::F39(v0), v2)
    } else {
        if f40(v2, Str::lit("provider_effect")) {
            Ok(f41(f42(v0)))
        } else {
            f19(v1)
        }
    }
}

pub fn f17(v0: Result<Adt92, Adt88>) -> Result<Adt36, Adt88> {
    let m54 = v0;
    match m54 {
        Ok(v1) => {
            let m55 = v1;
            match m55 {
                Adt92::C0(v2) => {
                    Ok::<Adt36, Adt88>(Adt36::C5(v2))
                }
                Adt92::C1(v3) => {
                    Ok::<Adt36, Adt88>(Adt36::C12(v3))
                }
                Adt92::C2(v4) => {
                    Err::<Adt36, Adt88>(Adt88::C15(v4))
                }
            }
        }
        Err(v5) => {
            Err::<Adt36, Adt88>(v5)
        }
    }
}

pub fn f18(v0: List<Adt1>, v1: Adt26) -> Result<Adt92, Adt88> {
    let m56 = v0;
    match m56.uncons() {
        None => {
            Ok::<Adt92, Adt88>(Adt92::C2(f43(4u64, match v1 {
                Adt26::C0(_, h57, _, _) => {
                    h57
                }
            })))
        }
        Some((v2, v3)) => {
            let m58 = v3;
            match m58.uncons() {
                None => {
                    let a60 = match v2.clone() {
                        Adt1::C0(h59, _) => {
                            h59
                        }
                    };
                    let a61 = 4u64;
                    if nat_eq(a60, a61) {
                        Ok::<Adt92, Adt88>(Adt92::C0(f44(match v1 {
                            Adt26::C0(_, h62, _, _) => {
                                h62
                            }
                        }, f45(Str::lit("item:"), f46(match v2 {
                            Adt1::C0(_, h63) => {
                                h63
                            }
                        })))))
                    } else {
                        Err::<Adt92, Adt88>(f47())
                    }
                }
                Some((_, _)) => {
                    Err::<Adt92, Adt88>(f47())
                }
            }
        }
    }
}

pub fn f19(v0: Adt18) -> R<Result<Adt36, Adt88>> {
    let m64 = f48(f14()?)?;
    match m64 {
        None => {
            f49(v0)
        }
        Some(v1) => {
            Ok(Err::<Adt36, Adt88>(v1))
        }
    }
}

pub fn f20(v0: Result<Adt93, Adt88>) -> Result<Adt36, Adt88> {
    let m65 = v0;
    match m65 {
        Ok(v1) => {
            let m66 = v1;
            match m66 {
                Adt93::C0(v2) => {
                    Ok::<Adt36, Adt88>(Adt36::C8(v2))
                }
                Adt93::C1(v3) => {
                    Ok::<Adt36, Adt88>(Adt36::C12(v3))
                }
                Adt93::C2(v4) => {
                    Err::<Adt36, Adt88>(Adt88::C15(v4))
                }
            }
        }
        Err(v5) => {
            Err::<Adt36, Adt88>(v5)
        }
    }
}

pub fn f21(v0: List<Adt1>, v1: Adt27) -> Result<Adt93, Adt88> {
    let m67 = v0;
    match m67.uncons() {
        None => {
            Ok::<Adt93, Adt88>(Adt93::C2(f43(5u64, match v1 {
                Adt27::C0(_, h68, _, _, _) => {
                    h68
                }
            })))
        }
        Some((v2, v3)) => {
            let m69 = v3;
            match m69.uncons() {
                None => {
                    let a71 = match v2.clone() {
                        Adt1::C0(h70, _) => {
                            h70
                        }
                    };
                    let a72 = 5u64;
                    if nat_eq(a71, a72) {
                        Ok::<Adt93, Adt88>(Adt93::C0(f50(f45(Str::lit("prompt:"), f46(match v2 {
                            Adt1::C0(_, h73) => {
                                h73
                            }
                        })))))
                    } else {
                        Err::<Adt93, Adt88>(f47())
                    }
                }
                Some((_, _)) => {
                    Err::<Adt93, Adt88>(f47())
                }
            }
        }
    }
}

pub fn f22(v0: Result<Adt94, Adt2>) -> R<bool> {
    let m74 = v0;
    match m74 {
        Ok(v2) => {
            Ok(f51(match v2 {
                Adt94::C0(_, h75, _, _, _, _, _) => {
                    h75
                }
            }, f52(Str::lit("byte-serve"))?))
        }
        Err(_) => {
            Ok(false)
        }
    }
}

pub fn f23(v0: Bytes) -> R<Result<Adt94, Adt2>> {
    let m76 = f7(v0.clone(), 0u64)?;
    match m76 {
        None => {
            Ok(Err::<Adt94, Adt2>(Adt2::C0(0u64)))
        }
        Some(v1) => {
            f53(v0.clone(), v1.clone(), f7(v0, match v1 {
                Adt90::C0(_, h77) => {
                    h77
                }
            })?)
        }
    }
}

pub fn f24(v0: Adt3, v1: Fn1, v2: Bytes) -> R<List<u64>> {
    let m78 = f23(v2)?;
    match m78 {
        Ok(v4) => {
            Ok(f54(f56(v0, v1, v4)?))
        }
        Err(_) => {
            Ok(f54(f55()))
        }
    }
}

pub fn f25(v0: Adt18) -> R<Result<Adt36, Adt88>> {
    f4(Adt17::C0(List::<Adt1>::nil(), v0))
}

pub fn f26(v0: Bytes, v1: Adt90, v2: Option<u64>) -> R<Result<Adt0, Adt2>> {
    let m79 = v2;
    match m79 {
        None => {
            Ok(Err::<Adt0, Adt2>(Adt2::C0(match v1 {
                Adt90::C0(_, h80) => {
                    h80
                }
            })))
        }
        Some(v3) => {
            let m84 = f57(v0.clone(), {
                let a82 = match v1.clone() {
                    Adt90::C0(_, h81) => {
                        h81
                    }
                };
                let a83 = 4u64;
                nat_add(a82, a83)?
            }, v3, List::<Adt1>::nil())?;
            match m84 {
                Ok(v5) => {
                    Ok(f58(v0, match v1 {
                        Adt90::C0(h85, _) => {
                            h85
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
    let a89 = {
        let a86 = v1;
        let a87 = 4u64;
        nat_add(a86, a87)?
    };
    let a90 = {
        let a88 = v0.clone();
        length_bytes(a88)
    };
    if nat_le(a89, a90) {
        Ok(Some({
            let a107 = {
                let a91 = f59(v0.clone(), v1)?;
                let a92 = 16777216u64;
                nat_mul(a91, a92)?
            };
            let a108 = {
                let a105 = {
                    let a95 = f59(v0.clone(), {
                        let a93 = v1;
                        let a94 = 1u64;
                        nat_add(a93, a94)?
                    })?;
                    let a96 = 65536u64;
                    nat_mul(a95, a96)?
                };
                let a106 = {
                    let a103 = {
                        let a99 = f59(v0.clone(), {
                            let a97 = v1;
                            let a98 = 2u64;
                            nat_add(a97, a98)?
                        })?;
                        let a100 = 256u64;
                        nat_mul(a99, a100)?
                    };
                    let a104 = f59(v0, {
                        let a101 = v1;
                        let a102 = 3u64;
                        nat_add(a101, a102)?
                    })?;
                    nat_add(a103, a104)?
                };
                nat_add(a105, a106)?
            };
            nat_add(a107, a108)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f28(v0: u64) -> List<u64> {
    List::cons({
        let a112 = {
            let a109 = v0;
            let a110 = 16777216u64;
            let a111 = 0u64;
            nat_quot(a109, a110, a111)
        };
        let a113 = 256u64;
        let a114 = 0u64;
        nat_rem(a112, a113, a114)
    }, List::cons({
        let a118 = {
            let a115 = v0;
            let a116 = 65536u64;
            let a117 = 0u64;
            nat_quot(a115, a116, a117)
        };
        let a119 = 256u64;
        let a120 = 0u64;
        nat_rem(a118, a119, a120)
    }, List::cons({
        let a124 = {
            let a121 = v0;
            let a122 = 256u64;
            let a123 = 0u64;
            nat_quot(a121, a122, a123)
        };
        let a125 = 256u64;
        let a126 = 0u64;
        nat_rem(a124, a125, a126)
    }, List::cons({
        let a127 = v0;
        let a128 = 256u64;
        let a129 = 0u64;
        nat_rem(a127, a128, a129)
    }, List::<u64>::nil()))))
}

pub fn f29(v0: Bytes) -> R<List<u64>> {
    f60(v0.clone(), {
        let a130 = v0;
        length_bytes(a130)
    }, List::<u64>::nil())
}

pub fn f30() -> Adt91 {
    Adt91::C0(f54(f55()))
}

pub fn f31(v0: Adt3, v1: Fn0, v2: Adt0, v3: Result<Adt94, Adt2>) -> R<Adt91> {
    let m131 = v3;
    match m131 {
        Ok(v5) => {
            f61(v0, Fn1::F62(v1, v2), v5)
        }
        Err(_) => {
            Ok(f30())
        }
    }
}

pub fn f32() -> Adt8 {
    Adt8::C0(List::<(Str, Adt9)>::nil())
}

pub fn f33(v0: List<Adt39>) -> R<Result<List<Adt15>, Adt96>> {
    let m132 = f63(v0.clone());
    match m132 {
        None => {
            f64(v0, List::<Adt15>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt15>, Adt96>(v1))
        }
    }
}

pub fn f34() -> R<List<Adt39>> {
    Ok(List::cons(f65(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f66()), List::cons(f65(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f67()), List::cons(f65(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f66()), List::cons(f65(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f68()), List::cons(f65(Str::lit("test_image_content"), Str::lit("Tests image content response"), f68()), List::cons(f65(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f68()), List::cons(f65(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f68()), List::cons(f65(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f68()), List::cons(f65(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f68()), List::cons(f65(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f68()), List::cons(f65(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f69()), List::cons(f65(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f70()?), List::cons(f65(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f71()), List::cons(f72(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f68(), f73()), List::cons(f72(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f68(), f73()), List::cons(f65(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f68()), List::cons(f65(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f68()), List::cons(f65(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f68()), List::cons(f65(Str::lit("test_streaming_elicitation"), Str::lit("Diagnostic tool validating response progress streams"), f68()), List::cons(f65(Str::lit("test_logging_tool"), Str::lit("Diagnostic logging validator tool"), f68()), List::<Adt39>::nil())))))))))))))))))))))
}

pub fn f35() -> List<Adt39> {
    List::cons(f74(Str::lit("test_input_required_result_elicitation"), Str::lit("MRTR: returns InputRequiredResult with elicitation request")), List::cons(f74(Str::lit("test_input_required_result_sampling"), Str::lit("MRTR: returns InputRequiredResult with sampling request")), List::cons(f74(Str::lit("test_input_required_result_list_roots"), Str::lit("MRTR: returns InputRequiredResult with roots/list request")), List::cons(f74(Str::lit("test_input_required_result_request_state"), Str::lit("MRTR: returns InputRequiredResult with requestState")), List::cons(f74(Str::lit("test_input_required_result_multiple_inputs"), Str::lit("MRTR: returns InputRequiredResult with multiple input requests")), List::cons(f74(Str::lit("test_input_required_result_multi_round"), Str::lit("MRTR: multi-round InputRequiredResult workflow")), List::cons(f74(Str::lit("test_input_required_result_tampered_state"), Str::lit("MRTR: HMAC-signed requestState integrity test")), List::cons(f74(Str::lit("test_input_required_result_capabilities"), Str::lit("MRTR: respects client capabilities in inputRequests")), List::<Adt39>::nil()))))))))
}

pub fn f36(v0: Adt25) -> bool {
    if f40(v0.clone(), Str::lit("effect_read")) {
        true
    } else {
        if f40(v0.clone(), Str::lit("effect_chain")) {
            true
        } else {
            if f40(v0.clone(), Str::lit("effect_edge_ok")) {
                true
            } else {
                f40(v0, Str::lit("effect_edge_over"))
            }
        }
    }
}

pub fn f37(v0: List<Adt39>, v1: Fn2, v2: Adt25) -> R<Result<Adt36, Adt88>> {
    let m134 = f75(match v2.clone() {
        Adt25::C0(_, h133, _, _, _) => {
            h133
        }
    }, v0);
    match m134 {
        None => {
            Ok(Err::<Adt36, Adt88>(Adt88::C0(match v2 {
                Adt25::C0(_, h135, _, _, _) => {
                    h135
                }
            })))
        }
        Some(v3) => {
            f76(v3, v1, v2)
        }
    }
}

pub fn f38() -> R<List<Adt39>> {
    Ok(List::cons(f65(Str::lit("effect_read"), Str::lit("Reads a key through the host"), f77()?), List::cons(f65(Str::lit("effect_chain"), Str::lit("Reads two keys through the host, the second after the first"), f78()?), List::cons(f65(Str::lit("effect_edge_ok"), Str::lit("Asks for the largest effect tag"), f66()), List::cons(f65(Str::lit("effect_edge_over"), Str::lit("Asks for a tag that does not fit thirty-two bits"), f66()), List::<Adt39>::nil())))))
}

pub fn f39(v0: List<Adt1>, v1: Adt25) -> Result<Adt53, Adt88> {
    f79(v0, v1)
}

pub fn f40(v0: Adt25, v1: Str) -> bool {
    let a137 = match v0 {
        Adt25::C0(_, h136, _, _, _) => {
            h136
        }
    };
    let a138 = v1;
    equal(a137, a138)
}

pub fn f41(v0: Result<Adt97, Adt88>) -> Result<Adt36, Adt88> {
    let m139 = v0;
    match m139 {
        Ok(v1) => {
            let m140 = v1;
            match m140 {
                Adt97::C0(v2) => {
                    Ok::<Adt36, Adt88>(Adt36::C2(v2))
                }
                Adt97::C1(v3) => {
                    Ok::<Adt36, Adt88>(Adt36::C12(v3))
                }
                Adt97::C2(v4) => {
                    Ok::<Adt36, Adt88>(Adt36::C13(v4))
                }
                Adt97::C3(v5) => {
                    Err::<Adt36, Adt88>(Adt88::C15(v5))
                }
            }
        }
        Err(v6) => {
            Err::<Adt36, Adt88>(v6)
        }
    }
}

pub fn f42(v0: List<Adt1>) -> Result<Adt97, Adt88> {
    let m141 = v0;
    match m141.uncons() {
        None => {
            Ok::<Adt97, Adt88>(Adt97::C3(f43(3u64, Str::lit("call"))))
        }
        Some((v1, v2)) => {
            let m142 = v2;
            match m142.uncons() {
                None => {
                    let a144 = match v1.clone() {
                        Adt1::C0(h143, _) => {
                            h143
                        }
                    };
                    let a145 = 3u64;
                    if nat_eq(a144, a145) {
                        Ok::<Adt97, Adt88>(Adt97::C0(f80(f45(Str::lit("call:"), f46(match v1 {
                            Adt1::C0(_, h146) => {
                                h146
                            }
                        })), false)))
                    } else {
                        Err::<Adt97, Adt88>(f47())
                    }
                }
                Some((_, _)) => {
                    Err::<Adt97, Adt88>(f47())
                }
            }
        }
    }
}

pub fn f43(v0: u64, v1: Str) -> Adt89 {
    Adt89::C0(v0, {
        let a147 = v1;
        utf8_encode(a147)
    })
}

pub fn f44(v0: Str, v1: Str) -> Adt56 {
    Adt56::C0(f12(), List::cons(Adt50::C0(Adt51::C0(v0, Some(Str::lit("text/plain")), v1, None::<Adt8>)), List::<Adt50>::nil()))
}

pub fn f45(v0: Str, v1: Str) -> Str {
    let a148 = List::cons(v0, List::cons(v1, List::<Str>::nil()));
    let a149 = Str::lit("");
    join(a148, a149)
}

pub fn f46(v0: Bytes) -> Str {
    let m151 = {
        let a150 = v0;
        utf8_decode(a150)
    };
    match m151 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f47() -> Adt88 {
    Adt88::C9(Str::lit("the outcomes of the effects do not match the request"))
}

pub fn f48(v0: List<Adt39>) -> R<Option<Adt88>> {
    let m152 = f33(v0)?;
    match m152 {
        Ok(_) => {
            Ok(None::<Adt88>)
        }
        Err(v1) => {
            Ok(Some(Adt88::C9(f81(v1))))
        }
    }
}

pub fn f49(v0: Adt18) -> R<Result<Adt36, Adt88>> {
    let m153 = v0;
    match m153 {
        Adt18::C0(_) => {
            Ok(Err::<Adt36, Adt88>(Adt88::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt18::C1(v1) => {
            Ok(f82(f12(), f14()?, v1))
        }
        Adt18::C2(v2) => {
            if f83(match v2.clone() {
                Adt25::C0(_, h154, _, _, _) => {
                    h154
                }
            }) {
                f84(f14()?, Fn3::F85, v2)
            } else {
                f37(f14()?, Fn2::F86, v2)
            }
        }
        Adt18::C3(v4) => {
            Ok(f87(v4))
        }
        Adt18::C4(v5) => {
            f88(v5)
        }
        Adt18::C5(v6) => {
            Ok(f89(v6))
        }
        Adt18::C6(v7) => {
            Ok(f90(v7))
        }
        Adt18::C7(v8) => {
            if f91(match v8.clone() {
                Adt27::C0(_, h155, _, _, _) => {
                    h155
                }
            }) {
                Ok(f92(v8))
            } else {
                Ok(f93(v8))
            }
        }
        Adt18::C8(v9) => {
            f94(v9)
        }
        Adt18::C9(_) => {
            Ok(Err::<Adt36, Adt88>(Adt88::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt18::C10(_) => {
            Ok(Err::<Adt36, Adt88>(Adt88::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt18::C11(_) => {
            Ok(Err::<Adt36, Adt88>(Adt88::C9(Str::lit("the fixture offers tools only"))))
        }
    }
}

pub fn f50(v0: Str) -> Adt62 {
    Adt62::C0(None::<Str>, List::cons(Adt63::C0(Adt45::C0, f95(v0)), List::<Adt63>::nil()))
}

pub fn f51(p162: List<u64>, p163: List<u64>) -> bool {
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

pub fn f52(v0: Str) -> R<List<u64>> {
    f29({
        let a164 = v0;
        utf8_encode(a164)
    })
}

pub fn f53(v0: Bytes, v1: Adt90, v2: Option<Adt90>) -> R<Result<Adt94, Adt2>> {
    let m165 = v2;
    match m165 {
        None => {
            Ok(Err::<Adt94, Adt2>(Adt2::C0(match v1 {
                Adt90::C0(_, h166) => {
                    h166
                }
            })))
        }
        Some(v3) => {
            f96(v0.clone(), v1, v3.clone(), f27(v0, match v3 {
                Adt90::C0(_, h167) => {
                    h167
                }
            })?)
        }
    }
}

pub fn f54(v0: Adt98) -> List<u64> {
    let a188 = f28(match v0.clone() {
        Adt98::C0(h168, _, _, _) => {
            h168
        }
    });
    let a189 = {
        let a186 = f28({
            let a170 = match v0.clone() {
                Adt98::C0(_, h169, _, _) => {
                    h169
                }
            };
            length_list(a170)
        });
        let a187 = {
            let a184 = f97(match v0.clone() {
                Adt98::C0(_, h171, _, _) => {
                    h171
                }
            });
            let a185 = {
                let a182 = f28({
                    let a173 = match v0.clone() {
                        Adt98::C0(_, _, h172, _) => {
                            h172
                        }
                    };
                    length_list(a173)
                });
                let a183 = {
                    let a180 = f98(match v0.clone() {
                        Adt98::C0(_, _, h174, _) => {
                            h174
                        }
                    });
                    let a181 = {
                        let a178 = f28({
                            let a176 = match v0.clone() {
                                Adt98::C0(_, _, _, h175) => {
                                    h175
                                }
                            };
                            length_list(a176)
                        });
                        let a179 = match v0 {
                            Adt98::C0(_, _, _, h177) => {
                                h177
                            }
                        };
                        append_list(a178, a179)
                    };
                    append_list(a180, a181)
                };
                append_list(a182, a183)
            };
            append_list(a184, a185)
        };
        append_list(a186, a187)
    };
    append_list(a188, a189)
}

pub fn f55() -> Adt98 {
    Adt98::C0(f99(), f100(), List::<List<u64>>::nil(), f101())
}

pub fn f56(v0: Adt3, v1: Fn1, v2: Adt94) -> R<Adt98> {
    f102(v0, v1, v2.clone(), f103(match v2.clone() {
        Adt94::C0(_, _, h190, _, _, _, _) => {
            h190
        }
    }, match v2 {
        Adt94::C0(_, _, _, _, _, h191, _) => {
            h191
        }
    })?)
}

pub fn f57(v0: Bytes, p202: u64, p203: u64, p204: List<Adt1>) -> R<Result<(List<Adt1>, u64), Adt2>> {
    let m201 = (p202, (p203, p204));
    {
        let mut m201 = m201;
        loop {
            let step = {
                let (v1, (v2, v3)) = m201;
                let m192 = v2;
                if m192 == 0 {
                    Err::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>(Ok(Ok::<(List<Adt1>, u64), Adt2>((f104(v3, List::<Adt1>::nil()), v1))))
                } else {
                    let v4 = m192 - 1;
                    let m193 = f27(v0.clone(), v1)?;
                    match m193 {
                        None => {
                            Err::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>(Ok(Err::<(List<Adt1>, u64), Adt2>(Adt2::C0(v1))))
                        }
                        Some(v5) => {
                            let m196 = f7(v0.clone(), {
                                let a194 = v1;
                                let a195 = 4u64;
                                nat_add(a194, a195)?
                            })?;
                            match m196 {
                                None => {
                                    Err::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>(Ok(Err::<(List<Adt1>, u64), Adt2>(Adt2::C0({
                                        let a197 = v1;
                                        let a198 = 4u64;
                                        nat_add(a197, a198)?
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>((match v6.clone() {
                                        Adt90::C0(_, h199) => {
                                            h199
                                        }
                                    }, (v4, List::cons(Adt1::C0(v5, match v6 {
                                        Adt90::C0(h200, _) => {
                                            h200
                                        }
                                    }), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m201 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Bytes, v1: Bytes, v2: (List<Adt1>, u64)) -> Result<Adt0, Adt2> {
    f105({
        let a207 = {
            let (_, h205) = v2.clone();
            h205
        };
        let a208 = {
            let a206 = v0;
            length_bytes(a206)
        };
        nat_eq(a207, a208)
    }, v1, {
        let (h209, _) = v2;
        h209
    }, {
        let (_, h210) = v2;
        h210
    })
}

pub fn f59(v0: Bytes, v1: u64) -> R<u64> {
    let m213 = {
        let a211 = v0;
        let a212 = v1;
        index_bytes(a211, a212)
    };
    match m213 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a258 = {
                let a234 = {
                    let a222 = {
                        let a216 = {
                            let a214 = v3;
                            let a215 = 128u8;
                            fixed_u8::bit_and(a214, a215)
                        };
                        let a217 = 128u8;
                        if equal(a216, a217) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a223 = {
                        let a220 = {
                            let a218 = v3;
                            let a219 = 64u8;
                            fixed_u8::bit_and(a218, a219)
                        };
                        let a221 = 64u8;
                        if equal(a220, a221) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a222, a223)?
                };
                let a235 = {
                    let a232 = {
                        let a226 = {
                            let a224 = v3;
                            let a225 = 32u8;
                            fixed_u8::bit_and(a224, a225)
                        };
                        let a227 = 32u8;
                        if equal(a226, a227) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a233 = {
                        let a230 = {
                            let a228 = v3;
                            let a229 = 16u8;
                            fixed_u8::bit_and(a228, a229)
                        };
                        let a231 = 16u8;
                        if equal(a230, a231) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a232, a233)?
                };
                nat_add(a234, a235)?
            };
            let a259 = {
                let a256 = {
                    let a244 = {
                        let a238 = {
                            let a236 = v3;
                            let a237 = 8u8;
                            fixed_u8::bit_and(a236, a237)
                        };
                        let a239 = 8u8;
                        if equal(a238, a239) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a245 = {
                        let a242 = {
                            let a240 = v3;
                            let a241 = 4u8;
                            fixed_u8::bit_and(a240, a241)
                        };
                        let a243 = 4u8;
                        if equal(a242, a243) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a244, a245)?
                };
                let a257 = {
                    let a254 = {
                        let a248 = {
                            let a246 = v3;
                            let a247 = 2u8;
                            fixed_u8::bit_and(a246, a247)
                        };
                        let a249 = 2u8;
                        if equal(a248, a249) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a255 = {
                        let a252 = {
                            let a250 = v3;
                            let a251 = 1u8;
                            fixed_u8::bit_and(a250, a251)
                        };
                        let a253 = 1u8;
                        if equal(a252, a253) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a254, a255)?
                };
                nat_add(a256, a257)?
            };
            nat_add(a258, a259)
        }
    }
}

pub fn f60(v0: Bytes, p262: u64, p263: List<u64>) -> R<List<u64>> {
    let m261 = (p262, p263);
    {
        let mut m261 = m261;
        loop {
            let step = {
                let (v1, v2) = m261;
                let m260 = v1;
                if m260 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m260 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f59(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m261 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Adt3, v1: Fn1, v2: Adt94) -> R<Adt91> {
    f106(v0, v1, v2.clone(), f103(match v2.clone() {
        Adt94::C0(_, _, h264, _, _, _, _) => {
            h264
        }
    }, match v2 {
        Adt94::C0(_, _, _, _, _, h265, _) => {
            h265
        }
    })?)
}

pub fn f62(v0: Fn0, v1: Adt0, v2: Adt18) -> R<Result<Adt36, Adt88>> {
    let c266: Fn0 = v0;
    c266.apply(Adt17::C0(match v1 {
        Adt0::C0(_, h267) => {
            h267
        }
    }, v2))
}

pub fn f63(v0: List<Adt39>) -> Option<Adt96> {
    let m268 = f107(v0, List::<Str>::nil());
    match m268 {
        None => {
            None::<Adt96>
        }
        Some(v1) => {
            Some(Adt96::C1(v1))
        }
    }
}

pub fn f64(p272: List<Adt39>, p273: List<Adt15>) -> R<Result<List<Adt15>, Adt96>> {
    let m271 = (p272, p273);
    {
        let mut m271 = m271;
        loop {
            let step = {
                let (v0, v1) = m271;
                let m269 = v0;
                match m269.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt15>), R<Result<List<Adt15>, Adt96>>>(Ok(Ok::<List<Adt15>, Adt96>(f108(v1, List::<Adt15>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m270 = f109(v2)?;
                        match m270 {
                            Ok(v5) => {
                                Ok::<(List<Adt39>, List<Adt15>), R<Result<List<Adt15>, Adt96>>>((v3, f108(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt39>, List<Adt15>), R<Result<List<Adt15>, Adt96>>>(Ok(Err::<List<Adt15>, Adt96>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m271 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Str, v1: Str, v2: Adt8) -> Adt39 {
    Adt39::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), v2, None::<Adt8>, None::<Adt40>, None::<Adt8>)
}

pub fn f66() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::<(Str, Adt9)>::nil())), List::cons((Str::lit("required"), Adt9::C4(List::<Adt9>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f67() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("region"), f110(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f111()), List::cons((Str::lit("limit"), f110(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f110(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt9)>::nil())))))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("region")), List::cons(Adt9::C3(Str::lit("query")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil()))))
}

pub fn f68() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil())))
}

pub fn f69() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("$schema"), Adt9::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt9::C5(List::cons((Str::lit("address"), Adt9::C5(List::cons((Str::lit("$anchor"), Adt9::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("street"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("city"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("name"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("address"), Adt9::C5(List::cons((Str::lit("$ref"), Adt9::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("contactMethod"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt9::C4(List::cons(Adt9::C3(Str::lit("phone")), List::cons(Adt9::C3(Str::lit("email")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("phone"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("email"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))))))), List::cons((Str::lit("allOf"), Adt9::C4(List::cons(Adt9::C5(List::cons((Str::lit("anyOf"), Adt9::C4(List::cons(Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("phone")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())), List::cons(Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("email")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil())), List::<Adt9>::nil()))), List::cons((Str::lit("if"), Adt9::C5(List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("contactMethod"), Adt9::C5(List::cons((Str::lit("const"), Adt9::C3(Str::lit("phone"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("contactMethod")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("then"), Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("phone")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("else"), Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("email")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))))))
}

pub fn f70() -> R<Adt8> {
    Ok(Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("name"), f111()), List::cons((Str::lit("label"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f112(1u64)?), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("count"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f112(0u64)?), List::cons((Str::lit("maximum"), f112(10u64)?), List::<(Str, Adt9)>::nil()))))), List::cons((Str::lit("mode"), Adt9::C5(List::cons((Str::lit("enum"), Adt9::C4(List::cons(Adt9::C3(Str::lit("fast")), List::cons(Adt9::C3(Str::lit("slow")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("tags"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("maxItems"), f112(3u64)?), List::<(Str, Adt9)>::nil()))))), List::cons((Str::lit("nested"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("flag"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("boolean"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))))))))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("name")), List::<Adt9>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))
}

pub fn f71() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("target"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("zone"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("x-mcp-header"), Adt9::C3(Str::lit("Zone"))), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::<Adt9>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f72(v0: Str, v1: Str, v2: Adt8, v3: Adt8) -> Adt39 {
    Adt39::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), v2, Some(v3), None::<Adt40>, None::<Adt8>)
}

pub fn f73() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("sum"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("label"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("sum")), List::cons(Adt9::C3(Str::lit("label")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil()))))
}

pub fn f74(v0: Str, v1: Str) -> Adt39 {
    Adt39::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::<(Str, Adt9)>::nil())), List::<(Str, Adt9)>::nil()))), None::<Adt8>, None::<Adt40>, None::<Adt8>)
}

pub fn f75(v0: Str, p279: List<Adt39>) -> Option<Adt39> {
    let m278 = p279;
    {
        let mut m278 = m278;
        loop {
            let step = {
                let v1 = m278;
                let m274 = v1;
                match m274.uncons() {
                    None => {
                        Err::<List<Adt39>, Option<Adt39>>(None::<Adt39>)
                    }
                    Some((v2, v3)) => {
                        let a276 = match v2.clone() {
                            Adt39::C0(h275, _, _, _, _, _, _, _) => {
                                h275
                            }
                        };
                        let a277 = v0.clone();
                        if equal(a276, a277) {
                            Err::<List<Adt39>, Option<Adt39>>(Some(v2))
                        } else {
                            Ok::<List<Adt39>, Option<Adt39>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: Adt39, v1: Fn2, v2: Adt25) -> R<Result<Adt36, Adt88>> {
    f113(v0.clone(), v2, Fn4::F114(v1, v0))
}

pub fn f77() -> R<Adt8> {
    Ok(Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("key"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f112(1u64)?), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("key")), List::<Adt9>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))
}

pub fn f78() -> R<Adt8> {
    Ok(Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("first"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f112(1u64)?), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("second"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f112(1u64)?), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("first")), List::cons(Adt9::C3(Str::lit("second")), List::<Adt9>::nil())))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))
}

pub fn f79(v0: List<Adt1>, v1: Adt25) -> Result<Adt53, Adt88> {
    if f40(v1.clone(), Str::lit("effect_read")) {
        f115(v0, v1)
    } else {
        if f40(v1.clone(), Str::lit("effect_chain")) {
            f116(v0, v1)
        } else {
            if f40(v1, Str::lit("effect_edge_ok")) {
                f117(v0, 4294967295u64)
            } else {
                f117(v0, 4294967296u64)
            }
        }
    }
}

pub fn f80(v0: Str, v1: bool) -> Adt41 {
    Adt41::C0(List::cons(Adt42::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>)), List::<Adt42>::nil()), None::<Adt9>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f81(v0: Adt96) -> Str {
    let m280 = v0;
    match m280 {
        Adt96::C0(v1) => {
            let a281 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a282 = Str::lit(" ");
            join(a281, a282)
        }
        Adt96::C1(v2) => {
            let a283 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a284 = Str::lit(" ");
            join(a283, a284)
        }
        Adt96::C2(v3) => {
            let a285 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a286 = Str::lit(" ");
            join(a285, a286)
        }
        Adt96::C3(v4) => {
            let a287 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a288 = Str::lit(" ");
            join(a287, a288)
        }
        Adt96::C4(v5, v6) => {
            let a289 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a290 = Str::lit(" ");
            join(a289, a290)
        }
        Adt96::C5(v7, v8) => {
            let a291 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a292 = Str::lit(" ");
            join(a291, a292)
        }
        Adt96::C6(v9, v10) => {
            let a293 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a294 = Str::lit(" ");
            join(a293, a294)
        }
        Adt96::C7(v11) => {
            let a295 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a296 = Str::lit(" ");
            join(a295, a296)
        }
        Adt96::C8(v12) => {
            let a297 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a298 = Str::lit(" ");
            join(a297, a298)
        }
        Adt96::C9(v13) => {
            let a299 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a300 = Str::lit(" ");
            join(a299, a300)
        }
        Adt96::C10(v14) => {
            let a301 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a302 = Str::lit(" ");
            join(a301, a302)
        }
        Adt96::C11(v15) => {
            let a303 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a304 = Str::lit(" ");
            join(a303, a304)
        }
        Adt96::C12(v16, v17) => {
            let a305 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a306 = Str::lit(" ");
            join(a305, a306)
        }
    }
}

pub fn f82(v0: Adt13, v1: List<Adt39>, v2: Adt24) -> Result<Adt36, Adt88> {
    let m308 = match v2 {
        Adt24::C0(_, h307) => {
            h307
        }
    };
    match m308 {
        None => {
            Ok::<Adt36, Adt88>(Adt36::C1(Adt38::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt36, Adt88>(Adt88::C5(v3))
        }
    }
}

pub fn f83(v0: Str) -> bool {
    let a309 = v0.clone();
    let a310 = Str::lit("test_input_required_result_elicitation");
    if equal(a309, a310) {
        true
    } else {
        let a311 = v0.clone();
        let a312 = Str::lit("test_input_required_result_sampling");
        if equal(a311, a312) {
            true
        } else {
            let a313 = v0.clone();
            let a314 = Str::lit("test_input_required_result_list_roots");
            if equal(a313, a314) {
                true
            } else {
                let a315 = v0.clone();
                let a316 = Str::lit("test_input_required_result_request_state");
                if equal(a315, a316) {
                    true
                } else {
                    let a317 = v0.clone();
                    let a318 = Str::lit("test_input_required_result_multiple_inputs");
                    if equal(a317, a318) {
                        true
                    } else {
                        let a319 = v0.clone();
                        let a320 = Str::lit("test_input_required_result_multi_round");
                        if equal(a319, a320) {
                            true
                        } else {
                            let a321 = v0.clone();
                            let a322 = Str::lit("test_input_required_result_tampered_state");
                            if equal(a321, a322) {
                                true
                            } else {
                                let a323 = v0;
                                let a324 = Str::lit("test_input_required_result_capabilities");
                                equal(a323, a324)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f84(v0: List<Adt39>, v1: Fn3, v2: Adt25) -> R<Result<Adt36, Adt88>> {
    let m326 = f75(match v2.clone() {
        Adt25::C0(_, h325, _, _, _) => {
            h325
        }
    }, v0);
    match m326 {
        None => {
            Ok(Err::<Adt36, Adt88>(Adt88::C0(match v2 {
                Adt25::C0(_, h327, _, _, _) => {
                    h327
                }
            })))
        }
        Some(v3) => {
            f113(v3.clone(), v2, Fn4::F118(v1, v3))
        }
    }
}

pub fn f85(v0: Adt25) -> R<Result<Adt99, Adt88>> {
    if f119(v0.clone(), Str::lit("test_input_required_result_elicitation")) {
        Ok(f120(v0))
    } else {
        if f119(v0.clone(), Str::lit("test_input_required_result_sampling")) {
            Ok(f121(v0))
        } else {
            if f119(v0.clone(), Str::lit("test_input_required_result_list_roots")) {
                f122(v0)
            } else {
                if f119(v0.clone(), Str::lit("test_input_required_result_request_state")) {
                    Ok(f123(v0))
                } else {
                    if f119(v0.clone(), Str::lit("test_input_required_result_multiple_inputs")) {
                        f124(v0)
                    } else {
                        if f119(v0.clone(), Str::lit("test_input_required_result_multi_round")) {
                            f125(v0)
                        } else {
                            if f119(v0.clone(), Str::lit("test_input_required_result_tampered_state")) {
                                Ok(f126(v0))
                            } else {
                                if f119(v0.clone(), Str::lit("test_input_required_result_capabilities")) {
                                    Ok(f127(v0))
                                } else {
                                    Ok(Err::<Adt99, Adt88>(Adt88::C9(Str::lit("not an input-required tool"))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f86(v0: Adt25) -> R<Result<Adt53, Adt88>> {
    if f40(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt53, Adt88>(f128(List::cons(f95(Str::lit("This is a simple text response for testing.")), List::<Adt42>::nil()))))
    } else {
        if f40(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt53, Adt88>(f128(List::cons(f129(), List::<Adt42>::nil()))))
        } else {
            if f40(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt53, Adt88>(f128(List::cons(f130(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt42>::nil()))))
            } else {
                if f40(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt53, Adt88>(f128(List::cons(f131(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt42>::nil()))))
                } else {
                    if f40(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt53, Adt88>(f128(List::cons(f95(Str::lit("Multiple content types test:")), List::cons(f129(), List::cons(f131(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt42>::nil()))))))
                    } else {
                        if f40(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt53, Adt88>(f132(List::cons(f95(Str::lit("This tool intentionally returns an error for testing")), List::<Adt42>::nil()), None::<Adt9>, true, List::<Adt54>::nil())))
                        } else {
                            if f40(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt53, Adt88>(f133(v0, f134()?)))
                            } else {
                                if f40(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt53, Adt88>(f133(v0, f135()?)))
                                } else {
                                    if f40(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt53, Adt88>(f132(List::cons(f95(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt42>::nil()), Some(f136()?), false, List::<Adt54>::nil())))
                                    } else {
                                        if f40(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt53, Adt88>(f132(List::cons(f95(Str::lit("{\"sum\":\"three\"}")), List::<Adt42>::nil()), Some(f137()), false, List::<Adt54>::nil())))
                                        } else {
                                            if f40(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt53, Adt88>(f128(List::cons(f138(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt42>::nil()))))
                                            } else {
                                                if f40(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt53, Adt88>(f128(List::cons(f95(f139()), List::<Adt42>::nil()))))
                                                } else {
                                                    if f40(v0.clone(), Str::lit("test_streaming_elicitation")) {
                                                        Ok(Ok::<Adt53, Adt88>(f128(List::cons(f95(Str::lit("Streaming complete")), List::<Adt42>::nil()))))
                                                    } else {
                                                        if f40(v0.clone(), Str::lit("test_logging_tool")) {
                                                            Ok(Ok::<Adt53, Adt88>(f128(List::cons(f95(Str::lit("Logging evaluated")), List::<Adt42>::nil()))))
                                                        } else {
                                                            Ok(f140(v0))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f87(v0: Adt24) -> Result<Adt36, Adt88> {
    f141(f142(), f143(), v0)
}

pub fn f88(v0: Adt26) -> R<Result<Adt36, Adt88>> {
    f144(f145(), Fn5::F146, v0)
}

pub fn f89(v0: Adt24) -> Result<Adt36, Adt88> {
    f147(f142(), f148(), v0)
}

pub fn f90(v0: Adt24) -> Result<Adt36, Adt88> {
    f149(f142(), f150(), v0)
}

pub fn f91(v0: Str) -> bool {
    let a328 = v0;
    let a329 = f151();
    equal(a328, a329)
}

pub fn f92(v0: Adt27) -> Result<Adt36, Adt88> {
    let m331 = f152(Str::lit("user_context"), Adt100::C0, match v0 {
        Adt27::C0(_, _, _, h330, _) => {
            h330
        }
    });
    match m331 {
        Adt102::C0 => {
            Ok::<Adt36, Adt88>(Adt36::C12(Adt85::C0(f155(Str::lit("user_context"), f156(Str::lit("What context should the prompt use?"), Str::lit("context"), Str::lit("string"))))))
        }
        Adt102::C1 => {
            Err::<Adt36, Adt88>(Adt88::C11)
        }
        Adt102::C2(v1) => {
            Ok::<Adt36, Adt88>(Adt36::C8(f153(f154(v1, Str::lit("context"), Str::lit("unknown")))))
        }
    }
}

pub fn f93(v0: Adt27) -> Result<Adt36, Adt88> {
    f157(f150(), Fn6::F158, v0)
}

pub fn f94(v0: Adt28) -> R<Result<Adt36, Adt88>> {
    f159(Fn7::F160, v0)
}

pub fn f95(v0: Str) -> Adt42 {
    Adt42::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>))
}

pub fn f96(v0: Bytes, v1: Adt90, v2: Adt90, v3: Option<u64>) -> R<Result<Adt94, Adt2>> {
    let m332 = v3;
    match m332 {
        None => {
            Ok(Err::<Adt94, Adt2>(Adt2::C0(match v2 {
                Adt90::C0(_, h333) => {
                    h333
                }
            })))
        }
        Some(v4) => {
            let m337 = f161(v0.clone(), {
                let a335 = match v2.clone() {
                    Adt90::C0(_, h334) => {
                        h334
                    }
                };
                let a336 = 4u64;
                nat_add(a335, a336)?
            }, v4, List::<Adt95>::nil())?;
            match m337 {
                Ok(v6) => {
                    f162(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt94, Adt2>(v5))
                }
            }
        }
    }
}

pub fn f97(v0: List<Adt95>) -> List<u64> {
    f163(f164(v0, List::<Adt95>::nil()), List::<u64>::nil())
}

pub fn f98(v0: List<List<u64>>) -> List<u64> {
    f165(f166(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f99() -> u64 {
    500u64
}

pub fn f100() -> List<Adt95> {
    List::cons(Adt95::C0(f167(), f168()), List::<Adt95>::nil())
}

pub fn f101() -> List<u64> {
    let a340 = f169();
    let a341 = {
        let a338 = f170();
        let a339 = f171();
        append_list(a338, a339)
    };
    append_list(a340, a341)
}

pub fn f102(v0: Adt3, v1: Fn1, v2: Adt94, v3: Option<Adt101>) -> R<Adt98> {
    let m342 = v3;
    match m342 {
        None => {
            f173(v0, v1, v2)
        }
        Some(v4) => {
            f172(v4)
        }
    }
}

pub fn f103(v0: List<Adt95>, v1: List<List<u64>>) -> R<Option<Adt101>> {
    let m343 = f174(v0.clone(), v1.clone())?;
    match m343 {
        None => {
            f175(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f104(p346: List<Adt1>, p347: List<Adt1>) -> List<Adt1> {
    let m345 = (p346, p347);
    {
        let mut m345 = m345;
        loop {
            let step = {
                let (v0, v1) = m345;
                let m344 = v0;
                match m344.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m345 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: bool, v1: Bytes, v2: List<Adt1>, v3: u64) -> Result<Adt0, Adt2> {
    if v0 {
        Ok::<Adt0, Adt2>(Adt0::C0(v1, v2))
    } else {
        Err::<Adt0, Adt2>(Adt2::C1(v3))
    }
}

pub fn f106(v0: Adt3, v1: Fn1, v2: Adt94, v3: Option<Adt101>) -> R<Adt91> {
    let m348 = v3;
    match m348 {
        None => {
            f176(v0, v1, v2)
        }
        Some(v4) => {
            Ok(Adt91::C0(f54(f172(v4)?)))
        }
    }
}

pub fn f107(p354: List<Adt39>, p355: List<Str>) -> Option<Str> {
    let m353 = (p354, p355);
    {
        let mut m353 = m353;
        loop {
            let step = {
                let (v0, v1) = m353;
                let m349 = v0;
                match m349.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f177(match v2.clone() {
                            Adt39::C0(h350, _, _, _, _, _, _, _) => {
                                h350
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt39>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt39::C0(h351, _, _, _, _, _, _, _) => {
                                    h351
                                }
                            }))
                        } else {
                            Ok::<(List<Adt39>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt39::C0(h352, _, _, _, _, _, _, _) => {
                                    h352
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m353 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(p358: List<Adt15>, p359: List<Adt15>) -> List<Adt15> {
    let m357 = (p358, p359);
    {
        let mut m357 = m357;
        loop {
            let step = {
                let (v0, v1) = m357;
                let m356 = v0;
                match m356.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt15>), List<Adt15>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt15>, List<Adt15>), List<Adt15>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m357 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: Adt39) -> R<Result<List<Adt15>, Adt96>> {
    let m362 = f178(match v0.clone() {
        Adt39::C0(h360, _, _, _, _, _, _, _) => {
            h360
        }
    }, f179(match v0.clone() {
        Adt39::C0(h361, _, _, _, _, _, _, _) => {
            h361
        }
    })?);
    match m362 {
        None => {
            f180(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt15>, Adt96>(v1))
        }
    }
}

pub fn f110(v0: Str, v1: Str) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil())))
}

pub fn f111() -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))
}

pub fn f112(v0: u64) -> R<Adt9> {
    Ok(Adt9::C2(Adt10::C0(f181(v0, 0i64)?, 0i64)))
}

pub fn f113(v0: Adt39, v1: Adt25, v2: Fn4) -> R<Result<Adt36, Adt88>> {
    let m369 = f182(Adt9::C5({
        let m365 = match v0.clone() {
            Adt39::C0(_, _, _, _, h364, _, _, _) => {
                h364
            }
        };
        match m365 {
            Adt8::C0(h363) => {
                h363
            }
        }
    }), Adt9::C5({
        let m368 = match v0 {
            Adt39::C0(_, _, _, _, h367, _, _, _) => {
                h367
            }
        };
        match m368 {
            Adt8::C0(h366) => {
                h366
            }
        }
    }), f183(v1.clone()))?;
    match m369 {
        Adt105::C0 => {
            let c370: Fn4 = v2;
            c370.apply(v1)
        }
        Adt105::C1(v3, v4) => {
            Ok(Ok::<Adt36, Adt88>(Adt36::C2(f80(f184(v3, v4)?, true))))
        }
        Adt105::C2(_) => {
            Ok(Err::<Adt36, Adt88>(Adt88::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt105::C3(_) => {
            Ok(Err::<Adt36, Adt88>(Adt88::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt105::C4 => {
            Ok(Err::<Adt36, Adt88>(Adt88::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f114(v0: Fn2, v1: Adt39, v2: Adt25) -> R<Result<Adt36, Adt88>> {
    f185(v1, v0, v2)
}

pub fn f115(v0: List<Adt1>, v1: Adt25) -> Result<Adt53, Adt88> {
    let m371 = v0;
    match m371.uncons() {
        None => {
            f186(1u64, f187(Str::lit("key"), v1))
        }
        Some((v2, v3)) => {
            let m372 = v3;
            match m372.uncons() {
                None => {
                    let a374 = match v2.clone() {
                        Adt1::C0(h373, _) => {
                            h373
                        }
                    };
                    let a375 = 1u64;
                    if nat_eq(a374, a375) {
                        f188(f45(Str::lit("read:"), f46(match v2 {
                            Adt1::C0(_, h376) => {
                                h376
                            }
                        })))
                    } else {
                        f189()
                    }
                }
                Some((_, _)) => {
                    f189()
                }
            }
        }
    }
}

pub fn f116(v0: List<Adt1>, v1: Adt25) -> Result<Adt53, Adt88> {
    let m377 = v0;
    match m377.uncons() {
        None => {
            f186(1u64, f187(Str::lit("first"), v1))
        }
        Some((v2, v3)) => {
            let m378 = v3;
            match m378.uncons() {
                None => {
                    let a380 = match v2.clone() {
                        Adt1::C0(h379, _) => {
                            h379
                        }
                    };
                    let a381 = 1u64;
                    if nat_eq(a380, a381) {
                        f186(2u64, f45(f46(match v2 {
                            Adt1::C0(_, h382) => {
                                h382
                            }
                        }), f187(Str::lit("second"), v1)))
                    } else {
                        f189()
                    }
                }
                Some((v4, v5)) => {
                    let m383 = v5;
                    match m383.uncons() {
                        None => {
                            let a385 = match v2.clone() {
                                Adt1::C0(h384, _) => {
                                    h384
                                }
                            };
                            let a386 = 1u64;
                            if nat_eq(a385, a386) {
                                let a388 = match v4.clone() {
                                    Adt1::C0(h387, _) => {
                                        h387
                                    }
                                };
                                let a389 = 2u64;
                                if nat_eq(a388, a389) {
                                    f188(f45(f45(Str::lit("chain:"), f46(match v2 {
                                        Adt1::C0(_, h390) => {
                                            h390
                                        }
                                    })), f45(Str::lit("|"), f46(match v4 {
                                        Adt1::C0(_, h391) => {
                                            h391
                                        }
                                    }))))
                                } else {
                                    f189()
                                }
                            } else {
                                f189()
                            }
                        }
                        Some((_, _)) => {
                            f189()
                        }
                    }
                }
            }
        }
    }
}

pub fn f117(v0: List<Adt1>, v1: u64) -> Result<Adt53, Adt88> {
    let m392 = v0;
    match m392.uncons() {
        None => {
            f186(v1, Str::lit("edge"))
        }
        Some((_, _)) => {
            f189()
        }
    }
}

pub fn f118(v0: Fn3, v1: Adt39, v2: Adt25) -> R<Result<Adt36, Adt88>> {
    f190(v1, {
        let c393: Fn3 = v0;
        c393.apply(v2)?
    })
}

pub fn f119(v0: Adt25, v1: Str) -> bool {
    let a395 = match v0 {
        Adt25::C0(_, h394, _, _, _) => {
            h394
        }
    };
    let a396 = v1;
    equal(a395, a396)
}

pub fn f120(v0: Adt25) -> Result<Adt99, Adt88> {
    let m398 = f152(Str::lit("user_name"), Adt100::C0, match v0 {
        Adt25::C0(_, _, _, h397, _) => {
            h397
        }
    });
    match m398 {
        Adt102::C0 => {
            f193(f155(Str::lit("user_name"), f194()))
        }
        Adt102::C1 => {
            f195()
        }
        Adt102::C2(v1) => {
            f191(f192(List::cons(Str::lit("Hello, "), List::cons(f154(v1, Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("!"), List::<Str>::nil())))))
        }
    }
}

pub fn f121(v0: Adt25) -> Result<Adt99, Adt88> {
    let m400 = f152(Str::lit("capital_question"), Adt100::C1, match v0 {
        Adt25::C0(_, _, _, h399, _) => {
            h399
        }
    });
    match m400 {
        Adt102::C0 => {
            f193(f155(Str::lit("capital_question"), f196(Str::lit("What is the capital of France?"), 100u64)))
        }
        Adt102::C1 => {
            f195()
        }
        Adt102::C2(v1) => {
            f191(f192(List::cons(Str::lit("Sampling result: "), List::cons(f154(v1, Str::lit("text"), Str::lit("no response")), List::<Str>::nil()))))
        }
    }
}

pub fn f122(v0: Adt25) -> R<Result<Adt99, Adt88>> {
    let m402 = f152(Str::lit("client_roots"), Adt100::C2, match v0 {
        Adt25::C0(_, _, _, h401, _) => {
            h401
        }
    });
    match m402 {
        Adt102::C0 => {
            Ok(f193(f155(Str::lit("client_roots"), f199())))
        }
        Adt102::C1 => {
            Ok(f195())
        }
        Adt102::C2(v1) => {
            Ok(f191(f192(List::cons(Str::lit("Found "), List::cons(f197(f198(v1))?, List::cons(Str::lit(" root(s)"), List::<Str>::nil()))))))
        }
    }
}

pub fn f123(v0: Adt25) -> Result<Adt99, Adt88> {
    f200(v0, Str::lit("request-state"), Str::lit("state-ok: requestState validated"))
}

pub fn f124(v0: Adt25) -> R<Result<Adt99, Adt88>> {
    if f201(v0.clone(), Str::lit("multiple-inputs")) {
        f202(f152(Str::lit("user_name"), Adt100::C0, match v0.clone() {
            Adt25::C0(_, _, _, h403, _) => {
                h403
            }
        }), f152(Str::lit("greeting"), Adt100::C1, match v0.clone() {
            Adt25::C0(_, _, _, h404, _) => {
                h404
            }
        }), f152(Str::lit("client_roots"), Adt100::C2, match v0 {
            Adt25::C0(_, _, _, h405, _) => {
                h405
            }
        }))
    } else {
        Ok(f203(f204(Adt102::C0, Adt102::C0, Adt102::C0), Str::lit("multiple-inputs")))
    }
}

pub fn f125(v0: Adt25) -> R<Result<Adt99, Adt88>> {
    let v1: u64 = f205(f206(v0.clone()))?;
    let a406 = v1;
    let a407 = 49u64;
    if nat_eq(a406, a407) {
        Ok(f207(v0))
    } else {
        let a408 = v1;
        let a409 = 50u64;
        if nat_eq(a408, a409) {
            Ok(f208(v0))
        } else {
            Ok(f203(f155(Str::lit("step1"), f209()), Str::lit("1")))
        }
    }
}

pub fn f126(v0: Adt25) -> Result<Adt99, Adt88> {
    f200(v0, Str::lit("tamper-test"), Str::lit("integrity-ok: state verified"))
}

pub fn f127(v0: Adt25) -> Result<Adt99, Adt88> {
    let m411 = match v0.clone() {
        Adt25::C0(_, _, _, h410, _) => {
            h410
        }
    };
    match m411 {
        None => {
            f210(f211({
                let m414 = match v0 {
                    Adt25::C0(h413, _, _, _, _) => {
                        h413
                    }
                };
                match m414 {
                    Adt20::C0(_, h412, _, _, _, _) => {
                        h412
                    }
                }
            }))
        }
        Some(v1) => {
            let m416 = match v1.clone() {
                Adt8::C0(h415) => {
                    h415
                }
            };
            match m416.uncons() {
                None => {
                    f210(f211({
                        let m419 = match v0 {
                            Adt25::C0(h418, _, _, _, _) => {
                                h418
                            }
                        };
                        match m419 {
                            Adt20::C0(_, h417, _, _, _, _) => {
                                h417
                            }
                        }
                    }))
                }
                Some((_, _)) => {
                    f191(f192(List::cons(Str::lit("capabilities-ok: received "), List::cons({
                        let a421 = f212(match v1 {
                            Adt8::C0(h420) => {
                                h420
                            }
                        });
                        let a422 = Str::lit(",");
                        join(a421, a422)
                    }, List::<Str>::nil()))))
                }
            }
        }
    }
}

pub fn f128(v0: List<Adt42>) -> Adt53 {
    f132(v0, None::<Adt9>, false, List::<Adt54>::nil())
}

pub fn f129() -> Adt42 {
    f138(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f130(v0: Str, v1: Str) -> Adt42 {
    Adt42::C2(Adt47::C0(v0, v1, None::<Adt44>, None::<Adt8>))
}

pub fn f131(v0: Str, v1: Str, v2: Str) -> Adt42 {
    Adt42::C4(Adt49::C0(Adt50::C0(Adt51::C0(v0, Some(v1), v2, None::<Adt8>)), None::<Adt44>, None::<Adt8>))
}

pub fn f132(v0: List<Adt42>, v1: Option<Adt9>, v2: bool, v3: List<Adt54>) -> Adt53 {
    Adt53::C0(Adt41::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f133(v0: Adt25, v1: List<Adt54>) -> Adt53 {
    f132(List::cons(f95(f213({
        let m425 = match v0 {
            Adt25::C0(h424, _, _, _, _) => {
                h424
            }
        };
        match m425 {
            Adt20::C0(_, _, _, _, h423, _) => {
                h423
            }
        }
    })), List::<Adt42>::nil()), None::<Adt9>, false, v1)
}

pub fn f134() -> R<List<Adt54>> {
    Ok(List::cons(f214(0u64, 100u64)?, List::cons(f214(50u64, 100u64)?, List::cons(f214(100u64, 100u64)?, List::<Adt54>::nil()))))
}

pub fn f135() -> R<List<Adt54>> {
    Ok(List::cons(f214(50u64, 100u64)?, List::cons(f214(10u64, 100u64)?, List::<Adt54>::nil())))
}

pub fn f136() -> R<Adt9> {
    Ok(Adt9::C5(List::cons((Str::lit("sum"), f112(3u64)?), List::cons((Str::lit("label"), Adt9::C3(Str::lit("three"))), List::<(Str, Adt9)>::nil()))))
}

pub fn f137() -> Adt9 {
    Adt9::C5(List::cons((Str::lit("sum"), Adt9::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt9::C3(Str::lit("three"))), List::<(Str, Adt9)>::nil())))
}

pub fn f138(v0: Str, v1: Str) -> Adt42 {
    Adt42::C1(Adt46::C0(v0, v1, None::<Adt44>, None::<Adt8>))
}

pub fn f139() -> Str {
    let m427 = {
        let a426 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a426)
    };
    match m427 {
        None => {
            Str::lit("unreachable")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f140(v0: Adt25) -> Result<Adt53, Adt88> {
    if f40(v0.clone(), Str::lit("test_missing_capability")) {
        let m433 = {
            let m432 = {
                let m431 = match v0 {
                    Adt25::C0(h430, _, _, _, _) => {
                        h430
                    }
                };
                match m431 {
                    Adt20::C0(_, h429, _, _, _, _) => {
                        h429
                    }
                }
            };
            match m432 {
                Adt21::C0(_, _, h428, _, _) => {
                    h428
                }
            }
        };
        match m433 {
            None => {
                Err::<Adt53, Adt88>(Adt88::C8(f215()))
            }
            Some(_) => {
                Ok::<Adt53, Adt88>(f216())
            }
        }
    } else {
        if f40(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt53, Adt88>(f128(List::cons(f95(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt42>::nil())))
        } else {
            if f40(v0, Str::lit("test_arguments")) {
                Ok::<Adt53, Adt88>(f128(List::cons(f95(Str::lit("arguments accepted")), List::<Adt42>::nil())))
            } else {
                Ok::<Adt53, Adt88>(f216())
            }
        }
    }
}

pub fn f141(v0: Adt13, v1: List<Adt48>, v2: Adt24) -> Result<Adt36, Adt88> {
    let m435 = match v2 {
        Adt24::C0(_, h434) => {
            h434
        }
    };
    match m435 {
        None => {
            Ok::<Adt36, Adt88>(Adt36::C4(Adt55::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt36, Adt88>(Adt88::C5(v3))
        }
    }
}

pub fn f142() -> Adt13 {
    Adt13::C0(300000u64, Adt14::C0)
}

pub fn f143() -> List<Adt48> {
    f217(f218())
}

pub fn f144(v0: Adt13, v1: Fn5, v2: Adt26) -> R<Result<Adt36, Adt88>> {
    Ok(f219(v0, match v2.clone() {
        Adt26::C0(_, h436, _, _) => {
            h436
        }
    }, {
        let c437: Fn5 = v1;
        c437.apply(match v2 {
            Adt26::C0(_, h438, _, _) => {
                h438
            }
        })?
    }))
}

pub fn f145() -> Adt13 {
    Adt13::C0(300000u64, Adt14::C1)
}

pub fn f146(v0: Str) -> R<Option<List<Adt50>>> {
    let m439 = f220(v0.clone(), f218());
    match m439 {
        None => {
            f221(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f147(v0: Adt13, v1: List<Adt58>, v2: Adt24) -> Result<Adt36, Adt88> {
    let m441 = match v2 {
        Adt24::C0(_, h440) => {
            h440
        }
    };
    match m441 {
        None => {
            Ok::<Adt36, Adt88>(Adt36::C6(Adt57::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt36, Adt88>(Adt88::C5(v3))
        }
    }
}

pub fn f148() -> List<Adt58> {
    List::cons(Adt58::C0(Str::lit("template"), Some(Str::lit("Resource Template")), None::<List<Adt5>>, f222(), Some(Str::lit("A resource template with parameter substitution")), Some(Str::lit("application/json")), None::<Adt44>, None::<Adt8>), List::<Adt58>::nil())
}

pub fn f149(v0: Adt13, v1: List<Adt60>, v2: Adt24) -> Result<Adt36, Adt88> {
    let m443 = match v2 {
        Adt24::C0(_, h442) => {
            h442
        }
    };
    match m443 {
        None => {
            Ok::<Adt36, Adt88>(Adt36::C7(Adt59::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt36, Adt88>(Adt88::C5(v3))
        }
    }
}

pub fn f150() -> List<Adt60> {
    List::cons(f223(Str::lit("test_simple_prompt"), Str::lit("Simple Test Prompt"), Str::lit("A simple prompt without arguments"), None::<List<Adt61>>), List::cons(f223(Str::lit("test_prompt_with_arguments"), Str::lit("Prompt With Arguments"), Str::lit("A prompt with required arguments"), Some(List::cons(f224(Str::lit("arg1"), Str::lit("First test argument")), List::cons(f224(Str::lit("arg2"), Str::lit("Second test argument")), List::<Adt61>::nil())))), List::cons(f223(Str::lit("test_prompt_with_embedded_resource"), Str::lit("Prompt With Embedded Resource"), Str::lit("A prompt that includes an embedded resource"), Some(List::cons(f224(Str::lit("resourceUri"), Str::lit("URI of the resource to embed")), List::<Adt61>::nil()))), List::cons(f223(Str::lit("test_prompt_with_image"), Str::lit("Prompt With Image"), Str::lit("A prompt that includes image content"), None::<List<Adt61>>), List::<Adt60>::nil()))))
}

pub fn f151() -> Str {
    Str::lit("test_input_required_result_prompt")
}

pub fn f152(v0: Str, v1: Adt100, v2: Option<Adt8>) -> Adt102 {
    let m444 = v2;
    match m444 {
        None => {
            Adt102::C0
        }
        Some(v3) => {
            let m445 = f225(v0, v3);
            match m445 {
                Adt110::C0 => {
                    Adt102::C0
                }
                Adt110::C1(v4) => {
                    let m446 = f226(v4);
                    match m446 {
                        None => {
                            Adt102::C1
                        }
                        Some(v5) => {
                            if f227(v1, v5.clone()) {
                                Adt102::C2(v5)
                            } else {
                                Adt102::C1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f153(v0: Str) -> Adt62 {
    Adt62::C0(None::<Str>, List::cons(Adt63::C0(Adt45::C0, Adt42::C0(Adt43::C0(f192(List::cons(Str::lit("Prompt with context: "), List::cons(v0, List::<Str>::nil()))), None::<Adt44>, None::<Adt8>))), List::<Adt63>::nil()))
}

pub fn f154(v0: Adt8, v1: Str, v2: Str) -> Str {
    let m447 = f225(Str::lit("content"), v0);
    match m447 {
        Adt110::C0 => {
            v2
        }
        Adt110::C1(v3) => {
            let m448 = f226(v3);
            match m448 {
                None => {
                    v2
                }
                Some(v4) => {
                    f228(v1, v4, v2)
                }
            }
        }
    }
}

pub fn f155(v0: Str, v1: Adt68) -> List<(Str, Adt68)> {
    List::cons((v0, v1), List::<(Str, Adt68)>::nil())
}

pub fn f156(v0: Str, v1: Str, v2: Str) -> Adt68 {
    Adt68::C0(Adt69::C0(Adt70::C0(None::<Adt71>, v0, f229(v1, v2))))
}

pub fn f157(v0: List<Adt60>, v1: Fn6, v2: Adt27) -> Result<Adt36, Adt88> {
    let m450 = f230(match v2.clone() {
        Adt27::C0(_, h449, _, _, _) => {
            h449
        }
    }, v0);
    match m450 {
        None => {
            Err::<Adt36, Adt88>(Adt88::C1(match v2 {
                Adt27::C0(_, h451, _, _, _) => {
                    h451
                }
            }))
        }
        Some(v3) => {
            let m453 = f231(f232(v3), f233(match v2.clone() {
                Adt27::C0(_, _, h452, _, _) => {
                    h452
                }
            }));
            match m453 {
                None => {
                    Ok::<Adt36, Adt88>(Adt36::C8({
                        let c456: Fn6 = v1;
                        c456.apply(match v2.clone() {
                            Adt27::C0(_, h457, _, _, _) => {
                                h457
                            }
                        }, f233(match v2 {
                            Adt27::C0(_, _, h458, _, _) => {
                                h458
                            }
                        }))
                    }))
                }
                Some(v4) => {
                    Err::<Adt36, Adt88>(Adt88::C4({
                        let a454 = List::cons(Str::lit("missing required argument:"), List::cons(v4, List::<Str>::nil()));
                        let a455 = Str::lit(" ");
                        join(a454, a455)
                    }))
                }
            }
        }
    }
}

pub fn f158(v0: Str, v1: List<(Str, Str)>) -> Adt62 {
    let a459 = v0.clone();
    let a460 = Str::lit("test_simple_prompt");
    if equal(a459, a460) {
        f234(List::cons(f235(Str::lit("This is a simple prompt for testing.")), List::<Adt63>::nil()))
    } else {
        let a461 = v0.clone();
        let a462 = Str::lit("test_prompt_with_arguments");
        if equal(a461, a462) {
            f234(List::cons(f235({
                let a463 = List::cons(Str::lit("Prompt with arguments: arg1='"), List::cons(f236(Str::lit("arg1"), v1.clone()), List::cons(Str::lit("', arg2='"), List::cons(f236(Str::lit("arg2"), v1), List::cons(Str::lit("'"), List::<Str>::nil())))));
                let a464 = Str::lit("");
                join(a463, a464)
            }), List::<Adt63>::nil()))
        } else {
            let a465 = v0.clone();
            let a466 = Str::lit("test_prompt_with_embedded_resource");
            if equal(a465, a466) {
                f234(List::cons(Adt63::C0(Adt45::C0, Adt42::C4(Adt49::C0(f237(f236(Str::lit("resourceUri"), v1), Str::lit("text/plain"), Str::lit("Embedded resource content for testing.")), None::<Adt44>, None::<Adt8>))), List::cons(f235(Str::lit("Please process the embedded resource above.")), List::<Adt63>::nil())))
            } else {
                let a467 = v0;
                let a468 = Str::lit("test_prompt_with_image");
                if equal(a467, a468) {
                    f234(List::cons(Adt63::C0(Adt45::C0, Adt42::C1(Adt46::C0(f238(), Str::lit("image/png"), None::<Adt44>, None::<Adt8>))), List::cons(f235(Str::lit("Please analyze the image above.")), List::<Adt63>::nil())))
                } else {
                    f234(List::<Adt63>::nil())
                }
            }
        }
    }
}

pub fn f159(v0: Fn7, v1: Adt28) -> R<Result<Adt36, Adt88>> {
    let m470 = {
        let c469: Fn7 = v0;
        c469.apply(v1)?
    };
    match m470 {
        Ok(v3) => {
            Ok(Ok::<Adt36, Adt88>(Adt36::C9(f239(v3)?)))
        }
        Err(v2) => {
            Ok(Err::<Adt36, Adt88>(v2))
        }
    }
}

pub fn f160(v0: Adt28) -> R<Result<List<Str>, Adt88>> {
    let m472 = match v0.clone() {
        Adt28::C0(_, h471, _, _) => {
            h471
        }
    };
    match m472 {
        Adt29::C0(v1) => {
            let m474 = f230(match v1.clone() {
                Adt30::C0(h473, _) => {
                    h473
                }
            }, f150());
            match m474 {
                None => {
                    Ok(Err::<List<Str>, Adt88>(Adt88::C1(match v1 {
                        Adt30::C0(h475, _) => {
                            h475
                        }
                    })))
                }
                Some(v2) => {
                    if f240(f232(v2), {
                        let m478 = match v0.clone() {
                            Adt28::C0(_, _, h477, _) => {
                                h477
                            }
                        };
                        match m478 {
                            Adt31::C0(h476, _) => {
                                h476
                            }
                        }
                    }) {
                        Ok(Ok::<List<Str>, Adt88>(f241({
                            let m481 = match v0.clone() {
                                Adt28::C0(_, _, h480, _) => {
                                    h480
                                }
                            };
                            match m481 {
                                Adt31::C0(_, h479) => {
                                    h479
                                }
                            }
                        }, f242(match v1 {
                            Adt30::C0(h482, _) => {
                                h482
                            }
                        }, {
                            let m485 = match v0 {
                                Adt28::C0(_, _, h484, _) => {
                                    h484
                                }
                            };
                            match m485 {
                                Adt31::C0(h483, _) => {
                                    h483
                                }
                            }
                        }))?))
                    } else {
                        Ok(Err::<List<Str>, Adt88>(Adt88::C4(Str::lit("the prompt declares no such argument"))))
                    }
                }
            }
        }
        Adt29::C1(v3) => {
            let a486 = v3.clone();
            let a487 = f222();
            if equal(a486, a487) {
                let a491 = {
                    let m490 = match v0.clone() {
                        Adt28::C0(_, _, h489, _) => {
                            h489
                        }
                    };
                    match m490 {
                        Adt31::C0(h488, _) => {
                            h488
                        }
                    }
                };
                let a492 = Str::lit("id");
                if equal(a491, a492) {
                    Ok(Ok::<List<Str>, Adt88>(f241({
                        let m495 = match v0 {
                            Adt28::C0(_, _, h494, _) => {
                                h494
                            }
                        };
                        match m495 {
                            Adt31::C0(_, h493) => {
                                h493
                            }
                        }
                    }, f243())?))
                } else {
                    Ok(Err::<List<Str>, Adt88>(Adt88::C4(Str::lit("the template declares no such argument"))))
                }
            } else {
                Ok(Err::<List<Str>, Adt88>(Adt88::C2(v3)))
            }
        }
    }
}

pub fn f161(v0: Bytes, p505: u64, p506: u64, p507: List<Adt95>) -> R<Result<(List<Adt95>, u64), Adt2>> {
    let m504 = (p505, (p506, p507));
    {
        let mut m504 = m504;
        loop {
            let step = {
                let (v1, (v2, v3)) = m504;
                let m496 = v2;
                if m496 == 0 {
                    Err::<(u64, (u64, List<Adt95>)), R<Result<(List<Adt95>, u64), Adt2>>>(Ok(Ok::<(List<Adt95>, u64), Adt2>((f244(v3, List::<Adt95>::nil()), v1))))
                } else {
                    let v4 = m496 - 1;
                    let m497 = f7(v0.clone(), v1)?;
                    match m497 {
                        None => {
                            Err::<(u64, (u64, List<Adt95>)), R<Result<(List<Adt95>, u64), Adt2>>>(Ok(Err::<(List<Adt95>, u64), Adt2>(Adt2::C0(v1))))
                        }
                        Some(v5) => {
                            let m499 = f7(v0.clone(), match v5.clone() {
                                Adt90::C0(_, h498) => {
                                    h498
                                }
                            })?;
                            match m499 {
                                None => {
                                    Err::<(u64, (u64, List<Adt95>)), R<Result<(List<Adt95>, u64), Adt2>>>(Ok(Err::<(List<Adt95>, u64), Adt2>(Adt2::C0(match v5 {
                                        Adt90::C0(_, h500) => {
                                            h500
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt95>)), R<Result<(List<Adt95>, u64), Adt2>>>((match v6.clone() {
                                        Adt90::C0(_, h501) => {
                                            h501
                                        }
                                    }, (v4, List::cons(Adt95::C0(f29(match v5 {
                                        Adt90::C0(h502, _) => {
                                            h502
                                        }
                                    })?, f29(match v6 {
                                        Adt90::C0(h503, _) => {
                                            h503
                                        }
                                    })?), v3))))
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

pub fn f162(v0: Bytes, v1: Adt90, v2: Adt90, v3: (List<Adt95>, u64)) -> R<Result<Adt94, Adt2>> {
    let m509 = f7(v0.clone(), {
        let (_, h508) = v3.clone();
        h508
    })?;
    match m509 {
        None => {
            Ok(Err::<Adt94, Adt2>(Adt2::C0({
                let (_, h510) = v3;
                h510
            })))
        }
        Some(v4) => {
            let m512 = f27(v0.clone(), match v4.clone() {
                Adt90::C0(_, h511) => {
                    h511
                }
            })?;
            match m512 {
                None => {
                    Ok(Err::<Adt94, Adt2>(Adt2::C0(match v4 {
                        Adt90::C0(_, h513) => {
                            h513
                        }
                    })))
                }
                Some(v5) => {
                    let m517 = f245(v0.clone(), {
                        let a515 = match v4.clone() {
                            Adt90::C0(_, h514) => {
                                h514
                            }
                        };
                        let a516 = 4u64;
                        nat_add(a515, a516)?
                    })?;
                    match m517 {
                        None => {
                            Ok(Err::<Adt94, Adt2>(Adt2::C0({
                                let a519 = match v4 {
                                    Adt90::C0(_, h518) => {
                                        h518
                                    }
                                };
                                let a520 = 4u64;
                                nat_add(a519, a520)?
                            })))
                        }
                        Some(v6) => {
                            f246(v0, Adt94::C0(f29(match v1 {
                                Adt90::C0(h521, _) => {
                                    h521
                                }
                            })?, f29(match v2 {
                                Adt90::C0(h522, _) => {
                                    h522
                                }
                            })?, {
                                let (h523, _) = v3;
                                h523
                            }, match v4 {
                                Adt90::C0(h524, _) => {
                                    h524
                                }
                            }, v5, List::<List<u64>>::nil(), match v6.clone() {
                                Adt90::C0(h525, _) => {
                                    h525
                                }
                            }), match v6 {
                                Adt90::C0(_, h526) => {
                                    h526
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

pub fn f163(p531: List<Adt95>, p532: List<u64>) -> List<u64> {
    let m530 = (p531, p532);
    {
        let mut m530 = m530;
        loop {
            let step = {
                let (v0, v1) = m530;
                let m527 = v0;
                match m527.uncons() {
                    None => {
                        Err::<(List<Adt95>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt95>, List<u64>), List<u64>>((v3, {
                            let a528 = f247(v2);
                            let a529 = v1;
                            append_list(a528, a529)
                        }))
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

pub fn f164(p535: List<Adt95>, p536: List<Adt95>) -> List<Adt95> {
    let m534 = (p535, p536);
    {
        let mut m534 = m534;
        loop {
            let step = {
                let (v0, v1) = m534;
                let m533 = v0;
                match m533.uncons() {
                    None => {
                        Err::<(List<Adt95>, List<Adt95>), List<Adt95>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt95>, List<Adt95>), List<Adt95>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m534 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(p541: List<List<u64>>, p542: List<u64>) -> List<u64> {
    let m540 = (p541, p542);
    {
        let mut m540 = m540;
        loop {
            let step = {
                let (v0, v1) = m540;
                let m537 = v0;
                match m537.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a538 = f248(v2);
                            let a539 = v1;
                            append_list(a538, a539)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m540 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(p545: List<List<u64>>, p546: List<List<u64>>) -> List<List<u64>> {
    let m544 = (p545, p546);
    {
        let mut m544 = m544;
        loop {
            let step = {
                let (v0, v1) = m544;
                let m543 = v0;
                match m543.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m544 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f168() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f169() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f170() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f171() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f172(v0: Adt101) -> R<Adt98> {
    f249(f250(v0.clone()), f251(None::<Adt103>, f252(v0)))
}

pub fn f173(v0: Adt3, v1: Fn1, v2: Adt94) -> R<Adt98> {
    if f253(match v2.clone() {
        Adt94::C0(h547, _, _, _, _, _, _) => {
            h547
        }
    }) {
        f254(f255(v0, match v2.clone() {
            Adt94::C0(_, _, _, _, _, _, h548) => {
                h548
            }
        }), v1, match v2.clone() {
            Adt94::C0(_, _, _, _, h549, _, _) => {
                h549
            }
        }, match v2.clone() {
            Adt94::C0(_, _, h550, _, _, _, _) => {
                h550
            }
        }, f256(match v2 {
            Adt94::C0(_, _, _, h551, _, _, _) => {
                h551
            }
        })?)
    } else {
        Ok(f257())
    }
}

pub fn f174(v0: List<Adt95>, v1: List<List<u64>>) -> R<Option<Adt101>> {
    let m552 = f258(v0, f259())?;
    match m552 {
        Adt114::C0 => {
            Ok(None::<Adt101>)
        }
        Adt114::C1 => {
            Ok(Some(Adt101::C0))
        }
        Adt114::C2 => {
            Ok(Some(Adt101::C0))
        }
        Adt114::C3(v2) => {
            f260(f261(v2), v1, Adt101::C0)
        }
    }
}

pub fn f175(v0: List<Adt95>, v1: List<List<u64>>) -> R<Option<Adt101>> {
    let m553 = f258(v0, f262())?;
    match m553 {
        Adt114::C0 => {
            Ok(None::<Adt101>)
        }
        Adt114::C1 => {
            Ok(Some(Adt101::C1))
        }
        Adt114::C2 => {
            Ok(Some(Adt101::C1))
        }
        Adt114::C3(v2) => {
            f260(f263(v2), v1, Adt101::C1)
        }
    }
}

pub fn f176(v0: Adt3, v1: Fn1, v2: Adt94) -> R<Adt91> {
    if f253(match v2.clone() {
        Adt94::C0(h554, _, _, _, _, _, _) => {
            h554
        }
    }) {
        f264(f255(v0, match v2.clone() {
            Adt94::C0(_, _, _, _, _, _, h555) => {
                h555
            }
        }), v1, match v2.clone() {
            Adt94::C0(_, _, _, _, h556, _, _) => {
                h556
            }
        }, match v2.clone() {
            Adt94::C0(_, _, h557, _, _, _, _) => {
                h557
            }
        }, f256(match v2 {
            Adt94::C0(_, _, _, h558, _, _, _) => {
                h558
            }
        })?)
    } else {
        Ok(Adt91::C0(f54(f257())))
    }
}

pub fn f177(v0: Str, p563: List<Str>) -> bool {
    let m562 = p563;
    {
        let mut m562 = m562;
        loop {
            let step = {
                let v1 = m562;
                let m559 = v1;
                match m559.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a560 = v0.clone();
                        let a561 = v2;
                        if equal(a560, a561) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m562 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: Str, v1: List<u64>) -> Option<Adt96> {
    if f265(v1) {
        Some(Adt96::C0(v0))
    } else {
        None::<Adt96>
    }
}

pub fn f179(v0: Str) -> R<List<u64>> {
    f29({
        let a564 = v0;
        utf8_encode(a564)
    })
}

pub fn f180(v0: Adt39) -> R<Result<List<Adt15>, Adt96>> {
    let m565 = f266(v0.clone());
    match m565 {
        None => {
            f267(v0.clone(), f268(f269(), match v0.clone() {
                Adt39::C0(h566, _, _, _, _, _, _, _) => {
                    h566
                }
            }, Adt9::C5({
                let m569 = match v0.clone() {
                    Adt39::C0(_, _, _, _, h568, _, _, _) => {
                        h568
                    }
                };
                match m569 {
                    Adt8::C0(h567) => {
                        h567
                    }
                }
            }), List::cons(Adt104::C0(Adt9::C5({
                let m572 = match v0 {
                    Adt39::C0(_, _, _, _, h571, _, _, _) => {
                        h571
                    }
                };
                match m572 {
                    Adt8::C0(h570) => {
                        h570
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt104>::nil()), f270())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt15>, Adt96>(v1))
        }
    }
}

pub fn f181(p577: u64, p578: i64) -> R<i64> {
    let m576 = (p577, p578);
    {
        let mut m576 = m576;
        loop {
            let step = {
                let (v0, v1) = m576;
                let m573 = v0;
                if m573 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m573 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a574 = v1;
                        let a575 = -1i64;
                        int_sub(a574, a575)?
                    }))
                }
            };
            match step {
                Ok(next) => { m576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Adt9, v1: Adt9, v2: Adt9) -> R<Adt105> {
    f271(v0, f272(), Adt107::C0, List::cons(Adt108::C0(v1, v2, List::<Adt106>::nil()), List::<Adt108>::nil()))
}

pub fn f183(v0: Adt25) -> Adt9 {
    let m580 = match v0 {
        Adt25::C0(_, _, h579, _, _) => {
            h579
        }
    };
    match m580 {
        None => {
            Adt9::C5(List::<(Str, Adt9)>::nil())
        }
        Some(v1) => {
            Adt9::C5(match v1 {
                Adt8::C0(h581) => {
                    h581
                }
            })
        }
    }
}

pub fn f184(v0: List<Adt106>, v1: Str) -> R<Str> {
    let v2: Str = f273(v0)?;
    let a584 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a582 = v2.clone();
        let a583 = Str::lit("");
        if equal(a582, a583) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a585 = Str::lit(" ");
    Ok(join(a584, a585))
}

pub fn f185(v0: Adt39, v1: Fn2, v2: Adt25) -> R<Result<Adt36, Adt88>> {
    let m587 = {
        let c586: Fn2 = v1;
        c586.apply(v2)?
    };
    match m587 {
        Ok(v4) => {
            f274(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt36, Adt88>(v3))
        }
    }
}

pub fn f186(v0: u64, v1: Str) -> Result<Adt53, Adt88> {
    Err::<Adt53, Adt88>(Adt88::C15(f43(v0, v1)))
}

pub fn f187(v0: Str, v1: Adt25) -> Str {
    let m589 = match v1 {
        Adt25::C0(_, _, h588, _, _) => {
            h588
        }
    };
    match m589 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m590 = f225(v0, v2);
            match m590 {
                Adt110::C0 => {
                    Str::lit("")
                }
                Adt110::C1(v3) => {
                    let m591 = f275(v3);
                    match m591 {
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

pub fn f188(v0: Str) -> Result<Adt53, Adt88> {
    Ok::<Adt53, Adt88>(f128(List::cons(f95(v0), List::<Adt42>::nil())))
}

pub fn f189() -> Result<Adt53, Adt88> {
    Err::<Adt53, Adt88>(f47())
}

pub fn f190(v0: Adt39, v1: Result<Adt99, Adt88>) -> R<Result<Adt36, Adt88>> {
    let m592 = v1;
    match m592 {
        Ok(v3) => {
            let m593 = v3;
            match m593 {
                Adt99::C0(v4) => {
                    f274(v0, v4)
                }
                Adt99::C1(v5) => {
                    Ok(Ok::<Adt36, Adt88>(Adt36::C12(v5)))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt36, Adt88>(v2))
        }
    }
}

pub fn f191(v0: Str) -> Result<Adt99, Adt88> {
    Ok::<Adt99, Adt88>(Adt99::C0(Adt53::C0(Adt41::C0(List::cons(Adt42::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>)), List::<Adt42>::nil()), None::<Adt9>, None::<bool>), List::<Adt54>::nil())))
}

pub fn f192(v0: List<Str>) -> Str {
    let a594 = v0;
    let a595 = Str::lit("");
    join(a594, a595)
}

pub fn f193(v0: List<(Str, Adt68)>) -> Result<Adt99, Adt88> {
    Ok::<Adt99, Adt88>(Adt99::C1(Adt85::C0(v0)))
}

pub fn f194() -> Adt68 {
    f156(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f195() -> Result<Adt99, Adt88> {
    Err::<Adt99, Adt88>(Adt88::C11)
}

pub fn f196(v0: Str, v1: u64) -> Adt68 {
    Adt68::C1(Adt73::C0(List::cons(f276(v0), List::<Adt74>::nil()), None::<Adt79>, None::<Str>, None::<Adt81>, None::<Adt10>, v1, None::<List<Str>>, None::<Adt8>, None::<List<Adt39>>, None::<Adt82>))
}

pub fn f197(v0: u64) -> R<Str> {
    let a596 = f277(v0, 0i64)?;
    Ok(format_i64(a596))
}

pub fn f198(v0: Adt8) -> u64 {
    let m597 = f225(Str::lit("roots"), v0);
    match m597 {
        Adt110::C0 => {
            0u64
        }
        Adt110::C1(v1) => {
            let m598 = f278(v1);
            match m598 {
                None => {
                    0u64
                }
                Some(v2) => {
                    let a599 = v2;
                    length_list(a599)
                }
            }
        }
    }
}

pub fn f199() -> Adt68 {
    Adt68::C2(Some(f32()))
}

pub fn f200(v0: Adt25, v1: Str, v2: Str) -> Result<Adt99, Adt88> {
    if f201(v0.clone(), v1.clone()) {
        let m601 = f152(Str::lit("confirm"), Adt100::C0, match v0 {
            Adt25::C0(_, _, _, h600, _) => {
                h600
            }
        });
        match m601 {
            Adt102::C0 => {
                f203(f155(Str::lit("confirm"), f280()), v1)
            }
            Adt102::C1 => {
                f195()
            }
            Adt102::C2(v3) => {
                if f279(v3) {
                    f191(v2)
                } else {
                    f203(f155(Str::lit("confirm"), f280()), v1)
                }
            }
        }
    } else {
        f203(f155(Str::lit("confirm"), f280()), v1)
    }
}

pub fn f201(v0: Adt25, v1: Str) -> bool {
    let m603 = match v0 {
        Adt25::C0(_, _, _, _, h602) => {
            h602
        }
    };
    match m603 {
        None => {
            false
        }
        Some(v2) => {
            let a604 = v2;
            let a605 = v1;
            equal(a604, a605)
        }
    }
}

pub fn f202(v0: Adt102, v1: Adt102, v2: Adt102) -> R<Result<Adt99, Adt88>> {
    if if f281(v0.clone()) {
        true
    } else {
        if f281(v1.clone()) {
            true
        } else {
            f281(v2.clone())
        }
    } {
        Ok(f195())
    } else {
        let m606 = f204(v0.clone(), v1.clone(), v2.clone());
        match m606.uncons() {
            None => {
                Ok(f191(f192(List::cons(Str::lit("Name: "), List::cons(f154(f282(v0), Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("; Greeting: "), List::cons(f154(f282(v1), Str::lit("text"), Str::lit("Hello there!")), List::cons(Str::lit("; Roots: "), List::cons(f197(f198(f282(v2)))?, List::<Str>::nil())))))))))
            }
            Some((_, _)) => {
                Ok(f203(f204(v0, v1, v2), Str::lit("multiple-inputs")))
            }
        }
    }
}

pub fn f203(v0: List<(Str, Adt68)>, v1: Str) -> Result<Adt99, Adt88> {
    Ok::<Adt99, Adt88>(Adt99::C1(Adt85::C2(v0, v1)))
}

pub fn f204(v0: Adt102, v1: Adt102, v2: Adt102) -> List<(Str, Adt68)> {
    let a609 = if f283(v0) {
        f155(Str::lit("user_name"), f194())
    } else {
        List::<(Str, Adt68)>::nil()
    };
    let a610 = {
        let a607 = if f283(v1) {
            f155(Str::lit("greeting"), f196(Str::lit("Generate a greeting"), 50u64))
        } else {
            List::<(Str, Adt68)>::nil()
        };
        let a608 = if f283(v2) {
            f155(Str::lit("client_roots"), f199())
        } else {
            List::<(Str, Adt68)>::nil()
        };
        append_list(a607, a608)
    };
    append_list(a609, a610)
}

pub fn f205(v0: Str) -> R<u64> {
    f59({
        let a611 = v0;
        utf8_encode(a611)
    }, 0u64)
}

pub fn f206(v0: Adt25) -> Str {
    let m613 = match v0 {
        Adt25::C0(_, _, _, _, h612) => {
            h612
        }
    };
    match m613 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f207(v0: Adt25) -> Result<Adt99, Adt88> {
    let m615 = f152(Str::lit("step1"), Adt100::C0, match v0 {
        Adt25::C0(_, _, _, h614, _) => {
            h614
        }
    });
    match m615 {
        Adt102::C0 => {
            f203(f155(Str::lit("step1"), f209()), Str::lit("1"))
        }
        Adt102::C1 => {
            f195()
        }
        Adt102::C2(v1) => {
            f203(f155(Str::lit("step2"), f284()), f192(List::cons(Str::lit("2"), List::cons(f154(v1, Str::lit("name"), Str::lit("unknown")), List::<Str>::nil()))))
        }
    }
}

pub fn f208(v0: Adt25) -> Result<Adt99, Adt88> {
    let m617 = f152(Str::lit("step2"), Adt100::C0, match v0.clone() {
        Adt25::C0(_, _, _, h616, _) => {
            h616
        }
    });
    match m617 {
        Adt102::C0 => {
            f203(f155(Str::lit("step2"), f284()), f206(v0))
        }
        Adt102::C1 => {
            f195()
        }
        Adt102::C2(v1) => {
            f191(f192(List::cons(Str::lit("Multi-round complete for "), List::cons(f285(f206(v0)), List::cons(Str::lit(" who likes "), List::cons(f154(v1, Str::lit("color"), Str::lit("unknown")), List::<Str>::nil()))))))
        }
    }
}

pub fn f209() -> Adt68 {
    f156(Str::lit("Step 1: What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f210(v0: List<(Str, Adt68)>) -> Result<Adt99, Adt88> {
    let m618 = v0.clone();
    match m618.uncons() {
        None => {
            f191(Str::lit("No supported capabilities declared"))
        }
        Some((_, _)) => {
            f203(v0, Str::lit("capabilities-test"))
        }
    }
}

pub fn f211(v0: Adt21) -> List<(Str, Adt68)> {
    let a621 = if f286(match v0.clone() {
        Adt21::C0(_, _, _, h619, _) => {
            h619
        }
    }) {
        f155(Str::lit("elicit_input"), f156(Str::lit("Elicitation input"), Str::lit("value"), Str::lit("string")))
    } else {
        List::<(Str, Adt68)>::nil()
    };
    let a622 = if f286(match v0 {
        Adt21::C0(_, _, h620, _, _) => {
            h620
        }
    }) {
        f155(Str::lit("sample_input"), f196(Str::lit("Sample request"), 50u64))
    } else {
        List::<(Str, Adt68)>::nil()
    };
    append_list(a621, a622)
}

pub fn f212(v0: List<(Str, Adt9)>) -> List<Str> {
    f287(f288(v0, List::<Str>::nil()), List::<Str>::nil())
}

pub fn f213(v0: Option<Adt23>) -> Str {
    let m623 = v0;
    match m623 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m624 = v1;
            match m624 {
                Adt23::C0(v2) => {
                    v2
                }
                Adt23::C1(v3) => {
                    let a625 = v3;
                    format_i64(a625)
                }
            }
        }
    }
}

pub fn f214(v0: u64, v1: u64) -> R<Adt54> {
    Ok(Adt54::C0(f289(v0)?, Some(f289(v1)?), Some({
        let a628 = List::cons(Str::lit("Completed step"), List::cons({
            let a626 = f277(v0, 0i64)?;
            format_i64(a626)
        }, List::cons(Str::lit("of"), List::cons({
            let a627 = f277(v1, 0i64)?;
            format_i64(a627)
        }, List::<Str>::nil()))));
        let a629 = Str::lit(" ");
        join(a628, a629)
    })))
}

pub fn f215() -> Adt21 {
    Adt21::C0(None::<Adt8>, None::<Adt8>, Some(f32()), None::<Adt8>, None::<Adt8>)
}

pub fn f216() -> Adt53 {
    f128(List::cons(f95(Str::lit("ok")), List::<Adt42>::nil()))
}

pub fn f217(v0: List<(Adt48, List<Adt50>)>) -> List<Adt48> {
    f290(f291(v0, List::<Adt48>::nil()), List::<Adt48>::nil())
}

pub fn f218() -> List<(Adt48, List<Adt50>)> {
    List::cons(f292(f293(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f237(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f292(f293(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f294(Str::lit("test://static-binary"), Str::lit("image/png"), f238())), List::cons(f292(f293(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f237(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f292(f293(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f237(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt48, List<Adt50>)>::nil()))))
}

pub fn f219(v0: Adt13, v1: Str, v2: Option<List<Adt50>>) -> Result<Adt36, Adt88> {
    let m630 = v2;
    match m630 {
        None => {
            Err::<Adt36, Adt88>(Adt88::C2(v1))
        }
        Some(v3) => {
            let m631 = v3.clone();
            match m631.uncons() {
                None => {
                    Err::<Adt36, Adt88>(Adt88::C9(Str::lit("a listed resource has no contents")))
                }
                Some((_, _)) => {
                    Ok::<Adt36, Adt88>(Adt36::C5(Adt56::C0(v0, v3)))
                }
            }
        }
    }
}

pub fn f220(v0: Str, p640: List<(Adt48, List<Adt50>)>) -> Option<List<Adt50>> {
    let m639 = p640;
    {
        let mut m639 = m639;
        loop {
            let step = {
                let v1 = m639;
                let m632 = v1;
                match m632.uncons() {
                    None => {
                        Err::<List<(Adt48, List<Adt50>)>, Option<List<Adt50>>>(None::<List<Adt50>>)
                    }
                    Some((v2, v3)) => {
                        let a636 = {
                            let m635 = {
                                let (h634, _) = v2;
                                h634
                            };
                            match m635 {
                                Adt48::C0(_, _, _, h633, _, _, _, _, _) => {
                                    h633
                                }
                            }
                        };
                        let a637 = v0.clone();
                        if equal(a636, a637) {
                            Err::<List<(Adt48, List<Adt50>)>, Option<List<Adt50>>>(Some({
                                let (_, h638) = v2;
                                h638
                            }))
                        } else {
                            Ok::<List<(Adt48, List<Adt50>)>, Option<List<Adt50>>>(v3)
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

pub fn f221(v0: Str) -> R<Option<List<Adt50>>> {
    let m641 = f295(v0.clone())?;
    match m641 {
        None => {
            Ok(None::<List<Adt50>>)
        }
        Some(v1) => {
            let m642 = f296(v1)?;
            match m642 {
                None => {
                    Ok(None::<List<Adt50>>)
                }
                Some(v2) => {
                    Ok(Some(List::cons(f237(v0, Str::lit("application/json"), v2), List::<Adt50>::nil())))
                }
            }
        }
    }
}

pub fn f222() -> Str {
    Str::lit("test://template/{id}/data")
}

pub fn f223(v0: Str, v1: Str, v2: Str, v3: Option<List<Adt61>>) -> Adt60 {
    Adt60::C0(v0, Some(v1), None::<List<Adt5>>, Some(v2), v3, None::<Adt8>)
}

pub fn f224(v0: Str, v1: Str) -> Adt61 {
    Adt61::C0(v0, None::<Str>, Some(v1), Some(true))
}

pub fn f225(v0: Str, v1: Adt8) -> Adt110 {
    f297(v0, match v1 {
        Adt8::C0(h643) => {
            h643
        }
    })
}

pub fn f226(v0: Adt9) -> Option<Adt8> {
    let m644 = v0;
    match m644 {
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

pub fn f227(v0: Adt100, v1: Adt8) -> bool {
    let m645 = v0;
    match m645 {
        Adt100::C0 => {
            f298(v1)
        }
        Adt100::C1 => {
            f299(v1)
        }
        Adt100::C2 => {
            f300(v1)
        }
    }
}

pub fn f228(v0: Str, v1: Adt8, v2: Str) -> Str {
    let m646 = f225(v0, v1);
    match m646 {
        Adt110::C0 => {
            v2
        }
        Adt110::C1(v3) => {
            let m647 = f275(v3);
            match m647 {
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

pub fn f229(v0: Str, v1: Str) -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((v0.clone(), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(v0), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))))
}

pub fn f230(v0: Str, p653: List<Adt60>) -> Option<Adt60> {
    let m652 = p653;
    {
        let mut m652 = m652;
        loop {
            let step = {
                let v1 = m652;
                let m648 = v1;
                match m648.uncons() {
                    None => {
                        Err::<List<Adt60>, Option<Adt60>>(None::<Adt60>)
                    }
                    Some((v2, v3)) => {
                        let a650 = match v2.clone() {
                            Adt60::C0(h649, _, _, _, _, _) => {
                                h649
                            }
                        };
                        let a651 = v0.clone();
                        if equal(a650, a651) {
                            Err::<List<Adt60>, Option<Adt60>>(Some(v2))
                        } else {
                            Ok::<List<Adt60>, Option<Adt60>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m652 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f231(p659: List<Adt61>, v1: List<(Str, Str)>) -> Option<Str> {
    let m658 = p659;
    {
        let mut m658 = m658;
        loop {
            let step = {
                let v0 = m658;
                let m654 = v0;
                match m654.uncons() {
                    None => {
                        Err::<List<Adt61>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f301(v2.clone()) {
                            let m656 = f302(match v2.clone() {
                                Adt61::C0(h655, _, _, _) => {
                                    h655
                                }
                            }, v1.clone());
                            match m656 {
                                None => {
                                    Err::<List<Adt61>, Option<Str>>(Some(match v2 {
                                        Adt61::C0(h657, _, _, _) => {
                                            h657
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
                Ok(next) => { m658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: Adt60) -> List<Adt61> {
    let m661 = match v0 {
        Adt60::C0(_, _, _, _, h660, _) => {
            h660
        }
    };
    match m661 {
        None => {
            List::<Adt61>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f233(v0: Option<List<(Str, Str)>>) -> List<(Str, Str)> {
    let m662 = v0;
    match m662 {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f234(v0: List<Adt63>) -> Adt62 {
    Adt62::C0(None::<Str>, v0)
}

pub fn f235(v0: Str) -> Adt63 {
    Adt63::C0(Adt45::C0, Adt42::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>)))
}

pub fn f236(v0: Str, v1: List<(Str, Str)>) -> Str {
    let m663 = f302(v0, v1);
    match m663 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f237(v0: Str, v1: Str, v2: Str) -> Adt50 {
    Adt50::C0(Adt51::C0(v0, Some(v1), v2, None::<Adt8>))
}

pub fn f238() -> Str {
    Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==")
}

pub fn f239(v0: List<Str>) -> R<Adt64> {
    let v1: u64 = f303(v0.clone(), 0u64)?;
    Ok(Adt64::C0(Adt65::C0(f304(v0), Some(v1), Some({
        let a664 = f305();
        let a665 = v1;
        nat_lt(a664, a665)
    }))))
}

pub fn f240(p671: List<Adt61>, v1: Str) -> bool {
    let m670 = p671;
    {
        let mut m670 = m670;
        loop {
            let step = {
                let v0 = m670;
                let m666 = v0;
                match m666.uncons() {
                    None => {
                        Err::<List<Adt61>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a668 = match v2 {
                            Adt61::C0(h667, _, _, _) => {
                                h667
                            }
                        };
                        let a669 = v1.clone();
                        if equal(a668, a669) {
                            Err::<List<Adt61>, bool>(true)
                        } else {
                            Ok::<List<Adt61>, bool>(v3)
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

pub fn f241(v0: Str, v1: List<Str>) -> R<List<Str>> {
    Ok(f287(f306(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
}

pub fn f242(v0: Str, v1: Str) -> List<Str> {
    let a672 = v0;
    let a673 = Str::lit("test_prompt_with_embedded_resource");
    if if equal(a672, a673) {
        let a674 = v1;
        let a675 = Str::lit("resourceUri");
        equal(a674, a675)
    } else {
        false
    } {
        f307()
    } else {
        List::<Str>::nil()
    }
}

pub fn f243() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f244(p678: List<Adt95>, p679: List<Adt95>) -> List<Adt95> {
    let m677 = (p678, p679);
    {
        let mut m677 = m677;
        loop {
            let step = {
                let (v0, v1) = m677;
                let m676 = v0;
                match m676.uncons() {
                    None => {
                        Err::<(List<Adt95>, List<Adt95>), List<Adt95>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt95>, List<Adt95>), List<Adt95>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m677 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f245(v0: Bytes, v1: u64) -> R<Option<Adt90>> {
    let a681 = v1;
    let a682 = {
        let a680 = v0.clone();
        length_bytes(a680)
    };
    if nat_eq(a681, a682) {
        Ok(Some(Adt90::C0(Bytes::lit(&[]), v1)))
    } else {
        f7(v0, v1)
    }
}

pub fn f246(v0: Bytes, v1: Adt94, v2: u64) -> R<Result<Adt94, Adt2>> {
    let a684 = v2;
    let a685 = {
        let a683 = v0.clone();
        length_bytes(a683)
    };
    if nat_eq(a684, a685) {
        Ok(Ok::<Adt94, Adt2>(v1))
    } else {
        let m686 = f27(v0.clone(), v2)?;
        match m686 {
            None => {
                Ok(Err::<Adt94, Adt2>(Adt2::C0(v2)))
            }
            Some(v3) => {
                let m689 = f308(v0.clone(), {
                    let a687 = v2;
                    let a688 = 4u64;
                    nat_add(a687, a688)?
                }, v3, List::<List<u64>>::nil())?;
                match m689 {
                    Ok(v5) => {
                        let a692 = {
                            let (_, h690) = v5.clone();
                            h690
                        };
                        let a693 = {
                            let a691 = v0;
                            length_bytes(a691)
                        };
                        if nat_eq(a692, a693) {
                            Ok(Ok::<Adt94, Adt2>(f309(v1, {
                                let (h694, _) = v5;
                                h694
                            })))
                        } else {
                            Ok(Err::<Adt94, Adt2>(Adt2::C1({
                                let (_, h695) = v5;
                                h695
                            })))
                        }
                    }
                    Err(v4) => {
                        Ok(Err::<Adt94, Adt2>(v4))
                    }
                }
            }
        }
    }
}

pub fn f247(v0: Adt95) -> List<u64> {
    let a706 = f28({
        let a697 = match v0.clone() {
            Adt95::C0(h696, _) => {
                h696
            }
        };
        length_list(a697)
    });
    let a707 = {
        let a704 = match v0.clone() {
            Adt95::C0(h698, _) => {
                h698
            }
        };
        let a705 = {
            let a702 = f28({
                let a700 = match v0.clone() {
                    Adt95::C0(_, h699) => {
                        h699
                    }
                };
                length_list(a700)
            });
            let a703 = match v0 {
                Adt95::C0(_, h701) => {
                    h701
                }
            };
            append_list(a702, a703)
        };
        append_list(a704, a705)
    };
    append_list(a706, a707)
}

pub fn f248(v0: List<u64>) -> List<u64> {
    let a709 = f28({
        let a708 = v0.clone();
        length_list(a708)
    });
    let a710 = v0;
    append_list(a709, a710)
}

pub fn f249(v0: u64, v1: Adt9) -> R<Adt98> {
    let m711 = f310(v1)?;
    match m711 {
        Ok(v3) => {
            Ok(Adt98::C0(v0, f100(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f55())
        }
    }
}

pub fn f250(v0: Adt101) -> u64 {
    let m712 = v0;
    match m712 {
        Adt101::C0 => {
            f311()
        }
        Adt101::C1 => {
            f311()
        }
    }
}

pub fn f251(v0: Option<Adt103>, v1: Adt84) -> Adt9 {
    Adt9::C5(f312(v0, v1))
}

pub fn f252(v0: Adt101) -> Adt84 {
    Adt84::C0(f313(), f314(v0), None::<Adt9>)
}

pub fn f253(v0: List<u64>) -> bool {
    f51(v0, f315())
}

pub fn f254(v0: Adt3, v1: Fn1, v2: u64, v3: List<Adt95>, v4: Result<Adt9, Adt111>) -> R<Adt98> {
    let m713 = v4;
    match m713 {
        Ok(v6) => {
            f318(v0, v1, v2, v3, f319(v6)?)
        }
        Err(_) => {
            f249(f316(), f251(None::<Adt103>, f317(Adt112::C0)))
        }
    }
}

pub fn f255(v0: Adt3, v1: Bytes) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h714, _, _, _, _, _, _) => {
            h714
        }
    }, match v0.clone() {
        Adt3::C0(_, h715, _, _, _, _, _) => {
            h715
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h716, _, _, _, _) => {
            h716
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h717, _, _, _) => {
            h717
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h718, _, _) => {
            h718
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, h719, _) => {
            h719
        }
    }, v1)
}

pub fn f256(v0: Bytes) -> R<Result<Adt9, Adt111>> {
    f320(f29(v0)?)
}

pub fn f257() -> Adt98 {
    Adt98::C0(f321(), List::cons(Adt95::C0(f322(), f323()), List::<Adt95>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f258(v0: List<Adt95>, v1: List<u64>) -> R<Adt114> {
    let m720 = f324(v0, v1, Adt115::C0)?;
    match m720 {
        Adt115::C0 => {
            Ok(Adt114::C0)
        }
        Adt115::C1(v2) => {
            let m721 = f325(v2);
            match m721 {
                None => {
                    Ok(Adt114::C1)
                }
                Some(v3) => {
                    Ok(Adt114::C3(v3))
                }
            }
        }
        Adt115::C2 => {
            Ok(Adt114::C2)
        }
    }
}

pub fn f259() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f260(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt101) -> R<Option<Adt101>> {
    let m722 = v0;
    match m722 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f326(v3, v1)? {
                Ok(None::<Adt101>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f261(v0: List<u64>) -> Option<List<u64>> {
    let m723 = f327(v0);
    match m723 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f263(v1)
        }
    }
}

pub fn f262() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f263(v0: List<u64>) -> Option<List<u64>> {
    let m724 = v0.clone();
    match m724.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a725 = v1;
            let a726 = 91u64;
            if nat_eq(a725, a726) {
                f328(f329(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f328(f330(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f264(v0: Adt3, v1: Fn1, v2: u64, v3: List<Adt95>, v4: Result<Adt9, Adt111>) -> R<Adt91> {
    let m727 = v4;
    match m727 {
        Ok(v6) => {
            f331(v0, v1, v2, v3, f319(v6)?)
        }
        Err(_) => {
            Ok(Adt91::C0(f54(f249(f316(), f251(None::<Adt103>, f317(Adt112::C0)))?)))
        }
    }
}

pub fn f265(v0: List<u64>) -> bool {
    let m728 = v0.clone();
    match m728.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a732 = if f332(v0.clone(), false) {
                let a730 = {
                    let a729 = v0;
                    length_list(a729)
                };
                let a731 = f333();
                nat_le(a730, a731)
            } else {
                false
            };
            bool_not(a732)
        }
    }
}

pub fn f266(v0: Adt39) -> Option<Adt96> {
    let m737 = f334(match v0.clone() {
        Adt39::C0(h733, _, _, _, _, _, _, _) => {
            h733
        }
    }, {
        let m736 = match v0.clone() {
            Adt39::C0(_, _, _, _, h735, _, _, _) => {
                h735
            }
        };
        match m736 {
            Adt8::C0(h734) => {
                h734
            }
        }
    });
    match m737 {
        None => {
            f335(match v0.clone() {
                Adt39::C0(h738, _, _, _, _, _, _, _) => {
                    h738
                }
            }, {
                let m741 = match v0 {
                    Adt39::C0(_, _, _, _, h740, _, _, _) => {
                        h740
                    }
                };
                match m741 {
                    Adt8::C0(h739) => {
                        h739
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f267(v0: Adt39, v1: Result<Adt116, Adt96>) -> R<Result<List<Adt15>, Adt96>> {
    let m742 = v1;
    match m742 {
        Ok(v3) => {
            let m743 = f336(v0)?;
            match m743 {
                None => {
                    Ok(Ok::<List<Adt15>, Adt96>(f108(match v3 {
                        Adt116::C0(h744, _) => {
                            h744
                        }
                    }, List::<Adt15>::nil())))
                }
                Some(v4) => {
                    Ok(Err::<List<Adt15>, Adt96>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<List<Adt15>, Adt96>(v2))
        }
    }
}

pub fn f268(p751: u64, v1: Str, v2: Adt9, p752: List<Adt104>, p753: Adt116) -> R<Result<Adt116, Adt96>> {
    let m750 = (p751, (p752, p753));
    {
        let mut m750 = m750;
        loop {
            let step = {
                let (v0, (v3, v4)) = m750;
                let m745 = v0;
                if m745 == 0 {
                    Err::<(u64, (List<Adt104>, Adt116)), R<Result<Adt116, Adt96>>>(Ok(Err::<Adt116, Adt96>(Adt96::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m745 - 1;
                    let m746 = v3;
                    match m746.uncons() {
                        None => {
                            Err::<(u64, (List<Adt104>, Adt116)), R<Result<Adt116, Adt96>>>(Ok(Ok::<Adt116, Adt96>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m747 = f337(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m747 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt104>, Adt116)), R<Result<Adt116, Adt96>>>((v5, ({
                                        let (h748, _) = v9;
                                        h748
                                    }, {
                                        let (_, h749) = v9;
                                        h749
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt104>, Adt116)), R<Result<Adt116, Adt96>>>(Ok(Err::<Adt116, Adt96>(v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m750 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f269() -> u64 {
    67108864u64
}

pub fn f270() -> Adt116 {
    Adt116::C0(List::<Adt15>::nil(), List::<List<u64>>::nil())
}

pub fn f271(v0: Adt9, p772: u64, p773: Adt107, p774: List<Adt108>) -> R<Adt105> {
    let m771 = (p772, (p773, p774));
    {
        let mut m771 = m771;
        loop {
            let step = {
                let (v1, (v2, v3)) = m771;
                let m754 = v1;
                if m754 == 0 {
                    Err::<(u64, (Adt107, List<Adt108>)), R<Adt105>>(Ok(Adt105::C4))
                } else {
                    let v4 = m754 - 1;
                    let m755 = v3;
                    match m755.uncons() {
                        None => {
                            let m756 = v2;
                            match m756 {
                                Adt107::C0 => {
                                    Err::<(u64, (Adt107, List<Adt108>)), R<Adt105>>(Ok(Adt105::C0))
                                }
                                Adt107::C1(v5, v6) => {
                                    Err::<(u64, (Adt107, List<Adt108>)), R<Adt105>>(Ok(Adt105::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m757 = v2.clone();
                            match m757 {
                                Adt107::C0 => {
                                    let m758 = v7.clone();
                                    match m758 {
                                        Adt108::C0(v9, v10, v11) => {
                                            let m759 = f338(v0.clone(), v9, v10, v11.clone())?;
                                            match m759 {
                                                Adt119::C0(v12) => {
                                                    Ok::<(u64, (Adt107, List<Adt108>)), R<Adt105>>((v4, (Adt107::C1(v11, v12), v8)))
                                                }
                                                Adt119::C1(v13) => {
                                                    Err::<(u64, (Adt107, List<Adt108>)), R<Adt105>>(Ok(Adt105::C2(v13)))
                                                }
                                                Adt119::C2(v14) => {
                                                    Err::<(u64, (Adt107, List<Adt108>)), R<Adt105>>(Ok(Adt105::C3(v14)))
                                                }
                                                Adt119::C3(v15) => {
                                                    let v16: List<Adt108> = {
                                                        let a760 = v15;
                                                        let a761 = v8;
                                                        append_list(a760, a761)
                                                    };
                                                    Ok::<(u64, (Adt107, List<Adt108>)), R<Adt105>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt108::C1(_, _, _) => {
                                            Ok::<(u64, (Adt107, List<Adt108>)), R<Adt105>>((v4, (v2, v8)))
                                        }
                                        Adt108::C2(_, _, _, _) => {
                                            let v24: Adt117 = f339(v7, v8)?;
                                            Ok::<(u64, (Adt107, List<Adt108>)), R<Adt105>>((v4, (match v24.clone() {
                                                Adt117::C0(h762, _) => {
                                                    h762
                                                }
                                            }, match v24 {
                                                Adt117::C0(_, h763) => {
                                                    h763
                                                }
                                            })))
                                        }
                                        Adt108::C3(v25) => {
                                            Ok::<(u64, (Adt107, List<Adt108>)), R<Adt105>>((v4, (Adt107::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt108::C4(_, _, _, _) => {
                                            let v39: Adt117 = f339(v7, v8)?;
                                            Ok::<(u64, (Adt107, List<Adt108>)), R<Adt105>>((v4, (match v39.clone() {
                                                Adt117::C0(h767, _) => {
                                                    h767
                                                }
                                            }, match v39 {
                                                Adt117::C0(_, h768) => {
                                                    h768
                                                }
                                            })))
                                        }
                                        Adt108::C5(v26, v27, v28, v29) => {
                                            let m764 = f340(v26, v27, v28, v29.clone());
                                            match m764 {
                                                Adt119::C0(v30) => {
                                                    Ok::<(u64, (Adt107, List<Adt108>)), R<Adt105>>((v4, (Adt107::C1(v29, v30), v8)))
                                                }
                                                Adt119::C1(v31) => {
                                                    Err::<(u64, (Adt107, List<Adt108>)), R<Adt105>>(Ok(Adt105::C2(v31)))
                                                }
                                                Adt119::C2(v32) => {
                                                    Err::<(u64, (Adt107, List<Adt108>)), R<Adt105>>(Ok(Adt105::C3(v32)))
                                                }
                                                Adt119::C3(v33) => {
                                                    let v34: List<Adt108> = {
                                                        let a765 = v33;
                                                        let a766 = v8;
                                                        append_list(a765, a766)
                                                    };
                                                    Ok::<(u64, (Adt107, List<Adt108>)), R<Adt105>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt107::C1(v40, v41) => {
                                    let v42: Adt117 = f341(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt107, List<Adt108>)), R<Adt105>>((v4, (match v42.clone() {
                                        Adt117::C0(h769, _) => {
                                            h769
                                        }
                                    }, match v42 {
                                        Adt117::C0(_, h770) => {
                                            h770
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m771 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f272() -> u64 {
    67108864u64
}

pub fn f273(v0: List<Adt106>) -> R<Str> {
    let a775 = List::cons(Str::lit(""), f342(v0, List::<Str>::nil())?);
    let a776 = Str::lit("/");
    Ok(join(a775, a776))
}

pub fn f274(v0: Adt39, v1: Adt53) -> R<Result<Adt36, Adt88>> {
    if f343({
        let m779 = match v1.clone() {
            Adt53::C0(h778, _) => {
                h778
            }
        };
        match m779 {
            Adt41::C0(h777, _, _) => {
                h777
            }
        }
    })? {
        f344(v0, v1)
    } else {
        Ok(Err::<Adt36, Adt88>(Adt88::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f275(v0: Adt9) -> Option<Str> {
    let m780 = v0;
    match m780 {
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

pub fn f276(v0: Str) -> Adt74 {
    Adt74::C0(Adt45::C0, Adt75::C0(Adt76::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>))), None::<Adt8>)
}

pub fn f277(p783: u64, p784: i64) -> R<i64> {
    let m782 = (p783, p784);
    {
        let mut m782 = m782;
        loop {
            let step = {
                let (v0, v1) = m782;
                let m781 = v0;
                if m781 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m781 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f345(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m782 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f278(v0: Adt9) -> Option<List<Adt9>> {
    let m785 = v0;
    match m785 {
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

pub fn f279(v0: Adt8) -> bool {
    let m786 = f225(Str::lit("content"), v0);
    match m786 {
        Adt110::C0 => {
            false
        }
        Adt110::C1(v1) => {
            let m787 = f226(v1);
            match m787 {
                None => {
                    false
                }
                Some(v2) => {
                    let m788 = f225(Str::lit("ok"), v2);
                    match m788 {
                        Adt110::C0 => {
                            false
                        }
                        Adt110::C1(v3) => {
                            let m789 = f346(v3);
                            match m789 {
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

pub fn f280() -> Adt68 {
    f156(Str::lit("Please confirm"), Str::lit("ok"), Str::lit("boolean"))
}

pub fn f281(v0: Adt102) -> bool {
    let m790 = v0;
    match m790 {
        Adt102::C0 => {
            false
        }
        Adt102::C1 => {
            true
        }
        Adt102::C2(_) => {
            false
        }
    }
}

pub fn f282(v0: Adt102) -> Adt8 {
    let m791 = v0;
    match m791 {
        Adt102::C0 => {
            f32()
        }
        Adt102::C1 => {
            f32()
        }
        Adt102::C2(v1) => {
            v1
        }
    }
}

pub fn f283(v0: Adt102) -> bool {
    let m792 = v0;
    match m792 {
        Adt102::C0 => {
            true
        }
        Adt102::C1 => {
            false
        }
        Adt102::C2(_) => {
            false
        }
    }
}

pub fn f284() -> Adt68 {
    f156(Str::lit("Step 2: What is your favorite color?"), Str::lit("color"), Str::lit("string"))
}

pub fn f285(v0: Str) -> Str {
    let v1: Bytes = {
        let a793 = v0;
        utf8_encode(a793)
    };
    let m800 = {
        let a797 = v1.clone();
        let a798 = 1u64;
        let a799 = {
            let a795 = {
                let a794 = v1;
                length_bytes(a794)
            };
            let a796 = 1u64;
            nat_sub(a795, a796)
        };
        slice_bytes(a797, a798, a799)
    };
    match m800 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m802 = {
                let a801 = v2;
                utf8_decode(a801)
            };
            match m802 {
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

pub fn f286(v0: Option<Adt8>) -> bool {
    let m803 = v0;
    match m803 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f287(p806: List<Str>, p807: List<Str>) -> List<Str> {
    let m805 = (p806, p807);
    {
        let mut m805 = m805;
        loop {
            let step = {
                let (v0, v1) = m805;
                let m804 = v0;
                match m804.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m805 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(p811: List<(Str, Adt9)>, p812: List<Str>) -> List<Str> {
    let m810 = (p811, p812);
    {
        let mut m810 = m810;
        loop {
            let step = {
                let (v0, v1) = m810;
                let m808 = v0;
                match m808.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt9)>, List<Str>), List<Str>>((v3, List::cons({
                            let (h809, _) = v2;
                            h809
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m810 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f289(v0: u64) -> R<Adt10> {
    Ok(Adt10::C0(f277(v0, 0i64)?, 0i64))
}

pub fn f290(p815: List<Adt48>, p816: List<Adt48>) -> List<Adt48> {
    let m814 = (p815, p816);
    {
        let mut m814 = m814;
        loop {
            let step = {
                let (v0, v1) = m814;
                let m813 = v0;
                match m813.uncons() {
                    None => {
                        Err::<(List<Adt48>, List<Adt48>), List<Adt48>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt48>, List<Adt48>), List<Adt48>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(p820: List<(Adt48, List<Adt50>)>, p821: List<Adt48>) -> List<Adt48> {
    let m819 = (p820, p821);
    {
        let mut m819 = m819;
        loop {
            let step = {
                let (v0, v1) = m819;
                let m817 = v0;
                match m817.uncons() {
                    None => {
                        Err::<(List<(Adt48, List<Adt50>)>, List<Adt48>), List<Adt48>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt48, List<Adt50>)>, List<Adt48>), List<Adt48>>((v3, List::cons({
                            let (h818, _) = v2;
                            h818
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m819 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f292(v0: Adt48, v1: Adt50) -> (Adt48, List<Adt50>) {
    (v0, List::cons(v1, List::<Adt50>::nil()))
}

pub fn f293(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt48 {
    Adt48::C0(v0, v1, None::<List<Adt5>>, v2, Some(v3), Some(v4), None::<Adt44>, None::<u64>, None::<Adt8>)
}

pub fn f294(v0: Str, v1: Str, v2: Str) -> Adt50 {
    Adt50::C1(Adt52::C0(v0, Some(v1), v2, None::<Adt8>))
}

pub fn f295(v0: Str) -> R<Option<Str>> {
    let m822 = f347(v0)?;
    match m822 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a823 = f348(f349(), v1);
            Ok(utf8_decode(a823))
        }
    }
}

pub fn f296(v0: Str) -> R<Option<Str>> {
    let m826 = f350(Adt9::C5(List::cons((Str::lit("id"), Adt9::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt9::C1(true)), List::cons((Str::lit("data"), Adt9::C3({
        let a824 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a825 = Str::lit(" ");
        join(a824, a825)
    })), List::<(Str, Adt9)>::nil())))))?;
    match m826 {
        Ok(v1) => {
            let a827 = v1;
            Ok(utf8_decode(a827))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f297(v0: Str, p833: List<(Str, Adt9)>) -> Adt110 {
    let m832 = p833;
    {
        let mut m832 = m832;
        loop {
            let step = {
                let v1 = m832;
                let m828 = v1;
                match m828.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, Adt110>(Adt110::C0)
                    }
                    Some((v2, v3)) => {
                        let m830 = f351({
                            let (h829, _) = v2;
                            h829
                        }, v0.clone());
                        match m830 {
                            Adt120::C0 => {
                                Err::<List<(Str, Adt9)>, Adt110>(Adt110::C1({
                                    let (_, h831) = v2;
                                    h831
                                }))
                            }
                            Adt120::C1 => {
                                Ok::<List<(Str, Adt9)>, Adt110>(v3)
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

pub fn f298(v0: Adt8) -> bool {
    if f352(v0.clone()) {
        f353(v0)
    } else {
        false
    }
}

pub fn f299(v0: Adt8) -> bool {
    if f354(v0.clone()) {
        if f355(f225(Str::lit("model"), v0.clone())) {
            f356(v0)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f300(v0: Adt8) -> bool {
    let m834 = f225(Str::lit("roots"), v0);
    match m834 {
        Adt110::C0 => {
            false
        }
        Adt110::C1(v1) => {
            let m835 = f278(v1);
            match m835 {
                None => {
                    false
                }
                Some(v2) => {
                    f357(v2)
                }
            }
        }
    }
}

pub fn f301(v0: Adt61) -> bool {
    let m837 = match v0 {
        Adt61::C0(_, _, _, h836) => {
            h836
        }
    };
    match m837 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f302(v0: Str, p844: List<(Str, Str)>) -> Option<Str> {
    let m843 = p844;
    {
        let mut m843 = m843;
        loop {
            let step = {
                let v1 = m843;
                let m838 = v1;
                match m838.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a840 = {
                            let (h839, _) = v2;
                            h839
                        };
                        let a841 = v0.clone();
                        if equal(a840, a841) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h842) = v2;
                                h842
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m843 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f303(p849: List<Str>, p850: u64) -> R<u64> {
    let m848 = (p849, p850);
    {
        let mut m848 = m848;
        loop {
            let step = {
                let (v0, v1) = m848;
                let m845 = v0;
                match m845.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a846 = v1;
                            let a847 = 1u64;
                            nat_add(a846, a847)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m848 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: List<Str>) -> List<Str> {
    f287(f358(v0, f305(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f305() -> u64 {
    100u64
}

pub fn f306(v0: Str, p853: List<Str>, p854: List<Str>) -> R<List<Str>> {
    let m852 = (p853, p854);
    {
        let mut m852 = m852;
        loop {
            let step = {
                let (v1, v2) = m852;
                let m851 = v1;
                match m851.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f359(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
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

pub fn f307() -> List<Str> {
    f287(f360(f143(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f308(v0: Bytes, p860: u64, p861: u64, p862: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt2>> {
    let m859 = (p860, (p861, p862));
    {
        let mut m859 = m859;
        loop {
            let step = {
                let (v1, (v2, v3)) = m859;
                let m855 = v2;
                if m855 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt2>>>(Ok(Ok::<(List<List<u64>>, u64), Adt2>((f166(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m855 - 1;
                    let m856 = f7(v0.clone(), v1)?;
                    match m856 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt2>>>(Ok(Err::<(List<List<u64>>, u64), Adt2>(Adt2::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt2>>>((match v5.clone() {
                                Adt90::C0(_, h857) => {
                                    h857
                                }
                            }, (v4, List::cons(f29(match v5 {
                                Adt90::C0(h858, _) => {
                                    h858
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m859 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f309(v0: Adt94, v1: List<List<u64>>) -> Adt94 {
    Adt94::C0(match v0.clone() {
        Adt94::C0(h863, _, _, _, _, _, _) => {
            h863
        }
    }, match v0.clone() {
        Adt94::C0(_, h864, _, _, _, _, _) => {
            h864
        }
    }, match v0.clone() {
        Adt94::C0(_, _, h865, _, _, _, _) => {
            h865
        }
    }, match v0.clone() {
        Adt94::C0(_, _, _, h866, _, _, _) => {
            h866
        }
    }, match v0.clone() {
        Adt94::C0(_, _, _, _, h867, _, _) => {
            h867
        }
    }, v1, match v0 {
        Adt94::C0(_, _, _, _, _, _, h868) => {
            h868
        }
    })
}

pub fn f310(v0: Adt9) -> R<Result<List<u64>, Adt111>> {
    f361(f362(), v0)
}

pub fn f311() -> u64 {
    403u64
}

pub fn f312(v0: Option<Adt103>, v1: Adt84) -> List<(Str, Adt9)> {
    List::cons((Str::lit("jsonrpc"), Adt9::C3(f363())), {
        let a869 = f364(v0);
        let a870 = List::cons((Str::lit("error"), f365(v1)), List::<(Str, Adt9)>::nil());
        append_list(a869, a870)
    })
}

pub fn f313() -> i64 {
    -32000i64
}

pub fn f314(v0: Adt101) -> Str {
    let m871 = v0;
    match m871 {
        Adt101::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt101::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f315() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f316() -> u64 {
    400u64
}

pub fn f317(v0: Adt112) -> Adt84 {
    Adt84::C0(f366(v0.clone()), f367(v0), None::<Adt9>)
}

pub fn f318(v0: Adt3, v1: Fn1, v2: u64, v3: List<Adt95>, v4: Result<Adt118, Adt112>) -> R<Adt98> {
    let m872 = v4;
    match m872 {
        Ok(v6) => {
            f370(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            f249(f368(v5.clone()), f251(f369(v5.clone()), f317(v5)))
        }
    }
}

pub fn f319(v0: Adt9) -> R<Result<Adt118, Adt112>> {
    let m873 = v0;
    match m873 {
        Adt9::C0 => {
            Ok(f372(None::<Adt103>, Adt113::C0))
        }
        Adt9::C1(_) => {
            Ok(f372(None::<Adt103>, Adt113::C0))
        }
        Adt9::C2(_) => {
            Ok(f372(None::<Adt103>, Adt113::C0))
        }
        Adt9::C3(_) => {
            Ok(f372(None::<Adt103>, Adt113::C0))
        }
        Adt9::C4(_) => {
            Ok(f372(None::<Adt103>, Adt113::C1))
        }
        Adt9::C5(v1) => {
            f371(Adt8::C0(v1))
        }
    }
}

pub fn f320(v0: List<u64>) -> R<Result<Adt9, Adt111>> {
    let m874 = f373(v0.clone())?;
    match m874 {
        Adt127::C0 => {
            f374(f375(), 0u64, v0)
        }
        Adt127::C1(v1) => {
            Ok(Err::<Adt9, Adt111>(Adt111::C0(v1)))
        }
        Adt127::C2(v2) => {
            Ok(Err::<Adt9, Adt111>(Adt111::C1(v2)))
        }
    }
}

pub fn f321() -> u64 {
    405u64
}

pub fn f322() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f323() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f324(p879: List<Adt95>, v1: List<u64>, p880: Adt115) -> R<Adt115> {
    let m878 = (p879, p880);
    {
        let mut m878 = m878;
        loop {
            let step = {
                let (v0, v2) = m878;
                let m875 = v0;
                match m875.uncons() {
                    None => {
                        Err::<(List<Adt95>, Adt115), R<Adt115>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f376(match v3.clone() {
                            Adt95::C0(h876, _) => {
                                h876
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt95>, Adt115), R<Adt115>>((v4, f377(v2, match v3 {
                                Adt95::C0(_, h877) => {
                                    h877
                                }
                            })))
                        } else {
                            Ok::<(List<Adt95>, Adt115), R<Adt115>>((v4, v2))
                        }
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

pub fn f325(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f378(v0);
    if f379(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f326(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f376(v0.clone(), f380())? {
        Ok(true)
    } else {
        if f376(v0.clone(), f381())? {
            Ok(true)
        } else {
            if f376(v0.clone(), f382())? {
                Ok(true)
            } else {
                f383(v0, v1)
            }
        }
    }
}

pub fn f327(p885: List<u64>) -> Option<List<u64>> {
    let m884 = p885;
    {
        let mut m884 = m884;
        loop {
            let step = {
                let v0 = m884;
                let m881 = v0;
                match m881.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a882 = v1;
                        let a883 = 58u64;
                        if nat_eq(a882, a883) {
                            Err::<List<u64>, Option<List<u64>>>(f384(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m884 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f328(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m886 = v0;
    match m886 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f385({
                let (_, h887) = v1;
                h887
            }) {
                Some({
                    let (h888, _) = v1;
                    h888
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f329(p893: List<u64>, p894: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m892 = (p893, p894);
    {
        let mut m892 = m892;
        loop {
            let step = {
                let (v0, v1) = m892;
                let m889 = v0;
                match m889.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a890 = v2;
                        let a891 = 93u64;
                        if nat_eq(a890, a891) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f386(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m892 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f330(p899: List<u64>, p900: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m898 = (p899, p900);
    {
        let mut m898 = m898;
        loop {
            let step = {
                let (v0, v1) = m898;
                let m895 = v0;
                match m895.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f386(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a896 = v2;
                        let a897 = 58u64;
                        if nat_eq(a896, a897) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f386(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m898 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(v0: Adt3, v1: Fn1, v2: u64, v3: List<Adt95>, v4: Result<Adt118, Adt112>) -> R<Adt91> {
    let m901 = v4;
    match m901 {
        Ok(v6) => {
            f387(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            Ok(Adt91::C0(f54(f249(f368(v5.clone()), f251(f369(v5.clone()), f317(v5)))?)))
        }
    }
}

pub fn f332(p904: List<u64>, v1: bool) -> bool {
    let m903 = p904;
    {
        let mut m903 = m903;
        loop {
            let step = {
                let v0 = m903;
                let m902 = v0;
                match m902.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f388(v2)
                        } else {
                            f389(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f333() -> u64 {
    64u64
}

pub fn f334(v0: Str, v1: List<(Str, Adt9)>) -> Option<Adt96> {
    let m905 = f297(Str::lit("type"), v1);
    match m905 {
        Adt110::C0 => {
            Some(Adt96::C3(v0))
        }
        Adt110::C1(v2) => {
            let m906 = f275(v2);
            match m906 {
                None => {
                    Some(Adt96::C3(v0))
                }
                Some(v3) => {
                    let a907 = v3;
                    let a908 = Str::lit("object");
                    if equal(a907, a908) {
                        None::<Adt96>
                    } else {
                        Some(Adt96::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f335(v0: Str, v1: List<(Str, Adt9)>) -> Option<Adt96> {
    let m909 = f297(Str::lit("$schema"), v1);
    match m909 {
        Adt110::C0 => {
            None::<Adt96>
        }
        Adt110::C1(v2) => {
            let m910 = f275(v2);
            match m910 {
                None => {
                    Some(Adt96::C2(v0))
                }
                Some(v3) => {
                    let a911 = v3;
                    let a912 = f390();
                    if equal(a911, a912) {
                        None::<Adt96>
                    } else {
                        Some(Adt96::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f336(v0: Adt39) -> R<Option<Adt96>> {
    let m914 = match v0.clone() {
        Adt39::C0(_, _, _, _, _, h913, _, _) => {
            h913
        }
    };
    match m914 {
        None => {
            Ok(None::<Adt96>)
        }
        Some(v1) => {
            let m917 = f335(match v0.clone() {
                Adt39::C0(h915, _, _, _, _, _, _, _) => {
                    h915
                }
            }, match v1.clone() {
                Adt8::C0(h916) => {
                    h916
                }
            });
            match m917 {
                None => {
                    let m921 = f268(f269(), match v0 {
                        Adt39::C0(h918, _, _, _, _, _, _, _) => {
                            h918
                        }
                    }, Adt9::C5(match v1.clone() {
                        Adt8::C0(h919) => {
                            h919
                        }
                    }), List::cons(Adt104::C0(Adt9::C5(match v1 {
                        Adt8::C0(h920) => {
                            h920
                        }
                    }), false, List::<Str>::nil()), List::<Adt104>::nil()), f270())?;
                    match m921 {
                        Ok(_) => {
                            Ok(None::<Adt96>)
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

pub fn f337(v0: Str, v1: Adt9, v2: Adt104, v3: Adt116, v4: List<Adt104>) -> R<Result<(List<Adt104>, Adt116), Adt96>> {
    let m922 = v2;
    match m922 {
        Adt104::C0(v5, v6, v7) => {
            let m923 = f346(v5.clone());
            match m923 {
                None => {
                    let m924 = f226(v5);
                    match m924 {
                        None => {
                            Ok(Err::<(List<Adt104>, Adt116), Adt96>(Adt96::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f391(v0, v1, match v9 {
                                Adt8::C0(h925) => {
                                    h925
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt104>, Adt116), Adt96>((v4, v3)))
                }
            }
        }
    }
}

pub fn f338(v0: Adt9, v1: Adt9, v2: Adt9, v3: List<Adt106>) -> R<Adt119> {
    let m926 = f346(v1.clone());
    match m926 {
        None => {
            let m927 = f226(v1);
            match m927 {
                None => {
                    Ok(Adt119::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f392(v0, match v5 {
                        Adt8::C0(h928) => {
                            h928
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt119::C3(List::<Adt108>::nil()))
            } else {
                Ok(Adt119::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f339(v0: Adt108, v1: List<Adt108>) -> R<Adt117> {
    let m929 = v0;
    match m929 {
        Adt108::C0(_, _, _) => {
            Ok(Adt117::C0(Adt107::C0, v1))
        }
        Adt108::C1(_, _, _) => {
            Ok(Adt117::C0(Adt107::C0, v1))
        }
        Adt108::C2(v8, v9, v10, v11) => {
            Ok(f393(v8, v9, v10, {
                let a930 = v11;
                let a931 = 1u64;
                nat_add(a930, a931)?
            }, v1))
        }
        Adt108::C3(v12) => {
            Ok(Adt117::C0(Adt107::C1(v12, Str::lit("not")), v1))
        }
        Adt108::C4(v13, _, v15, v16) => {
            Ok(f394(v13, v15, v16, v1))
        }
        Adt108::C5(_, _, _, _) => {
            Ok(Adt117::C0(Adt107::C0, v1))
        }
    }
}

pub fn f340(v0: Adt109, v1: Adt9, v2: Adt9, v3: List<Adt106>) -> Adt119 {
    let m932 = f226(v1);
    match m932 {
        None => {
            Adt119::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m933 = v0;
            match m933 {
                Adt109::C0 => {
                    f395(match v4 {
                        Adt8::C0(h934) => {
                            h934
                        }
                    }, v2, v3)
                }
                Adt109::C1 => {
                    f396(match v4 {
                        Adt8::C0(h935) => {
                            h935
                        }
                    }, v2, v3)
                }
                Adt109::C2 => {
                    f397(match v4 {
                        Adt8::C0(h936) => {
                            h936
                        }
                    }, v2, v3)
                }
                Adt109::C3 => {
                    f398(match v4 {
                        Adt8::C0(h937) => {
                            h937
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f341(v0: Adt108, v1: List<Adt108>, v2: List<Adt106>, v3: Str) -> Adt117 {
    let m938 = v0;
    match m938 {
        Adt108::C0(_, _, _) => {
            Adt117::C0(Adt107::C1(v2, v3), v1)
        }
        Adt108::C1(v7, v8, v9) => {
            f399(v7, v8, v9, v1)
        }
        Adt108::C2(v10, v11, v12, v13) => {
            f393(v10, v11, v12, v13, v1)
        }
        Adt108::C3(_) => {
            Adt117::C0(Adt107::C0, v1)
        }
        Adt108::C4(_, v16, v17, v18) => {
            f394(v16, v17, v18, v1)
        }
        Adt108::C5(_, _, _, _) => {
            Adt117::C0(Adt107::C1(v2, v3), v1)
        }
    }
}

pub fn f342(p941: List<Adt106>, p942: List<Str>) -> R<List<Str>> {
    let m940 = (p941, p942);
    {
        let mut m940 = m940;
        loop {
            let step = {
                let (v0, v1) = m940;
                let m939 = v0;
                match m939.uncons() {
                    None => {
                        Err::<(List<Adt106>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt106>, List<Str>), R<List<Str>>>((v3, List::cons(f400(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m940 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(p951: List<Adt42>) -> R<bool> {
    let m950 = p951;
    {
        let mut m950 = m950;
        loop {
            let step = {
                let v0 = m950;
                let m943 = v0;
                match m943.uncons() {
                    None => {
                        Err::<List<Adt42>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m944 = v1;
                        match m944 {
                            Adt42::C0(_) => {
                                Ok::<List<Adt42>, R<bool>>(v2)
                            }
                            Adt42::C1(v4) => {
                                if f401(match v4 {
                                    Adt46::C0(h945, _, _, _) => {
                                        h945
                                    }
                                })? {
                                    Ok::<List<Adt42>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt42>, R<bool>>(Ok(false))
                                }
                            }
                            Adt42::C2(v5) => {
                                if f401(match v5 {
                                    Adt47::C0(h946, _, _, _) => {
                                        h946
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
                                let m948 = match v7 {
                                    Adt49::C0(h947, _, _) => {
                                        h947
                                    }
                                };
                                match m948 {
                                    Adt50::C0(_) => {
                                        Ok::<List<Adt42>, R<bool>>(v2)
                                    }
                                    Adt50::C1(v9) => {
                                        if f401(match v9 {
                                            Adt52::C0(_, _, h949, _) => {
                                                h949
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
                Ok(next) => { m950 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(v0: Adt39, v1: Adt53) -> R<Result<Adt36, Adt88>> {
    if f402(match v1.clone() {
        Adt53::C0(h952, _) => {
            h952
        }
    }) {
        f403(v1)
    } else {
        let m954 = match v0 {
            Adt39::C0(_, _, _, _, _, h953, _, _) => {
                h953
            }
        };
        match m954 {
            None => {
                f403(v1)
            }
            Some(v2) => {
                f404(v2, v1)
            }
        }
    }
}

pub fn f345(v0: i64, v1: i64) -> R<i64> {
    let a956 = v0;
    let a957 = {
        let a955 = v1;
        int_neg(a955)?
    };
    int_sub(a956, a957)
}

pub fn f346(v0: Adt9) -> Option<bool> {
    let m958 = v0;
    match m958 {
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

pub fn f347(v0: Str) -> R<Option<List<u64>>> {
    let m959 = f405(f406(f407())?, f406(v0)?);
    match m959 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m960 = f405(f408(List::cons(47u64, f406(f409())?)), f408(v1));
            match m960 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v2) => {
                    let v3: List<u64> = f408(v2);
                    if if f410(v3.clone()) {
                        f411(v3.clone())
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

pub fn f348(v0: Bytes, v1: List<u64>) -> Bytes {
    let a961 = v0;
    let a962 = f412(f349(), f413(List::<Bytes>::nil(), v1));
    append_bytes(a961, a962)
}

pub fn f349() -> Bytes {
    Bytes::lit(&[])
}

pub fn f350(v0: Adt9) -> R<Result<Bytes, Adt111>> {
    let m963 = f310(v0)?;
    match m963 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt111>(f348(f349(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt111>(v2))
        }
    }
}

pub fn f351(v0: Str, v1: Str) -> Adt120 {
    let a964 = v0;
    let a965 = v1;
    if equal(a964, a965) {
        Adt120::C0
    } else {
        Adt120::C1
    }
}

pub fn f352(v0: Adt8) -> bool {
    if f414(f225(Str::lit("action"), v0.clone()), Str::lit("accept")) {
        true
    } else {
        if f414(f225(Str::lit("action"), v0.clone()), Str::lit("decline")) {
            true
        } else {
            f414(f225(Str::lit("action"), v0), Str::lit("cancel"))
        }
    }
}

pub fn f353(v0: Adt8) -> bool {
    let m966 = f225(Str::lit("content"), v0);
    match m966 {
        Adt110::C0 => {
            true
        }
        Adt110::C1(v1) => {
            let m967 = f226(v1);
            match m967 {
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

pub fn f354(v0: Adt8) -> bool {
    if f414(f225(Str::lit("role"), v0.clone()), Str::lit("user")) {
        true
    } else {
        f414(f225(Str::lit("role"), v0), Str::lit("assistant"))
    }
}

pub fn f355(v0: Adt110) -> bool {
    let m968 = v0;
    match m968 {
        Adt110::C0 => {
            false
        }
        Adt110::C1(v1) => {
            let m969 = f275(v1);
            match m969 {
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

pub fn f356(v0: Adt8) -> bool {
    let m970 = f225(Str::lit("content"), v0);
    match m970 {
        Adt110::C0 => {
            false
        }
        Adt110::C1(v1) => {
            let m971 = f226(v1.clone());
            match m971 {
                None => {
                    let m972 = f278(v1);
                    match m972 {
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

pub fn f357(p976: List<Adt9>) -> bool {
    let m975 = p976;
    {
        let mut m975 = m975;
        loop {
            let step = {
                let v0 = m975;
                let m973 = v0;
                match m973.uncons() {
                    None => {
                        Err::<List<Adt9>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m974 = f226(v1);
                        match m974 {
                            None => {
                                Err::<List<Adt9>, bool>(false)
                            }
                            Some(v3) => {
                                if f355(f225(Str::lit("uri"), v3)) {
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
                Ok(next) => { m975 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(p980: List<Str>, p981: u64, p982: List<Str>) -> List<Str> {
    let m979 = (p980, (p981, p982));
    {
        let mut m979 = m979;
        loop {
            let step = {
                let (v0, (v1, v2)) = m979;
                let m977 = v0;
                match m977.uncons() {
                    None => {
                        Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                    }
                    Some((v3, v4)) => {
                        let m978 = v1;
                        if m978 == 0 {
                            Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                        } else {
                            let v5 = m978 - 1;
                            Ok::<(List<Str>, (u64, List<Str>)), List<Str>>((v4, (v5, List::cons(v3, v2))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m979 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: Str, v1: Str) -> R<bool> {
    Ok(f415(f406(v0)?, f406(v1)?))
}

pub fn f360(p986: List<Adt48>, p987: List<Str>) -> List<Str> {
    let m985 = (p986, p987);
    {
        let mut m985 = m985;
        loop {
            let step = {
                let (v0, v1) = m985;
                let m983 = v0;
                match m983.uncons() {
                    None => {
                        Err::<(List<Adt48>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt48>, List<Str>), List<Str>>((v3, List::cons(match v2 {
                            Adt48::C0(_, _, _, h984, _, _, _, _, _) => {
                                h984
                            }
                        }, v1)))
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

pub fn f361(v0: u64, v1: Adt9) -> R<Result<List<u64>, Adt111>> {
    let m988 = f416(v0, List::cons(Adt121::C1(0u64, v1), List::<Adt121>::nil()), List::<u64>::nil())?;
    match m988 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt111>(f417(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt111>(v3))
        }
    }
}

pub fn f362() -> u64 {
    67108864u64
}

pub fn f363() -> Str {
    Str::lit("2.0")
}

pub fn f364(v0: Option<Adt103>) -> List<(Str, Adt9)> {
    let m989 = v0;
    match m989 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f418(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f365(v0: Adt84) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("code"), Adt9::C2(Adt10::C0(match v0.clone() {
        Adt84::C0(h990, _, _) => {
            h990
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt9::C3(match v0.clone() {
        Adt84::C0(_, h991, _) => {
            h991
        }
    })), f419(match v0 {
        Adt84::C0(_, _, h992) => {
            h992
        }
    }))))
}

pub fn f366(v0: Adt112) -> i64 {
    let m993 = v0;
    match m993 {
        Adt112::C0 => {
            f420()
        }
        Adt112::C1(_, _) => {
            f421()
        }
    }
}

pub fn f367(v0: Adt112) -> Str {
    let m994 = v0;
    match m994 {
        Adt112::C0 => {
            Str::lit("Parse error")
        }
        Adt112::C1(_, v2) => {
            f422(v2)
        }
    }
}

pub fn f368(v0: Adt112) -> u64 {
    let m995 = v0;
    match m995 {
        Adt112::C0 => {
            f316()
        }
        Adt112::C1(_, _) => {
            f316()
        }
    }
}

pub fn f369(v0: Adt112) -> Option<Adt103> {
    let m996 = v0;
    match m996 {
        Adt112::C0 => {
            None::<Adt103>
        }
        Adt112::C1(v1, _) => {
            v1
        }
    }
}

pub fn f370(v0: Adt3, v1: Fn1, v2: u64, v3: List<Adt95>, v4: Adt118) -> R<Adt98> {
    f423(f424(v3.clone())?, f425(v0.clone(), v1, v2, Some(f426(v3, match v0 {
        Adt3::C0(_, _, _, _, h997, _, _) => {
            h997
        }
    }, f427(v4.clone()), match v4.clone() {
        Adt118::C0(_, h998, _) => {
            h998
        }
    })?), f428(), v4)?)
}

pub fn f371(v0: Adt8) -> R<Result<Adt118, Adt112>> {
    let m999 = f429(v0.clone())?;
    match m999 {
        Adt142::C0 => {
            let m1000 = f225(Str::lit("method"), v0.clone());
            match m1000 {
                Adt110::C0 => {
                    Ok(f372(None::<Adt103>, f430(v0)))
                }
                Adt110::C1(_) => {
                    Ok(f372(None::<Adt103>, if f431(v0) {
                        Adt113::C8
                    } else {
                        Adt113::C7
                    }))
                }
            }
        }
        Adt142::C1(v2) => {
            Ok(f432(v2, v0))
        }
        Adt142::C2 => {
            Ok(f372(None::<Adt103>, Adt113::C5))
        }
    }
}

pub fn f372(v0: Option<Adt103>, v1: Adt113) -> Result<Adt118, Adt112> {
    Err::<Adt118, Adt112>(Adt112::C1(v0, v1))
}

pub fn f373(v0: List<u64>) -> R<Adt127> {
    Ok(f433({
        let a1001 = v0.clone();
        length_list(a1001)
    }, f434(Adt128::C0, 0u64, 0u64, v0)?))
}

pub fn f374(p1008: Adt129, p1009: u64, p1010: List<u64>) -> R<Result<Adt9, Adt111>> {
    let m1007 = (p1008, (p1009, p1010));
    {
        let mut m1007 = m1007;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1007;
                let m1002 = v2;
                match m1002.uncons() {
                    None => {
                        let m1003 = f435(v0, v1)?;
                        match m1003 {
                            Ok(v3) => {
                                Err::<(Adt129, (u64, List<u64>)), R<Result<Adt9, Adt111>>>(Ok(Ok::<Adt9, Adt111>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt129, (u64, List<u64>)), R<Result<Adt9, Adt111>>>(Ok(Err::<Adt9, Adt111>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m1004 = f436(v0, v1, v5)?;
                        match m1004 {
                            Adt143::C0(v7) => {
                                Err::<(Adt129, (u64, List<u64>)), R<Result<Adt9, Adt111>>>(Ok(Err::<Adt9, Adt111>(v7)))
                            }
                            Adt143::C1(v8) => {
                                Ok::<(Adt129, (u64, List<u64>)), R<Result<Adt9, Adt111>>>((v8, ({
                                    let a1005 = v1;
                                    let a1006 = 1u64;
                                    nat_add(a1005, a1006)?
                                }, v6)))
                            }
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

pub fn f375() -> Adt129 {
    Adt129::C0(Adt130::C0, List::<Adt134>::nil(), 0u64)
}

pub fn f376(p1017: List<u64>, p1018: List<u64>) -> R<bool> {
    let m1016 = (p1017, p1018);
    {
        let mut m1016 = m1016;
        loop {
            let step = {
                let (v0, v1) = m1016;
                let m1011 = v0;
                match m1011.uncons() {
                    None => {
                        let m1012 = v1;
                        match m1012.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1013 = v1;
                        match m1013.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a1014 = f437(v4)?;
                                let a1015 = f437(v6)?;
                                if nat_eq(a1014, a1015) {
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
                Ok(next) => { m1016 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f377(v0: Adt115, v1: List<u64>) -> Adt115 {
    let m1019 = v0;
    match m1019 {
        Adt115::C0 => {
            Adt115::C1(v1)
        }
        Adt115::C1(_) => {
            Adt115::C2
        }
        Adt115::C2 => {
            Adt115::C2
        }
    }
}

pub fn f378(v0: List<u64>) -> List<u64> {
    f386(f438(f386(f438(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f379(p1022: List<u64>) -> bool {
    let m1021 = p1022;
    {
        let mut m1021 = m1021;
        loop {
            let step = {
                let v0 = m1021;
                let m1020 = v0;
                match m1020.uncons() {
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
                Ok(next) => { m1021 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f381() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f382() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f383(v0: List<u64>, p1025: List<List<u64>>) -> R<bool> {
    let m1024 = p1025;
    {
        let mut m1024 = m1024;
        loop {
            let step = {
                let v1 = m1024;
                let m1023 = v1;
                match m1023.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f376(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1024 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(v0: List<u64>) -> Option<List<u64>> {
    let m1026 = v0;
    match m1026.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m1027 = v2;
            match m1027.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a1028 = v1;
                    let a1029 = 47u64;
                    if if nat_eq(a1028, a1029) {
                        let a1030 = v3;
                        let a1031 = 47u64;
                        nat_eq(a1030, a1031)
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

pub fn f385(v0: List<u64>) -> bool {
    let m1032 = v0;
    match m1032.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a1033 = v1;
            let a1034 = 58u64;
            if nat_eq(a1033, a1034) {
                let m1035 = v2.clone();
                match m1035.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f440(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f386(p1038: List<u64>, p1039: List<u64>) -> List<u64> {
    let m1037 = (p1038, p1039);
    {
        let mut m1037 = m1037;
        loop {
            let step = {
                let (v0, v1) = m1037;
                let m1036 = v0;
                match m1036.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f387(v0: Adt3, v1: Fn1, v2: u64, v3: List<Adt95>, v4: Adt118) -> R<Adt91> {
    f441(f424(v3.clone())?, f425(v0.clone(), v1, v2, Some(f426(v3, match v0 {
        Adt3::C0(_, _, _, _, h1040, _, _) => {
            h1040
        }
    }, f427(v4.clone()), match v4.clone() {
        Adt118::C0(_, h1041, _) => {
            h1041
        }
    })?), f428(), v4)?)
}

pub fn f388(v0: u64) -> bool {
    let a1042 = 48u64;
    let a1043 = v0;
    if if nat_le(a1042, a1043) {
        let a1044 = v0;
        let a1045 = 57u64;
        nat_le(a1044, a1045)
    } else {
        false
    } {
        true
    } else {
        let a1046 = 65u64;
        let a1047 = v0;
        if if nat_le(a1046, a1047) {
            let a1048 = v0;
            let a1049 = 90u64;
            nat_le(a1048, a1049)
        } else {
            false
        } {
            true
        } else {
            let a1050 = 97u64;
            let a1051 = v0;
            if if nat_le(a1050, a1051) {
                let a1052 = v0;
                let a1053 = 122u64;
                nat_le(a1052, a1053)
            } else {
                false
            } {
                true
            } else {
                let a1054 = v0;
                let a1055 = 33u64;
                if nat_eq(a1054, a1055) {
                    true
                } else {
                    let a1056 = v0;
                    let a1057 = 35u64;
                    if nat_eq(a1056, a1057) {
                        true
                    } else {
                        let a1058 = v0;
                        let a1059 = 36u64;
                        if nat_eq(a1058, a1059) {
                            true
                        } else {
                            let a1060 = v0;
                            let a1061 = 37u64;
                            if nat_eq(a1060, a1061) {
                                true
                            } else {
                                let a1062 = v0;
                                let a1063 = 38u64;
                                if nat_eq(a1062, a1063) {
                                    true
                                } else {
                                    let a1064 = v0;
                                    let a1065 = 39u64;
                                    if nat_eq(a1064, a1065) {
                                        true
                                    } else {
                                        let a1066 = v0;
                                        let a1067 = 42u64;
                                        if nat_eq(a1066, a1067) {
                                            true
                                        } else {
                                            let a1068 = v0;
                                            let a1069 = 43u64;
                                            if nat_eq(a1068, a1069) {
                                                true
                                            } else {
                                                let a1070 = v0;
                                                let a1071 = 45u64;
                                                if nat_eq(a1070, a1071) {
                                                    true
                                                } else {
                                                    let a1072 = v0;
                                                    let a1073 = 46u64;
                                                    if nat_eq(a1072, a1073) {
                                                        true
                                                    } else {
                                                        let a1074 = v0;
                                                        let a1075 = 94u64;
                                                        if nat_eq(a1074, a1075) {
                                                            true
                                                        } else {
                                                            let a1076 = v0;
                                                            let a1077 = 95u64;
                                                            if nat_eq(a1076, a1077) {
                                                                true
                                                            } else {
                                                                let a1078 = v0;
                                                                let a1079 = 96u64;
                                                                if nat_eq(a1078, a1079) {
                                                                    true
                                                                } else {
                                                                    let a1080 = v0;
                                                                    let a1081 = 124u64;
                                                                    if nat_eq(a1080, a1081) {
                                                                        true
                                                                    } else {
                                                                        let a1082 = v0;
                                                                        let a1083 = 126u64;
                                                                        nat_eq(a1082, a1083)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f389(v0: u64) -> bool {
    let a1084 = 48u64;
    let a1085 = v0;
    if if nat_le(a1084, a1085) {
        let a1086 = v0;
        let a1087 = 57u64;
        nat_le(a1086, a1087)
    } else {
        false
    } {
        true
    } else {
        let a1088 = 65u64;
        let a1089 = v0;
        if if nat_le(a1088, a1089) {
            let a1090 = v0;
            let a1091 = 90u64;
            nat_le(a1090, a1091)
        } else {
            false
        } {
            true
        } else {
            let a1092 = 97u64;
            let a1093 = v0;
            if if nat_le(a1092, a1093) {
                let a1094 = v0;
                let a1095 = 122u64;
                nat_le(a1094, a1095)
            } else {
                false
            } {
                true
            } else {
                let a1096 = v0;
                let a1097 = 95u64;
                if nat_eq(a1096, a1097) {
                    true
                } else {
                    let a1098 = v0;
                    let a1099 = 45u64;
                    if nat_eq(a1098, a1099) {
                        true
                    } else {
                        let a1100 = v0;
                        let a1101 = 46u64;
                        nat_eq(a1100, a1101)
                    }
                }
            }
        }
    }
}

pub fn f390() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f391(v0: Str, v1: Adt9, v2: List<(Str, Adt9)>, v3: bool, v4: List<Str>, v5: Adt116, v6: List<Adt104>) -> R<Result<(List<Adt104>, Adt116), Adt96>> {
    let m1102 = f442(v0.clone(), v1, v2.clone());
    match m1102 {
        None => {
            let m1103 = f443(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1103 {
                Adt144::C0(v8) => {
                    Ok(Err::<(List<Adt104>, Adt116), Adt96>(v8))
                }
                Adt144::C1(v9) => {
                    Ok(Ok::<(List<Adt104>, Adt116), Adt96>((f444(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt104>, Adt116), Adt96>(v7))
        }
    }
}

pub fn f392(v0: Adt9, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt106>) -> R<Adt119> {
    let m1104 = f445(v1.clone(), v2.clone())?;
    match m1104 {
        Adt119::C0(v4) => {
            Ok(Adt119::C0(v4))
        }
        Adt119::C1(v5) => {
            Ok(Adt119::C1(v5))
        }
        Adt119::C2(v6) => {
            Ok(Adt119::C2(v6))
        }
        Adt119::C3(_) => {
            f446(v0, v1, v2, v3)
        }
    }
}

pub fn f393(v0: List<Adt9>, v1: Adt9, v2: List<Adt106>, v3: u64, v4: List<Adt108>) -> Adt117 {
    let m1105 = v0;
    match m1105.uncons() {
        None => {
            let a1106 = v3;
            let a1107 = 1u64;
            if nat_eq(a1106, a1107) {
                Adt117::C0(Adt107::C0, v4)
            } else {
                Adt117::C0(Adt107::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt117::C0(Adt107::C0, List::cons(Adt108::C0(v5, v1.clone(), v2.clone()), List::cons(Adt108::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f394(v0: Option<Adt9>, v1: Adt9, v2: List<Adt106>, v3: List<Adt108>) -> Adt117 {
    let m1108 = v0;
    match m1108 {
        None => {
            Adt117::C0(Adt107::C0, v3)
        }
        Some(v4) => {
            Adt117::C0(Adt107::C0, List::cons(Adt108::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f395(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt106>) -> Adt119 {
    let m1109 = f297(Str::lit("anyOf"), v0);
    match m1109 {
        Adt110::C0 => {
            Adt119::C3(List::<Adt108>::nil())
        }
        Adt110::C1(v3) => {
            let m1110 = f278(v3);
            match m1110 {
                None => {
                    Adt119::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1111 = v4;
                    match m1111.uncons() {
                        None => {
                            Adt119::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt119::C3(List::cons(Adt108::C0(v5, v1.clone(), v2.clone()), List::cons(Adt108::C1(v6, v1, v2), List::<Adt108>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f396(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt106>) -> Adt119 {
    let m1112 = f297(Str::lit("oneOf"), v0);
    match m1112 {
        Adt110::C0 => {
            Adt119::C3(List::<Adt108>::nil())
        }
        Adt110::C1(v3) => {
            let m1113 = f278(v3);
            match m1113 {
                None => {
                    Adt119::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1114 = v4;
                    match m1114.uncons() {
                        None => {
                            Adt119::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt119::C3(List::cons(Adt108::C0(v5, v1.clone(), v2.clone()), List::cons(Adt108::C2(v6, v1, v2, 0u64), List::<Adt108>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f397(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt106>) -> Adt119 {
    let m1115 = f297(Str::lit("not"), v0);
    match m1115 {
        Adt110::C0 => {
            Adt119::C3(List::<Adt108>::nil())
        }
        Adt110::C1(v3) => {
            Adt119::C3(List::cons(Adt108::C0(v3, v1, v2.clone()), List::cons(Adt108::C3(v2), List::<Adt108>::nil())))
        }
    }
}

pub fn f398(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt106>) -> Adt119 {
    let m1116 = f297(Str::lit("if"), v0.clone());
    match m1116 {
        Adt110::C0 => {
            Adt119::C3(List::<Adt108>::nil())
        }
        Adt110::C1(v3) => {
            Adt119::C3(List::cons(Adt108::C0(v3, v1.clone(), v2.clone()), List::cons(Adt108::C4(f447(Str::lit("then"), v0.clone()), f447(Str::lit("else"), v0), v1, v2), List::<Adt108>::nil())))
        }
    }
}

pub fn f399(v0: List<Adt9>, v1: Adt9, v2: List<Adt106>, v3: List<Adt108>) -> Adt117 {
    let m1117 = v0;
    match m1117.uncons() {
        None => {
            Adt117::C0(Adt107::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt117::C0(Adt107::C0, List::cons(Adt108::C0(v4, v1.clone(), v2.clone()), List::cons(Adt108::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f400(v0: Adt106) -> R<Str> {
    let m1118 = v0;
    match m1118 {
        Adt106::C0(v1) => {
            f448(v1)
        }
        Adt106::C1(v2) => {
            let a1119 = f277(v2, 0i64)?;
            Ok(format_i64(a1119))
        }
    }
}

pub fn f401(v0: Str) -> R<bool> {
    let m1121 = f449(f29({
        let a1120 = v0;
        utf8_encode(a1120)
    })?)?;
    match m1121 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f402(v0: Adt41) -> bool {
    let m1123 = match v0 {
        Adt41::C0(_, _, h1122) => {
            h1122
        }
    };
    match m1123 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f403(v0: Adt53) -> R<Result<Adt36, Adt88>> {
    let m1125 = match v0.clone() {
        Adt53::C0(_, h1124) => {
            h1124
        }
    };
    match m1125.uncons() {
        None => {
            Ok(Ok::<Adt36, Adt88>(Adt36::C2(match v0 {
                Adt53::C0(h1126, _) => {
                    h1126
                }
            })))
        }
        Some((_, _)) => {
            if f450(match v0.clone() {
                Adt53::C0(_, h1127) => {
                    h1127
                }
            })? {
                Ok(Ok::<Adt36, Adt88>(Adt36::C3(v0)))
            } else {
                Ok(Err::<Adt36, Adt88>(Adt88::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f404(v0: Adt8, v1: Adt53) -> R<Result<Adt36, Adt88>> {
    let m1131 = {
        let m1130 = match v1.clone() {
            Adt53::C0(h1129, _) => {
                h1129
            }
        };
        match m1130 {
            Adt41::C0(_, h1128, _) => {
                h1128
            }
        }
    };
    match m1131 {
        None => {
            Ok(Err::<Adt36, Adt88>(Adt88::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1134 = f182(Adt9::C5(match v0.clone() {
                Adt8::C0(h1132) => {
                    h1132
                }
            }), Adt9::C5(match v0 {
                Adt8::C0(h1133) => {
                    h1133
                }
            }), v2)?;
            match m1134 {
                Adt105::C0 => {
                    f403(v1)
                }
                Adt105::C1(_, _) => {
                    Ok(Err::<Adt36, Adt88>(Adt88::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt105::C2(_) => {
                    Ok(Err::<Adt36, Adt88>(Adt88::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt105::C3(_) => {
                    Ok(Err::<Adt36, Adt88>(Adt88::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt105::C4 => {
                    Ok(Err::<Adt36, Adt88>(Adt88::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f405(p1140: List<u64>, p1141: List<u64>) -> Option<List<u64>> {
    let m1139 = (p1140, p1141);
    {
        let mut m1139 = m1139;
        loop {
            let step = {
                let (v0, v1) = m1139;
                let m1135 = v0;
                match m1135.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1136 = v1;
                        match m1136.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1137 = v2;
                                let a1138 = v4;
                                if nat_eq(a1137, a1138) {
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
                Ok(next) => { m1139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(v0: Str) -> R<List<u64>> {
    f29({
        let a1142 = v0;
        utf8_encode(a1142)
    })
}

pub fn f407() -> Str {
    Str::lit("test://template/")
}

pub fn f408(v0: List<u64>) -> List<u64> {
    f417(v0, List::<u64>::nil())
}

pub fn f409() -> Str {
    Str::lit("data")
}

pub fn f410(v0: List<u64>) -> bool {
    let m1143 = v0;
    match m1143.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f411(p1148: List<u64>) -> bool {
    let m1147 = p1148;
    {
        let mut m1147 = m1147;
        loop {
            let step = {
                let v0 = m1147;
                let m1144 = v0;
                match m1144.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1145 = v1;
                        let a1146 = 47u64;
                        if nat_eq(a1145, a1146) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(p1153: Bytes, p1154: List<Bytes>) -> Bytes {
    let m1152 = (p1153, p1154);
    {
        let mut m1152 = m1152;
        loop {
            let step = {
                let (v0, v1) = m1152;
                let m1149 = v1;
                match m1149.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1150 = v2;
                            let a1151 = v0;
                            append_bytes(a1150, a1151)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1152 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f413(p1157: List<Bytes>, p1158: List<u64>) -> List<Bytes> {
    let m1156 = (p1157, p1158);
    {
        let mut m1156 = m1156;
        loop {
            let step = {
                let (v0, v1) = m1156;
                let m1155 = v1;
                match m1155.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f451(f452(v2), v0), v3))
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

pub fn f414(v0: Adt110, v1: Str) -> bool {
    let m1159 = v0;
    match m1159 {
        Adt110::C0 => {
            false
        }
        Adt110::C1(v2) => {
            let m1160 = f275(v2);
            match m1160 {
                None => {
                    false
                }
                Some(v3) => {
                    let a1161 = v3;
                    let a1162 = v1;
                    equal(a1161, a1162)
                }
            }
        }
    }
}

pub fn f415(p1168: List<u64>, p1169: List<u64>) -> bool {
    let m1167 = (p1168, p1169);
    {
        let mut m1167 = m1167;
        loop {
            let step = {
                let (v0, v1) = m1167;
                let m1163 = v0;
                match m1163.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m1164 = v1;
                        match m1164.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a1165 = v2;
                                let a1166 = v4;
                                if nat_eq(a1165, a1166) {
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
                Ok(next) => { m1167 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416(p1174: u64, p1175: List<Adt121>, p1176: List<u64>) -> R<Result<List<u64>, Adt111>> {
    let m1173 = (p1174, (p1175, p1176));
    {
        let mut m1173 = m1173;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1173;
                let m1170 = v0;
                if m1170 == 0 {
                    Err::<(u64, (List<Adt121>, List<u64>)), R<Result<List<u64>, Adt111>>>(Ok(Err::<List<u64>, Adt111>(Adt111::C13)))
                } else {
                    let v3 = m1170 - 1;
                    let m1171 = v1;
                    match m1171.uncons() {
                        None => {
                            Err::<(u64, (List<Adt121>, List<u64>)), R<Result<List<u64>, Adt111>>>(Ok(Ok::<List<u64>, Adt111>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m1172 = f453(v4, v5, v2)?;
                            match m1172 {
                                Adt146::C0(v6) => {
                                    Err::<(u64, (List<Adt121>, List<u64>)), R<Result<List<u64>, Adt111>>>(Ok(Err::<List<u64>, Adt111>(v6)))
                                }
                                Adt146::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt121>, List<u64>)), R<Result<List<u64>, Adt111>>>((v3, (v7, v8)))
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

pub fn f417(p1179: List<u64>, p1180: List<u64>) -> List<u64> {
    let m1178 = (p1179, p1180);
    {
        let mut m1178 = m1178;
        loop {
            let step = {
                let (v0, v1) = m1178;
                let m1177 = v0;
                match m1177.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f418(v0: Adt103) -> Adt9 {
    let m1181 = v0;
    match m1181 {
        Adt103::C0(v1) => {
            Adt9::C3(v1)
        }
        Adt103::C1(v2) => {
            Adt9::C2(Adt10::C0(v2, 0i64))
        }
    }
}

pub fn f419(v0: Option<Adt9>) -> List<(Str, Adt9)> {
    let m1182 = v0;
    match m1182 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f420() -> i64 {
    -32700i64
}

pub fn f421() -> i64 {
    -32600i64
}

pub fn f422(v0: Adt113) -> Str {
    let m1183 = v0;
    match m1183 {
        Adt113::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt113::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt113::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt113::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt113::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt113::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt113::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt113::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt113::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f423(v0: bool, v1: Adt135) -> R<Adt98> {
    let m1185 = match v1.clone() {
        Adt135::C0(_, h1184) => {
            h1184
        }
    };
    match m1185 {
        Ok(v2) => {
            let m1186 = f454(v2.clone())?;
            match m1186 {
                None => {
                    Ok(f55())
                }
                Some(v3) => {
                    Ok(f455(v0, f249(f456(), f457(match v1 {
                        Adt135::C0(h1187, _) => {
                            h1187
                        }
                    }, v3))?, f458(match v2 {
                        Adt136::C0(_, _, h1188) => {
                            h1188
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v4) => {
            f249(f459(v4.clone()), f251(Some(match v1 {
                Adt135::C0(h1189, _) => {
                    h1189
                }
            }), f460(v4)?))
        }
    }
}

pub fn f424(p1194: List<Adt95>) -> R<bool> {
    let m1193 = p1194;
    {
        let mut m1193 = m1193;
        loop {
            let step = {
                let v0 = m1193;
                let m1190 = v0;
                match m1190.uncons() {
                    None => {
                        Err::<List<Adt95>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f376(match v1.clone() {
                            Adt95::C0(h1191, _) => {
                                h1191
                            }
                        }, f461())? {
                            if f462(f463(44u64, match v1 {
                                Adt95::C0(_, h1192) => {
                                    h1192
                                }
                            }, List::<u64>::nil(), List::<List<u64>>::nil()))? {
                                Err::<List<Adt95>, R<bool>>(Ok(true))
                            } else {
                                Ok::<List<Adt95>, R<bool>>(v2)
                            }
                        } else {
                            Ok::<List<Adt95>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1193 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425(v0: Adt3, v1: Fn1, v2: u64, v3: Option<Adt122>, v4: List<Str>, v5: Adt118) -> R<Adt135> {
    Ok(Adt135::C0(match v5.clone() {
        Adt118::C0(h1195, _, _) => {
            h1195
        }
    }, f464(v0, v1, v2, f465(v3, v4, v5)?)?))
}

pub fn f426(v0: List<Adt95>, v1: List<Adt15>, v2: Option<Str>, v3: Str) -> R<Adt122> {
    let v4: Adt141 = f466(Adt124::C0, f258(v0.clone(), f467())?);
    let v5: Adt141 = f466(Adt124::C1, f258(v0.clone(), f468())?);
    let v6: Adt141 = if f469(v3) {
        f466(Adt124::C2, f470(f258(v0.clone(), f471())?)?)
    } else {
        f472()
    };
    Ok(Adt122::C0(match v4.clone() {
        Adt141::C0(h1196, _) => {
            h1196
        }
    }, match v5.clone() {
        Adt141::C0(h1197, _) => {
            h1197
        }
    }, match v6.clone() {
        Adt141::C0(h1198, _) => {
            h1198
        }
    }, f473(match v4 {
        Adt141::C0(_, h1199) => {
            h1199
        }
    }, f473(match v5 {
        Adt141::C0(_, h1200) => {
            h1200
        }
    }, match v6 {
        Adt141::C0(_, h1201) => {
            h1201
        }
    })), f474(v0, v1, v2)?))
}

pub fn f427(v0: Adt118) -> Option<Str> {
    let a1203 = match v0.clone() {
        Adt118::C0(_, h1202, _) => {
            h1202
        }
    };
    let a1204 = Str::lit("tools/call");
    if equal(a1203, a1204) {
        let m1206 = match v0 {
            Adt118::C0(_, _, h1205) => {
                h1205
            }
        };
        match m1206 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m1207 = f225(Str::lit("name"), v1);
                match m1207 {
                    Adt110::C0 => {
                        None::<Str>
                    }
                    Adt110::C1(v2) => {
                        f275(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f428() -> List<Str> {
    List::cons(f475(), List::<Str>::nil())
}

pub fn f429(v0: Adt8) -> R<Adt142> {
    let m1208 = f225(Str::lit("id"), v0);
    match m1208 {
        Adt110::C0 => {
            Ok(Adt142::C0)
        }
        Adt110::C1(v1) => {
            let m1209 = v1;
            match m1209 {
                Adt9::C0 => {
                    Ok(Adt142::C2)
                }
                Adt9::C1(_) => {
                    Ok(Adt142::C2)
                }
                Adt9::C2(v3) => {
                    let m1210 = f476(v3)?;
                    match m1210 {
                        None => {
                            Ok(Adt142::C2)
                        }
                        Some(v4) => {
                            Ok(Adt142::C1(Adt103::C1(v4)))
                        }
                    }
                }
                Adt9::C3(v2) => {
                    Ok(Adt142::C1(Adt103::C0(v2)))
                }
                Adt9::C4(_) => {
                    Ok(Adt142::C2)
                }
                Adt9::C5(_) => {
                    Ok(Adt142::C2)
                }
            }
        }
    }
}

pub fn f430(v0: Adt8) -> Adt113 {
    if f431(v0) {
        Adt113::C8
    } else {
        Adt113::C3
    }
}

pub fn f431(v0: Adt8) -> bool {
    if f477(Str::lit("result"), v0.clone()) {
        true
    } else {
        f477(Str::lit("error"), v0)
    }
}

pub fn f432(v0: Adt103, v1: Adt8) -> Result<Adt118, Adt112> {
    let m1211 = f225(Str::lit("jsonrpc"), v1.clone());
    match m1211 {
        Adt110::C0 => {
            f372(Some(v0), Adt113::C2)
        }
        Adt110::C1(v2) => {
            let m1212 = f275(v2);
            match m1212 {
                None => {
                    f372(Some(v0), Adt113::C2)
                }
                Some(v3) => {
                    let a1213 = v3;
                    let a1214 = f363();
                    if equal(a1213, a1214) {
                        f478(v0, v1)
                    } else {
                        f372(Some(v0), Adt113::C2)
                    }
                }
            }
        }
    }
}

pub fn f433(v0: u64, v1: u64) -> Adt127 {
    let a1215 = v1;
    let a1216 = 0u64;
    if nat_eq(a1215, a1216) {
        Adt127::C0
    } else {
        let a1217 = v1;
        let a1218 = 2u64;
        if nat_eq(a1217, a1218) {
            Adt127::C2(v0)
        } else {
            Adt127::C1({
                let a1222 = v0;
                let a1223 = {
                    let a1219 = v1;
                    let a1220 = 3u64;
                    let a1221 = 0u64;
                    nat_quot(a1219, a1220, a1221)
                };
                nat_sub(a1222, a1223)
            })
        }
    }
}

pub fn f434(p1251: Adt128, p1252: u64, p1253: u64, p1254: List<u64>) -> R<u64> {
    let m1250 = (p1251, (p1252, (p1253, p1254)));
    {
        let mut m1250 = m1250;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1250;
                let m1224 = v3;
                match m1224.uncons() {
                    None => {
                        let m1225 = v0;
                        match m1225 {
                            Adt128::C0 => {
                                Err::<(Adt128, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt128::C1 => {
                                Err::<(Adt128, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt128::C2 => {
                                Err::<(Adt128, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt128::C3 => {
                                Err::<(Adt128, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1226 = v0;
                        match m1226 {
                            Adt128::C0 => {
                                let a1227 = v4;
                                let a1228 = 128u64;
                                if nat_lt(a1227, a1228) {
                                    Ok::<(Adt128, (u64, (u64, List<u64>))), R<u64>>((Adt128::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a1229 = v4;
                                    let a1230 = 194u64;
                                    if nat_lt(a1229, a1230) {
                                        let v6: u64 = f479({
                                            let a1231 = List::cons(v4, v5);
                                            length_list(a1231)
                                        })?;
                                        Err::<(Adt128, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a1232 = v4;
                                        let a1233 = 224u64;
                                        if nat_lt(a1232, a1233) {
                                            Ok::<(Adt128, (u64, (u64, List<u64>))), R<u64>>((Adt128::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a1234 = v4;
                                            let a1235 = 224u64;
                                            if nat_eq(a1234, a1235) {
                                                Ok::<(Adt128, (u64, (u64, List<u64>))), R<u64>>((Adt128::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a1236 = v4;
                                                let a1237 = 237u64;
                                                if nat_eq(a1236, a1237) {
                                                    Ok::<(Adt128, (u64, (u64, List<u64>))), R<u64>>((Adt128::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a1238 = v4;
                                                    let a1239 = 240u64;
                                                    if nat_lt(a1238, a1239) {
                                                        Ok::<(Adt128, (u64, (u64, List<u64>))), R<u64>>((Adt128::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a1240 = v4;
                                                        let a1241 = 240u64;
                                                        if nat_eq(a1240, a1241) {
                                                            Ok::<(Adt128, (u64, (u64, List<u64>))), R<u64>>((Adt128::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a1242 = v4;
                                                            let a1243 = 244u64;
                                                            if nat_eq(a1242, a1243) {
                                                                Ok::<(Adt128, (u64, (u64, List<u64>))), R<u64>>((Adt128::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a1244 = v4;
                                                                let a1245 = 244u64;
                                                                if nat_lt(a1244, a1245) {
                                                                    Ok::<(Adt128, (u64, (u64, List<u64>))), R<u64>>((Adt128::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f479({
                                                                        let a1246 = List::cons(v4, v5);
                                                                        length_list(a1246)
                                                                    })?;
                                                                    Err::<(Adt128, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt128::C1 => {
                                if f480(v4, v1, v2) {
                                    Ok::<(Adt128, (u64, (u64, List<u64>))), R<u64>>((Adt128::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f479({
                                        let a1247 = List::cons(v4, v5);
                                        length_list(a1247)
                                    })?;
                                    Err::<(Adt128, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt128::C2 => {
                                if f480(v4, v1, v2) {
                                    Ok::<(Adt128, (u64, (u64, List<u64>))), R<u64>>((Adt128::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f479({
                                        let a1248 = List::cons(v4, v5);
                                        length_list(a1248)
                                    })?;
                                    Err::<(Adt128, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt128::C3 => {
                                if f480(v4, v1, v2) {
                                    Ok::<(Adt128, (u64, (u64, List<u64>))), R<u64>>((Adt128::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f479({
                                        let a1249 = List::cons(v4, v5);
                                        length_list(a1249)
                                    })?;
                                    Err::<(Adt128, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
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

pub fn f435(v0: Adt129, v1: u64) -> R<Result<Adt9, Adt111>> {
    let m1256 = match v0.clone() {
        Adt129::C0(h1255, _, _) => {
            h1255
        }
    };
    match m1256 {
        Adt130::C0 => {
            Ok(Err::<Adt9, Adt111>(Adt111::C3))
        }
        Adt130::C1 => {
            Ok(Err::<Adt9, Adt111>(Adt111::C3))
        }
        Adt130::C2 => {
            Ok(Err::<Adt9, Adt111>(Adt111::C3))
        }
        Adt130::C3 => {
            Ok(Err::<Adt9, Adt111>(Adt111::C3))
        }
        Adt130::C4 => {
            Ok(Err::<Adt9, Adt111>(Adt111::C3))
        }
        Adt130::C5 => {
            Ok(Err::<Adt9, Adt111>(Adt111::C3))
        }
        Adt130::C6(_, _, _) => {
            Ok(Err::<Adt9, Adt111>(Adt111::C3))
        }
        Adt130::C7(_, _) => {
            Ok(Err::<Adt9, Adt111>(Adt111::C3))
        }
        Adt130::C8(v3) => {
            let m1257 = f481(v3)?;
            match m1257 {
                Adt149::C0 => {
                    Ok(Err::<Adt9, Adt111>(Adt111::C7(v1)))
                }
                Adt149::C1 => {
                    Ok(Err::<Adt9, Adt111>(Adt111::C8(v1)))
                }
                Adt149::C2(v4) => {
                    let m1262 = {
                        let m1261 = f482(v4, match v0.clone() {
                            Adt129::C0(_, h1259, _) => {
                                h1259
                            }
                        }, match v0 {
                            Adt129::C0(_, _, h1260) => {
                                h1260
                            }
                        });
                        match m1261 {
                            Adt129::C0(h1258, _, _) => {
                                h1258
                            }
                        }
                    };
                    match m1262 {
                        Adt130::C0 => {
                            Ok(Err::<Adt9, Adt111>(Adt111::C3))
                        }
                        Adt130::C1 => {
                            Ok(Err::<Adt9, Adt111>(Adt111::C3))
                        }
                        Adt130::C2 => {
                            Ok(Err::<Adt9, Adt111>(Adt111::C3))
                        }
                        Adt130::C3 => {
                            Ok(Err::<Adt9, Adt111>(Adt111::C3))
                        }
                        Adt130::C4 => {
                            Ok(Err::<Adt9, Adt111>(Adt111::C3))
                        }
                        Adt130::C5 => {
                            Ok(Err::<Adt9, Adt111>(Adt111::C3))
                        }
                        Adt130::C6(_, _, _) => {
                            Ok(Err::<Adt9, Adt111>(Adt111::C3))
                        }
                        Adt130::C7(_, _) => {
                            Ok(Err::<Adt9, Adt111>(Adt111::C3))
                        }
                        Adt130::C8(_) => {
                            Ok(Err::<Adt9, Adt111>(Adt111::C3))
                        }
                        Adt130::C9(v5) => {
                            Ok(Ok::<Adt9, Adt111>(v5))
                        }
                    }
                }
            }
        }
        Adt130::C9(v2) => {
            Ok(Ok::<Adt9, Adt111>(v2))
        }
    }
}

pub fn f436(v0: Adt129, v1: u64, v2: u64) -> R<Adt143> {
    let m1264 = match v0.clone() {
        Adt129::C0(h1263, _, _) => {
            h1263
        }
    };
    match m1264 {
        Adt130::C0 => {
            if f483(v2) {
                Ok(Adt143::C1(v0))
            } else {
                f484(v0, v1, v2)
            }
        }
        Adt130::C1 => {
            if f483(v2) {
                Ok(Adt143::C1(v0))
            } else {
                let a1265 = v2;
                let a1266 = 93u64;
                if nat_eq(a1265, a1266) {
                    Ok(f485(v0, v1))
                } else {
                    f484(v0, v1, v2)
                }
            }
        }
        Adt130::C2 => {
            Ok(f486(v0, v1, v2))
        }
        Adt130::C3 => {
            if f483(v2) {
                Ok(Adt143::C1(v0))
            } else {
                let a1267 = v2;
                let a1268 = 125u64;
                if nat_eq(a1267, a1268) {
                    Ok(f487(v0, v1))
                } else {
                    let a1269 = v2;
                    let a1270 = 34u64;
                    if nat_eq(a1269, a1270) {
                        Ok(f488(v0, Adt130::C6(Adt131::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f489(Adt111::C2(v1)))
                    }
                }
            }
        }
        Adt130::C4 => {
            if f483(v2) {
                Ok(Adt143::C1(v0))
            } else {
                let a1271 = v2;
                let a1272 = 34u64;
                if nat_eq(a1271, a1272) {
                    Ok(f488(v0, Adt130::C6(Adt131::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f489(Adt111::C2(v1)))
                }
            }
        }
        Adt130::C5 => {
            if f483(v2) {
                Ok(Adt143::C1(v0))
            } else {
                let a1273 = v2;
                let a1274 = 58u64;
                if nat_eq(a1273, a1274) {
                    Ok(f488(v0, Adt130::C0))
                } else {
                    Ok(f489(Adt111::C2(v1)))
                }
            }
        }
        Adt130::C6(v3, v4, v5) => {
            f490(v0, v1, v2, v3, v4, v5)
        }
        Adt130::C7(v6, v7) => {
            f491(v0, v1, v2, v6, v7)
        }
        Adt130::C8(v8) => {
            f492(v0, v1, v2, v8)
        }
        Adt130::C9(_) => {
            Ok(f486(v0, v1, v2))
        }
    }
}

pub fn f437(v0: u64) -> R<u64> {
    let a1275 = 64u64;
    let a1276 = v0;
    if if nat_lt(a1275, a1276) {
        let a1277 = v0;
        let a1278 = 91u64;
        nat_lt(a1277, a1278)
    } else {
        false
    } {
        let a1279 = v0;
        let a1280 = 32u64;
        nat_add(a1279, a1280)
    } else {
        Ok(v0)
    }
}

pub fn f438(p1283: List<u64>) -> List<u64> {
    let m1282 = p1283;
    {
        let mut m1282 = m1282;
        loop {
            let step = {
                let v0 = m1282;
                let m1281 = v0;
                match m1281.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f493(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
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

pub fn f439(v0: u64) -> bool {
    let a1284 = v0;
    let a1285 = 9u64;
    if nat_eq(a1284, a1285) {
        true
    } else {
        let a1286 = 31u64;
        let a1287 = v0;
        if nat_lt(a1286, a1287) {
            let a1288 = v0;
            let a1289 = 127u64;
            nat_lt(a1288, a1289)
        } else {
            false
        }
    }
}

pub fn f440(p1292: List<u64>) -> bool {
    let m1291 = p1292;
    {
        let mut m1291 = m1291;
        loop {
            let step = {
                let v0 = m1291;
                let m1290 = v0;
                match m1290.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f494(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: bool, v1: Adt135) -> R<Adt91> {
    let m1294 = f495(match v1.clone() {
        Adt135::C0(_, h1293) => {
            h1293
        }
    });
    match m1294 {
        None => {
            Ok(Adt91::C0(f54(f423(v0, v1)?)))
        }
        Some(v2) => {
            f496(v0, v1, v2)
        }
    }
}

pub fn f442(v0: Str, v1: Adt9, v2: List<(Str, Adt9)>) -> Option<Adt96> {
    let m1295 = f497(f498(), v2.clone());
    match m1295 {
        None => {
            let m1296 = f499(f500(), v2.clone());
            match m1296 {
                None => {
                    f501(v0, v1, v2)
                }
                Some(v4) => {
                    Some(Adt96::C5(v0, v4))
                }
            }
        }
        Some(v3) => {
            Some(Adt96::C4(v0, v3))
        }
    }
}

pub fn f443(v0: Str, v1: List<(Str, Adt9)>, v2: bool, v3: List<Str>, v4: Adt116) -> R<Adt144> {
    let m1297 = f297(Str::lit("x-mcp-header"), v1.clone());
    match m1297 {
        Adt110::C0 => {
            Ok(Adt144::C1(v4))
        }
        Adt110::C1(v5) => {
            f502(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f444(v0: List<(Str, Adt9)>, v1: bool, v2: List<Str>, v3: List<Adt104>) -> List<Adt104> {
    f503(Str::lit("properties"), v0.clone(), v1, v2, f503(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f504(Str::lit("items"), v0.clone(), f504(Str::lit("additionalProperties"), v0.clone(), f505(Str::lit("allOf"), v0.clone(), f505(Str::lit("anyOf"), v0.clone(), f505(Str::lit("oneOf"), v0.clone(), f504(Str::lit("not"), v0.clone(), f504(Str::lit("if"), v0.clone(), f504(Str::lit("then"), v0.clone(), f504(Str::lit("else"), v0, v3)))))))))))
}

pub fn f445(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt119> {
    Ok(f506(List::cons((Str::lit("type"), f507(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f508(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f509(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f510(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f510(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f510(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f510(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1298 = f511(v0.clone(), v1.clone())?;
        let a1299 = List::cons((Str::lit("required"), f512(v0, v1)), List::<(Str, Adt145)>::nil());
        append_list(a1298, a1299)
    })))))))))
}

pub fn f446(v0: Adt9, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt106>) -> R<Adt119> {
    Ok(f513(f514(v0, v1.clone(), v2.clone(), v3.clone()), f513(f515(v1.clone(), v2.clone(), v3.clone()), f513(f516(v1.clone(), v2.clone(), v3.clone()), f513(f517(v1.clone(), v2.clone(), v3.clone())?, f513(f518(v1.clone(), v2.clone(), v3.clone()), f513(f519(Adt109::C0, f395(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f513(f519(Adt109::C1, f396(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f513(f519(Adt109::C2, f397(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f519(Adt109::C3, f398(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f447(v0: Str, v1: List<(Str, Adt9)>) -> Option<Adt9> {
    let m1300 = f297(v0, v1);
    match m1300 {
        Adt110::C0 => {
            None::<Adt9>
        }
        Adt110::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f448(v0: Str) -> R<Str> {
    let m1303 = {
        let a1302 = f348(f349(), f417(f520(f29({
            let a1301 = v0.clone();
            utf8_encode(a1301)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1302)
    };
    match m1303 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f449(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1305 = f521({
        let a1304 = v0.clone();
        length_list(a1304)
    }, v0, List::<u64>::nil())?;
    match m1305 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f417(v1, List::<u64>::nil())))
        }
    }
}

pub fn f450(v0: List<Adt54>) -> R<bool> {
    let m1306 = v0;
    match m1306.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f522(v2, match v1 {
                Adt54::C0(h1307, _, _) => {
                    h1307
                }
            })
        }
    }
}

pub fn f451(p1316: Bytes, p1317: List<Bytes>) -> List<Bytes> {
    let m1315 = (p1316, p1317);
    {
        let mut m1315 = m1315;
        loop {
            let step = {
                let (v0, v1) = m1315;
                let m1308 = v1;
                match m1308.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1311 = {
                            let a1309 = v2.clone();
                            length_bytes(a1309)
                        };
                        let a1312 = {
                            let a1310 = v0.clone();
                            length_bytes(a1310)
                        };
                        if nat_eq(a1311, a1312) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1313 = v2;
                                let a1314 = v0;
                                append_bytes(a1313, a1314)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1315 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452(v0: u64) -> Bytes {
    let m1321 = {
        let a1318 = f523();
        let a1319 = v0;
        let a1320 = 1u64;
        slice_bytes(a1318, a1319, a1320)
    };
    match m1321 {
        None => {
            f349()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f453(v0: Adt121, v1: List<Adt121>, v2: List<u64>) -> R<Adt146> {
    let m1322 = v0;
    match m1322 {
        Adt121::C0(v3) => {
            Ok(Adt146::C1(v1, f417(v3, v2)))
        }
        Adt121::C1(v4, v5) => {
            f524(v4, v5, v1, v2)
        }
        Adt121::C2(v6, v7) => {
            let m1323 = v7;
            match m1323.uncons() {
                None => {
                    Ok(Adt146::C1(List::cons(Adt121::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt146::C1(List::cons(Adt121::C1(v6, v8), List::cons(Adt121::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt121::C3(v10, v11) => {
            let m1324 = v11.clone();
            match m1324.uncons() {
                None => {
                    Ok(Adt146::C1(List::cons(Adt121::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt146::C1(List::cons(Adt121::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt121::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt121::C4(v14, v15) => {
            let m1325 = v15;
            match m1325.uncons() {
                None => {
                    Ok(Adt146::C1(List::cons(Adt121::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt146::C1(List::cons(Adt121::C0({
                        let a1327 = f525({
                            let (h1326, _) = v16;
                            h1326
                        })?;
                        let a1328 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1327, a1328)
                    }), List::cons(Adt121::C1(v14, {
                        let (_, h1329) = v16;
                        h1329
                    }), List::cons(Adt121::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt121::C5(v18, v19) => {
            let m1330 = v19.clone();
            match m1330.uncons() {
                None => {
                    Ok(Adt146::C1(List::cons(Adt121::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt146::C1(List::cons(Adt121::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt121::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f454(v0: Adt136) -> R<Option<Adt8>> {
    let m1332 = f526(match v0.clone() {
        Adt136::C0(h1331, _, _) => {
            h1331
        }
    })?;
    match m1332 {
        None => {
            Ok(None::<Adt8>)
        }
        Some(v1) => {
            Ok(Some(Adt8::C0({
                let a1335 = v1;
                let a1336 = {
                    let m1334 = f527(v0);
                    match m1334 {
                        Adt8::C0(h1333) => {
                            h1333
                        }
                    }
                };
                append_list(a1335, a1336)
            })))
        }
    }
}

pub fn f455(v0: bool, v1: Adt98, v2: Option<List<List<u64>>>) -> Adt98 {
    let m1337 = v2;
    match m1337 {
        None => {
            f55()
        }
        Some(v3) => {
            f528(v0, v1, f166(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f456() -> u64 {
    200u64
}

pub fn f457(v0: Adt103, v1: Adt8) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("jsonrpc"), Adt9::C3(f363())), List::cons((Str::lit("id"), f418(v0)), List::cons((Str::lit("result"), Adt9::C5(match v1 {
        Adt8::C0(h1338) => {
            h1338
        }
    })), List::<(Str, Adt9)>::nil()))))
}

pub fn f458(p1342: List<Adt138>, p1343: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1341 = (p1342, p1343);
    {
        let mut m1341 = m1341;
        loop {
            let step = {
                let (v0, v1) = m1341;
                let m1339 = v0;
                match m1339.uncons() {
                    None => {
                        Err::<(List<Adt138>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1340 = f529(v2)?;
                        match m1340 {
                            None => {
                                Err::<(List<Adt138>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt138>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
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

pub fn f459(v0: Adt139) -> u64 {
    let m1344 = v0;
    match m1344 {
        Adt139::C0(_) => {
            f316()
        }
        Adt139::C1(_, _) => {
            f316()
        }
        Adt139::C2(_) => {
            f316()
        }
        Adt139::C3(_) => {
            f530()
        }
        Adt139::C4(v6) => {
            f531(v6)
        }
    }
}

pub fn f460(v0: Adt139) -> R<Adt84> {
    Ok(Adt84::C0(f532(v0.clone()), f533(v0.clone()), f534(v0)?))
}

pub fn f461() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f462(p1347: List<List<u64>>) -> R<bool> {
    let m1346 = p1347;
    {
        let mut m1346 = m1346;
        loop {
            let step = {
                let v0 = m1346;
                let m1345 = v0;
                match m1345.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f535(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1346 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: u64, p1352: List<u64>, p1353: List<u64>, p1354: List<List<u64>>) -> List<List<u64>> {
    let m1351 = (p1352, (p1353, p1354));
    {
        let mut m1351 = m1351;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1351;
                let m1348 = v1;
                match m1348.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f166(List::cons(f386(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1349 = v4;
                        let a1350 = v0;
                        if nat_eq(a1349, a1350) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f386(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
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

pub fn f464(v0: Adt3, v1: Fn1, v2: u64, v3: Result<Adt147, Adt139>) -> R<Result<Adt136, Adt139>> {
    let m1355 = v3;
    match m1355 {
        Ok(v5) => {
            f536(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt136, Adt139>(v4))
        }
    }
}

pub fn f465(v0: Option<Adt122>, v1: List<Str>, v2: Adt118) -> R<Result<Adt147, Adt139>> {
    Ok(f537(f538(match v2.clone() {
        Adt118::C0(_, h1356, _) => {
            h1356
        }
    }), f539(v0, v1, v2)?))
}

pub fn f466(v0: Adt124, v1: Adt114) -> Adt141 {
    let m1357 = v1;
    match m1357 {
        Adt114::C0 => {
            f472()
        }
        Adt114::C1 => {
            Adt141::C0(None::<Str>, Some(Adt123::C6(v0)))
        }
        Adt114::C2 => {
            Adt141::C0(None::<Str>, Some(Adt123::C7(v0)))
        }
        Adt114::C3(v2) => {
            let m1358 = f540(v2);
            match m1358 {
                None => {
                    Adt141::C0(None::<Str>, Some(Adt123::C6(v0)))
                }
                Some(v3) => {
                    Adt141::C0(Some(v3), None::<Adt123>)
                }
            }
        }
    }
}

pub fn f467() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f468() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f469(v0: Str) -> bool {
    let m1359 = f541(v0);
    match m1359 {
        Adt153::C0 => {
            false
        }
        Adt153::C1 => {
            true
        }
        Adt153::C2 => {
            true
        }
        Adt153::C3 => {
            true
        }
    }
}

pub fn f470(v0: Adt114) -> R<Adt114> {
    let m1360 = v0.clone();
    match m1360 {
        Adt114::C0 => {
            Ok(v0)
        }
        Adt114::C1 => {
            Ok(v0)
        }
        Adt114::C2 => {
            Ok(v0)
        }
        Adt114::C3(v1) => {
            let m1361 = f542(v1);
            match m1361 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1362 = f449(v2)?;
                    match m1362 {
                        None => {
                            Ok(Adt114::C1)
                        }
                        Some(v3) => {
                            Ok(Adt114::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f471() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f472() -> Adt141 {
    Adt141::C0(None::<Str>, None::<Adt123>)
}

pub fn f473(v0: Option<Adt123>, v1: Option<Adt123>) -> Option<Adt123> {
    let m1363 = v0.clone();
    match m1363 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f474(v0: List<Adt95>, v1: List<Adt15>, v2: Option<Str>) -> R<List<Adt125>> {
    Ok(f543(f544(v0, v1, v2, List::<Adt125>::nil())?, List::<Adt125>::nil()))
}

pub fn f475() -> Str {
    Str::lit("2026-07-28")
}

pub fn f476(v0: Adt10) -> R<Option<i64>> {
    let m1366 = f545(40u64, match v0.clone() {
        Adt10::C0(h1364, _) => {
            h1364
        }
    }, match v0 {
        Adt10::C0(_, h1365) => {
            h1365
        }
    })?;
    match m1366 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f546(v1))
        }
    }
}

pub fn f477(v0: Str, v1: Adt8) -> bool {
    f547(f225(v0, v1))
}

pub fn f478(v0: Adt103, v1: Adt8) -> Result<Adt118, Adt112> {
    let m1367 = f225(Str::lit("method"), v1.clone());
    match m1367 {
        Adt110::C0 => {
            f372(Some(v0), f430(v1))
        }
        Adt110::C1(v2) => {
            let m1368 = f275(v2);
            match m1368 {
                None => {
                    f372(Some(v0), Adt113::C4)
                }
                Some(v3) => {
                    f548(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f479(v0: u64) -> R<u64> {
    let a1371 = {
        let a1369 = v0;
        let a1370 = 3u64;
        nat_mul(a1369, a1370)?
    };
    let a1372 = 1u64;
    nat_add(a1371, a1372)
}

pub fn f480(v0: u64, v1: u64, v2: u64) -> bool {
    let a1373 = v1;
    let a1374 = v0;
    if nat_le(a1373, a1374) {
        let a1375 = v0;
        let a1376 = v2;
        nat_le(a1375, a1376)
    } else {
        false
    }
}

pub fn f481(v0: Adt132) -> R<Adt149> {
    if f549(match v0.clone() {
        Adt132::C0(_, h1377, _, _, _, _, _) => {
            h1377
        }
    }) {
        let v1: i64 = {
            let a1383 = if match v0.clone() {
                Adt132::C0(_, _, _, _, _, h1378, _) => {
                    h1378
                }
            } {
                let a1380 = match v0.clone() {
                    Adt132::C0(_, _, _, _, _, _, h1379) => {
                        h1379
                    }
                };
                int_neg(a1380)?
            } else {
                match v0.clone() {
                    Adt132::C0(_, _, _, _, _, _, h1381) => {
                        h1381
                    }
                }
            };
            let a1384 = match v0.clone() {
                Adt132::C0(_, _, _, _, h1382, _, _) => {
                    h1382
                }
            };
            int_sub(a1383, a1384)?
        };
        let m1387 = {
            let a1385 = v1;
            let a1386 = -9999i64;
            compare(a1385, a1386)
        };
        if if match m1387 {
            Ordering::Less => {
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
            let m1390 = {
                let a1388 = 9999i64;
                let a1389 = v1;
                compare(a1388, a1389)
            };
            match m1390 {
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
            Ok(Adt149::C1)
        } else {
            Ok(Adt149::C2(Adt9::C2(Adt10::C0(if match v0.clone() {
                Adt132::C0(h1391, _, _, _, _, _, _) => {
                    h1391
                }
            } {
                let a1393 = match v0 {
                    Adt132::C0(_, _, h1392, _, _, _, _) => {
                        h1392
                    }
                };
                int_neg(a1393)?
            } else {
                match v0 {
                    Adt132::C0(_, _, h1394, _, _, _, _) => {
                        h1394
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt149::C0)
    }
}

pub fn f482(v0: Adt9, v1: List<Adt134>, v2: u64) -> Adt129 {
    let m1395 = v1.clone();
    match m1395.uncons() {
        None => {
            Adt129::C0(Adt130::C9(v0), List::<Adt134>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1396 = v3;
            match m1396 {
                Adt134::C0(v5) => {
                    Adt129::C0(Adt130::C2, List::cons(Adt134::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt134::C1(v6, v7) => {
                    let m1397 = v7;
                    match m1397 {
                        None => {
                            Adt129::C0(Adt130::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt129::C0(Adt130::C2, List::cons(Adt134::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f483(v0: u64) -> bool {
    let a1398 = v0;
    let a1399 = 32u64;
    if nat_eq(a1398, a1399) {
        true
    } else {
        let a1400 = v0;
        let a1401 = 10u64;
        if nat_eq(a1400, a1401) {
            true
        } else {
            let a1402 = v0;
            let a1403 = 13u64;
            if nat_eq(a1402, a1403) {
                true
            } else {
                let a1404 = v0;
                let a1405 = 9u64;
                nat_eq(a1404, a1405)
            }
        }
    }
}

pub fn f484(v0: Adt129, v1: u64, v2: u64) -> R<Adt143> {
    let a1406 = v2;
    let a1407 = 34u64;
    if nat_eq(a1406, a1407) {
        Ok(f488(v0, Adt130::C6(Adt131::C0, List::<u64>::nil(), false)))
    } else {
        let a1408 = v2;
        let a1409 = 123u64;
        if nat_eq(a1408, a1409) {
            f550(v0, Adt134::C1(List::<(Str, Adt9)>::nil(), None::<Str>), Adt130::C3, v1)
        } else {
            let a1410 = v2;
            let a1411 = 91u64;
            if nat_eq(a1410, a1411) {
                f550(v0, Adt134::C0(List::<Adt9>::nil()), Adt130::C1, v1)
            } else {
                let a1412 = v2;
                let a1413 = 116u64;
                if nat_eq(a1412, a1413) {
                    Ok(f488(v0, Adt130::C7(0u64, 1u64)))
                } else {
                    let a1414 = v2;
                    let a1415 = 102u64;
                    if nat_eq(a1414, a1415) {
                        Ok(f488(v0, Adt130::C7(1u64, 1u64)))
                    } else {
                        let a1416 = v2;
                        let a1417 = 110u64;
                        if nat_eq(a1416, a1417) {
                            Ok(f488(v0, Adt130::C7(2u64, 1u64)))
                        } else {
                            let a1418 = v2;
                            let a1419 = 45u64;
                            if nat_eq(a1418, a1419) {
                                Ok(f488(v0, f551(true, Adt133::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1420 = v2;
                                let a1421 = 48u64;
                                if nat_eq(a1420, a1421) {
                                    Ok(f488(v0, f551(false, Adt133::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f480(v2, 49u64, 57u64) {
                                        Ok(f488(v0, f551(false, Adt133::C2, f552({
                                            let a1422 = v2;
                                            let a1423 = 48u64;
                                            nat_sub(a1422, a1423)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f489(Adt111::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f485(v0: Adt129, v1: u64) -> Adt143 {
    let m1425 = match v0.clone() {
        Adt129::C0(_, h1424, _) => {
            h1424
        }
    };
    match m1425.uncons() {
        None => {
            f489(Adt111::C2(v1))
        }
        Some((v2, v3)) => {
            let m1426 = v2;
            match m1426 {
                Adt134::C0(v4) => {
                    Adt143::C1(f482(Adt9::C4(f553(v4, List::<Adt9>::nil())), v3, {
                        let a1428 = match v0 {
                            Adt129::C0(_, _, h1427) => {
                                h1427
                            }
                        };
                        let a1429 = 1u64;
                        nat_sub(a1428, a1429)
                    }))
                }
                Adt134::C1(_, _) => {
                    f489(Adt111::C2(v1))
                }
            }
        }
    }
}

pub fn f486(v0: Adt129, v1: u64, v2: u64) -> Adt143 {
    let m1431 = match v0.clone() {
        Adt129::C0(h1430, _, _) => {
            h1430
        }
    };
    match m1431 {
        Adt130::C0 => {
            f489(Adt111::C2(v1))
        }
        Adt130::C1 => {
            f489(Adt111::C2(v1))
        }
        Adt130::C2 => {
            if f483(v2) {
                Adt143::C1(v0)
            } else {
                let a1432 = v2;
                let a1433 = 44u64;
                if nat_eq(a1432, a1433) {
                    f554(v0, v1)
                } else {
                    let a1434 = v2;
                    let a1435 = 93u64;
                    if nat_eq(a1434, a1435) {
                        f485(v0, v1)
                    } else {
                        let a1436 = v2;
                        let a1437 = 125u64;
                        if nat_eq(a1436, a1437) {
                            f487(v0, v1)
                        } else {
                            f489(Adt111::C2(v1))
                        }
                    }
                }
            }
        }
        Adt130::C3 => {
            f489(Adt111::C2(v1))
        }
        Adt130::C4 => {
            f489(Adt111::C2(v1))
        }
        Adt130::C5 => {
            f489(Adt111::C2(v1))
        }
        Adt130::C6(_, _, _) => {
            f489(Adt111::C2(v1))
        }
        Adt130::C7(_, _) => {
            f489(Adt111::C2(v1))
        }
        Adt130::C8(_) => {
            f489(Adt111::C2(v1))
        }
        Adt130::C9(_) => {
            if f483(v2) {
                Adt143::C1(v0)
            } else {
                f489(Adt111::C10(v1))
            }
        }
    }
}

pub fn f487(v0: Adt129, v1: u64) -> Adt143 {
    let m1439 = match v0.clone() {
        Adt129::C0(_, h1438, _) => {
            h1438
        }
    };
    match m1439.uncons() {
        None => {
            f489(Adt111::C2(v1))
        }
        Some((v2, v3)) => {
            let m1440 = v2;
            match m1440 {
                Adt134::C0(_) => {
                    f489(Adt111::C2(v1))
                }
                Adt134::C1(v5, _) => {
                    Adt143::C1(f482(Adt9::C5(f555(v5, List::<(Str, Adt9)>::nil())), v3, {
                        let a1442 = match v0 {
                            Adt129::C0(_, _, h1441) => {
                                h1441
                            }
                        };
                        let a1443 = 1u64;
                        nat_sub(a1442, a1443)
                    }))
                }
            }
        }
    }
}

pub fn f488(v0: Adt129, v1: Adt130) -> Adt143 {
    Adt143::C1(f556(v0, v1))
}

pub fn f489(v0: Adt111) -> Adt143 {
    Adt143::C0(v0)
}

pub fn f490(v0: Adt129, v1: u64, v2: u64, v3: Adt131, v4: List<u64>, v5: bool) -> R<Adt143> {
    let m1444 = v3;
    match m1444 {
        Adt131::C0 => {
            let a1445 = v2;
            let a1446 = 34u64;
            if nat_eq(a1445, a1446) {
                Ok(f557(v0, v1, v4, v5))
            } else {
                let a1447 = v2;
                let a1448 = 92u64;
                if nat_eq(a1447, a1448) {
                    Ok(f488(v0, Adt130::C6(Adt131::C1, v4, v5)))
                } else {
                    let a1449 = v2;
                    let a1450 = 32u64;
                    if nat_lt(a1449, a1450) {
                        Ok(f489(Adt111::C4(v1)))
                    } else {
                        Ok(f488(v0, Adt130::C6(Adt131::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt131::C1 => {
            let a1451 = v2;
            let a1452 = 117u64;
            if nat_eq(a1451, a1452) {
                Ok(f488(v0, Adt130::C6(Adt131::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1453 = f558(v2);
                let a1454 = 0u64;
                if nat_eq(a1453, a1454) {
                    Ok(f489(Adt111::C5(v1)))
                } else {
                    Ok(f488(v0, Adt130::C6(Adt131::C0, List::cons(f558(v2), v4), v5)))
                }
            }
        }
        Adt131::C2(v6, v7, v8) => {
            let a1455 = 15u64;
            let a1456 = f559(v2);
            if nat_lt(a1455, a1456) {
                Ok(f489(Adt111::C5(v1)))
            } else {
                let a1459 = {
                    let a1457 = v6;
                    let a1458 = 1u64;
                    nat_add(a1457, a1458)?
                };
                let a1460 = 4u64;
                if nat_eq(a1459, a1460) {
                    f560(v0, v1, {
                        let a1463 = {
                            let a1461 = v7;
                            let a1462 = 16u64;
                            nat_mul(a1461, a1462)?
                        };
                        let a1464 = f559(v2);
                        nat_add(a1463, a1464)?
                    }, v8, v4, v5)
                } else {
                    Ok(f488(v0, Adt130::C6(Adt131::C2({
                        let a1465 = v6;
                        let a1466 = 1u64;
                        nat_add(a1465, a1466)?
                    }, {
                        let a1469 = {
                            let a1467 = v7;
                            let a1468 = 16u64;
                            nat_mul(a1467, a1468)?
                        };
                        let a1470 = f559(v2);
                        nat_add(a1469, a1470)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt131::C3(v9) => {
            let a1471 = v2;
            let a1472 = 92u64;
            if nat_eq(a1471, a1472) {
                Ok(f488(v0, Adt130::C6(Adt131::C4(v9), v4, v5)))
            } else {
                Ok(f489(Adt111::C6(v1)))
            }
        }
        Adt131::C4(v10) => {
            let a1473 = v2;
            let a1474 = 117u64;
            if nat_eq(a1473, a1474) {
                Ok(f488(v0, Adt130::C6(Adt131::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f489(Adt111::C6(v1)))
            }
        }
    }
}

pub fn f491(v0: Adt129, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt143> {
    let a1475 = v2;
    let a1476 = f561(v3, v4);
    if nat_eq(a1475, a1476) {
        let a1480 = {
            let a1477 = v4;
            let a1478 = 1u64;
            nat_add(a1477, a1478)?
        };
        let a1481 = {
            let a1479 = f562(v3);
            length_list(a1479)
        };
        if nat_eq(a1480, a1481) {
            Ok(f563(v0, f564(v3)))
        } else {
            Ok(f488(v0, Adt130::C7(v3, {
                let a1482 = v4;
                let a1483 = 1u64;
                nat_add(a1482, a1483)?
            })))
        }
    } else {
        Ok(f489(Adt111::C2(v1)))
    }
}

pub fn f492(v0: Adt129, v1: u64, v2: u64, v3: Adt132) -> R<Adt143> {
    let m1485 = match v3.clone() {
        Adt132::C0(_, h1484, _, _, _, _, _) => {
            h1484
        }
    };
    match m1485 {
        Adt133::C0 => {
            let a1486 = v2;
            let a1487 = 48u64;
            if nat_eq(a1486, a1487) {
                Ok(f565(v0, v3, Adt133::C1))
            } else {
                if f480(v2, 49u64, 57u64) {
                    f566(v0, v1, v2, v3, false)
                } else {
                    Ok(f489(Adt111::C7(v1)))
                }
            }
        }
        Adt133::C1 => {
            let a1488 = v2;
            let a1489 = 46u64;
            if nat_eq(a1488, a1489) {
                Ok(f565(v0, v3, Adt133::C3))
            } else {
                if f567(v2) {
                    Ok(f565(v0, v3, Adt133::C5))
                } else {
                    if f568(v2) {
                        Ok(f489(Adt111::C7(v1)))
                    } else {
                        f569(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt133::C2 => {
            if f568(v2) {
                f566(v0, v1, v2, v3, false)
            } else {
                let a1490 = v2;
                let a1491 = 46u64;
                if nat_eq(a1490, a1491) {
                    Ok(f565(v0, v3, Adt133::C3))
                } else {
                    if f567(v2) {
                        Ok(f565(v0, v3, Adt133::C5))
                    } else {
                        f569(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt133::C3 => {
            if f568(v2) {
                f566(v0, v1, v2, v3, true)
            } else {
                Ok(f489(Adt111::C7(v1)))
            }
        }
        Adt133::C4 => {
            if f568(v2) {
                f566(v0, v1, v2, v3, true)
            } else {
                if f567(v2) {
                    Ok(f565(v0, v3, Adt133::C5))
                } else {
                    f569(v0, v1, v2, v3)
                }
            }
        }
        Adt133::C5 => {
            let a1492 = v2;
            let a1493 = 43u64;
            if nat_eq(a1492, a1493) {
                Ok(f488(v0, f551(match v3.clone() {
                    Adt132::C0(h1494, _, _, _, _, _, _) => {
                        h1494
                    }
                }, Adt133::C6, match v3.clone() {
                    Adt132::C0(_, _, h1495, _, _, _, _) => {
                        h1495
                    }
                }, match v3.clone() {
                    Adt132::C0(_, _, _, h1496, _, _, _) => {
                        h1496
                    }
                }, match v3 {
                    Adt132::C0(_, _, _, _, h1497, _, _) => {
                        h1497
                    }
                }, false, 0i64)))
            } else {
                let a1498 = v2;
                let a1499 = 45u64;
                if nat_eq(a1498, a1499) {
                    Ok(f488(v0, f551(match v3.clone() {
                        Adt132::C0(h1500, _, _, _, _, _, _) => {
                            h1500
                        }
                    }, Adt133::C6, match v3.clone() {
                        Adt132::C0(_, _, h1501, _, _, _, _) => {
                            h1501
                        }
                    }, match v3.clone() {
                        Adt132::C0(_, _, _, h1502, _, _, _) => {
                            h1502
                        }
                    }, match v3 {
                        Adt132::C0(_, _, _, _, h1503, _, _) => {
                            h1503
                        }
                    }, true, 0i64)))
                } else {
                    if f568(v2) {
                        f570(v0, v1, v2, Adt132::C0(match v3.clone() {
                            Adt132::C0(h1504, _, _, _, _, _, _) => {
                                h1504
                            }
                        }, match v3.clone() {
                            Adt132::C0(_, h1505, _, _, _, _, _) => {
                                h1505
                            }
                        }, match v3.clone() {
                            Adt132::C0(_, _, h1506, _, _, _, _) => {
                                h1506
                            }
                        }, match v3.clone() {
                            Adt132::C0(_, _, _, h1507, _, _, _) => {
                                h1507
                            }
                        }, match v3 {
                            Adt132::C0(_, _, _, _, h1508, _, _) => {
                                h1508
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f489(Adt111::C7(v1)))
                    }
                }
            }
        }
        Adt133::C6 => {
            if f568(v2) {
                f570(v0, v1, v2, v3.clone(), match v3 {
                    Adt132::C0(_, _, _, _, _, h1509, _) => {
                        h1509
                    }
                })
            } else {
                Ok(f489(Adt111::C7(v1)))
            }
        }
        Adt133::C7 => {
            if f568(v2) {
                f570(v0, v1, v2, v3.clone(), match v3 {
                    Adt132::C0(_, _, _, _, _, h1510, _) => {
                        h1510
                    }
                })
            } else {
                f569(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f493(v0: u64) -> bool {
    let a1511 = v0;
    let a1512 = 9u64;
    if nat_eq(a1511, a1512) {
        true
    } else {
        let a1513 = v0;
        let a1514 = 32u64;
        nat_eq(a1513, a1514)
    }
}

pub fn f494(v0: u64) -> bool {
    let a1515 = 47u64;
    let a1516 = v0;
    if nat_lt(a1515, a1516) {
        let a1517 = v0;
        let a1518 = 58u64;
        nat_lt(a1517, a1518)
    } else {
        false
    }
}

pub fn f495(v0: Result<Adt136, Adt139>) -> Option<Adt89> {
    let m1519 = v0;
    match m1519 {
        Ok(_) => {
            None::<Adt89>
        }
        Err(v2) => {
            let m1520 = v2;
            match m1520 {
                Adt139::C0(_) => {
                    None::<Adt89>
                }
                Adt139::C1(_, _) => {
                    None::<Adt89>
                }
                Adt139::C2(_) => {
                    None::<Adt89>
                }
                Adt139::C3(_) => {
                    None::<Adt89>
                }
                Adt139::C4(v8) => {
                    f571(v8)
                }
            }
        }
    }
}

pub fn f496(v0: bool, v1: Adt135, v2: Adt89) -> R<Adt91> {
    if f572(match v2.clone() {
        Adt89::C0(h1521, _) => {
            h1521
        }
    }) {
        Ok(Adt91::C1(v2))
    } else {
        Ok(Adt91::C0(f54(f423(v0, Adt135::C0(match v1 {
            Adt135::C0(h1522, _) => {
                h1522
            }
        }, Err::<Adt136, Adt139>(f573())))?)))
    }
}

pub fn f497(p1526: List<Str>, v1: List<(Str, Adt9)>) -> Option<Str> {
    let m1525 = p1526;
    {
        let mut m1525 = m1525;
        loop {
            let step = {
                let v0 = m1525;
                let m1523 = v0;
                match m1523.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1524 = f297(v2.clone(), v1.clone());
                        match m1524 {
                            Adt110::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt110::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1525 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f499(p1533: List<(Str, u64)>, v1: List<(Str, Adt9)>) -> Option<Str> {
    let m1532 = p1533;
    {
        let mut m1532 = m1532;
        loop {
            let step = {
                let v0 = m1532;
                let m1527 = v0;
                match m1527.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1529 = f297({
                            let (h1528, _) = v2.clone();
                            h1528
                        }, v1.clone());
                        match m1529 {
                            Adt110::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt110::C1(v4) => {
                                if f574({
                                    let (_, h1530) = v2;
                                    h1530
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1531, _) = v2;
                                        h1531
                                    }))
                                }
                            }
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

pub fn f500() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f501(v0: Str, v1: Adt9, v2: List<(Str, Adt9)>) -> Option<Adt96> {
    let m1534 = f297(Str::lit("$ref"), v2);
    match m1534 {
        Adt110::C0 => {
            None::<Adt96>
        }
        Adt110::C1(v3) => {
            let m1535 = f275(v3);
            match m1535 {
                None => {
                    None::<Adt96>
                }
                Some(v4) => {
                    let m1536 = f575(v1, v4.clone());
                    match m1536 {
                        None => {
                            Some(Adt96::C6(v0, v4))
                        }
                        Some(_) => {
                            None::<Adt96>
                        }
                    }
                }
            }
        }
    }
}

pub fn f502(v0: Str, v1: List<(Str, Adt9)>, v2: Adt9, v3: bool, v4: List<Str>, v5: Adt116) -> R<Adt144> {
    if v3 {
        let m1537 = f275(v2);
        match m1537 {
            None => {
                Ok(Adt144::C0(Adt96::C7(v0)))
            }
            Some(v6) => {
                f576(v0, v1, v6.clone(), f179(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt144::C0(Adt96::C10(v0)))
    }
}

pub fn f503(v0: Str, v1: List<(Str, Adt9)>, v2: bool, v3: List<Str>, v4: List<Adt104>) -> List<Adt104> {
    let m1538 = f297(v0.clone(), v1);
    match m1538 {
        Adt110::C0 => {
            v4
        }
        Adt110::C1(v5) => {
            let m1539 = f226(v5);
            match m1539 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1540 = v0;
                    let a1541 = Str::lit("properties");
                    if equal(a1540, a1541) {
                        f577(match v6 {
                            Adt8::C0(h1542) => {
                                h1542
                            }
                        }, v2, v3, v4)
                    } else {
                        f578(match v6 {
                            Adt8::C0(h1543) => {
                                h1543
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f504(v0: Str, v1: List<(Str, Adt9)>, v2: List<Adt104>) -> List<Adt104> {
    let m1544 = f297(v0, v1);
    match m1544 {
        Adt110::C0 => {
            v2
        }
        Adt110::C1(v3) => {
            List::cons(Adt104::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f505(v0: Str, v1: List<(Str, Adt9)>, v2: List<Adt104>) -> List<Adt104> {
    let m1545 = f297(v0, v1);
    match m1545 {
        Adt110::C0 => {
            v2
        }
        Adt110::C1(v3) => {
            let m1546 = f278(v3);
            match m1546 {
                None => {
                    v2
                }
                Some(v4) => {
                    f579(v4, v2)
                }
            }
        }
    }
}

pub fn f506(p1553: List<(Str, Adt145)>) -> Adt119 {
    let m1552 = p1553;
    {
        let mut m1552 = m1552;
        loop {
            let step = {
                let v0 = m1552;
                let m1547 = v0;
                match m1547.uncons() {
                    None => {
                        Err::<List<(Str, Adt145)>, Adt119>(Adt119::C3(List::<Adt108>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1549 = {
                            let (_, h1548) = v1;
                            h1548
                        };
                        match m1549 {
                            Adt145::C0 => {
                                Ok::<List<(Str, Adt145)>, Adt119>(v2)
                            }
                            Adt145::C1 => {
                                Err::<List<(Str, Adt145)>, Adt119>(Adt119::C0({
                                    let (h1550, _) = v1;
                                    h1550
                                }))
                            }
                            Adt145::C2 => {
                                Err::<List<(Str, Adt145)>, Adt119>(Adt119::C1({
                                    let (h1551, _) = v1;
                                    h1551
                                }))
                            }
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

pub fn f507(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt145> {
    let m1554 = f297(Str::lit("type"), v0);
    match m1554 {
        Adt110::C0 => {
            Ok(Adt145::C0)
        }
        Adt110::C1(v2) => {
            let m1555 = f275(v2.clone());
            match m1555 {
                None => {
                    let m1556 = f278(v2);
                    match m1556 {
                        None => {
                            Ok(Adt145::C2)
                        }
                        Some(v4) => {
                            f582(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f580(v3.clone()) {
                        if f581(v3, v1)? {
                            Ok(Adt145::C0)
                        } else {
                            Ok(Adt145::C1)
                        }
                    } else {
                        Ok(Adt145::C2)
                    }
                }
            }
        }
    }
}

pub fn f508(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt145> {
    let m1557 = f297(Str::lit("enum"), v0);
    match m1557 {
        Adt110::C0 => {
            Ok(Adt145::C0)
        }
        Adt110::C1(v2) => {
            let m1558 = f278(v2);
            match m1558 {
                None => {
                    Ok(Adt145::C2)
                }
                Some(v3) => {
                    f583(v3, v1)
                }
            }
        }
    }
}

pub fn f509(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt145> {
    let m1559 = f297(Str::lit("const"), v0);
    match m1559 {
        Adt110::C0 => {
            Ok(Adt145::C0)
        }
        Adt110::C1(v2) => {
            let m1560 = f584(v2, v1)?;
            match m1560 {
                Adt154::C0 => {
                    Ok(Adt145::C0)
                }
                Adt154::C1 => {
                    Ok(Adt145::C1)
                }
                Adt154::C2 => {
                    Ok(Adt145::C2)
                }
            }
        }
    }
}

pub fn f510(v0: Str, v1: u64, v2: List<(Str, Adt9)>, v3: Adt9) -> R<Adt145> {
    let m1561 = f297(v0, v2);
    match m1561 {
        Adt110::C0 => {
            Ok(Adt145::C0)
        }
        Adt110::C1(v4) => {
            let m1562 = f585(v4);
            match m1562 {
                None => {
                    Ok(Adt145::C2)
                }
                Some(v5) => {
                    let m1563 = f585(v3);
                    match m1563 {
                        None => {
                            Ok(Adt145::C0)
                        }
                        Some(v6) => {
                            if f586(v1, f587(v6, v5)?) {
                                Ok(Adt145::C0)
                            } else {
                                Ok(Adt145::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f511(v0: List<(Str, Adt9)>, v1: Adt9) -> R<List<(Str, Adt145)>> {
    let m1564 = v1;
    match m1564 {
        Adt9::C0 => {
            Ok(List::<(Str, Adt145)>::nil())
        }
        Adt9::C1(_) => {
            Ok(List::<(Str, Adt145)>::nil())
        }
        Adt9::C2(_) => {
            Ok(List::<(Str, Adt145)>::nil())
        }
        Adt9::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f588(Str::lit("minLength"), true, v0.clone(), f589(v4.clone())?)?), List::cons((Str::lit("maxLength"), f588(Str::lit("maxLength"), false, v0, f589(v4)?)?), List::<(Str, Adt145)>::nil())))
        }
        Adt9::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f588(Str::lit("minItems"), true, v0.clone(), f590(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f588(Str::lit("maxItems"), false, v0, f590(v5, 0u64)?)?), List::<(Str, Adt145)>::nil())))
        }
        Adt9::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f588(Str::lit("minProperties"), true, v0.clone(), f591(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f588(Str::lit("maxProperties"), false, v0, f591(v6, 0u64)?)?), List::<(Str, Adt145)>::nil())))
        }
    }
}

pub fn f512(v0: List<(Str, Adt9)>, v1: Adt9) -> Adt145 {
    let m1565 = f297(Str::lit("required"), v0);
    match m1565 {
        Adt110::C0 => {
            Adt145::C0
        }
        Adt110::C1(v2) => {
            let m1566 = f278(v2);
            match m1566 {
                None => {
                    Adt145::C2
                }
                Some(v3) => {
                    let m1567 = f226(v1);
                    match m1567 {
                        None => {
                            Adt145::C0
                        }
                        Some(v4) => {
                            f592(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f513(v0: Adt119, v1: Adt119) -> Adt119 {
    let m1568 = v0.clone();
    match m1568 {
        Adt119::C0(_) => {
            v0
        }
        Adt119::C1(_) => {
            v0
        }
        Adt119::C2(_) => {
            v0
        }
        Adt119::C3(v5) => {
            let m1569 = v1.clone();
            match m1569 {
                Adt119::C0(_) => {
                    v1
                }
                Adt119::C1(_) => {
                    v1
                }
                Adt119::C2(_) => {
                    v1
                }
                Adt119::C3(v9) => {
                    Adt119::C3({
                        let a1570 = v5;
                        let a1571 = v9;
                        append_list(a1570, a1571)
                    })
                }
            }
        }
    }
}

pub fn f514(v0: Adt9, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt106>) -> Adt119 {
    let m1572 = f297(Str::lit("$ref"), v1);
    match m1572 {
        Adt110::C0 => {
            Adt119::C3(List::<Adt108>::nil())
        }
        Adt110::C1(v4) => {
            let m1573 = f275(v4);
            match m1573 {
                None => {
                    Adt119::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1574 = f575(v0, v5.clone());
                    match m1574 {
                        None => {
                            Adt119::C2(v5)
                        }
                        Some(v6) => {
                            Adt119::C3(List::cons(Adt108::C0(v6, v2, v3), List::<Adt108>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f515(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt106>) -> Adt119 {
    let m1575 = f297(Str::lit("properties"), v0);
    match m1575 {
        Adt110::C0 => {
            Adt119::C3(List::<Adt108>::nil())
        }
        Adt110::C1(v3) => {
            let m1576 = f226(v3);
            match m1576 {
                None => {
                    Adt119::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1577 = f226(v1);
                    match m1577 {
                        None => {
                            Adt119::C3(List::<Adt108>::nil())
                        }
                        Some(v5) => {
                            Adt119::C3(f593(match v4 {
                                Adt8::C0(h1578) => {
                                    h1578
                                }
                            }, match v5 {
                                Adt8::C0(h1579) => {
                                    h1579
                                }
                            }, v2, List::<Adt108>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f516(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt106>) -> Adt119 {
    let m1580 = f297(Str::lit("additionalProperties"), v0.clone());
    match m1580 {
        Adt110::C0 => {
            Adt119::C3(List::<Adt108>::nil())
        }
        Adt110::C1(v3) => {
            let m1581 = f226(v1);
            match m1581 {
                None => {
                    Adt119::C3(List::<Adt108>::nil())
                }
                Some(v4) => {
                    let m1582 = f346(v3.clone());
                    match m1582 {
                        None => {
                            Adt119::C3(f596(match v4 {
                                Adt8::C0(h1584) => {
                                    h1584
                                }
                            }, f595(v0), v3, v2, List::<Adt108>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt119::C3(List::<Adt108>::nil())
                            } else {
                                if f594(match v4 {
                                    Adt8::C0(h1583) => {
                                        h1583
                                    }
                                }, f595(v0)) {
                                    Adt119::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt119::C3(List::<Adt108>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f517(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt106>) -> R<Adt119> {
    let m1585 = f297(Str::lit("items"), v0);
    match m1585 {
        Adt110::C0 => {
            Ok(Adt119::C3(List::<Adt108>::nil()))
        }
        Adt110::C1(v3) => {
            let m1586 = f278(v1);
            match m1586 {
                None => {
                    Ok(Adt119::C3(List::<Adt108>::nil()))
                }
                Some(v4) => {
                    Ok(Adt119::C3(f597(v4, 0u64, v3, v2, List::<Adt108>::nil())?))
                }
            }
        }
    }
}

pub fn f518(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt106>) -> Adt119 {
    let m1587 = f297(Str::lit("allOf"), v0);
    match m1587 {
        Adt110::C0 => {
            Adt119::C3(List::<Adt108>::nil())
        }
        Adt110::C1(v3) => {
            let m1588 = f278(v3);
            match m1588 {
                None => {
                    Adt119::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1589 = v4.clone();
                    match m1589.uncons() {
                        None => {
                            Adt119::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt119::C3(f598(v4, v1, v2, List::<Adt108>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f519(v0: Adt109, v1: Adt119, v2: List<(Str, Adt9)>, v3: Adt9, v4: List<Adt106>) -> Adt119 {
    let m1590 = v1.clone();
    match m1590 {
        Adt119::C0(_) => {
            v1
        }
        Adt119::C1(_) => {
            v1
        }
        Adt119::C2(_) => {
            v1
        }
        Adt119::C3(v8) => {
            let m1591 = v8;
            match m1591.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt119::C3(List::cons(Adt108::C5(v0, Adt9::C5(v2), v3, v4), List::<Adt108>::nil()))
                }
            }
        }
    }
}

pub fn f520(p1598: List<u64>, p1599: List<u64>) -> List<u64> {
    let m1597 = (p1598, p1599);
    {
        let mut m1597 = m1597;
        loop {
            let step = {
                let (v0, v1) = m1597;
                let m1592 = v0;
                match m1592.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1593 = v2;
                        let a1594 = 126u64;
                        if nat_eq(a1593, a1594) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1595 = v2;
                            let a1596 = 47u64;
                            if nat_eq(a1595, a1596) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(p1608: u64, p1609: List<u64>, p1610: List<u64>) -> R<Option<List<u64>>> {
    let m1607 = (p1608, (p1609, p1610));
    {
        let mut m1607 = m1607;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1607;
                let m1600 = v0;
                if m1600 == 0 {
                    let m1601 = v1;
                    match m1601.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1600 - 1;
                    let m1602 = v1;
                    match m1602.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1603 = v7;
                            match m1603.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1604 = v9;
                                    match m1604.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1605 = v11;
                                            match m1605.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1606 = f599(v6, v8, v10, v12, f600(v13.clone()))?;
                                                    match m1606 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f417(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1607 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f522(p1615: List<Adt54>, p1616: Adt10) -> R<bool> {
    let m1614 = (p1615, p1616);
    {
        let mut m1614 = m1614;
        loop {
            let step = {
                let (v0, v1) = m1614;
                let m1611 = v0;
                match m1611.uncons() {
                    None => {
                        Err::<(List<Adt54>, Adt10), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f601(f587(match v2.clone() {
                            Adt54::C0(h1612, _, _) => {
                                h1612
                            }
                        }, v1)?) {
                            Ok::<(List<Adt54>, Adt10), R<bool>>((v3, match v2 {
                                Adt54::C0(h1613, _, _) => {
                                    h1613
                                }
                            }))
                        } else {
                            Err::<(List<Adt54>, Adt10), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1614 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f524(v0: u64, v1: Adt9, v2: List<Adt121>, v3: List<u64>) -> R<Adt146> {
    let m1617 = v1;
    match m1617 {
        Adt9::C0 => {
            Ok(Adt146::C1(v2, f417(f602(2u64), v3)))
        }
        Adt9::C1(v4) => {
            Ok(Adt146::C1(v2, f417(if v4 {
                f602(0u64)
            } else {
                f602(1u64)
            }, v3)))
        }
        Adt9::C2(v5) => {
            if f603(v5.clone())? {
                Ok(Adt146::C1(v2, f417(f604(v5)?, v3)))
            } else {
                Ok(Adt146::C0(Adt111::C11))
            }
        }
        Adt9::C3(v6) => {
            Ok(Adt146::C1(v2, f417(f525(v6)?, v3)))
        }
        Adt9::C4(v7) => {
            let a1618 = v0;
            let a1619 = f605();
            if nat_lt(a1618, a1619) {
                Ok(Adt146::C1(f606(91u64, Adt121::C2({
                    let a1620 = v0;
                    let a1621 = 1u64;
                    nat_add(a1620, a1621)?
                }, v7), v2), v3))
            } else {
                Ok(Adt146::C0(Adt111::C12))
            }
        }
        Adt9::C5(v8) => {
            let a1622 = v0;
            let a1623 = f605();
            if nat_lt(a1622, a1623) {
                Ok(Adt146::C1(f606(123u64, Adt121::C4({
                    let a1624 = v0;
                    let a1625 = 1u64;
                    nat_add(a1624, a1625)?
                }, v8), v2), v3))
            } else {
                Ok(Adt146::C0(Adt111::C12))
            }
        }
    }
}

pub fn f525(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1627 = f417(f607(f29({
            let a1626 = v0;
            utf8_encode(a1626)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1628 = List::cons(34u64, List::<u64>::nil());
        append_list(a1627, a1628)
    }))
}

pub fn f526(v0: Adt36) -> R<Option<List<(Str, Adt9)>>> {
    let m1629 = v0;
    match m1629 {
        Adt36::C0(v1) => {
            Ok(Some(f608(v1)?))
        }
        Adt36::C1(v2) => {
            Ok(Some(f609(v2)?))
        }
        Adt36::C2(v3) => {
            f610(f611(v3)?)
        }
        Adt36::C3(v4) => {
            f610(f611(match v4 {
                Adt53::C0(h1630, _) => {
                    h1630
                }
            })?)
        }
        Adt36::C4(v5) => {
            Ok(Some(f612(v5)?))
        }
        Adt36::C5(v6) => {
            Ok(Some(f613(v6)?))
        }
        Adt36::C6(v7) => {
            Ok(Some(f614(v7)?))
        }
        Adt36::C7(v8) => {
            Ok(Some(f615(v8)?))
        }
        Adt36::C8(v9) => {
            f610(f616(v9)?)
        }
        Adt36::C9(v10) => {
            Ok(Some(f617(v10)?))
        }
        Adt36::C10(_) => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Adt36::C11 => {
            Ok(Some(List::<(Str, Adt9)>::nil()))
        }
        Adt36::C12(v12) => {
            f610(f618(v12)?)
        }
        Adt36::C13(_) => {
            Ok(None::<List<(Str, Adt9)>>)
        }
    }
}

pub fn f527(v0: Adt136) -> Adt8 {
    Adt8::C0(List::cons((Str::lit("resultType"), Adt9::C3(f619(f620(match v0.clone() {
        Adt136::C0(h1631, _, _) => {
            h1631
        }
    })))), List::cons((Str::lit("_meta"), Adt9::C5(f621({
        let m1634 = match v0 {
            Adt136::C0(_, h1633, _) => {
                h1633
            }
        };
        match m1634 {
            Adt137::C0(h1632, _) => {
                h1632
            }
        }
    }))), List::<(Str, Adt9)>::nil())))
}

pub fn f528(v0: bool, v1: Adt98, v2: List<List<u64>>) -> Adt98 {
    if f622(v0, match v1.clone() {
        Adt98::C0(h1635, _, _, _) => {
            h1635
        }
    }, v2.clone()) {
        f623(match v1.clone() {
            Adt98::C0(h1636, _, _, _) => {
                h1636
            }
        }, {
            let a1638 = v2;
            let a1639 = List::cons(match v1 {
                Adt98::C0(_, _, _, h1637) => {
                    h1637
                }
            }, List::<List<u64>>::nil());
            append_list(a1638, a1639)
        })
    } else {
        v1
    }
}

pub fn f529(v0: Adt138) -> R<Option<List<u64>>> {
    let m1640 = f624(f625(v0))?;
    match m1640 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1641 = f310(v1)?;
            match m1641 {
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

pub fn f530() -> u64 {
    404u64
}

pub fn f531(v0: Adt88) -> u64 {
    let m1642 = v0;
    match m1642 {
        Adt88::C0(_) => {
            f316()
        }
        Adt88::C1(_) => {
            f316()
        }
        Adt88::C2(_) => {
            f316()
        }
        Adt88::C3(_) => {
            f316()
        }
        Adt88::C4(_) => {
            f316()
        }
        Adt88::C5(_) => {
            f316()
        }
        Adt88::C6(_) => {
            f99()
        }
        Adt88::C7(_) => {
            f626()
        }
        Adt88::C8(_) => {
            f316()
        }
        Adt88::C9(_) => {
            f99()
        }
        Adt88::C10 => {
            f316()
        }
        Adt88::C11 => {
            f316()
        }
        Adt88::C12(_) => {
            f99()
        }
        Adt88::C13 => {
            f99()
        }
        Adt88::C14 => {
            f99()
        }
        Adt88::C15(_) => {
            f99()
        }
    }
}

pub fn f532(v0: Adt139) -> i64 {
    let m1643 = v0;
    match m1643 {
        Adt139::C0(_) => {
            f627()
        }
        Adt139::C1(_, _) => {
            f628()
        }
        Adt139::C2(_) => {
            f629()
        }
        Adt139::C3(_) => {
            f630()
        }
        Adt139::C4(v6) => {
            f631(v6)
        }
    }
}

pub fn f533(v0: Adt139) -> Str {
    let m1644 = v0;
    match m1644 {
        Adt139::C0(v1) => {
            f632(v1)
        }
        Adt139::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt139::C2(v4) => {
            f633(v4)
        }
        Adt139::C3(_) => {
            Str::lit("Method not found")
        }
        Adt139::C4(v6) => {
            let m1645 = v6;
            match m1645 {
                Adt88::C0(_) => {
                    Str::lit("Invalid params: unknown tool")
                }
                Adt88::C1(_) => {
                    Str::lit("Invalid params: unknown prompt")
                }
                Adt88::C2(_) => {
                    Str::lit("Invalid params: resource not found")
                }
                Adt88::C3(_) => {
                    Str::lit("Invalid params: unknown task")
                }
                Adt88::C4(_) => {
                    Str::lit("Invalid params: invalid arguments")
                }
                Adt88::C5(_) => {
                    Str::lit("Invalid params: invalid cursor")
                }
                Adt88::C6(_) => {
                    Str::lit("Internal error")
                }
                Adt88::C7(_) => {
                    Str::lit("Rate limit exceeded: too many tool calls")
                }
                Adt88::C8(_) => {
                    Str::lit("Missing required client capability")
                }
                Adt88::C9(_) => {
                    Str::lit("Internal error")
                }
                Adt88::C10 => {
                    Str::lit("Invalid params: requestState failed verification")
                }
                Adt88::C11 => {
                    Str::lit("Invalid params: inputResponses is not a valid InputResponses object")
                }
                Adt88::C12(_) => {
                    Str::lit("Internal error")
                }
                Adt88::C13 => {
                    Str::lit("Internal error")
                }
                Adt88::C14 => {
                    Str::lit("Internal error")
                }
                Adt88::C15(_) => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f534(v0: Adt139) -> R<Option<Adt9>> {
    let m1646 = v0;
    match m1646 {
        Adt139::C0(_) => {
            Ok(None::<Adt9>)
        }
        Adt139::C1(v2, v3) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("supported"), Adt9::C4(f634(v3))), List::cons((Str::lit("requested"), Adt9::C3(v2)), List::<(Str, Adt9)>::nil())))))
        }
        Adt139::C2(_) => {
            Ok(None::<Adt9>)
        }
        Adt139::C3(_) => {
            Ok(None::<Adt9>)
        }
        Adt139::C4(v6) => {
            let m1647 = v6;
            match m1647 {
                Adt88::C0(_) => {
                    Ok(None::<Adt9>)
                }
                Adt88::C1(_) => {
                    Ok(None::<Adt9>)
                }
                Adt88::C2(v9) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("uri"), Adt9::C3(v9)), List::<(Str, Adt9)>::nil()))))
                }
                Adt88::C3(_) => {
                    Ok(None::<Adt9>)
                }
                Adt88::C4(v11) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("reason"), Adt9::C3(v11)), List::<(Str, Adt9)>::nil()))))
                }
                Adt88::C5(_) => {
                    Ok(None::<Adt9>)
                }
                Adt88::C6(_) => {
                    Ok(None::<Adt9>)
                }
                Adt88::C7(v14) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("limit"), Adt9::C2(Adt10::C0(f277(v14, 0i64)?, 0i64))), List::<(Str, Adt9)>::nil()))))
                }
                Adt88::C8(v15) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("requiredCapabilities"), f635(v15)), List::<(Str, Adt9)>::nil()))))
                }
                Adt88::C9(_) => {
                    Ok(None::<Adt9>)
                }
                Adt88::C10 => {
                    Ok(None::<Adt9>)
                }
                Adt88::C11 => {
                    Ok(None::<Adt9>)
                }
                Adt88::C12(_) => {
                    Ok(None::<Adt9>)
                }
                Adt88::C13 => {
                    Ok(None::<Adt9>)
                }
                Adt88::C14 => {
                    Ok(None::<Adt9>)
                }
                Adt88::C15(_) => {
                    Ok(None::<Adt9>)
                }
            }
        }
    }
}

pub fn f535(v0: List<u64>) -> R<bool> {
    let m1648 = f463(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1648.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f636(v2)? {
                Ok(false)
            } else {
                f376(f378(v1), f637())
            }
        }
    }
}

pub fn f536(v0: Adt3, v1: Fn1, v2: u64, v3: Adt147) -> R<Result<Adt136, Adt139>> {
    f638(v0.clone(), v1, v2, v3.clone(), f639(match v0 {
        Adt3::C0(_, _, h1649, _, _, _, _) => {
            h1649
        }
    }, match v3 {
        Adt147::C0(_, h1650, _, _) => {
            h1650
        }
    }))
}

pub fn f537(v0: Adt150, v1: Adt151) -> Result<Adt147, Adt139> {
    let m1651 = v1;
    match m1651 {
        Adt151::C0(v3) => {
            f640(v3, v0)
        }
        Adt151::C1(v2) => {
            Err::<Adt147, Adt139>(v2)
        }
    }
}

pub fn f538(v0: Str) -> Adt150 {
    let a1652 = v0.clone();
    let a1653 = Str::lit("server/discover");
    if equal(a1652, a1653) {
        Adt150::C1(Adt148::C0)
    } else {
        let a1654 = v0.clone();
        let a1655 = Str::lit("tools/list");
        if equal(a1654, a1655) {
            Adt150::C1(Adt148::C1)
        } else {
            let a1656 = v0.clone();
            let a1657 = Str::lit("tools/call");
            if equal(a1656, a1657) {
                Adt150::C1(Adt148::C2)
            } else {
                let a1658 = v0.clone();
                let a1659 = Str::lit("resources/list");
                if equal(a1658, a1659) {
                    Adt150::C1(Adt148::C3)
                } else {
                    let a1660 = v0.clone();
                    let a1661 = Str::lit("resources/read");
                    if equal(a1660, a1661) {
                        Adt150::C1(Adt148::C4)
                    } else {
                        let a1662 = v0.clone();
                        let a1663 = Str::lit("resources/templates/list");
                        if equal(a1662, a1663) {
                            Adt150::C1(Adt148::C5)
                        } else {
                            let a1664 = v0.clone();
                            let a1665 = Str::lit("prompts/list");
                            if equal(a1664, a1665) {
                                Adt150::C1(Adt148::C6)
                            } else {
                                let a1666 = v0.clone();
                                let a1667 = Str::lit("prompts/get");
                                if equal(a1666, a1667) {
                                    Adt150::C1(Adt148::C7)
                                } else {
                                    let a1668 = v0.clone();
                                    let a1669 = Str::lit("completion/complete");
                                    if equal(a1668, a1669) {
                                        Adt150::C1(Adt148::C8)
                                    } else {
                                        let a1670 = v0.clone();
                                        let a1671 = Str::lit("tasks/get");
                                        if equal(a1670, a1671) {
                                            Adt150::C1(Adt148::C9)
                                        } else {
                                            let a1672 = v0.clone();
                                            let a1673 = Str::lit("tasks/update");
                                            if equal(a1672, a1673) {
                                                Adt150::C1(Adt148::C10)
                                            } else {
                                                let a1674 = v0;
                                                let a1675 = Str::lit("tasks/cancel");
                                                if equal(a1674, a1675) {
                                                    Adt150::C1(Adt148::C11)
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
            }
        }
    }
}

pub fn f539(v0: Option<Adt122>, v1: List<Str>, v2: Adt118) -> R<Adt151> {
    f641(v0, v1, v2.clone(), f642(match v2 {
        Adt118::C0(_, _, h1676) => {
            h1676
        }
    }))
}

pub fn f540(v0: List<u64>) -> Option<Str> {
    let a1677 = f348(f349(), v0);
    utf8_decode(a1677)
}

pub fn f541(v0: Str) -> Adt153 {
    let m1678 = f538(v0);
    match m1678 {
        Adt150::C0 => {
            Adt153::C0
        }
        Adt150::C1(v1) => {
            f643(v1)
        }
    }
}

pub fn f542(v0: List<u64>) -> Option<List<u64>> {
    let m1679 = f644(f645(), v0);
    match m1679 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1680 = f644(f646(), f386(v1, List::<u64>::nil()));
            match m1680 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f386(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f543(p1683: List<Adt125>, p1684: List<Adt125>) -> List<Adt125> {
    let m1682 = (p1683, p1684);
    {
        let mut m1682 = m1682;
        loop {
            let step = {
                let (v0, v1) = m1682;
                let m1681 = v0;
                match m1681.uncons() {
                    None => {
                        Err::<(List<Adt125>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt125>, List<Adt125>), List<Adt125>>((v3, List::cons(v2, v1)))
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

pub fn f544(v0: List<Adt95>, p1691: List<Adt15>, v2: Option<Str>, p1692: List<Adt125>) -> R<List<Adt125>> {
    let m1690 = (p1691, p1692);
    {
        let mut m1690 = m1690;
        loop {
            let step = {
                let (v1, v3) = m1690;
                let m1685 = v1;
                match m1685.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt125>), R<List<Adt125>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f647(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt15>, List<Adt125>), R<List<Adt125>>>((v5, List::cons(Adt125::C0(match v4.clone() {
                                Adt15::C0(_, h1686, _, _) => {
                                    h1686
                                }
                            }, match v4.clone() {
                                Adt15::C0(_, _, h1687, _) => {
                                    h1687
                                }
                            }, match v4.clone() {
                                Adt15::C0(_, _, _, h1688) => {
                                    h1688
                                }
                            }, f648(v0.clone(), match v4 {
                                Adt15::C0(_, h1689, _, _) => {
                                    h1689
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt15>, List<Adt125>), R<List<Adt125>>>((v5, v3))
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

pub fn f545(p1708: u64, p1709: i64, p1710: i64) -> R<Option<i64>> {
    let m1707 = (p1708, (p1709, p1710));
    {
        let mut m1707 = m1707;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1707;
                let m1693 = v0;
                if m1693 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1693 - 1;
                    if f649(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1696 = {
                            let a1694 = v2;
                            let a1695 = 0i64;
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
                            if f649({
                                let a1697 = v1;
                                let a1698 = 10i64;
                                let a1699 = 0i64;
                                int_rem(a1697, a1698, a1699)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1700 = v1;
                                    let a1701 = 10i64;
                                    let a1702 = 0i64;
                                    int_quot(a1700, a1701, a1702)?
                                }, f345(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1703 = v1;
                                let a1704 = 10i64;
                                int_mul(a1703, a1704)?
                            }, {
                                let a1705 = v2;
                                let a1706 = 1i64;
                                int_sub(a1705, a1706)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f546(v0: i64) -> Option<i64> {
    let m1713 = {
        let a1711 = 9007199254740991i64;
        let a1712 = v0;
        compare(a1711, a1712)
    };
    if if match m1713 {
        Ordering::Less => {
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
            let a1714 = v0;
            let a1715 = -9007199254740991i64;
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
    } {
        None::<i64>
    } else {
        Some(v0)
    }
}

pub fn f547(v0: Adt110) -> bool {
    let m1717 = v0;
    match m1717 {
        Adt110::C0 => {
            false
        }
        Adt110::C1(_) => {
            true
        }
    }
}

pub fn f548(v0: Adt103, v1: Str, v2: Adt8) -> Result<Adt118, Adt112> {
    let m1718 = f225(Str::lit("params"), v2);
    match m1718 {
        Adt110::C0 => {
            Ok::<Adt118, Adt112>(Adt118::C0(v0, v1, None::<Adt8>))
        }
        Adt110::C1(v3) => {
            let m1719 = f226(v3);
            match m1719 {
                None => {
                    f372(Some(v0), Adt113::C6)
                }
                Some(v4) => {
                    Ok::<Adt118, Adt112>(Adt118::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f549(v0: Adt133) -> bool {
    let m1720 = v0;
    match m1720 {
        Adt133::C0 => {
            false
        }
        Adt133::C1 => {
            true
        }
        Adt133::C2 => {
            true
        }
        Adt133::C3 => {
            false
        }
        Adt133::C4 => {
            true
        }
        Adt133::C5 => {
            false
        }
        Adt133::C6 => {
            false
        }
        Adt133::C7 => {
            true
        }
    }
}

pub fn f550(v0: Adt129, v1: Adt134, v2: Adt130, v3: u64) -> R<Adt143> {
    let a1722 = match v0.clone() {
        Adt129::C0(_, _, h1721) => {
            h1721
        }
    };
    let a1723 = f605();
    if nat_lt(a1722, a1723) {
        Ok(Adt143::C1(Adt129::C0(v2, List::cons(v1, match v0.clone() {
            Adt129::C0(_, h1724, _) => {
                h1724
            }
        }), {
            let a1726 = match v0 {
                Adt129::C0(_, _, h1725) => {
                    h1725
                }
            };
            let a1727 = 1u64;
            nat_add(a1726, a1727)?
        })))
    } else {
        Ok(f489(Adt111::C9(v3)))
    }
}

pub fn f551(v0: bool, v1: Adt133, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt130 {
    Adt130::C8(Adt132::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f552(v0: u64) -> i64 {
    let a1728 = v0;
    let a1729 = 0u64;
    if nat_eq(a1728, a1729) {
        0i64
    } else {
        let a1730 = v0;
        let a1731 = 1u64;
        if nat_eq(a1730, a1731) {
            1i64
        } else {
            let a1732 = v0;
            let a1733 = 2u64;
            if nat_eq(a1732, a1733) {
                2i64
            } else {
                let a1734 = v0;
                let a1735 = 3u64;
                if nat_eq(a1734, a1735) {
                    3i64
                } else {
                    let a1736 = v0;
                    let a1737 = 4u64;
                    if nat_eq(a1736, a1737) {
                        4i64
                    } else {
                        let a1738 = v0;
                        let a1739 = 5u64;
                        if nat_eq(a1738, a1739) {
                            5i64
                        } else {
                            let a1740 = v0;
                            let a1741 = 6u64;
                            if nat_eq(a1740, a1741) {
                                6i64
                            } else {
                                let a1742 = v0;
                                let a1743 = 7u64;
                                if nat_eq(a1742, a1743) {
                                    7i64
                                } else {
                                    let a1744 = v0;
                                    let a1745 = 8u64;
                                    if nat_eq(a1744, a1745) {
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

pub fn f553(p1748: List<Adt9>, p1749: List<Adt9>) -> List<Adt9> {
    let m1747 = (p1748, p1749);
    {
        let mut m1747 = m1747;
        loop {
            let step = {
                let (v0, v1) = m1747;
                let m1746 = v0;
                match m1746.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt9>, List<Adt9>), List<Adt9>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1747 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(v0: Adt129, v1: u64) -> Adt143 {
    let m1751 = match v0.clone() {
        Adt129::C0(_, h1750, _) => {
            h1750
        }
    };
    match m1751.uncons() {
        None => {
            f489(Adt111::C2(v1))
        }
        Some((v2, _)) => {
            let m1752 = v2;
            match m1752 {
                Adt134::C0(_) => {
                    f488(v0, Adt130::C0)
                }
                Adt134::C1(_, _) => {
                    f488(v0, Adt130::C4)
                }
            }
        }
    }
}

pub fn f555(p1755: List<(Str, Adt9)>, p1756: List<(Str, Adt9)>) -> List<(Str, Adt9)> {
    let m1754 = (p1755, p1756);
    {
        let mut m1754 = m1754;
        loop {
            let step = {
                let (v0, v1) = m1754;
                let m1753 = v0;
                match m1753.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Str, Adt9)>), List<(Str, Adt9)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt9)>, List<(Str, Adt9)>), List<(Str, Adt9)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1754 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f556(v0: Adt129, v1: Adt130) -> Adt129 {
    Adt129::C0(v1, match v0.clone() {
        Adt129::C0(_, h1757, _) => {
            h1757
        }
    }, match v0 {
        Adt129::C0(_, _, h1758) => {
            h1758
        }
    })
}

pub fn f557(v0: Adt129, v1: u64, v2: List<u64>, v3: bool) -> Adt143 {
    let m1760 = {
        let a1759 = f348(f349(), f417(v2, List::<u64>::nil()));
        utf8_decode(a1759)
    };
    match m1760 {
        None => {
            f489(Adt111::C14)
        }
        Some(v4) => {
            if v3 {
                f650(v0, v1, v4)
            } else {
                f563(v0, Adt9::C3(v4))
            }
        }
    }
}

pub fn f558(v0: u64) -> u64 {
    let a1761 = v0;
    let a1762 = 34u64;
    if nat_eq(a1761, a1762) {
        34u64
    } else {
        let a1763 = v0;
        let a1764 = 92u64;
        if nat_eq(a1763, a1764) {
            92u64
        } else {
            let a1765 = v0;
            let a1766 = 47u64;
            if nat_eq(a1765, a1766) {
                47u64
            } else {
                let a1767 = v0;
                let a1768 = 98u64;
                if nat_eq(a1767, a1768) {
                    8u64
                } else {
                    let a1769 = v0;
                    let a1770 = 102u64;
                    if nat_eq(a1769, a1770) {
                        12u64
                    } else {
                        let a1771 = v0;
                        let a1772 = 110u64;
                        if nat_eq(a1771, a1772) {
                            10u64
                        } else {
                            let a1773 = v0;
                            let a1774 = 114u64;
                            if nat_eq(a1773, a1774) {
                                13u64
                            } else {
                                let a1775 = v0;
                                let a1776 = 116u64;
                                if nat_eq(a1775, a1776) {
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

pub fn f559(v0: u64) -> u64 {
    let a1777 = v0;
    let a1778 = 48u64;
    if nat_lt(a1777, a1778) {
        16u64
    } else {
        let a1779 = v0;
        let a1780 = 58u64;
        if nat_lt(a1779, a1780) {
            let a1781 = v0;
            let a1782 = 48u64;
            nat_sub(a1781, a1782)
        } else {
            let a1783 = v0;
            let a1784 = 65u64;
            if nat_lt(a1783, a1784) {
                16u64
            } else {
                let a1785 = v0;
                let a1786 = 71u64;
                if nat_lt(a1785, a1786) {
                    let a1787 = v0;
                    let a1788 = 55u64;
                    nat_sub(a1787, a1788)
                } else {
                    let a1789 = v0;
                    let a1790 = 97u64;
                    if nat_lt(a1789, a1790) {
                        16u64
                    } else {
                        let a1791 = v0;
                        let a1792 = 103u64;
                        if nat_lt(a1791, a1792) {
                            let a1793 = v0;
                            let a1794 = 87u64;
                            nat_sub(a1793, a1794)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f560(v0: Adt129, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt143> {
    let a1795 = v3;
    let a1796 = 0u64;
    if nat_eq(a1795, a1796) {
        if f480(v2, 55296u64, 56319u64) {
            Ok(f488(v0, Adt130::C6(Adt131::C3(v2), v4, v5)))
        } else {
            if f480(v2, 56320u64, 57343u64) {
                Ok(f489(Adt111::C6(v1)))
            } else {
                Ok(f488(v0, Adt130::C6(Adt131::C0, f651(v2, v4)?, v5)))
            }
        }
    } else {
        if f480(v2, 56320u64, 57343u64) {
            Ok(f488(v0, Adt130::C6(Adt131::C0, f651({
                let a1805 = 65536u64;
                let a1806 = {
                    let a1803 = {
                        let a1799 = {
                            let a1797 = v3;
                            let a1798 = 55296u64;
                            nat_sub(a1797, a1798)
                        };
                        let a1800 = 1024u64;
                        nat_mul(a1799, a1800)?
                    };
                    let a1804 = {
                        let a1801 = v2;
                        let a1802 = 56320u64;
                        nat_sub(a1801, a1802)
                    };
                    nat_add(a1803, a1804)?
                };
                nat_add(a1805, a1806)?
            }, v4)?, v5)))
        } else {
            Ok(f489(Adt111::C6(v1)))
        }
    }
}

pub fn f561(v0: u64, v1: u64) -> u64 {
    let m1809 = {
        let a1807 = f562(v0);
        let a1808 = v1;
        index_list(a1807, a1808)
    };
    match m1809 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f562(v0: u64) -> List<u64> {
    let a1810 = v0;
    let a1811 = 0u64;
    if nat_eq(a1810, a1811) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1812 = v0;
        let a1813 = 1u64;
        if nat_eq(a1812, a1813) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f563(v0: Adt129, v1: Adt9) -> Adt143 {
    Adt143::C1(f482(v1, match v0.clone() {
        Adt129::C0(_, h1814, _) => {
            h1814
        }
    }, match v0 {
        Adt129::C0(_, _, h1815) => {
            h1815
        }
    }))
}

pub fn f564(v0: u64) -> Adt9 {
    let a1816 = v0;
    let a1817 = 0u64;
    if nat_eq(a1816, a1817) {
        Adt9::C1(true)
    } else {
        let a1818 = v0;
        let a1819 = 1u64;
        if nat_eq(a1818, a1819) {
            Adt9::C1(false)
        } else {
            Adt9::C0
        }
    }
}

pub fn f565(v0: Adt129, v1: Adt132, v2: Adt133) -> Adt143 {
    f488(v0, Adt130::C8(Adt132::C0(match v1.clone() {
        Adt132::C0(h1820, _, _, _, _, _, _) => {
            h1820
        }
    }, v2, match v1.clone() {
        Adt132::C0(_, _, h1821, _, _, _, _) => {
            h1821
        }
    }, match v1.clone() {
        Adt132::C0(_, _, _, h1822, _, _, _) => {
            h1822
        }
    }, match v1.clone() {
        Adt132::C0(_, _, _, _, h1823, _, _) => {
            h1823
        }
    }, match v1.clone() {
        Adt132::C0(_, _, _, _, _, h1824, _) => {
            h1824
        }
    }, match v1 {
        Adt132::C0(_, _, _, _, _, _, h1825) => {
            h1825
        }
    })))
}

pub fn f566(v0: Adt129, v1: u64, v2: u64, v3: Adt132, v4: bool) -> R<Adt143> {
    let v5: u64 = {
        let a1827 = match v3.clone() {
            Adt132::C0(_, _, _, h1826, _, _, _) => {
                h1826
            }
        };
        let a1828 = 0u64;
        if if nat_eq(a1827, a1828) {
            let a1829 = v2;
            let a1830 = 48u64;
            nat_eq(a1829, a1830)
        } else {
            false
        } {
            0u64
        } else {
            let a1832 = match v3.clone() {
                Adt132::C0(_, _, _, h1831, _, _, _) => {
                    h1831
                }
            };
            let a1833 = 1u64;
            nat_add(a1832, a1833)?
        }
    };
    let v6: i64 = if v4 {
        f345(match v3.clone() {
            Adt132::C0(_, _, _, _, h1834, _, _) => {
                h1834
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt132::C0(_, _, _, _, h1835, _, _) => {
                h1835
            }
        }
    };
    let a1836 = f652();
    let a1837 = v5;
    if nat_lt(a1836, a1837) {
        Ok(f489(Adt111::C8(v1)))
    } else {
        let m1840 = {
            let a1838 = 99999i64;
            let a1839 = v6;
            compare(a1838, a1839)
        };
        if match m1840 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f489(Adt111::C8(v1)))
        } else {
            Ok(f488(v0, f551(match v3.clone() {
                Adt132::C0(h1841, _, _, _, _, _, _) => {
                    h1841
                }
            }, if v4 {
                Adt133::C4
            } else {
                Adt133::C2
            }, f345({
                let a1843 = match v3.clone() {
                    Adt132::C0(_, _, h1842, _, _, _, _) => {
                        h1842
                    }
                };
                let a1844 = 10i64;
                int_mul(a1843, a1844)?
            }, f552({
                let a1845 = v2;
                let a1846 = 48u64;
                nat_sub(a1845, a1846)
            }))?, v5, v6, match v3.clone() {
                Adt132::C0(_, _, _, _, _, h1847, _) => {
                    h1847
                }
            }, match v3 {
                Adt132::C0(_, _, _, _, _, _, h1848) => {
                    h1848
                }
            })))
        }
    }
}

pub fn f567(v0: u64) -> bool {
    let a1849 = v0;
    let a1850 = 101u64;
    if nat_eq(a1849, a1850) {
        true
    } else {
        let a1851 = v0;
        let a1852 = 69u64;
        nat_eq(a1851, a1852)
    }
}

pub fn f568(v0: u64) -> bool {
    f480(v0, 48u64, 57u64)
}

pub fn f569(v0: Adt129, v1: u64, v2: u64, v3: Adt132) -> R<Adt143> {
    let m1853 = f481(v3)?;
    match m1853 {
        Adt149::C0 => {
            Ok(f489(Adt111::C7(v1)))
        }
        Adt149::C1 => {
            Ok(f489(Adt111::C8(v1)))
        }
        Adt149::C2(v4) => {
            Ok(f486(f482(v4, match v0.clone() {
                Adt129::C0(_, h1854, _) => {
                    h1854
                }
            }, match v0 {
                Adt129::C0(_, _, h1855) => {
                    h1855
                }
            }), v1, v2))
        }
    }
}

pub fn f570(v0: Adt129, v1: u64, v2: u64, v3: Adt132, v4: bool) -> R<Adt143> {
    let v5: i64 = f345({
        let a1857 = match v3.clone() {
            Adt132::C0(_, _, _, _, _, _, h1856) => {
                h1856
            }
        };
        let a1858 = 10i64;
        int_mul(a1857, a1858)?
    }, f552({
        let a1859 = v2;
        let a1860 = 48u64;
        nat_sub(a1859, a1860)
    }))?;
    let m1863 = {
        let a1861 = 99999i64;
        let a1862 = v5;
        compare(a1861, a1862)
    };
    if match m1863 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f489(Adt111::C8(v1)))
    } else {
        Ok(f488(v0, f551(match v3.clone() {
            Adt132::C0(h1864, _, _, _, _, _, _) => {
                h1864
            }
        }, Adt133::C7, match v3.clone() {
            Adt132::C0(_, _, h1865, _, _, _, _) => {
                h1865
            }
        }, match v3.clone() {
            Adt132::C0(_, _, _, h1866, _, _, _) => {
                h1866
            }
        }, match v3 {
            Adt132::C0(_, _, _, _, h1867, _, _) => {
                h1867
            }
        }, v4, v5)))
    }
}

pub fn f571(v0: Adt88) -> Option<Adt89> {
    let m1868 = v0;
    match m1868 {
        Adt88::C0(_) => {
            None::<Adt89>
        }
        Adt88::C1(_) => {
            None::<Adt89>
        }
        Adt88::C2(_) => {
            None::<Adt89>
        }
        Adt88::C3(_) => {
            None::<Adt89>
        }
        Adt88::C4(_) => {
            None::<Adt89>
        }
        Adt88::C5(_) => {
            None::<Adt89>
        }
        Adt88::C6(_) => {
            None::<Adt89>
        }
        Adt88::C7(_) => {
            None::<Adt89>
        }
        Adt88::C8(_) => {
            None::<Adt89>
        }
        Adt88::C9(_) => {
            None::<Adt89>
        }
        Adt88::C10 => {
            None::<Adt89>
        }
        Adt88::C11 => {
            None::<Adt89>
        }
        Adt88::C12(_) => {
            None::<Adt89>
        }
        Adt88::C13 => {
            None::<Adt89>
        }
        Adt88::C14 => {
            None::<Adt89>
        }
        Adt88::C15(v12) => {
            Some(v12)
        }
    }
}

pub fn f572(v0: u64) -> bool {
    let a1869 = v0;
    let a1870 = f653();
    nat_lt(a1869, a1870)
}

pub fn f573() -> Adt139 {
    Adt139::C4(Adt88::C9(Str::lit("the effect tag does not fit thirty-two bits")))
}

pub fn f574(v0: u64, v1: Adt9) -> bool {
    let a1871 = v0;
    let a1872 = 0u64;
    if nat_eq(a1871, a1872) {
        f654(v1)
    } else {
        let a1873 = v0;
        let a1874 = 1u64;
        if nat_eq(a1873, a1874) {
            f655(v1)
        } else {
            let a1875 = v0;
            let a1876 = 2u64;
            if nat_eq(a1875, a1876) {
                f656(v1)
            } else {
                let a1877 = v0;
                let a1878 = 3u64;
                if nat_eq(a1877, a1878) {
                    f657(v1)
                } else {
                    let a1879 = v0;
                    let a1880 = 4u64;
                    if nat_eq(a1879, a1880) {
                        f658(v1)
                    } else {
                        let a1881 = v0;
                        let a1882 = 5u64;
                        if nat_eq(a1881, a1882) {
                            f659(v1)
                        } else {
                            let a1883 = v0;
                            let a1884 = 6u64;
                            if nat_eq(a1883, a1884) {
                                f660(v1)
                            } else {
                                let a1885 = v0;
                                let a1886 = 7u64;
                                if nat_eq(a1885, a1886) {
                                    f661(v1)
                                } else {
                                    f662(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f575(v0: Adt9, v1: Str) -> Option<Adt9> {
    let a1887 = v1.clone();
    let a1888 = Str::lit("#");
    if equal(a1887, a1888) {
        Some(v0)
    } else {
        let m1889 = f226(v0);
        match m1889 {
            None => {
                None::<Adt9>
            }
            Some(v2) => {
                let m1890 = f225(Str::lit("$defs"), v2);
                match m1890 {
                    Adt110::C0 => {
                        None::<Adt9>
                    }
                    Adt110::C1(v3) => {
                        let m1891 = f226(v3);
                        match m1891 {
                            None => {
                                None::<Adt9>
                            }
                            Some(v4) => {
                                f663(v1, match v4 {
                                    Adt8::C0(h1892) => {
                                        h1892
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

pub fn f576(v0: Str, v1: List<(Str, Adt9)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt116) -> R<Adt144> {
    let m1893 = v3.clone();
    match m1893.uncons() {
        None => {
            Ok(Adt144::C0(Adt96::C8(v0)))
        }
        Some((_, _)) => {
            if f332(v3.clone(), true) {
                let m1894 = f664(v1);
                match m1894 {
                    None => {
                        Ok(Adt144::C0(Adt96::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f665(v3)?;
                        if f666(v9.clone(), match v5.clone() {
                            Adt116::C0(_, h1895) => {
                                h1895
                            }
                        }) {
                            Ok(Adt144::C0(Adt96::C12(v0, v2)))
                        } else {
                            Ok(Adt144::C1(Adt116::C0(List::cons(Adt15::C0(v0, v2, f287(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt116::C0(h1896, _) => {
                                    h1896
                                }
                            }), List::cons(v9, match v5 {
                                Adt116::C0(_, h1897) => {
                                    h1897
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt144::C0(Adt96::C9(v0)))
            }
        }
    }
}

pub fn f577(p1902: List<(Str, Adt9)>, v1: bool, v2: List<Str>, p1903: List<Adt104>) -> List<Adt104> {
    let m1901 = (p1902, p1903);
    {
        let mut m1901 = m1901;
        loop {
            let step = {
                let (v0, v3) = m1901;
                let m1898 = v0;
                match m1898.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt104>), List<Adt104>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt9)>, List<Adt104>), List<Adt104>>((v5, List::cons(Adt104::C0({
                            let (_, h1899) = v4;
                            h1899
                        }, v1, List::cons({
                            let (h1900, _) = v4;
                            h1900
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1901 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f578(p1907: List<(Str, Adt9)>, p1908: List<Adt104>) -> List<Adt104> {
    let m1906 = (p1907, p1908);
    {
        let mut m1906 = m1906;
        loop {
            let step = {
                let (v0, v1) = m1906;
                let m1904 = v0;
                match m1904.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt104>), List<Adt104>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt9)>, List<Adt104>), List<Adt104>>((v3, List::cons(Adt104::C0({
                            let (_, h1905) = v2;
                            h1905
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1906 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(p1911: List<Adt9>, p1912: List<Adt104>) -> List<Adt104> {
    let m1910 = (p1911, p1912);
    {
        let mut m1910 = m1910;
        loop {
            let step = {
                let (v0, v1) = m1910;
                let m1909 = v0;
                match m1909.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt104>), List<Adt104>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt9>, List<Adt104>), List<Adt104>>((v3, List::cons(Adt104::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1910 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f580(v0: Str) -> bool {
    let a1913 = v0.clone();
    let a1914 = Str::lit("null");
    if equal(a1913, a1914) {
        true
    } else {
        let a1915 = v0.clone();
        let a1916 = Str::lit("boolean");
        if equal(a1915, a1916) {
            true
        } else {
            let a1917 = v0.clone();
            let a1918 = Str::lit("object");
            if equal(a1917, a1918) {
                true
            } else {
                let a1919 = v0.clone();
                let a1920 = Str::lit("array");
                if equal(a1919, a1920) {
                    true
                } else {
                    let a1921 = v0.clone();
                    let a1922 = Str::lit("number");
                    if equal(a1921, a1922) {
                        true
                    } else {
                        let a1923 = v0.clone();
                        let a1924 = Str::lit("string");
                        if equal(a1923, a1924) {
                            true
                        } else {
                            let a1925 = v0;
                            let a1926 = Str::lit("integer");
                            equal(a1925, a1926)
                        }
                    }
                }
            }
        }
    }
}

pub fn f581(v0: Str, v1: Adt9) -> R<bool> {
    let a1927 = v0.clone();
    let a1928 = f667(v1.clone());
    if equal(a1927, a1928) {
        Ok(true)
    } else {
        let a1929 = v0;
        let a1930 = Str::lit("integer");
        if equal(a1929, a1930) {
            let m1931 = f585(v1);
            match m1931 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f668(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f582(p1934: List<Adt9>, v1: Adt9, p1935: bool) -> R<Adt145> {
    let m1933 = (p1934, p1935);
    {
        let mut m1933 = m1933;
        loop {
            let step = {
                let (v0, v2) = m1933;
                let m1932 = v0;
                match m1932.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt9>, bool), R<Adt145>>(Ok(Adt145::C0))
                        } else {
                            Err::<(List<Adt9>, bool), R<Adt145>>(Ok(Adt145::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f580(f669(v3.clone())) {
                            Ok::<(List<Adt9>, bool), R<Adt145>>((v4, if v2 {
                                true
                            } else {
                                f581(f669(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt9>, bool), R<Adt145>>(Ok(Adt145::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1933 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(p1939: List<Adt9>, v1: Adt9) -> R<Adt145> {
    let m1938 = p1939;
    {
        let mut m1938 = m1938;
        loop {
            let step = {
                let v0 = m1938;
                let m1936 = v0;
                match m1936.uncons() {
                    None => {
                        Err::<List<Adt9>, R<Adt145>>(Ok(Adt145::C1))
                    }
                    Some((v2, v3)) => {
                        let m1937 = f584(v2, v1.clone())?;
                        match m1937 {
                            Adt154::C0 => {
                                Err::<List<Adt9>, R<Adt145>>(Ok(Adt145::C0))
                            }
                            Adt154::C1 => {
                                Ok::<List<Adt9>, R<Adt145>>(v3)
                            }
                            Adt154::C2 => {
                                Err::<List<Adt9>, R<Adt145>>(Ok(Adt145::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1938 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f584(v0: Adt9, v1: Adt9) -> R<Adt154> {
    f670(f671(), List::cons((v0, v1), List::<(Adt9, Adt9)>::nil()))
}

pub fn f585(v0: Adt9) -> Option<Adt10> {
    let m1940 = v0;
    match m1940 {
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

pub fn f586(v0: u64, v1: Adt155) -> bool {
    let a1941 = v0;
    let a1942 = 0u64;
    if nat_eq(a1941, a1942) {
        let a1943 = f672(v1);
        bool_not(a1943)
    } else {
        let a1944 = v0;
        let a1945 = 1u64;
        if nat_eq(a1944, a1945) {
            let a1946 = f601(v1);
            bool_not(a1946)
        } else {
            let a1947 = v0;
            let a1948 = 2u64;
            if nat_eq(a1947, a1948) {
                f601(v1)
            } else {
                f672(v1)
            }
        }
    }
}

pub fn f587(v0: Adt10, v1: Adt10) -> R<Adt155> {
    f673(v0.clone(), v1.clone(), f674(match v0 {
        Adt10::C0(h1949, _) => {
            h1949
        }
    }), f674(match v1 {
        Adt10::C0(h1950, _) => {
            h1950
        }
    }))
}

pub fn f588(v0: Str, v1: bool, v2: List<(Str, Adt9)>, v3: u64) -> R<Adt145> {
    let m1951 = f297(v0, v2);
    match m1951 {
        Adt110::C0 => {
            Ok(Adt145::C0)
        }
        Adt110::C1(v4) => {
            let m1952 = f675(v4);
            match m1952 {
                None => {
                    Ok(Adt145::C2)
                }
                Some(v5) => {
                    let m1955 = {
                        let a1953 = v5;
                        let a1954 = 0i64;
                        compare(a1953, a1954)
                    };
                    if match m1955 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Ok(Adt145::C2)
                    } else {
                        if v1 {
                            if f672(f676(f277(v3, 0i64)?, v5)) {
                                Ok(Adt145::C1)
                            } else {
                                Ok(Adt145::C0)
                            }
                        } else {
                            if f601(f676(f277(v3, 0i64)?, v5)) {
                                Ok(Adt145::C1)
                            } else {
                                Ok(Adt145::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f589(v0: Str) -> R<u64> {
    f677(f29({
        let a1956 = v0;
        utf8_encode(a1956)
    })?, 0u64)
}

pub fn f590(p1961: List<Adt9>, p1962: u64) -> R<u64> {
    let m1960 = (p1961, p1962);
    {
        let mut m1960 = m1960;
        loop {
            let step = {
                let (v0, v1) = m1960;
                let m1957 = v0;
                match m1957.uncons() {
                    None => {
                        Err::<(List<Adt9>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt9>, u64), R<u64>>((v3, {
                            let a1958 = v1;
                            let a1959 = 1u64;
                            nat_add(a1958, a1959)?
                        }))
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

pub fn f591(p1967: List<(Str, Adt9)>, p1968: u64) -> R<u64> {
    let m1966 = (p1967, p1968);
    {
        let mut m1966 = m1966;
        loop {
            let step = {
                let (v0, v1) = m1966;
                let m1963 = v0;
                match m1963.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt9)>, u64), R<u64>>((v3, {
                            let a1964 = v1;
                            let a1965 = 1u64;
                            nat_add(a1964, a1965)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1966 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f592(p1972: List<Adt9>, v1: Adt8) -> Adt145 {
    let m1971 = p1972;
    {
        let mut m1971 = m1971;
        loop {
            let step = {
                let v0 = m1971;
                let m1969 = v0;
                match m1969.uncons() {
                    None => {
                        Err::<List<Adt9>, Adt145>(Adt145::C0)
                    }
                    Some((v2, v3)) => {
                        let m1970 = f275(v2);
                        match m1970 {
                            None => {
                                Err::<List<Adt9>, Adt145>(Adt145::C2)
                            }
                            Some(v4) => {
                                if f477(v4, v1.clone()) {
                                    Ok::<List<Adt9>, Adt145>(v3)
                                } else {
                                    Err::<List<Adt9>, Adt145>(Adt145::C1)
                                }
                            }
                        }
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

pub fn f593(p1979: List<(Str, Adt9)>, v1: List<(Str, Adt9)>, v2: List<Adt106>, p1980: List<Adt108>) -> List<Adt108> {
    let m1978 = (p1979, p1980);
    {
        let mut m1978 = m1978;
        loop {
            let step = {
                let (v0, v3) = m1978;
                let m1973 = v0;
                match m1973.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt108>), List<Adt108>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1975 = f297({
                            let (h1974, _) = v4.clone();
                            h1974
                        }, v1.clone());
                        match m1975 {
                            Adt110::C0 => {
                                Ok::<(List<(Str, Adt9)>, List<Adt108>), List<Adt108>>((v5, v3))
                            }
                            Adt110::C1(v6) => {
                                Ok::<(List<(Str, Adt9)>, List<Adt108>), List<Adt108>>((v5, List::cons(Adt108::C0({
                                    let (_, h1976) = v4;
                                    h1976
                                }, v6, List::cons(Adt106::C0({
                                    let (h1977, _) = v4;
                                    h1977
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1978 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f594(p1985: List<(Str, Adt9)>, v1: List<(Str, Adt9)>) -> bool {
    let m1984 = p1985;
    {
        let mut m1984 = m1984;
        loop {
            let step = {
                let v0 = m1984;
                let m1981 = v0;
                match m1981.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1983 = f297({
                            let (h1982, _) = v2;
                            h1982
                        }, v1.clone());
                        match m1983 {
                            Adt110::C0 => {
                                Err::<List<(Str, Adt9)>, bool>(true)
                            }
                            Adt110::C1(_) => {
                                Ok::<List<(Str, Adt9)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1984 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(v0: List<(Str, Adt9)>) -> List<(Str, Adt9)> {
    let m1986 = f297(Str::lit("properties"), v0);
    match m1986 {
        Adt110::C0 => {
            List::<(Str, Adt9)>::nil()
        }
        Adt110::C1(v1) => {
            let m1987 = f226(v1);
            match m1987 {
                None => {
                    List::<(Str, Adt9)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt8::C0(h1988) => {
                            h1988
                        }
                    }
                }
            }
        }
    }
}

pub fn f596(p1995: List<(Str, Adt9)>, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt106>, p1996: List<Adt108>) -> List<Adt108> {
    let m1994 = (p1995, p1996);
    {
        let mut m1994 = m1994;
        loop {
            let step = {
                let (v0, v4) = m1994;
                let m1989 = v0;
                match m1989.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt108>), List<Adt108>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1991 = f297({
                            let (h1990, _) = v5.clone();
                            h1990
                        }, v1.clone());
                        match m1991 {
                            Adt110::C0 => {
                                Ok::<(List<(Str, Adt9)>, List<Adt108>), List<Adt108>>((v6, List::cons(Adt108::C0(v2.clone(), {
                                    let (_, h1992) = v5;
                                    h1992
                                }, List::cons(Adt106::C0({
                                    let (h1993, _) = v5;
                                    h1993
                                }), v3.clone())), v4)))
                            }
                            Adt110::C1(_) => {
                                Ok::<(List<(Str, Adt9)>, List<Adt108>), List<Adt108>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1994 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f597(p2001: List<Adt9>, p2002: u64, v2: Adt9, v3: List<Adt106>, p2003: List<Adt108>) -> R<List<Adt108>> {
    let m2000 = (p2001, (p2002, p2003));
    {
        let mut m2000 = m2000;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2000;
                let m1997 = v0;
                match m1997.uncons() {
                    None => {
                        Err::<(List<Adt9>, (u64, List<Adt108>)), R<List<Adt108>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt9>, (u64, List<Adt108>)), R<List<Adt108>>>((v6, ({
                            let a1998 = v1;
                            let a1999 = 1u64;
                            nat_add(a1998, a1999)?
                        }, List::cons(Adt108::C0(v2.clone(), v5, List::cons(Adt106::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m2000 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f598(p2006: List<Adt9>, v1: Adt9, v2: List<Adt106>, p2007: List<Adt108>) -> List<Adt108> {
    let m2005 = (p2006, p2007);
    {
        let mut m2005 = m2005;
        loop {
            let step = {
                let (v0, v3) = m2005;
                let m2004 = v0;
                match m2004.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt108>), List<Adt108>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt9>, List<Adt108>), List<Adt108>>((v5, List::cons(Adt108::C0(v4, v1.clone(), v2.clone()), v3)))
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

pub fn f599(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m2008 = f678(v0)?;
    match m2008 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m2009 = f678(v1)?;
            match m2009 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a2010 = v2;
                    let a2011 = 61u64;
                    if nat_eq(a2010, a2011) {
                        f679(v5, v6, v4, v3)
                    } else {
                        let m2012 = f678(v2)?;
                        match m2012 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a2013 = v3;
                                let a2014 = 61u64;
                                if nat_eq(a2013, a2014) {
                                    f680(v5, v6, v7, v4)
                                } else {
                                    let m2015 = f678(v3)?;
                                    match m2015 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f681(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f600(v0: List<u64>) -> bool {
    let m2016 = v0;
    match m2016.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f601(v0: Adt155) -> bool {
    let m2017 = v0;
    match m2017 {
        Adt155::C0 => {
            false
        }
        Adt155::C1 => {
            false
        }
        Adt155::C2 => {
            true
        }
    }
}

pub fn f602(v0: u64) -> List<u64> {
    f562(v0)
}

pub fn f603(v0: Adt10) -> R<bool> {
    let a2035 = {
        let m2021 = {
            let a2019 = f682();
            let a2020 = match v0.clone() {
                Adt10::C0(h2018, _) => {
                    h2018
                }
            };
            compare(a2019, a2020)
        };
        if match m2021 {
            Ordering::Less => {
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
            let m2026 = {
                let a2024 = match v0.clone() {
                    Adt10::C0(h2022, _) => {
                        h2022
                    }
                };
                let a2025 = {
                    let a2023 = f682();
                    int_neg(a2023)?
                };
                compare(a2024, a2025)
            };
            if match m2026 {
                Ordering::Less => {
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
                let m2030 = {
                    let a2028 = 9999i64;
                    let a2029 = match v0.clone() {
                        Adt10::C0(_, h2027) => {
                            h2027
                        }
                    };
                    compare(a2028, a2029)
                };
                if match m2030 {
                    Ordering::Less => {
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
                    let m2034 = {
                        let a2032 = match v0 {
                            Adt10::C0(_, h2031) => {
                                h2031
                            }
                        };
                        let a2033 = -9999i64;
                        compare(a2032, a2033)
                    };
                    match m2034 {
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
    Ok(bool_not(a2035))
}

pub fn f604(v0: Adt10) -> R<List<u64>> {
    let m2039 = {
        let a2037 = match v0.clone() {
            Adt10::C0(_, h2036) => {
                h2036
            }
        };
        let a2038 = 0i64;
        compare(a2037, a2038)
    };
    if if match m2039 {
        Ordering::Less => {
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
        let m2043 = {
            let a2041 = 0i64;
            let a2042 = match v0.clone() {
                Adt10::C0(_, h2040) => {
                    h2040
                }
            };
            compare(a2041, a2042)
        };
        match m2043 {
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
        let a2046 = f683(match v0.clone() {
            Adt10::C0(h2044, _) => {
                h2044
            }
        })?;
        let a2047 = List::cons(101u64, f683(match v0 {
            Adt10::C0(_, h2045) => {
                h2045
            }
        })?);
        Ok(append_list(a2046, a2047))
    } else {
        f683(match v0 {
            Adt10::C0(h2048, _) => {
                h2048
            }
        })
    }
}

pub fn f605() -> u64 {
    512u64
}

pub fn f606(v0: u64, v1: Adt121, v2: List<Adt121>) -> List<Adt121> {
    List::cons(Adt121::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f607(p2051: List<u64>, p2052: List<u64>) -> R<List<u64>> {
    let m2050 = (p2051, p2052);
    {
        let mut m2050 = m2050;
        loop {
            let step = {
                let (v0, v1) = m2050;
                let m2049 = v0;
                match m2049.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f417(f684(v2)?, v1)))
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

pub fn f608(v0: Adt37) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f685(match v0.clone() {
        Adt37::C0(_, h2053, _, _) => {
            h2053
        }
    })), List::cons((Str::lit("capabilities"), f686(match v0.clone() {
        Adt37::C0(_, _, h2054, _) => {
            h2054
        }
    })), {
        let a2057 = f687(Str::lit("instructions"), match v0.clone() {
            Adt37::C0(_, _, _, h2055) => {
                h2055
            }
        });
        let a2058 = f688(match v0 {
            Adt37::C0(h2056, _, _, _) => {
                h2056
            }
        })?;
        append_list(a2057, a2058)
    })))
}

pub fn f609(v0: Adt38) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("tools"), Adt9::C4(f689(match v0.clone() {
        Adt38::C0(_, _, h2059) => {
            h2059
        }
    }))), {
        let a2062 = f687(Str::lit("nextCursor"), match v0.clone() {
            Adt38::C0(_, h2060, _) => {
                h2060
            }
        });
        let a2063 = f688(match v0 {
            Adt38::C0(h2061, _, _) => {
                h2061
            }
        })?;
        append_list(a2062, a2063)
    }))
}

pub fn f610(v0: Option<List<(Str, Adt9)>>) -> R<Option<List<(Str, Adt9)>>> {
    let m2064 = v0;
    match m2064 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            let m2065 = f624(Adt9::C5(v1))?;
            match m2065 {
                None => {
                    Ok(None::<List<(Str, Adt9)>>)
                }
                Some(v2) => {
                    let m2066 = f226(v2);
                    match m2066 {
                        None => {
                            Ok(None::<List<(Str, Adt9)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt8::C0(h2067) => {
                                    h2067
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f611(v0: Adt41) -> R<Option<List<(Str, Adt9)>>> {
    let m2069 = f690(match v0.clone() {
        Adt41::C0(h2068, _, _) => {
            h2068
        }
    })?;
    match m2069 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt9::C4(v1)), {
                let a2072 = f691(Str::lit("structuredContent"), match v0.clone() {
                    Adt41::C0(_, h2070, _) => {
                        h2070
                    }
                });
                let a2073 = f692(Str::lit("isError"), match v0 {
                    Adt41::C0(_, _, h2071) => {
                        h2071
                    }
                });
                append_list(a2072, a2073)
            })))
        }
    }
}

pub fn f612(v0: Adt55) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("resources"), Adt9::C4(f693(match v0.clone() {
        Adt55::C0(_, _, h2074) => {
            h2074
        }
    })?)), {
        let a2077 = f687(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h2075, _) => {
                h2075
            }
        });
        let a2078 = f688(match v0 {
            Adt55::C0(h2076, _, _) => {
                h2076
            }
        })?;
        append_list(a2077, a2078)
    }))
}

pub fn f613(v0: Adt56) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("contents"), Adt9::C4(f694(match v0.clone() {
        Adt56::C0(_, h2079) => {
            h2079
        }
    }))), f688(match v0 {
        Adt56::C0(h2080, _) => {
            h2080
        }
    })?))
}

pub fn f614(v0: Adt57) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt9::C4(f695(match v0.clone() {
        Adt57::C0(_, _, h2081) => {
            h2081
        }
    }))), {
        let a2084 = f687(Str::lit("nextCursor"), match v0.clone() {
            Adt57::C0(_, h2082, _) => {
                h2082
            }
        });
        let a2085 = f688(match v0 {
            Adt57::C0(h2083, _, _) => {
                h2083
            }
        })?;
        append_list(a2084, a2085)
    }))
}

pub fn f615(v0: Adt59) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("prompts"), Adt9::C4(f696(match v0.clone() {
        Adt59::C0(_, _, h2086) => {
            h2086
        }
    }))), {
        let a2089 = f687(Str::lit("nextCursor"), match v0.clone() {
            Adt59::C0(_, h2087, _) => {
                h2087
            }
        });
        let a2090 = f688(match v0 {
            Adt59::C0(h2088, _, _) => {
                h2088
            }
        })?;
        append_list(a2089, a2090)
    }))
}

pub fn f616(v0: Adt62) -> R<Option<List<(Str, Adt9)>>> {
    let m2092 = f697(match v0.clone() {
        Adt62::C0(_, h2091) => {
            h2091
        }
    })?;
    match m2092 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2094 = f687(Str::lit("description"), match v0 {
                    Adt62::C0(h2093, _) => {
                        h2093
                    }
                });
                let a2095 = List::cons((Str::lit("messages"), Adt9::C4(v1)), List::<(Str, Adt9)>::nil());
                append_list(a2094, a2095)
            }))
        }
    }
}

pub fn f617(v0: Adt64) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("completion"), Adt9::C5(List::cons((Str::lit("values"), Adt9::C4(f634({
        let m2098 = match v0.clone() {
            Adt64::C0(h2097) => {
                h2097
            }
        };
        match m2098 {
            Adt65::C0(h2096, _, _) => {
                h2096
            }
        }
    }))), {
        let a2105 = f698(Str::lit("total"), {
            let m2101 = match v0.clone() {
                Adt64::C0(h2100) => {
                    h2100
                }
            };
            match m2101 {
                Adt65::C0(_, h2099, _) => {
                    h2099
                }
            }
        })?;
        let a2106 = f692(Str::lit("hasMore"), {
            let m2104 = match v0 {
                Adt64::C0(h2103) => {
                    h2103
                }
            };
            match m2104 {
                Adt65::C0(_, _, h2102) => {
                    h2102
                }
            }
        });
        append_list(a2105, a2106)
    }))), List::<(Str, Adt9)>::nil()))
}

pub fn f618(v0: Adt85) -> R<Option<List<(Str, Adt9)>>> {
    let m2107 = f699(f700(v0.clone()), List::<(Str, Adt9)>::nil())?;
    match m2107 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2108 = f701(v1);
                let a2109 = f702(f703(v0));
                append_list(a2108, a2109)
            }))
        }
    }
}

pub fn f619(v0: Adt156) -> Str {
    let m2110 = v0;
    match m2110 {
        Adt156::C0 => {
            Str::lit("complete")
        }
        Adt156::C1 => {
            Str::lit("input_required")
        }
        Adt156::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f620(v0: Adt36) -> Adt156 {
    let m2111 = v0;
    match m2111 {
        Adt36::C0(_) => {
            Adt156::C0
        }
        Adt36::C1(_) => {
            Adt156::C0
        }
        Adt36::C2(_) => {
            Adt156::C0
        }
        Adt36::C3(_) => {
            Adt156::C0
        }
        Adt36::C4(_) => {
            Adt156::C0
        }
        Adt36::C5(_) => {
            Adt156::C0
        }
        Adt36::C6(_) => {
            Adt156::C0
        }
        Adt36::C7(_) => {
            Adt156::C0
        }
        Adt36::C8(_) => {
            Adt156::C0
        }
        Adt36::C9(_) => {
            Adt156::C0
        }
        Adt36::C10(_) => {
            Adt156::C0
        }
        Adt36::C11 => {
            Adt156::C0
        }
        Adt36::C12(_) => {
            Adt156::C1
        }
        Adt36::C13(_) => {
            Adt156::C2
        }
    }
}

pub fn f621(v0: Option<Adt4>) -> List<(Str, Adt9)> {
    let m2112 = v0;
    match m2112 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((f704(), f705(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f622(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f706(v2) {
            let a2113 = v1;
            let a2114 = 200u64;
            nat_eq(a2113, a2114)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f623(v0: u64, v1: List<List<u64>>) -> Adt98 {
    Adt98::C0(v0, f707(), List::<List<u64>>::nil(), f708(v1))
}

pub fn f624(v0: Adt9) -> R<Option<Adt9>> {
    f709(f710(), Adt157::C0(v0), List::<Adt158>::nil())
}

pub fn f625(v0: Adt138) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("jsonrpc"), Adt9::C3(f363())), List::cons((Str::lit("method"), Adt9::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt9::C5(List::cons((Str::lit("progressToken"), f711(match v0.clone() {
        Adt138::C0(h2115, _) => {
            h2115
        }
    })), List::cons((Str::lit("progress"), Adt9::C2({
        let m2118 = match v0.clone() {
            Adt138::C0(_, h2117) => {
                h2117
            }
        };
        match m2118 {
            Adt54::C0(h2116, _, _) => {
                h2116
            }
        }
    })), {
        let a2125 = f712({
            let m2121 = match v0.clone() {
                Adt138::C0(_, h2120) => {
                    h2120
                }
            };
            match m2121 {
                Adt54::C0(_, h2119, _) => {
                    h2119
                }
            }
        });
        let a2126 = f713({
            let m2124 = match v0 {
                Adt138::C0(_, h2123) => {
                    h2123
                }
            };
            match m2124 {
                Adt54::C0(_, _, h2122) => {
                    h2122
                }
            }
        });
        append_list(a2125, a2126)
    })))), List::<(Str, Adt9)>::nil()))))
}

pub fn f626() -> u64 {
    429u64
}

pub fn f627() -> i64 {
    -32602i64
}

pub fn f628() -> i64 {
    -32022i64
}

pub fn f629() -> i64 {
    -32020i64
}

pub fn f630() -> i64 {
    -32601i64
}

pub fn f631(v0: Adt88) -> i64 {
    let m2127 = v0;
    match m2127 {
        Adt88::C0(_) => {
            f627()
        }
        Adt88::C1(_) => {
            f627()
        }
        Adt88::C2(_) => {
            f627()
        }
        Adt88::C3(_) => {
            f627()
        }
        Adt88::C4(_) => {
            f627()
        }
        Adt88::C5(_) => {
            f627()
        }
        Adt88::C6(_) => {
            f714()
        }
        Adt88::C7(_) => {
            f715()
        }
        Adt88::C8(_) => {
            f716()
        }
        Adt88::C9(_) => {
            f714()
        }
        Adt88::C10 => {
            f627()
        }
        Adt88::C11 => {
            f627()
        }
        Adt88::C12(_) => {
            f714()
        }
        Adt88::C13 => {
            f714()
        }
        Adt88::C14 => {
            f714()
        }
        Adt88::C15(_) => {
            f714()
        }
    }
}

pub fn f632(v0: Adt140) -> Str {
    let m2128 = v0;
    match m2128 {
        Adt140::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt140::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt140::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt140::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt140::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt140::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt140::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt140::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt140::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f633(v0: Adt123) -> Str {
    let m2129 = v0;
    match m2129 {
        Adt123::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt123::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt123::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt123::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt123::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt123::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt123::C6(v1) => {
            let m2130 = v1;
            match m2130 {
                Adt124::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt124::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt124::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt123::C7(v2) => {
            let m2131 = v2;
            match m2131 {
                Adt124::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt124::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt124::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt123::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt123::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt123::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f634(v0: List<Str>) -> List<Adt9> {
    f553(f717(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f635(v0: Adt21) -> Adt9 {
    Adt9::C5({
        let a2143 = f718(Str::lit("experimental"), match v0.clone() {
            Adt21::C0(h2132, _, _, _, _) => {
                h2132
            }
        });
        let a2144 = {
            let a2141 = f718(Str::lit("roots"), match v0.clone() {
                Adt21::C0(_, h2133, _, _, _) => {
                    h2133
                }
            });
            let a2142 = {
                let a2139 = f718(Str::lit("sampling"), match v0.clone() {
                    Adt21::C0(_, _, h2134, _, _) => {
                        h2134
                    }
                });
                let a2140 = {
                    let a2137 = f718(Str::lit("elicitation"), match v0.clone() {
                        Adt21::C0(_, _, _, h2135, _) => {
                            h2135
                        }
                    });
                    let a2138 = f718(Str::lit("extensions"), match v0 {
                        Adt21::C0(_, _, _, _, h2136) => {
                            h2136
                        }
                    });
                    append_list(a2137, a2138)
                };
                append_list(a2139, a2140)
            };
            append_list(a2141, a2142)
        };
        append_list(a2143, a2144)
    })
}

pub fn f636(p2147: List<List<u64>>) -> R<bool> {
    let m2146 = p2147;
    {
        let mut m2146 = m2146;
        loop {
            let step = {
                let v0 = m2146;
                let m2145 = v0;
                match m2145.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f719(f378(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
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

pub fn f637() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f638(v0: Adt3, v1: Fn1, v2: u64, v3: Adt147, v4: bool) -> R<Result<Adt136, Adt139>> {
    if v4 {
        f720(v0, v1, v2, f721(match v3.clone() {
            Adt147::C0(_, h2148, _, _) => {
                h2148
            }
        }, match v3.clone() {
            Adt147::C0(_, _, h2149, _) => {
                h2149
            }
        }, match v3 {
            Adt147::C0(_, _, _, h2150) => {
                h2150
            }
        }))
    } else {
        Ok(Err::<Adt136, Adt139>(Adt139::C3(f722(match v3 {
            Adt147::C0(_, h2151, _, _) => {
                h2151
            }
        }))))
    }
}

pub fn f639(v0: Adt7, v1: Adt148) -> bool {
    let m2152 = v1;
    match m2152 {
        Adt148::C0 => {
            true
        }
        Adt148::C1 => {
            f723(match v0 {
                Adt7::C0(_, _, _, _, _, h2153, _) => {
                    h2153
                }
            })
        }
        Adt148::C2 => {
            f723(match v0 {
                Adt7::C0(_, _, _, _, _, h2154, _) => {
                    h2154
                }
            })
        }
        Adt148::C3 => {
            f724(match v0 {
                Adt7::C0(_, _, _, _, h2155, _, _) => {
                    h2155
                }
            })
        }
        Adt148::C4 => {
            f724(match v0 {
                Adt7::C0(_, _, _, _, h2156, _, _) => {
                    h2156
                }
            })
        }
        Adt148::C5 => {
            f724(match v0 {
                Adt7::C0(_, _, _, _, h2157, _, _) => {
                    h2157
                }
            })
        }
        Adt148::C6 => {
            f723(match v0 {
                Adt7::C0(_, _, _, h2158, _, _, _) => {
                    h2158
                }
            })
        }
        Adt148::C7 => {
            f723(match v0 {
                Adt7::C0(_, _, _, h2159, _, _, _) => {
                    h2159
                }
            })
        }
        Adt148::C8 => {
            f286(match v0 {
                Adt7::C0(_, _, h2160, _, _, _, _) => {
                    h2160
                }
            })
        }
        Adt148::C9 => {
            f725(match v0 {
                Adt7::C0(_, _, _, _, _, _, h2161) => {
                    h2161
                }
            }, f726())
        }
        Adt148::C10 => {
            f725(match v0 {
                Adt7::C0(_, _, _, _, _, _, h2162) => {
                    h2162
                }
            }, f726())
        }
        Adt148::C11 => {
            f725(match v0 {
                Adt7::C0(_, _, _, _, _, _, h2163) => {
                    h2163
                }
            }, f726())
        }
    }
}

pub fn f640(v0: Adt152, v1: Adt150) -> Result<Adt147, Adt139> {
    let m2164 = v1;
    match m2164 {
        Adt150::C0 => {
            Err::<Adt147, Adt139>(Adt139::C3(match v0 {
                Adt152::C0(_, h2165, _, _) => {
                    h2165
                }
            }))
        }
        Adt150::C1(v2) => {
            Ok::<Adt147, Adt139>(Adt147::C0(match v0.clone() {
                Adt152::C0(h2166, _, _, _) => {
                    h2166
                }
            }, v2, match v0.clone() {
                Adt152::C0(_, _, h2167, _) => {
                    h2167
                }
            }, match v0 {
                Adt152::C0(_, _, _, h2168) => {
                    h2168
                }
            }))
        }
    }
}

pub fn f641(v0: Option<Adt122>, v1: List<Str>, v2: Adt118, v3: Result<Adt20, Adt140>) -> R<Adt151> {
    let m2169 = v3;
    match m2169 {
        Ok(v5) => {
            Ok(f727(v1, v2.clone(), v5.clone(), f728(v0, f541(match v2.clone() {
                Adt118::C0(_, h2170, _) => {
                    h2170
                }
            }), match v2.clone() {
                Adt118::C0(_, h2171, _) => {
                    h2171
                }
            }, match v5 {
                Adt20::C0(h2172, _, _, _, _, _) => {
                    h2172
                }
            }, f729(match v2 {
                Adt118::C0(_, _, h2173) => {
                    h2173
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt151::C1(Adt139::C0(v4)))
        }
    }
}

pub fn f642(v0: Option<Adt8>) -> Result<Adt20, Adt140> {
    let m2174 = v0;
    match m2174 {
        None => {
            Err::<Adt20, Adt140>(Adt140::C0)
        }
        Some(v1) => {
            let m2175 = f225(Str::lit("_meta"), v1);
            match m2175 {
                Adt110::C0 => {
                    Err::<Adt20, Adt140>(Adt140::C1)
                }
                Adt110::C1(v2) => {
                    let m2176 = f226(v2);
                    match m2176 {
                        None => {
                            Err::<Adt20, Adt140>(Adt140::C4)
                        }
                        Some(v3) => {
                            f730(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f643(v0: Adt148) -> Adt153 {
    let m2177 = v0;
    match m2177 {
        Adt148::C0 => {
            Adt153::C0
        }
        Adt148::C1 => {
            Adt153::C0
        }
        Adt148::C2 => {
            Adt153::C1
        }
        Adt148::C3 => {
            Adt153::C0
        }
        Adt148::C4 => {
            Adt153::C2
        }
        Adt148::C5 => {
            Adt153::C0
        }
        Adt148::C6 => {
            Adt153::C0
        }
        Adt148::C7 => {
            Adt153::C1
        }
        Adt148::C8 => {
            Adt153::C0
        }
        Adt148::C9 => {
            Adt153::C3
        }
        Adt148::C10 => {
            Adt153::C3
        }
        Adt148::C11 => {
            Adt153::C3
        }
    }
}

pub fn f644(p2183: List<u64>, p2184: List<u64>) -> Option<List<u64>> {
    let m2182 = (p2183, p2184);
    {
        let mut m2182 = m2182;
        loop {
            let step = {
                let (v0, v1) = m2182;
                let m2178 = v0;
                match m2178.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2179 = v1;
                        match m2179.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a2180 = v4;
                                let a2181 = v2;
                                if nat_eq(a2180, a2181) {
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
                Ok(next) => { m2182 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f645() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f646() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f647(v0: Adt15, v1: Option<Str>) -> bool {
    let m2185 = v1;
    match m2185 {
        None => {
            false
        }
        Some(v2) => {
            let a2187 = match v0 {
                Adt15::C0(h2186, _, _, _) => {
                    h2186
                }
            };
            let a2188 = v2;
            equal(a2187, a2188)
        }
    }
}

pub fn f648(v0: List<Adt95>, v1: Str) -> R<Adt126> {
    let m2191 = f470(f258(v0, {
        let a2189 = f731();
        let a2190 = f52(v1)?;
        append_list(a2189, a2190)
    })?)?;
    match m2191 {
        Adt114::C0 => {
            Ok(Adt126::C0)
        }
        Adt114::C1 => {
            Ok(Adt126::C1)
        }
        Adt114::C2 => {
            Ok(Adt126::C1)
        }
        Adt114::C3(v2) => {
            let m2192 = f540(v2);
            match m2192 {
                None => {
                    Ok(Adt126::C1)
                }
                Some(v3) => {
                    Ok(Adt126::C2(v3))
                }
            }
        }
    }
}

pub fn f649(v0: i64) -> bool {
    let a2199 = {
        let m2195 = {
            let a2193 = v0;
            let a2194 = 0i64;
            compare(a2193, a2194)
        };
        if match m2195 {
            Ordering::Less => {
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
            let m2198 = {
                let a2196 = 0i64;
                let a2197 = v0;
                compare(a2196, a2197)
            };
            match m2198 {
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
    bool_not(a2199)
}

pub fn f650(v0: Adt129, v1: u64, v2: Str) -> Adt143 {
    let m2201 = match v0.clone() {
        Adt129::C0(_, h2200, _) => {
            h2200
        }
    };
    match m2201.uncons() {
        None => {
            f489(Adt111::C2(v1))
        }
        Some((v3, v4)) => {
            let m2202 = v3;
            match m2202 {
                Adt134::C0(_) => {
                    f489(Adt111::C2(v1))
                }
                Adt134::C1(v6, _) => {
                    Adt143::C1(Adt129::C0(Adt130::C5, List::cons(Adt134::C1(v6, Some(v2)), v4), match v0 {
                        Adt129::C0(_, _, h2203) => {
                            h2203
                        }
                    }))
                }
            }
        }
    }
}

pub fn f651(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a2204 = v0;
    let a2205 = 128u64;
    if nat_lt(a2204, a2205) {
        Ok(List::cons(v0, v1))
    } else {
        let a2206 = v0;
        let a2207 = 2048u64;
        if nat_lt(a2206, a2207) {
            Ok(List::cons({
                let a2208 = 128u64;
                let a2209 = f732(v0, 64u64);
                nat_add(a2208, a2209)?
            }, List::cons({
                let a2210 = 192u64;
                let a2211 = f733(v0, 64u64);
                nat_add(a2210, a2211)?
            }, v1)))
        } else {
            let a2212 = v0;
            let a2213 = 65536u64;
            if nat_lt(a2212, a2213) {
                Ok(List::cons({
                    let a2214 = 128u64;
                    let a2215 = f732(v0, 64u64);
                    nat_add(a2214, a2215)?
                }, List::cons({
                    let a2216 = 128u64;
                    let a2217 = f732(f733(v0, 64u64), 64u64);
                    nat_add(a2216, a2217)?
                }, List::cons({
                    let a2218 = 224u64;
                    let a2219 = f733(v0, 4096u64);
                    nat_add(a2218, a2219)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a2220 = 128u64;
                    let a2221 = f732(v0, 64u64);
                    nat_add(a2220, a2221)?
                }, List::cons({
                    let a2222 = 128u64;
                    let a2223 = f732(f733(v0, 64u64), 64u64);
                    nat_add(a2222, a2223)?
                }, List::cons({
                    let a2224 = 128u64;
                    let a2225 = f732(f733(v0, 4096u64), 64u64);
                    nat_add(a2224, a2225)?
                }, List::cons({
                    let a2226 = 240u64;
                    let a2227 = f733(v0, 262144u64);
                    nat_add(a2226, a2227)?
                }, v1)))))
            }
        }
    }
}

pub fn f652() -> u64 {
    18u64
}

pub fn f653() -> u64 {
    4294967296u64
}

pub fn f654(v0: Adt9) -> bool {
    let m2228 = f275(v0);
    match m2228 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f655(v0: Adt9) -> bool {
    let m2229 = f585(v0);
    match m2229 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f656(v0: Adt9) -> bool {
    let m2230 = f675(v0);
    match m2230 {
        None => {
            false
        }
        Some(v1) => {
            let a2234 = {
                let m2233 = {
                    let a2231 = v1;
                    let a2232 = 0i64;
                    compare(a2231, a2232)
                };
                match m2233 {
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
            bool_not(a2234)
        }
    }
}

pub fn f657(v0: Adt9) -> bool {
    let m2235 = f278(v0);
    match m2235 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f658(v0: Adt9) -> bool {
    let m2236 = f226(v0);
    match m2236 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f659(v0: Adt9) -> bool {
    let m2237 = f278(v0);
    match m2237 {
        None => {
            false
        }
        Some(v1) => {
            f734(v1)
        }
    }
}

pub fn f660(v0: Adt9) -> bool {
    let m2238 = f346(v0);
    match m2238 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f661(v0: Adt9) -> bool {
    let m2239 = f275(v0.clone());
    match m2239 {
        None => {
            let m2240 = f278(v0);
            match m2240 {
                None => {
                    false
                }
                Some(v2) => {
                    f735(v2)
                }
            }
        }
        Some(v1) => {
            f580(v1)
        }
    }
}

pub fn f662(v0: Adt9) -> bool {
    let m2241 = f278(v0);
    match m2241 {
        None => {
            false
        }
        Some(v1) => {
            let m2242 = v1;
            match m2242.uncons() {
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

pub fn f663(v0: Str, p2260: List<(Str, Adt9)>) -> Option<Adt9> {
    let m2259 = p2260;
    {
        let mut m2259 = m2259;
        loop {
            let step = {
                let v1 = m2259;
                let m2243 = v1;
                match m2243.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, Option<Adt9>>(None::<Adt9>)
                    }
                    Some((v2, v3)) => {
                        let a2247 = v0.clone();
                        let a2248 = {
                            let a2245 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2244, _) = v2.clone();
                                h2244
                            }, List::<Str>::nil()));
                            let a2246 = Str::lit("");
                            join(a2245, a2246)
                        };
                        if equal(a2247, a2248) {
                            Err::<List<(Str, Adt9)>, Option<Adt9>>(Some({
                                let (_, h2249) = v2;
                                h2249
                            }))
                        } else {
                            let m2251 = f226({
                                let (_, h2250) = v2.clone();
                                h2250
                            });
                            match m2251 {
                                None => {
                                    Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                }
                                Some(v4) => {
                                    let m2252 = f225(Str::lit("$anchor"), v4);
                                    match m2252 {
                                        Adt110::C0 => {
                                            Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                        }
                                        Adt110::C1(v5) => {
                                            let m2253 = f275(v5);
                                            match m2253 {
                                                None => {
                                                    Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2256 = v0.clone();
                                                    let a2257 = {
                                                        let a2254 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2255 = Str::lit("");
                                                        join(a2254, a2255)
                                                    };
                                                    if equal(a2256, a2257) {
                                                        Err::<List<(Str, Adt9)>, Option<Adt9>>(Some({
                                                            let (_, h2258) = v2;
                                                            h2258
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
                Ok(next) => { m2259 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f664(v0: List<(Str, Adt9)>) -> Option<Adt16> {
    let m2261 = f297(Str::lit("type"), v0);
    match m2261 {
        Adt110::C0 => {
            None::<Adt16>
        }
        Adt110::C1(v1) => {
            let m2262 = f275(v1);
            match m2262 {
                None => {
                    None::<Adt16>
                }
                Some(v2) => {
                    f736(v2)
                }
            }
        }
    }
}

pub fn f665(v0: List<u64>) -> R<List<u64>> {
    Ok(f417(f737(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f666(v0: List<u64>, p2265: List<List<u64>>) -> bool {
    let m2264 = p2265;
    {
        let mut m2264 = m2264;
        loop {
            let step = {
                let v1 = m2264;
                let m2263 = v1;
                match m2263.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f738(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2264 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f667(v0: Adt9) -> Str {
    let m2266 = v0;
    match m2266 {
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

pub fn f668(v0: Adt10) -> R<bool> {
    if f649(match v0.clone() {
        Adt10::C0(h2267, _) => {
            h2267
        }
    }) {
        Ok(true)
    } else {
        f739(40u64, match v0.clone() {
            Adt10::C0(h2268, _) => {
                h2268
            }
        }, match v0 {
            Adt10::C0(_, h2269) => {
                h2269
            }
        })
    }
}

pub fn f669(v0: Adt9) -> Str {
    let m2270 = f275(v0);
    match m2270 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f670(p2277: u64, p2278: List<(Adt9, Adt9)>) -> R<Adt154> {
    let m2276 = (p2277, p2278);
    {
        let mut m2276 = m2276;
        loop {
            let step = {
                let (v0, v1) = m2276;
                let m2271 = v0;
                if m2271 == 0 {
                    Err::<(u64, List<(Adt9, Adt9)>), R<Adt154>>(Ok(Adt154::C2))
                } else {
                    let v2 = m2271 - 1;
                    let m2272 = v1;
                    match m2272.uncons() {
                        None => {
                            Err::<(u64, List<(Adt9, Adt9)>), R<Adt154>>(Ok(Adt154::C0))
                        }
                        Some((v3, v4)) => {
                            let m2275 = f740({
                                let (h2273, _) = v3;
                                h2273
                            }, {
                                let (_, h2274) = v3;
                                h2274
                            }, v4)?;
                            match m2275 {
                                Adt159::C0 => {
                                    Err::<(u64, List<(Adt9, Adt9)>), R<Adt154>>(Ok(Adt154::C1))
                                }
                                Adt159::C1(v5) => {
                                    Ok::<(u64, List<(Adt9, Adt9)>), R<Adt154>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f671() -> u64 {
    67108864u64
}

pub fn f672(v0: Adt155) -> bool {
    let m2279 = v0;
    match m2279 {
        Adt155::C0 => {
            true
        }
        Adt155::C1 => {
            false
        }
        Adt155::C2 => {
            false
        }
    }
}

pub fn f673(v0: Adt10, v1: Adt10, v2: Adt155, v3: Adt155) -> R<Adt155> {
    let m2280 = v2;
    match m2280 {
        Adt155::C0 => {
            let m2282 = v3;
            match m2282 {
                Adt155::C0 => {
                    Ok(f741(Adt155::C0, f742(f743(match v0.clone() {
                        Adt10::C0(h2283, _) => {
                            h2283
                        }
                    })?, match v0 {
                        Adt10::C0(_, h2284) => {
                            h2284
                        }
                    }, f743(match v1.clone() {
                        Adt10::C0(h2285, _) => {
                            h2285
                        }
                    })?, match v1 {
                        Adt10::C0(_, h2286) => {
                            h2286
                        }
                    })?))
                }
                Adt155::C1 => {
                    Ok(Adt155::C0)
                }
                Adt155::C2 => {
                    Ok(Adt155::C0)
                }
            }
        }
        Adt155::C1 => {
            let m2281 = v3;
            match m2281 {
                Adt155::C0 => {
                    Ok(Adt155::C2)
                }
                Adt155::C1 => {
                    Ok(Adt155::C1)
                }
                Adt155::C2 => {
                    Ok(Adt155::C0)
                }
            }
        }
        Adt155::C2 => {
            let m2287 = v3;
            match m2287 {
                Adt155::C0 => {
                    Ok(Adt155::C2)
                }
                Adt155::C1 => {
                    Ok(Adt155::C2)
                }
                Adt155::C2 => {
                    f742(match v0.clone() {
                        Adt10::C0(h2288, _) => {
                            h2288
                        }
                    }, match v0 {
                        Adt10::C0(_, h2289) => {
                            h2289
                        }
                    }, match v1.clone() {
                        Adt10::C0(h2290, _) => {
                            h2290
                        }
                    }, match v1 {
                        Adt10::C0(_, h2291) => {
                            h2291
                        }
                    })
                }
            }
        }
    }
}

pub fn f674(v0: i64) -> Adt155 {
    f676(v0, 0i64)
}

pub fn f675(v0: Adt9) -> Option<i64> {
    let m2292 = v0;
    match m2292 {
        Adt9::C0 => {
            None::<i64>
        }
        Adt9::C1(_) => {
            None::<i64>
        }
        Adt9::C2(v2) => {
            if f649(match v2.clone() {
                Adt10::C0(_, h2293) => {
                    h2293
                }
            }) {
                Some(match v2 {
                    Adt10::C0(h2294, _) => {
                        h2294
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

pub fn f676(v0: i64, v1: i64) -> Adt155 {
    let m2297 = {
        let a2295 = v0;
        let a2296 = v1;
        compare(a2295, a2296)
    };
    if match m2297 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Adt155::C0
    } else {
        let m2300 = {
            let a2298 = v1;
            let a2299 = v0;
            compare(a2298, a2299)
        };
        if match m2300 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Adt155::C2
        } else {
            Adt155::C1
        }
    }
}

pub fn f677(p2309: List<u64>, p2310: u64) -> R<u64> {
    let m2308 = (p2309, p2310);
    {
        let mut m2308 = m2308;
        loop {
            let step = {
                let (v0, v1) = m2308;
                let m2301 = v0;
                match m2301.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2302 = v2;
                        let a2303 = 128u64;
                        if if nat_lt(a2302, a2303) {
                            true
                        } else {
                            let a2304 = 191u64;
                            let a2305 = v2;
                            nat_lt(a2304, a2305)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2306 = v1;
                                let a2307 = 1u64;
                                nat_add(a2306, a2307)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f678(v0: u64) -> R<Option<u64>> {
    let a2311 = 64u64;
    let a2312 = v0;
    if if nat_lt(a2311, a2312) {
        let a2313 = v0;
        let a2314 = 91u64;
        nat_lt(a2313, a2314)
    } else {
        false
    } {
        Ok(Some({
            let a2315 = v0;
            let a2316 = 65u64;
            nat_sub(a2315, a2316)
        }))
    } else {
        let a2317 = 96u64;
        let a2318 = v0;
        if if nat_lt(a2317, a2318) {
            let a2319 = v0;
            let a2320 = 123u64;
            nat_lt(a2319, a2320)
        } else {
            false
        } {
            Ok(Some({
                let a2321 = v0;
                let a2322 = 71u64;
                nat_sub(a2321, a2322)
            }))
        } else {
            let a2323 = 47u64;
            let a2324 = v0;
            if if nat_lt(a2323, a2324) {
                let a2325 = v0;
                let a2326 = 58u64;
                nat_lt(a2325, a2326)
            } else {
                false
            } {
                Ok(Some({
                    let a2327 = v0;
                    let a2328 = 4u64;
                    nat_add(a2327, a2328)?
                }))
            } else {
                let a2329 = v0;
                let a2330 = 43u64;
                if nat_eq(a2329, a2330) {
                    Ok(Some(62u64))
                } else {
                    let a2331 = v0;
                    let a2332 = 47u64;
                    if nat_eq(a2331, a2332) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f679(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2333 = v3;
        let a2334 = 61u64;
        if nat_eq(a2333, a2334) {
            let a2335 = f744(v1, 16u64);
            let a2336 = 0u64;
            nat_eq(a2335, a2336)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f745(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f680(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2337 = f744(v2, 4u64);
        let a2338 = 0u64;
        nat_eq(a2337, a2338)
    } else {
        false
    } {
        Ok(Some(f746(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f681(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2341 = {
            let a2339 = v0;
            let a2340 = 4u64;
            nat_mul(a2339, a2340)?
        };
        let a2342 = f747(v1, 16u64);
        nat_add(a2341, a2342)?
    }, List::cons({
        let a2345 = {
            let a2343 = f744(v1, 16u64);
            let a2344 = 16u64;
            nat_mul(a2343, a2344)?
        };
        let a2346 = f747(v2, 4u64);
        nat_add(a2345, a2346)?
    }, List::cons({
        let a2349 = {
            let a2347 = f744(v2, 4u64);
            let a2348 = 64u64;
            nat_mul(a2347, a2348)?
        };
        let a2350 = v3;
        nat_add(a2349, a2350)?
    }, List::<u64>::nil()))))
}

pub fn f682() -> i64 {
    999999999999999999i64
}

pub fn f683(v0: i64) -> R<List<u64>> {
    let m2353 = {
        let a2351 = v0;
        let a2352 = 0i64;
        compare(a2351, a2352)
    };
    if match m2353 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f748(20u64, {
            let a2354 = v0;
            int_neg(a2354)?
        }, List::<u64>::nil())?))
    } else {
        f748(20u64, v0, List::<u64>::nil())
    }
}

pub fn f684(v0: u64) -> R<List<u64>> {
    let a2355 = v0;
    let a2356 = 34u64;
    if nat_eq(a2355, a2356) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2357 = v0;
        let a2358 = 92u64;
        if nat_eq(a2357, a2358) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2359 = v0;
            let a2360 = 8u64;
            if nat_eq(a2359, a2360) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2361 = v0;
                let a2362 = 9u64;
                if nat_eq(a2361, a2362) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2363 = v0;
                    let a2364 = 10u64;
                    if nat_eq(a2363, a2364) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2365 = v0;
                        let a2366 = 12u64;
                        if nat_eq(a2365, a2366) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2367 = v0;
                            let a2368 = 13u64;
                            if nat_eq(a2367, a2368) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2369 = v0;
                                let a2370 = 32u64;
                                if nat_lt(a2369, a2370) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f749(f733(v0, 16u64))?, List::cons(f749(f732(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f685(v0: List<Str>) -> Adt9 {
    Adt9::C4(f634(v0))
}

pub fn f686(v0: Adt7) -> Adt9 {
    Adt9::C5({
        let a2388 = f718(Str::lit("experimental"), match v0.clone() {
            Adt7::C0(h2371, _, _, _, _, _, _) => {
                h2371
            }
        });
        let a2389 = {
            let a2386 = f718(Str::lit("logging"), match v0.clone() {
                Adt7::C0(_, h2372, _, _, _, _, _) => {
                    h2372
                }
            });
            let a2387 = {
                let a2384 = f718(Str::lit("completions"), match v0.clone() {
                    Adt7::C0(_, _, h2373, _, _, _, _) => {
                        h2373
                    }
                });
                let a2385 = {
                    let a2382 = f750(Str::lit("prompts"), match v0.clone() {
                        Adt7::C0(_, _, _, h2374, _, _, _) => {
                            h2374
                        }
                    });
                    let a2383 = {
                        let a2380 = f751(match v0.clone() {
                            Adt7::C0(_, _, _, _, h2375, _, _) => {
                                h2375
                            }
                        });
                        let a2381 = {
                            let a2378 = f750(Str::lit("tools"), match v0.clone() {
                                Adt7::C0(_, _, _, _, _, h2376, _) => {
                                    h2376
                                }
                            });
                            let a2379 = f718(Str::lit("extensions"), match v0 {
                                Adt7::C0(_, _, _, _, _, _, h2377) => {
                                    h2377
                                }
                            });
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
    })
}

pub fn f687(v0: Str, v1: Option<Str>) -> List<(Str, Adt9)> {
    let m2390 = v1;
    match m2390 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C3(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f688(v0: Adt13) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("ttlMs"), f112(match v0.clone() {
        Adt13::C0(h2391, _) => {
            h2391
        }
    })?), List::cons((Str::lit("cacheScope"), Adt9::C3(f752(match v0 {
        Adt13::C0(_, h2392) => {
            h2392
        }
    }))), List::<(Str, Adt9)>::nil())))
}

pub fn f689(v0: List<Adt39>) -> List<Adt9> {
    f553(f753(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f690(v0: List<Adt42>) -> R<Option<List<Adt9>>> {
    let m2393 = f754(v0, List::<Adt9>::nil())?;
    match m2393 {
        None => {
            Ok(None::<List<Adt9>>)
        }
        Some(v1) => {
            Ok(Some(f553(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f691(v0: Str, v1: Option<Adt9>) -> List<(Str, Adt9)> {
    let m2394 = v1;
    match m2394 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f692(v0: Str, v1: Option<bool>) -> List<(Str, Adt9)> {
    let m2395 = v1;
    match m2395 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C1(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f693(v0: List<Adt48>) -> R<List<Adt9>> {
    Ok(f553(f755(v0, List::<Adt9>::nil())?, List::<Adt9>::nil()))
}

pub fn f694(v0: List<Adt50>) -> List<Adt9> {
    f553(f756(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f695(v0: List<Adt58>) -> List<Adt9> {
    f553(f757(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f696(v0: List<Adt60>) -> List<Adt9> {
    f553(f758(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f697(v0: List<Adt63>) -> R<Option<List<Adt9>>> {
    let m2396 = f759(v0, List::<Adt9>::nil())?;
    match m2396 {
        None => {
            Ok(None::<List<Adt9>>)
        }
        Some(v1) => {
            Ok(Some(f553(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f698(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt9)>> {
    let m2397 = v1;
    match m2397 {
        None => {
            Ok(List::<(Str, Adt9)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f112(v2)?), List::<(Str, Adt9)>::nil()))
        }
    }
}

pub fn f699(p2403: List<(Str, Adt68)>, p2404: List<(Str, Adt9)>) -> R<Option<List<(Str, Adt9)>>> {
    let m2402 = (p2403, p2404);
    {
        let mut m2402 = m2402;
        loop {
            let step = {
                let (v0, v1) = m2402;
                let m2398 = v0;
                match m2398.uncons() {
                    None => {
                        Err::<(List<(Str, Adt68)>, List<(Str, Adt9)>), R<Option<List<(Str, Adt9)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2400 = f760({
                            let (_, h2399) = v2;
                            h2399
                        })?;
                        match m2400 {
                            None => {
                                Err::<(List<(Str, Adt68)>, List<(Str, Adt9)>), R<Option<List<(Str, Adt9)>>>>(Ok(None::<List<(Str, Adt9)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt68)>, List<(Str, Adt9)>), R<Option<List<(Str, Adt9)>>>>((v3, List::cons(({
                                    let (h2401, _) = v2;
                                    h2401
                                }, v4), v1)))
                            }
                        }
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

pub fn f700(v0: Adt85) -> List<(Str, Adt68)> {
    let m2405 = v0;
    match m2405 {
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

pub fn f701(v0: List<(Str, Adt9)>) -> List<(Str, Adt9)> {
    let m2406 = v0.clone();
    match m2406.uncons() {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt9::C5(f555(v0, List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f702(v0: Option<Str>) -> List<(Str, Adt9)> {
    let m2407 = v0;
    match m2407 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f703(v0: Adt85) -> Option<Str> {
    let m2408 = v0;
    match m2408 {
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

pub fn f704() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f705(v0: Adt4) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
        Adt4::C0(h2409, _, _, _, _, _) => {
            h2409
        }
    })), List::cons((Str::lit("version"), Adt9::C3(match v0.clone() {
        Adt4::C0(_, _, h2410, _, _, _) => {
            h2410
        }
    })), {
        let a2419 = f687(Str::lit("title"), match v0.clone() {
            Adt4::C0(_, h2411, _, _, _, _) => {
                h2411
            }
        });
        let a2420 = {
            let a2417 = f687(Str::lit("description"), match v0.clone() {
                Adt4::C0(_, _, _, h2412, _, _) => {
                    h2412
                }
            });
            let a2418 = {
                let a2415 = f687(Str::lit("websiteUrl"), match v0.clone() {
                    Adt4::C0(_, _, _, _, h2413, _) => {
                        h2413
                    }
                });
                let a2416 = f761(match v0 {
                    Adt4::C0(_, _, _, _, _, h2414) => {
                        h2414
                    }
                });
                append_list(a2415, a2416)
            };
            append_list(a2417, a2418)
        };
        append_list(a2419, a2420)
    })))
}

pub fn f706(v0: List<List<u64>>) -> bool {
    let m2421 = v0;
    match m2421.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f707() -> List<Adt95> {
    List::cons(Adt95::C0(f762(), f637()), List::cons(Adt95::C0(f763(), f764()), List::cons(Adt95::C0(f765(), f766()), List::<Adt95>::nil())))
}

pub fn f708(v0: List<List<u64>>) -> List<u64> {
    f767(f166(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f709(p2436: u64, p2437: Adt157, p2438: List<Adt158>) -> R<Option<Adt9>> {
    let m2435 = (p2436, (p2437, p2438));
    {
        let mut m2435 = m2435;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2435;
                let m2422 = v0;
                if m2422 == 0 {
                    Err::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>(Ok(None::<Adt9>))
                } else {
                    let v3 = m2422 - 1;
                    let m2423 = v1;
                    match m2423 {
                        Adt157::C0(v4) => {
                            let m2424 = v4.clone();
                            match m2424 {
                                Adt9::C0 => {
                                    Ok::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>((v3, (Adt157::C1(v4), v2)))
                                }
                                Adt9::C1(_) => {
                                    Ok::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>((v3, (Adt157::C1(v4), v2)))
                                }
                                Adt9::C2(_) => {
                                    Ok::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>((v3, (Adt157::C1(v4), v2)))
                                }
                                Adt9::C3(v7) => {
                                    let v8: Str = f768(v7)?;
                                    Ok::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>((v3, (Adt157::C1(Adt9::C3(v8)), v2)))
                                }
                                Adt9::C4(v9) => {
                                    let m2425 = v9;
                                    match m2425.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>((v3, (Adt157::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>((v3, (Adt157::C0(v10), List::cons(Adt158::C0(List::<Adt9>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt9::C5(v12) => {
                                    let m2426 = v12;
                                    match m2426.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>((v3, (Adt157::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f768({
                                                let (h2427, _) = v13;
                                                h2427
                                            })?;
                                            Ok::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>((v3, (Adt157::C0({
                                                let (_, h2428) = v13;
                                                h2428
                                            }), List::cons(Adt158::C1(v15, List::<(Str, Adt9)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt157::C1(v16) => {
                            let m2429 = v2;
                            match m2429.uncons() {
                                None => {
                                    Err::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m2430 = v17;
                                    match m2430 {
                                        Adt158::C0(v19, v20) => {
                                            let v21: List<Adt9> = List::cons(v16, v19);
                                            let m2431 = v20;
                                            match m2431.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>((v3, (Adt157::C1(Adt9::C4(f553(v21, List::<Adt9>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>((v3, (Adt157::C0(v22), List::cons(Adt158::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt158::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt9)> = List::cons((v24, v16), v25);
                                            let m2432 = v26;
                                            match m2432.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>((v3, (Adt157::C1(Adt9::C5(f555(v27, List::<(Str, Adt9)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f768({
                                                        let (h2433, _) = v28;
                                                        h2433
                                                    })?;
                                                    Ok::<(u64, (Adt157, List<Adt158>)), R<Option<Adt9>>>((v3, (Adt157::C0({
                                                        let (_, h2434) = v28;
                                                        h2434
                                                    }), List::cons(Adt158::C1(v30, v27, v29), v18))))
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
                Ok(next) => { m2435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f710() -> u64 {
    67108864u64
}

pub fn f711(v0: Adt23) -> Adt9 {
    let m2439 = v0;
    match m2439 {
        Adt23::C0(v1) => {
            Adt9::C3(v1)
        }
        Adt23::C1(v2) => {
            Adt9::C2(Adt10::C0(v2, 0i64))
        }
    }
}

pub fn f712(v0: Option<Adt10>) -> List<(Str, Adt9)> {
    let m2440 = v0;
    match m2440 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt9::C2(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f713(v0: Option<Str>) -> List<(Str, Adt9)> {
    let m2441 = v0;
    match m2441 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f714() -> i64 {
    -32603i64
}

pub fn f715() -> i64 {
    429i64
}

pub fn f716() -> i64 {
    -32021i64
}

pub fn f717(p2444: List<Str>, p2445: List<Adt9>) -> List<Adt9> {
    let m2443 = (p2444, p2445);
    {
        let mut m2443 = m2443;
        loop {
            let step = {
                let (v0, v1) = m2443;
                let m2442 = v0;
                match m2442.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt9>), List<Adt9>>((v3, List::cons(Adt9::C3(v2), v1)))
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

pub fn f718(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt9)> {
    let m2446 = v1;
    match m2446 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C5(match v2 {
                Adt8::C0(h2447) => {
                    h2447
                }
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f719(v0: List<u64>) -> R<bool> {
    let m2448 = v0;
    match m2448.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2449 = v2;
            match m2449.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2450 = f437(v1)?;
                    let a2451 = 113u64;
                    if if nat_eq(a2450, a2451) {
                        let a2452 = v3;
                        let a2453 = 61u64;
                        nat_eq(a2452, a2453)
                    } else {
                        false
                    } {
                        Ok(f769(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f720(v0: Adt3, v1: Fn1, v2: u64, v3: Result<Adt18, Adt140>) -> R<Result<Adt136, Adt139>> {
    let m2454 = v3;
    match m2454 {
        Ok(v5) => {
            f770(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt136, Adt139>(Adt139::C0(v4)))
        }
    }
}

pub fn f721(v0: Adt148, v1: Adt20, v2: Adt8) -> Result<Adt18, Adt140> {
    let m2455 = v0;
    match m2455 {
        Adt148::C0 => {
            f771(v1, v2)
        }
        Adt148::C1 => {
            f772(v1, v2)
        }
        Adt148::C2 => {
            f773(v1, v2)
        }
        Adt148::C3 => {
            f774(v1, v2)
        }
        Adt148::C4 => {
            f775(v1, v2)
        }
        Adt148::C5 => {
            f776(v1, v2)
        }
        Adt148::C6 => {
            f777(v1, v2)
        }
        Adt148::C7 => {
            f778(v1, v2)
        }
        Adt148::C8 => {
            f779(v1, v2)
        }
        Adt148::C9 => {
            f780(v1, v2)
        }
        Adt148::C10 => {
            f781(v1, v2)
        }
        Adt148::C11 => {
            f782(v1, v2)
        }
    }
}

pub fn f722(v0: Adt148) -> Str {
    let m2456 = v0;
    match m2456 {
        Adt148::C0 => {
            Str::lit("server/discover")
        }
        Adt148::C1 => {
            Str::lit("tools/list")
        }
        Adt148::C2 => {
            Str::lit("tools/call")
        }
        Adt148::C3 => {
            Str::lit("resources/list")
        }
        Adt148::C4 => {
            Str::lit("resources/read")
        }
        Adt148::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt148::C6 => {
            Str::lit("prompts/list")
        }
        Adt148::C7 => {
            Str::lit("prompts/get")
        }
        Adt148::C8 => {
            Str::lit("completion/complete")
        }
        Adt148::C9 => {
            Str::lit("tasks/get")
        }
        Adt148::C10 => {
            Str::lit("tasks/update")
        }
        Adt148::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f723(v0: Option<Adt11>) -> bool {
    let m2457 = v0;
    match m2457 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f724(v0: Option<Adt12>) -> bool {
    let m2458 = v0;
    match m2458 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f725(v0: Option<Adt8>, v1: Str) -> bool {
    let m2459 = v0;
    match m2459 {
        None => {
            false
        }
        Some(v2) => {
            f477(v1, v2)
        }
    }
}

pub fn f726() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f727(v0: List<Str>, v1: Adt118, v2: Adt20, v3: Option<Adt123>) -> Adt151 {
    let m2460 = v3;
    match m2460 {
        None => {
            f783(v1, v2.clone(), f784(match v2 {
                Adt20::C0(h2461, _, _, _, _, _) => {
                    h2461
                }
            }, v0))
        }
        Some(v4) => {
            Adt151::C1(Adt139::C2(v4))
        }
    }
}

pub fn f728(v0: Option<Adt122>, v1: Adt153, v2: Str, v3: Str, v4: Adt8) -> R<Option<Adt123>> {
    let m2462 = v0;
    match m2462 {
        None => {
            Ok(None::<Adt123>)
        }
        Some(v5) => {
            let m2464 = match v5.clone() {
                Adt122::C0(_, _, _, h2463, _) => {
                    h2463
                }
            };
            match m2464 {
                None => {
                    f785(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f729(v0: Option<Adt8>) -> Adt8 {
    let m2465 = v0;
    match m2465 {
        None => {
            f32()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f730(v0: Adt8) -> Result<Adt20, Adt140> {
    let m2466 = f786(v0.clone());
    match m2466 {
        Adt160::C0 => {
            f787(v0.clone(), f225(f788(), v0.clone()), f225(f789(), v0))
        }
        Adt160::C1 => {
            Err::<Adt20, Adt140>(Adt140::C2)
        }
        Adt160::C2 => {
            Err::<Adt20, Adt140>(Adt140::C3)
        }
    }
}

pub fn f731() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f732(v0: u64, v1: u64) -> u64 {
    let a2467 = v0;
    let a2468 = v1;
    let a2469 = 0u64;
    nat_rem(a2467, a2468, a2469)
}

pub fn f733(v0: u64, v1: u64) -> u64 {
    let a2470 = v0;
    let a2471 = v1;
    let a2472 = 0u64;
    nat_quot(a2470, a2471, a2472)
}

pub fn f734(p2476: List<Adt9>) -> bool {
    let m2475 = p2476;
    {
        let mut m2475 = m2475;
        loop {
            let step = {
                let v0 = m2475;
                let m2473 = v0;
                match m2473.uncons() {
                    None => {
                        Err::<List<Adt9>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2474 = f275(v1);
                        match m2474 {
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
                Ok(next) => { m2475 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f735(p2480: List<Adt9>) -> bool {
    let m2479 = p2480;
    {
        let mut m2479 = m2479;
        loop {
            let step = {
                let v0 = m2479;
                let m2477 = v0;
                match m2477.uncons() {
                    None => {
                        Err::<List<Adt9>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2478 = f275(v1);
                        match m2478 {
                            None => {
                                Err::<List<Adt9>, bool>(false)
                            }
                            Some(v3) => {
                                if f580(v3) {
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
                Ok(next) => { m2479 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f736(v0: Str) -> Option<Adt16> {
    let a2481 = v0.clone();
    let a2482 = Str::lit("string");
    if equal(a2481, a2482) {
        Some(Adt16::C0)
    } else {
        let a2483 = v0.clone();
        let a2484 = Str::lit("integer");
        if equal(a2483, a2484) {
            Some(Adt16::C1)
        } else {
            let a2485 = v0;
            let a2486 = Str::lit("boolean");
            if equal(a2485, a2486) {
                Some(Adt16::C2)
            } else {
                None::<Adt16>
            }
        }
    }
}

pub fn f737(p2495: List<u64>, p2496: List<u64>) -> R<List<u64>> {
    let m2494 = (p2495, p2496);
    {
        let mut m2494 = m2494;
        loop {
            let step = {
                let (v0, v1) = m2494;
                let m2487 = v0;
                match m2487.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2488 = 65u64;
                        let a2489 = v2;
                        if if nat_le(a2488, a2489) {
                            let a2490 = v2;
                            let a2491 = 90u64;
                            nat_le(a2490, a2491)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2492 = v2;
                                let a2493 = 32u64;
                                nat_add(a2492, a2493)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f738(p2503: List<u64>, p2504: List<u64>) -> bool {
    let m2502 = (p2503, p2504);
    {
        let mut m2502 = m2502;
        loop {
            let step = {
                let (v0, v1) = m2502;
                let m2497 = v0;
                match m2497.uncons() {
                    None => {
                        let m2498 = v1;
                        match m2498.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2499 = v1;
                        match m2499.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2500 = v4;
                                let a2501 = v6;
                                if nat_eq(a2500, a2501) {
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
                Ok(next) => { m2502 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f739(p2516: u64, p2517: i64, p2518: i64) -> R<bool> {
    let m2515 = (p2516, (p2517, p2518));
    {
        let mut m2515 = m2515;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2515;
                let m2505 = v0;
                if m2505 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2505 - 1;
                    let m2508 = {
                        let a2506 = v2;
                        let a2507 = 0i64;
                        compare(a2506, a2507)
                    };
                    if match m2508 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f649({
                            let a2509 = v1;
                            let a2510 = 10i64;
                            let a2511 = 0i64;
                            int_rem(a2509, a2510, a2511)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2512 = v1;
                                let a2513 = 10i64;
                                let a2514 = 0i64;
                                int_quot(a2512, a2513, a2514)?
                            }, f345(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f740(v0: Adt9, v1: Adt9, v2: List<(Adt9, Adt9)>) -> R<Adt159> {
    let a2519 = f790(v0.clone());
    let a2520 = f790(v1.clone());
    if nat_eq(a2519, a2520) {
        let m2521 = v0;
        match m2521 {
            Adt9::C0 => {
                Ok(Adt159::C1(v2))
            }
            Adt9::C1(v3) => {
                let m2522 = f346(v1);
                match m2522 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v4) => {
                        let a2523 = v3;
                        let a2524 = v4;
                        if equal(a2523, a2524) {
                            Ok(Adt159::C1(v2))
                        } else {
                            Ok(Adt159::C0)
                        }
                    }
                }
            }
            Adt9::C2(v5) => {
                let m2525 = f585(v1);
                match m2525 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v6) => {
                        if f791(f587(v5, v6)?) {
                            Ok(Adt159::C1(v2))
                        } else {
                            Ok(Adt159::C0)
                        }
                    }
                }
            }
            Adt9::C3(v7) => {
                let m2526 = f275(v1);
                match m2526 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v8) => {
                        let a2527 = v7;
                        let a2528 = v8;
                        if equal(a2527, a2528) {
                            Ok(Adt159::C1(v2))
                        } else {
                            Ok(Adt159::C0)
                        }
                    }
                }
            }
            Adt9::C4(v9) => {
                let m2529 = f278(v1);
                match m2529 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v10) => {
                        Ok(f792(f793(v9, v10, List::<(Adt9, Adt9)>::nil()), v2))
                    }
                }
            }
            Adt9::C5(v11) => {
                let m2530 = f226(v1);
                match m2530 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v12) => {
                        let a2532 = f591(v11.clone(), 0u64)?;
                        let a2533 = f591(match v12.clone() {
                            Adt8::C0(h2531) => {
                                h2531
                            }
                        }, 0u64)?;
                        if nat_eq(a2532, a2533) {
                            Ok(f792(f794(v11, match v12 {
                                Adt8::C0(h2534) => {
                                    h2534
                                }
                            }, List::<(Adt9, Adt9)>::nil()), v2))
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

pub fn f741(v0: Adt155, v1: Adt155) -> Adt155 {
    let m2535 = v0;
    match m2535 {
        Adt155::C0 => {
            f795(v1)
        }
        Adt155::C1 => {
            Adt155::C1
        }
        Adt155::C2 => {
            v1
        }
    }
}

pub fn f742(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt155> {
    let m2536 = f676(f796(v0, v1)?, f796(v2, v3)?);
    match m2536 {
        Adt155::C0 => {
            Ok(Adt155::C0)
        }
        Adt155::C1 => {
            f797(v0, f798(v0)?, v2, f798(v2)?)
        }
        Adt155::C2 => {
            Ok(Adt155::C2)
        }
    }
}

pub fn f743(v0: i64) -> R<i64> {
    let m2539 = {
        let a2537 = v0;
        let a2538 = 0i64;
        compare(a2537, a2538)
    };
    if match m2539 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a2540 = v0;
        int_neg(a2540)
    } else {
        Ok(v0)
    }
}

pub fn f744(v0: u64, v1: u64) -> u64 {
    let a2541 = v0;
    let a2542 = v1;
    let a2543 = 0u64;
    nat_rem(a2541, a2542, a2543)
}

pub fn f745(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2546 = {
            let a2544 = v0;
            let a2545 = 4u64;
            nat_mul(a2544, a2545)?
        };
        let a2547 = f747(v1, 16u64);
        nat_add(a2546, a2547)?
    }, List::<u64>::nil()))
}

pub fn f746(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2550 = {
            let a2548 = v0;
            let a2549 = 4u64;
            nat_mul(a2548, a2549)?
        };
        let a2551 = f747(v1, 16u64);
        nat_add(a2550, a2551)?
    }, List::cons({
        let a2554 = {
            let a2552 = f744(v1, 16u64);
            let a2553 = 16u64;
            nat_mul(a2552, a2553)?
        };
        let a2555 = f747(v2, 4u64);
        nat_add(a2554, a2555)?
    }, List::<u64>::nil())))
}

pub fn f747(v0: u64, v1: u64) -> u64 {
    let a2556 = v0;
    let a2557 = v1;
    let a2558 = 0u64;
    nat_quot(a2556, a2557, a2558)
}

pub fn f748(p2570: u64, p2571: i64, p2572: List<u64>) -> R<List<u64>> {
    let m2569 = (p2570, (p2571, p2572));
    {
        let mut m2569 = m2569;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2569;
                let m2559 = v0;
                if m2559 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2559 - 1;
                    let v4: i64 = {
                        let a2560 = v1;
                        let a2561 = 10i64;
                        let a2562 = 0i64;
                        int_quot(a2560, a2561, a2562)?
                    };
                    let v5: i64 = {
                        let a2563 = v1;
                        let a2564 = 10i64;
                        let a2565 = 0i64;
                        int_rem(a2563, a2564, a2565)
                    };
                    let m2568 = {
                        let a2566 = v4;
                        let a2567 = 1i64;
                        compare(a2566, a2567)
                    };
                    if match m2568 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f799(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f799(v5), v2))))
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

pub fn f749(v0: u64) -> R<u64> {
    let a2573 = v0;
    let a2574 = 10u64;
    if nat_lt(a2573, a2574) {
        let a2575 = 48u64;
        let a2576 = v0;
        nat_add(a2575, a2576)
    } else {
        let a2577 = 87u64;
        let a2578 = v0;
        nat_add(a2577, a2578)
    }
}

pub fn f750(v0: Str, v1: Option<Adt11>) -> List<(Str, Adt9)> {
    let m2579 = v1;
    match m2579 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f800(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f751(v0: Option<Adt12>) -> List<(Str, Adt9)> {
    let m2580 = v0;
    match m2580 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt9::C5({
                let a2583 = f692(Str::lit("subscribe"), match v1.clone() {
                    Adt12::C0(h2581, _) => {
                        h2581
                    }
                });
                let a2584 = f692(Str::lit("listChanged"), match v1 {
                    Adt12::C0(_, h2582) => {
                        h2582
                    }
                });
                append_list(a2583, a2584)
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f752(v0: Adt14) -> Str {
    let m2585 = v0;
    match m2585 {
        Adt14::C0 => {
            Str::lit("public")
        }
        Adt14::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f753(p2588: List<Adt39>, p2589: List<Adt9>) -> List<Adt9> {
    let m2587 = (p2588, p2589);
    {
        let mut m2587 = m2587;
        loop {
            let step = {
                let (v0, v1) = m2587;
                let m2586 = v0;
                match m2586.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt39>, List<Adt9>), List<Adt9>>((v3, List::cons(f801(v2), v1)))
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

pub fn f754(p2593: List<Adt42>, p2594: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m2592 = (p2593, p2594);
    {
        let mut m2592 = m2592;
        loop {
            let step = {
                let (v0, v1) = m2592;
                let m2590 = v0;
                match m2590.uncons() {
                    None => {
                        Err::<(List<Adt42>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2591 = f802(v2)?;
                        match m2591 {
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
                Ok(next) => { m2592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f755(p2597: List<Adt48>, p2598: List<Adt9>) -> R<List<Adt9>> {
    let m2596 = (p2597, p2598);
    {
        let mut m2596 = m2596;
        loop {
            let step = {
                let (v0, v1) = m2596;
                let m2595 = v0;
                match m2595.uncons() {
                    None => {
                        Err::<(List<Adt48>, List<Adt9>), R<List<Adt9>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt48>, List<Adt9>), R<List<Adt9>>>((v3, List::cons(f803(v2)?, v1)))
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

pub fn f756(p2601: List<Adt50>, p2602: List<Adt9>) -> List<Adt9> {
    let m2600 = (p2601, p2602);
    {
        let mut m2600 = m2600;
        loop {
            let step = {
                let (v0, v1) = m2600;
                let m2599 = v0;
                match m2599.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt50>, List<Adt9>), List<Adt9>>((v3, List::cons(f804(v2), v1)))
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

pub fn f757(p2605: List<Adt58>, p2606: List<Adt9>) -> List<Adt9> {
    let m2604 = (p2605, p2606);
    {
        let mut m2604 = m2604;
        loop {
            let step = {
                let (v0, v1) = m2604;
                let m2603 = v0;
                match m2603.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt9>), List<Adt9>>((v3, List::cons(f805(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2604 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f758(p2609: List<Adt60>, p2610: List<Adt9>) -> List<Adt9> {
    let m2608 = (p2609, p2610);
    {
        let mut m2608 = m2608;
        loop {
            let step = {
                let (v0, v1) = m2608;
                let m2607 = v0;
                match m2607.uncons() {
                    None => {
                        Err::<(List<Adt60>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt60>, List<Adt9>), List<Adt9>>((v3, List::cons(f806(v2), v1)))
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

pub fn f759(p2614: List<Adt63>, p2615: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m2613 = (p2614, p2615);
    {
        let mut m2613 = m2613;
        loop {
            let step = {
                let (v0, v1) = m2613;
                let m2611 = v0;
                match m2611.uncons() {
                    None => {
                        Err::<(List<Adt63>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2612 = f807(v2)?;
                        match m2612 {
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
                Ok(next) => { m2613 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f760(v0: Adt68) -> R<Option<Adt9>> {
    let m2616 = v0;
    match m2616 {
        Adt68::C0(v1) => {
            Ok(Some(f808(Str::lit("elicitation/create"), Some(f809(v1)))))
        }
        Adt68::C1(v2) => {
            let m2617 = f810(v2)?;
            match m2617 {
                None => {
                    Ok(None::<Adt9>)
                }
                Some(v3) => {
                    Ok(Some(f808(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt68::C2(v4) => {
            Ok(Some(f808(Str::lit("roots/list"), {
                let m2618 = v4;
                match m2618 {
                    None => {
                        None::<Adt9>
                    }
                    Some(v5) => {
                        Some(Adt9::C5(match v5 {
                            Adt8::C0(h2619) => {
                                h2619
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f761(v0: Option<List<Adt5>>) -> List<(Str, Adt9)> {
    let m2620 = v0;
    match m2620 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt9::C4(f811(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f762() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f763() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f764() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f765() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f766() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f767(p2625: List<List<u64>>, p2626: List<u64>) -> List<u64> {
    let m2624 = (p2625, p2626);
    {
        let mut m2624 = m2624;
        loop {
            let step = {
                let (v0, v1) = m2624;
                let m2621 = v0;
                match m2621.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2622 = f812(v2);
                            let a2623 = v1;
                            append_list(a2622, a2623)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2624 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f768(v0: Str) -> R<Str> {
    let m2629 = {
        let a2628 = f348(f349(), f417(f813(f29({
            let a2627 = v0;
            utf8_encode(a2627)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2628)
    };
    match m2629 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f769(v0: List<u64>) -> bool {
    let m2630 = v0;
    match m2630.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2631 = v1;
            let a2632 = 48u64;
            if nat_eq(a2631, a2632) {
                f814(v2)
            } else {
                false
            }
        }
    }
}

pub fn f770(v0: Adt3, v1: Fn1, v2: u64, v3: Adt18) -> R<Result<Adt136, Adt139>> {
    if f815(f816(v3.clone())) {
        Ok(Ok::<Adt136, Adt139>(f817(v0.clone(), Adt36::C0(f818(v0)))))
    } else {
        Ok(f819(v0.clone(), v3.clone(), f820(match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h2633) => {
                h2633
            }
        }, match v0 {
            Adt3::C0(_, _, _, _, _, h2634, _) => {
                h2634
            }
        }, v2, v1, v3)?))
    }
}

pub fn f771(v0: Adt20, _: Adt8) -> Result<Adt18, Adt140> {
    Ok::<Adt18, Adt140>(Adt18::C0(Adt19::C0(v0)))
}

pub fn f772(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt140> {
    let m2635 = f821(Str::lit("cursor"), v1);
    match m2635 {
        Ok(v2) => {
            Ok::<Adt18, Adt140>(Adt18::C1(Adt24::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt140>(v3)
        }
    }
}

pub fn f773(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt140> {
    let m2636 = f822(Str::lit("name"), v1.clone());
    match m2636 {
        Ok(v2) => {
            let m2637 = f823(Str::lit("arguments"), v1.clone());
            match m2637 {
                Ok(v3) => {
                    let m2638 = f823(Str::lit("inputResponses"), v1.clone());
                    match m2638 {
                        Ok(v4) => {
                            let m2639 = f821(Str::lit("requestState"), v1);
                            match m2639 {
                                Ok(v5) => {
                                    Ok::<Adt18, Adt140>(Adt18::C2(Adt25::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt18, Adt140>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt18, Adt140>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt18, Adt140>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt18, Adt140>(v9)
        }
    }
}

pub fn f774(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt140> {
    let m2640 = f821(Str::lit("cursor"), v1);
    match m2640 {
        Ok(v2) => {
            Ok::<Adt18, Adt140>(Adt18::C3(Adt24::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt140>(v3)
        }
    }
}

pub fn f775(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt140> {
    let m2641 = f822(Str::lit("uri"), v1.clone());
    match m2641 {
        Ok(v2) => {
            let m2642 = f823(Str::lit("inputResponses"), v1.clone());
            match m2642 {
                Ok(v3) => {
                    let m2643 = f821(Str::lit("requestState"), v1);
                    match m2643 {
                        Ok(v4) => {
                            Ok::<Adt18, Adt140>(Adt18::C4(Adt26::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt18, Adt140>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt18, Adt140>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt18, Adt140>(v7)
        }
    }
}

pub fn f776(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt140> {
    let m2644 = f821(Str::lit("cursor"), v1);
    match m2644 {
        Ok(v2) => {
            Ok::<Adt18, Adt140>(Adt18::C5(Adt24::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt140>(v3)
        }
    }
}

pub fn f777(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt140> {
    let m2645 = f821(Str::lit("cursor"), v1);
    match m2645 {
        Ok(v2) => {
            Ok::<Adt18, Adt140>(Adt18::C6(Adt24::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt140>(v3)
        }
    }
}

pub fn f778(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt140> {
    let m2646 = f822(Str::lit("name"), v1.clone());
    match m2646 {
        Ok(v2) => {
            let m2647 = f824(Str::lit("arguments"), v1.clone());
            match m2647 {
                Ok(v3) => {
                    let m2648 = f823(Str::lit("inputResponses"), v1.clone());
                    match m2648 {
                        Ok(v4) => {
                            let m2649 = f821(Str::lit("requestState"), v1);
                            match m2649 {
                                Ok(v5) => {
                                    Ok::<Adt18, Adt140>(Adt18::C7(Adt27::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt18, Adt140>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt18, Adt140>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt18, Adt140>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt18, Adt140>(v9)
        }
    }
}

pub fn f779(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt140> {
    let m2650 = f825(Str::lit("ref"), v1.clone());
    match m2650 {
        Ok(v2) => {
            let m2651 = f826(v2);
            match m2651 {
                Ok(v3) => {
                    let m2652 = f825(Str::lit("argument"), v1.clone());
                    match m2652 {
                        Ok(v4) => {
                            let m2653 = f827(v4);
                            match m2653 {
                                Ok(v5) => {
                                    let m2654 = f828(v1);
                                    match m2654 {
                                        Ok(v6) => {
                                            Ok::<Adt18, Adt140>(Adt18::C8(Adt28::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt18, Adt140>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt18, Adt140>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt18, Adt140>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt18, Adt140>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt18, Adt140>(v11)
        }
    }
}

pub fn f780(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt140> {
    let m2655 = f822(Str::lit("taskId"), v1);
    match m2655 {
        Ok(v2) => {
            Ok::<Adt18, Adt140>(Adt18::C9(Adt33::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt140>(v3)
        }
    }
}

pub fn f781(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt140> {
    let m2656 = f822(Str::lit("taskId"), v1.clone());
    match m2656 {
        Ok(v2) => {
            let m2657 = f825(Str::lit("inputResponses"), v1);
            match m2657 {
                Ok(v3) => {
                    Ok::<Adt18, Adt140>(Adt18::C10(Adt34::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt18, Adt140>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt18, Adt140>(v5)
        }
    }
}

pub fn f782(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt140> {
    let m2658 = f822(Str::lit("taskId"), v1);
    match m2658 {
        Ok(v2) => {
            Ok::<Adt18, Adt140>(Adt18::C11(Adt35::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt140>(v3)
        }
    }
}

pub fn f783(v0: Adt118, v1: Adt20, v2: Option<Adt139>) -> Adt151 {
    let m2659 = v2;
    match m2659 {
        None => {
            Adt151::C0(Adt152::C0(match v0.clone() {
                Adt118::C0(h2660, _, _) => {
                    h2660
                }
            }, match v0.clone() {
                Adt118::C0(_, h2661, _) => {
                    h2661
                }
            }, v1, f729(match v0 {
                Adt118::C0(_, _, h2662) => {
                    h2662
                }
            })))
        }
        Some(v3) => {
            Adt151::C1(v3)
        }
    }
}

pub fn f784(v0: Str, v1: List<Str>) -> Option<Adt139> {
    f829(f830(v0.clone(), v1.clone()), v0, v1)
}

pub fn f785(v0: Adt122, v1: Adt153, v2: Str, v3: Str, v4: Adt8) -> R<Option<Adt123>> {
    let m2664 = match v0.clone() {
        Adt122::C0(h2663, _, _, _, _) => {
            h2663
        }
    };
    match m2664 {
        None => {
            Ok(Some(Adt123::C0))
        }
        Some(v5) => {
            let a2665 = v5;
            let a2666 = v3;
            if equal(a2665, a2666) {
                let m2668 = match v0.clone() {
                    Adt122::C0(_, h2667, _, _, _) => {
                        h2667
                    }
                };
                match m2668 {
                    None => {
                        Ok(Some(Adt123::C1))
                    }
                    Some(v6) => {
                        let a2669 = v6;
                        let a2670 = v2;
                        if equal(a2669, a2670) {
                            f831(f832(match v0.clone() {
                                Adt122::C0(_, _, h2671, _, _) => {
                                    h2671
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt122::C0(_, _, _, _, h2672) => {
                                    h2672
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt123::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt123::C3))
            }
        }
    }
}

pub fn f786(v0: Adt8) -> Adt160 {
    f833(f225(f788(), v0.clone()), f225(f789(), v0))
}

pub fn f787(v0: Adt8, v1: Adt110, v2: Adt110) -> Result<Adt20, Adt140> {
    let m2673 = v1;
    match m2673 {
        Adt110::C0 => {
            Err::<Adt20, Adt140>(Adt140::C2)
        }
        Adt110::C1(v3) => {
            let m2674 = v2;
            match m2674 {
                Adt110::C0 => {
                    Err::<Adt20, Adt140>(Adt140::C3)
                }
                Adt110::C1(v4) => {
                    f834(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f788() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f789() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f790(v0: Adt9) -> u64 {
    let m2675 = v0;
    match m2675 {
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

pub fn f791(v0: Adt155) -> bool {
    let m2676 = v0;
    match m2676 {
        Adt155::C0 => {
            false
        }
        Adt155::C1 => {
            true
        }
        Adt155::C2 => {
            false
        }
    }
}

pub fn f792(v0: Option<List<(Adt9, Adt9)>>, v1: List<(Adt9, Adt9)>) -> Adt159 {
    let m2677 = v0;
    match m2677 {
        None => {
            Adt159::C0
        }
        Some(v2) => {
            Adt159::C1({
                let a2678 = v2;
                let a2679 = v1;
                append_list(a2678, a2679)
            })
        }
    }
}

pub fn f793(p2684: List<Adt9>, p2685: List<Adt9>, p2686: List<(Adt9, Adt9)>) -> Option<List<(Adt9, Adt9)>> {
    let m2683 = (p2684, (p2685, p2686));
    {
        let mut m2683 = m2683;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2683;
                let m2680 = v0;
                match m2680.uncons() {
                    None => {
                        let m2681 = v1;
                        match m2681.uncons() {
                            None => {
                                Err::<(List<Adt9>, (List<Adt9>, List<(Adt9, Adt9)>)), Option<List<(Adt9, Adt9)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt9>, (List<Adt9>, List<(Adt9, Adt9)>)), Option<List<(Adt9, Adt9)>>>(None::<List<(Adt9, Adt9)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2682 = v1;
                        match m2682.uncons() {
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
                Ok(next) => { m2683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f794(p2692: List<(Str, Adt9)>, v1: List<(Str, Adt9)>, p2693: List<(Adt9, Adt9)>) -> Option<List<(Adt9, Adt9)>> {
    let m2691 = (p2692, p2693);
    {
        let mut m2691 = m2691;
        loop {
            let step = {
                let (v0, v2) = m2691;
                let m2687 = v0;
                match m2687.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Adt9, Adt9)>), Option<List<(Adt9, Adt9)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2689 = f297({
                            let (h2688, _) = v3;
                            h2688
                        }, v1.clone());
                        match m2689 {
                            Adt110::C0 => {
                                Err::<(List<(Str, Adt9)>, List<(Adt9, Adt9)>), Option<List<(Adt9, Adt9)>>>(None::<List<(Adt9, Adt9)>>)
                            }
                            Adt110::C1(v5) => {
                                Ok::<(List<(Str, Adt9)>, List<(Adt9, Adt9)>), Option<List<(Adt9, Adt9)>>>((v4, List::cons(({
                                    let (_, h2690) = v3;
                                    h2690
                                }, v5), v2)))
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

pub fn f795(v0: Adt155) -> Adt155 {
    let m2694 = v0;
    match m2694 {
        Adt155::C0 => {
            Adt155::C2
        }
        Adt155::C1 => {
            Adt155::C1
        }
        Adt155::C2 => {
            Adt155::C0
        }
    }
}

pub fn f796(v0: i64, v1: i64) -> R<i64> {
    f345(f277(f798(v0)?, 0i64)?, v1)
}

pub fn f797(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt155> {
    let a2695 = v1;
    let a2696 = v3;
    if nat_lt(a2695, a2696) {
        Ok(f676(f835({
            let a2697 = v3;
            let a2698 = v1;
            nat_sub(a2697, a2698)
        }, v0)?, v2))
    } else {
        let a2699 = v3;
        let a2700 = v1;
        if nat_lt(a2699, a2700) {
            Ok(f676(v0, f835({
                let a2701 = v1;
                let a2702 = v3;
                nat_sub(a2701, a2702)
            }, v2)?))
        } else {
            Ok(f676(v0, v2))
        }
    }
}

pub fn f798(v0: i64) -> R<u64> {
    f836(20u64, v0, 0u64)
}

pub fn f799(v0: i64) -> u64 {
    let m2705 = {
        let a2703 = v0;
        let a2704 = 1i64;
        compare(a2703, a2704)
    };
    if match m2705 {
        Ordering::Less => {
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
        let m2708 = {
            let a2706 = v0;
            let a2707 = 2i64;
            compare(a2706, a2707)
        };
        if match m2708 {
            Ordering::Less => {
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
            let m2711 = {
                let a2709 = v0;
                let a2710 = 3i64;
                compare(a2709, a2710)
            };
            if match m2711 {
                Ordering::Less => {
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
                let m2714 = {
                    let a2712 = v0;
                    let a2713 = 4i64;
                    compare(a2712, a2713)
                };
                if match m2714 {
                    Ordering::Less => {
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
                    let m2717 = {
                        let a2715 = v0;
                        let a2716 = 5i64;
                        compare(a2715, a2716)
                    };
                    if match m2717 {
                        Ordering::Less => {
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
                        let m2720 = {
                            let a2718 = v0;
                            let a2719 = 6i64;
                            compare(a2718, a2719)
                        };
                        if match m2720 {
                            Ordering::Less => {
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
                            let m2723 = {
                                let a2721 = v0;
                                let a2722 = 7i64;
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
                                54u64
                            } else {
                                let m2726 = {
                                    let a2724 = v0;
                                    let a2725 = 8i64;
                                    compare(a2724, a2725)
                                };
                                if match m2726 {
                                    Ordering::Less => {
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
                                    let m2729 = {
                                        let a2727 = v0;
                                        let a2728 = 9i64;
                                        compare(a2727, a2728)
                                    };
                                    if match m2729 {
                                        Ordering::Less => {
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

pub fn f800(v0: Adt11) -> Adt9 {
    Adt9::C5(f692(Str::lit("listChanged"), match v0 {
        Adt11::C0(h2730) => {
            h2730
        }
    }))
}

pub fn f801(v0: Adt39) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
        Adt39::C0(h2731, _, _, _, _, _, _, _) => {
            h2731
        }
    })), {
        let a2749 = f687(Str::lit("title"), match v0.clone() {
            Adt39::C0(_, h2732, _, _, _, _, _, _) => {
                h2732
            }
        });
        let a2750 = {
            let a2747 = f761(match v0.clone() {
                Adt39::C0(_, _, h2733, _, _, _, _, _) => {
                    h2733
                }
            });
            let a2748 = {
                let a2745 = f687(Str::lit("description"), match v0.clone() {
                    Adt39::C0(_, _, _, h2734, _, _, _, _) => {
                        h2734
                    }
                });
                let a2746 = List::cons((Str::lit("inputSchema"), Adt9::C5({
                    let m2737 = match v0.clone() {
                        Adt39::C0(_, _, _, _, h2736, _, _, _) => {
                            h2736
                        }
                    };
                    match m2737 {
                        Adt8::C0(h2735) => {
                            h2735
                        }
                    }
                })), {
                    let a2743 = f718(Str::lit("outputSchema"), match v0.clone() {
                        Adt39::C0(_, _, _, _, _, h2738, _, _) => {
                            h2738
                        }
                    });
                    let a2744 = {
                        let a2741 = f837(match v0.clone() {
                            Adt39::C0(_, _, _, _, _, _, h2739, _) => {
                                h2739
                            }
                        });
                        let a2742 = f838(match v0 {
                            Adt39::C0(_, _, _, _, _, _, _, h2740) => {
                                h2740
                            }
                        });
                        append_list(a2741, a2742)
                    };
                    append_list(a2743, a2744)
                });
                append_list(a2745, a2746)
            };
            append_list(a2747, a2748)
        };
        append_list(a2749, a2750)
    }))
}

pub fn f802(v0: Adt42) -> R<Option<Adt9>> {
    let m2751 = v0;
    match m2751 {
        Adt42::C0(v1) => {
            Ok(Some(f839(v1)))
        }
        Adt42::C1(v2) => {
            Ok(Some(f840(Str::lit("image"), match v2.clone() {
                Adt46::C0(h2752, _, _, _) => {
                    h2752
                }
            }, match v2.clone() {
                Adt46::C0(_, h2753, _, _) => {
                    h2753
                }
            }, match v2.clone() {
                Adt46::C0(_, _, h2754, _) => {
                    h2754
                }
            }, match v2 {
                Adt46::C0(_, _, _, h2755) => {
                    h2755
                }
            })))
        }
        Adt42::C2(v3) => {
            Ok(Some(f840(Str::lit("audio"), match v3.clone() {
                Adt47::C0(h2756, _, _, _) => {
                    h2756
                }
            }, match v3.clone() {
                Adt47::C0(_, h2757, _, _) => {
                    h2757
                }
            }, match v3.clone() {
                Adt47::C0(_, _, h2758, _) => {
                    h2758
                }
            }, match v3 {
                Adt47::C0(_, _, _, h2759) => {
                    h2759
                }
            })))
        }
        Adt42::C3(v4) => {
            Ok(Some(f841(v4)?))
        }
        Adt42::C4(v5) => {
            Ok(Some(f842(v5)))
        }
    }
}

pub fn f803(v0: Adt48) -> R<Adt9> {
    Ok(Adt9::C5({
        let a2783 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt48::C0(h2760, _, _, _, _, _, _, _, _) => {
                h2760
            }
        })), List::<(Str, Adt9)>::nil());
        let a2784 = {
            let a2781 = f687(Str::lit("title"), match v0.clone() {
                Adt48::C0(_, h2761, _, _, _, _, _, _, _) => {
                    h2761
                }
            });
            let a2782 = {
                let a2779 = f761(match v0.clone() {
                    Adt48::C0(_, _, h2762, _, _, _, _, _, _) => {
                        h2762
                    }
                });
                let a2780 = {
                    let a2777 = List::cons((Str::lit("uri"), Adt9::C3(match v0.clone() {
                        Adt48::C0(_, _, _, h2763, _, _, _, _, _) => {
                            h2763
                        }
                    })), List::<(Str, Adt9)>::nil());
                    let a2778 = {
                        let a2775 = f687(Str::lit("description"), match v0.clone() {
                            Adt48::C0(_, _, _, _, h2764, _, _, _, _) => {
                                h2764
                            }
                        });
                        let a2776 = {
                            let a2773 = f687(Str::lit("mimeType"), match v0.clone() {
                                Adt48::C0(_, _, _, _, _, h2765, _, _, _) => {
                                    h2765
                                }
                            });
                            let a2774 = {
                                let a2771 = f843(match v0.clone() {
                                    Adt48::C0(_, _, _, _, _, _, h2766, _, _) => {
                                        h2766
                                    }
                                });
                                let a2772 = {
                                    let a2769 = f844(match v0.clone() {
                                        Adt48::C0(_, _, _, _, _, _, _, h2767, _) => {
                                            h2767
                                        }
                                    })?;
                                    let a2770 = f838(match v0 {
                                        Adt48::C0(_, _, _, _, _, _, _, _, h2768) => {
                                            h2768
                                        }
                                    });
                                    append_list(a2769, a2770)
                                };
                                append_list(a2771, a2772)
                            };
                            append_list(a2773, a2774)
                        };
                        append_list(a2775, a2776)
                    };
                    append_list(a2777, a2778)
                };
                append_list(a2779, a2780)
            };
            append_list(a2781, a2782)
        };
        append_list(a2783, a2784)
    }))
}

pub fn f804(v0: Adt50) -> Adt9 {
    let m2785 = v0;
    match m2785 {
        Adt50::C0(v1) => {
            Adt9::C5(List::cons((Str::lit("uri"), Adt9::C3(match v1.clone() {
                Adt51::C0(h2786, _, _, _) => {
                    h2786
                }
            })), {
                let a2790 = f687(Str::lit("mimeType"), match v1.clone() {
                    Adt51::C0(_, h2787, _, _) => {
                        h2787
                    }
                });
                let a2791 = List::cons((Str::lit("text"), Adt9::C3(match v1.clone() {
                    Adt51::C0(_, _, h2788, _) => {
                        h2788
                    }
                })), f838(match v1 {
                    Adt51::C0(_, _, _, h2789) => {
                        h2789
                    }
                }));
                append_list(a2790, a2791)
            }))
        }
        Adt50::C1(v2) => {
            Adt9::C5(List::cons((Str::lit("uri"), Adt9::C3(match v2.clone() {
                Adt52::C0(h2792, _, _, _) => {
                    h2792
                }
            })), {
                let a2796 = f687(Str::lit("mimeType"), match v2.clone() {
                    Adt52::C0(_, h2793, _, _) => {
                        h2793
                    }
                });
                let a2797 = List::cons((Str::lit("blob"), Adt9::C3(match v2.clone() {
                    Adt52::C0(_, _, h2794, _) => {
                        h2794
                    }
                })), f838(match v2 {
                    Adt52::C0(_, _, _, h2795) => {
                        h2795
                    }
                }));
                append_list(a2796, a2797)
            }))
        }
    }
}

pub fn f805(v0: Adt58) -> Adt9 {
    Adt9::C5({
        let a2818 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt58::C0(h2798, _, _, _, _, _, _, _) => {
                h2798
            }
        })), List::<(Str, Adt9)>::nil());
        let a2819 = {
            let a2816 = f687(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h2799, _, _, _, _, _, _) => {
                    h2799
                }
            });
            let a2817 = {
                let a2814 = f761(match v0.clone() {
                    Adt58::C0(_, _, h2800, _, _, _, _, _) => {
                        h2800
                    }
                });
                let a2815 = {
                    let a2812 = List::cons((Str::lit("uriTemplate"), Adt9::C3(match v0.clone() {
                        Adt58::C0(_, _, _, h2801, _, _, _, _) => {
                            h2801
                        }
                    })), List::<(Str, Adt9)>::nil());
                    let a2813 = {
                        let a2810 = f687(Str::lit("description"), match v0.clone() {
                            Adt58::C0(_, _, _, _, h2802, _, _, _) => {
                                h2802
                            }
                        });
                        let a2811 = {
                            let a2808 = f687(Str::lit("mimeType"), match v0.clone() {
                                Adt58::C0(_, _, _, _, _, h2803, _, _) => {
                                    h2803
                                }
                            });
                            let a2809 = {
                                let a2806 = f843(match v0.clone() {
                                    Adt58::C0(_, _, _, _, _, _, h2804, _) => {
                                        h2804
                                    }
                                });
                                let a2807 = f838(match v0 {
                                    Adt58::C0(_, _, _, _, _, _, _, h2805) => {
                                        h2805
                                    }
                                });
                                append_list(a2806, a2807)
                            };
                            append_list(a2808, a2809)
                        };
                        append_list(a2810, a2811)
                    };
                    append_list(a2812, a2813)
                };
                append_list(a2814, a2815)
            };
            append_list(a2816, a2817)
        };
        append_list(a2818, a2819)
    })
}

pub fn f806(v0: Adt60) -> Adt9 {
    Adt9::C5({
        let a2835 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt60::C0(h2820, _, _, _, _, _) => {
                h2820
            }
        })), List::<(Str, Adt9)>::nil());
        let a2836 = {
            let a2833 = f687(Str::lit("title"), match v0.clone() {
                Adt60::C0(_, h2821, _, _, _, _) => {
                    h2821
                }
            });
            let a2834 = {
                let a2831 = f761(match v0.clone() {
                    Adt60::C0(_, _, h2822, _, _, _) => {
                        h2822
                    }
                });
                let a2832 = {
                    let a2829 = f687(Str::lit("description"), match v0.clone() {
                        Adt60::C0(_, _, _, h2823, _, _) => {
                            h2823
                        }
                    });
                    let a2830 = {
                        let a2827 = {
                            let m2825 = match v0.clone() {
                                Adt60::C0(_, _, _, _, h2824, _) => {
                                    h2824
                                }
                            };
                            match m2825 {
                                None => {
                                    List::<(Str, Adt9)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt9::C4(f845(v1))), List::<(Str, Adt9)>::nil())
                                }
                            }
                        };
                        let a2828 = f838(match v0 {
                            Adt60::C0(_, _, _, _, _, h2826) => {
                                h2826
                            }
                        });
                        append_list(a2827, a2828)
                    };
                    append_list(a2829, a2830)
                };
                append_list(a2831, a2832)
            };
            append_list(a2833, a2834)
        };
        append_list(a2835, a2836)
    })
}

pub fn f807(v0: Adt63) -> R<Option<Adt9>> {
    let m2838 = f802(match v0.clone() {
        Adt63::C0(_, h2837) => {
            h2837
        }
    })?;
    match m2838 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("role"), Adt9::C3(f846(match v0 {
                Adt63::C0(h2839, _) => {
                    h2839
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt9)>::nil())))))
        }
    }
}

pub fn f808(v0: Str, v1: Option<Adt9>) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("method"), Adt9::C3(v0)), f691(Str::lit("params"), v1)))
}

pub fn f809(v0: Adt69) -> Adt9 {
    let m2840 = v0;
    match m2840 {
        Adt69::C0(v1) => {
            f847(v1)
        }
        Adt69::C1(v2) => {
            Adt9::C5(List::cons((Str::lit("mode"), Adt9::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt9::C3(match v2.clone() {
                Adt72::C0(h2841, _) => {
                    h2841
                }
            })), List::cons((Str::lit("url"), Adt9::C3(match v2 {
                Adt72::C0(_, h2842) => {
                    h2842
                }
            })), List::<(Str, Adt9)>::nil()))))
        }
    }
}

pub fn f810(v0: Adt73) -> R<Option<Adt9>> {
    let m2844 = f848(match v0.clone() {
        Adt73::C0(h2843, _, _, _, _, _, _, _, _, _) => {
            h2843
        }
    })?;
    match m2844 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("messages"), Adt9::C4(v1)), {
                let a2846 = f849(v0.clone());
                let a2847 = List::cons((Str::lit("maxTokens"), f112(match v0.clone() {
                    Adt73::C0(_, _, _, _, _, h2845, _, _, _, _) => {
                        h2845
                    }
                })?), f850(v0));
                append_list(a2846, a2847)
            }))))
        }
    }
}

pub fn f811(v0: List<Adt5>) -> List<Adt9> {
    f553(f851(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f812(v0: List<u64>) -> List<u64> {
    let a2850 = f852();
    let a2851 = {
        let a2848 = v0;
        let a2849 = f853();
        append_list(a2848, a2849)
    };
    append_list(a2850, a2851)
}

pub fn f813(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f854(v0, 0u64, v1)
}

pub fn f814(p2858: List<u64>) -> bool {
    let m2857 = p2858;
    {
        let mut m2857 = m2857;
        loop {
            let step = {
                let v0 = m2857;
                let m2852 = v0;
                match m2852.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2853 = v1;
                        let a2854 = 48u64;
                        if if nat_eq(a2853, a2854) {
                            true
                        } else {
                            let a2855 = v1;
                            let a2856 = 46u64;
                            nat_eq(a2855, a2856)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f815(v0: Adt148) -> bool {
    let m2859 = v0;
    match m2859 {
        Adt148::C0 => {
            true
        }
        Adt148::C1 => {
            false
        }
        Adt148::C2 => {
            false
        }
        Adt148::C3 => {
            false
        }
        Adt148::C4 => {
            false
        }
        Adt148::C5 => {
            false
        }
        Adt148::C6 => {
            false
        }
        Adt148::C7 => {
            false
        }
        Adt148::C8 => {
            false
        }
        Adt148::C9 => {
            false
        }
        Adt148::C10 => {
            false
        }
        Adt148::C11 => {
            false
        }
    }
}

pub fn f816(v0: Adt18) -> Adt148 {
    let m2860 = v0;
    match m2860 {
        Adt18::C0(_) => {
            Adt148::C0
        }
        Adt18::C1(_) => {
            Adt148::C1
        }
        Adt18::C2(_) => {
            Adt148::C2
        }
        Adt18::C3(_) => {
            Adt148::C3
        }
        Adt18::C4(_) => {
            Adt148::C4
        }
        Adt18::C5(_) => {
            Adt148::C5
        }
        Adt18::C6(_) => {
            Adt148::C6
        }
        Adt18::C7(_) => {
            Adt148::C7
        }
        Adt18::C8(_) => {
            Adt148::C8
        }
        Adt18::C9(_) => {
            Adt148::C9
        }
        Adt18::C10(_) => {
            Adt148::C10
        }
        Adt18::C11(_) => {
            Adt148::C11
        }
    }
}

pub fn f817(v0: Adt3, v1: Adt36) -> Adt136 {
    Adt136::C0(v1, Adt137::C0(Some(match v0 {
        Adt3::C0(h2861, _, _, _, _, _, _) => {
            h2861
        }
    }), None::<Adt8>), List::<Adt138>::nil())
}

pub fn f818(v0: Adt3) -> Adt37 {
    Adt37::C0(match v0.clone() {
        Adt3::C0(_, _, _, h2862, _, _, _) => {
            h2862
        }
    }, f428(), match v0.clone() {
        Adt3::C0(_, _, h2863, _, _, _, _) => {
            h2863
        }
    }, match v0 {
        Adt3::C0(_, h2864, _, _, _, _, _) => {
            h2864
        }
    })
}

pub fn f819(v0: Adt3, v1: Adt18, v2: Result<Adt36, Adt88>) -> Result<Adt136, Adt139> {
    let m2865 = v2;
    match m2865 {
        Ok(v4) => {
            if f855(f816(v1.clone()), v4.clone()) {
                f856(v0, v1, v4)
            } else {
                Err::<Adt136, Adt139>(Adt139::C4(f857()))
            }
        }
        Err(v3) => {
            Err::<Adt136, Adt139>(Adt139::C4(v3))
        }
    }
}

pub fn f820(v0: Bytes, v1: u64, v2: u64, v3: Fn1, v4: Adt18) -> R<Result<Adt36, Adt88>> {
    f858(v0.clone(), v1, v2, v3, v4.clone(), f859(v0, v4)?)
}

pub fn f821(v0: Str, v1: Adt8) -> Result<Option<Str>, Adt140> {
    let m2866 = f225(v0.clone(), v1);
    match m2866 {
        Adt110::C0 => {
            Ok::<Option<Str>, Adt140>(None::<Str>)
        }
        Adt110::C1(v2) => {
            let m2867 = f275(v2);
            match m2867 {
                None => {
                    Err::<Option<Str>, Adt140>(Adt140::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt140>(Some(v3))
                }
            }
        }
    }
}

pub fn f822(v0: Str, v1: Adt8) -> Result<Str, Adt140> {
    let m2868 = f225(v0.clone(), v1);
    match m2868 {
        Adt110::C0 => {
            Err::<Str, Adt140>(Adt140::C7(v0))
        }
        Adt110::C1(v2) => {
            let m2869 = f275(v2);
            match m2869 {
                None => {
                    Err::<Str, Adt140>(Adt140::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt140>(v3)
                }
            }
        }
    }
}

pub fn f823(v0: Str, v1: Adt8) -> Result<Option<Adt8>, Adt140> {
    let m2870 = f225(v0.clone(), v1);
    match m2870 {
        Adt110::C0 => {
            Ok::<Option<Adt8>, Adt140>(None::<Adt8>)
        }
        Adt110::C1(v2) => {
            let m2871 = f226(v2);
            match m2871 {
                None => {
                    Err::<Option<Adt8>, Adt140>(Adt140::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt8>, Adt140>(Some(v3))
                }
            }
        }
    }
}

pub fn f824(v0: Str, v1: Adt8) -> Result<Option<List<(Str, Str)>>, Adt140> {
    let m2872 = f225(v0.clone(), v1);
    match m2872 {
        Adt110::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt140>(None::<List<(Str, Str)>>)
        }
        Adt110::C1(v2) => {
            let m2873 = f226(v2);
            match m2873 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt140>(Adt140::C8(v0))
                }
                Some(v3) => {
                    let m2875 = f860(v0, match v3 {
                        Adt8::C0(h2874) => {
                            h2874
                        }
                    });
                    match m2875 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt140>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt140>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f825(v0: Str, v1: Adt8) -> Result<Adt8, Adt140> {
    let m2876 = f225(v0.clone(), v1);
    match m2876 {
        Adt110::C0 => {
            Err::<Adt8, Adt140>(Adt140::C7(v0))
        }
        Adt110::C1(v2) => {
            let m2877 = f226(v2);
            match m2877 {
                None => {
                    Err::<Adt8, Adt140>(Adt140::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt8, Adt140>(v3)
                }
            }
        }
    }
}

pub fn f826(v0: Adt8) -> Result<Adt29, Adt140> {
    let m2878 = f822(Str::lit("type"), v0.clone());
    match m2878 {
        Ok(v1) => {
            let a2879 = v1.clone();
            let a2880 = Str::lit("ref/prompt");
            if equal(a2879, a2880) {
                let m2881 = f822(Str::lit("name"), v0.clone());
                match m2881 {
                    Ok(v2) => {
                        let m2882 = f821(Str::lit("title"), v0);
                        match m2882 {
                            Ok(v3) => {
                                Ok::<Adt29, Adt140>(Adt29::C0(Adt30::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt29, Adt140>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt29, Adt140>(v5)
                    }
                }
            } else {
                let a2883 = v1;
                let a2884 = Str::lit("ref/resource");
                if equal(a2883, a2884) {
                    let m2885 = f822(Str::lit("uri"), v0);
                    match m2885 {
                        Ok(v6) => {
                            Ok::<Adt29, Adt140>(Adt29::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt29, Adt140>(v7)
                        }
                    }
                } else {
                    Err::<Adt29, Adt140>(Adt140::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt29, Adt140>(v8)
        }
    }
}

pub fn f827(v0: Adt8) -> Result<Adt31, Adt140> {
    let m2886 = f822(Str::lit("name"), v0.clone());
    match m2886 {
        Ok(v1) => {
            let m2887 = f822(Str::lit("value"), v0);
            match m2887 {
                Ok(v2) => {
                    Ok::<Adt31, Adt140>(Adt31::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt31, Adt140>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt31, Adt140>(v4)
        }
    }
}

pub fn f828(v0: Adt8) -> Result<Option<Adt32>, Adt140> {
    let m2888 = f225(Str::lit("context"), v0);
    match m2888 {
        Adt110::C0 => {
            Ok::<Option<Adt32>, Adt140>(None::<Adt32>)
        }
        Adt110::C1(v1) => {
            let m2889 = f226(v1);
            match m2889 {
                None => {
                    Err::<Option<Adt32>, Adt140>(Adt140::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2890 = f824(Str::lit("arguments"), v2);
                    match m2890 {
                        Ok(v3) => {
                            Ok::<Option<Adt32>, Adt140>(Some(Adt32::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt32>, Adt140>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f829(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt139> {
    if v0 {
        None::<Adt139>
    } else {
        Some(Adt139::C1(v1, v2))
    }
}

pub fn f830(v0: Str, p2895: List<Str>) -> bool {
    let m2894 = p2895;
    {
        let mut m2894 = m2894;
        loop {
            let step = {
                let v1 = m2894;
                let m2891 = v1;
                match m2891.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2892 = v2;
                        let a2893 = v0.clone();
                        if equal(a2892, a2893) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2894 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f831(v0: Option<Adt123>, v1: List<Adt125>, v2: Adt8) -> R<Option<Adt123>> {
    let m2896 = v0;
    match m2896 {
        None => {
            f861(v1, f862(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f832(v0: Option<Str>, v1: Adt153, v2: Adt8) -> Option<Adt123> {
    let m2897 = v1.clone();
    match m2897 {
        Adt153::C0 => {
            None::<Adt123>
        }
        Adt153::C1 => {
            f863(v0, f864(v1, v2))
        }
        Adt153::C2 => {
            f863(v0, f864(v1, v2))
        }
        Adt153::C3 => {
            f863(v0, f864(v1, v2))
        }
    }
}

pub fn f833(v0: Adt110, v1: Adt110) -> Adt160 {
    let m2898 = v0;
    match m2898 {
        Adt110::C0 => {
            Adt160::C1
        }
        Adt110::C1(_) => {
            let m2899 = v1;
            match m2899 {
                Adt110::C0 => {
                    Adt160::C2
                }
                Adt110::C1(_) => {
                    Adt160::C0
                }
            }
        }
    }
}

pub fn f834(v0: Adt8, v1: Adt9, v2: Adt9) -> Result<Adt20, Adt140> {
    let m2900 = f275(v1);
    match m2900 {
        None => {
            Err::<Adt20, Adt140>(Adt140::C5)
        }
        Some(v3) => {
            let m2901 = f865(v2);
            match m2901 {
                None => {
                    Err::<Adt20, Adt140>(Adt140::C6)
                }
                Some(v4) => {
                    let m2902 = f823(f866(), v0.clone());
                    match m2902 {
                        Ok(v6) => {
                            let m2903 = f867(v0.clone());
                            match m2903 {
                                Ok(v8) => {
                                    let m2904 = f868(v0.clone());
                                    match m2904 {
                                        Ok(v10) => {
                                            Ok::<Adt20, Adt140>(Adt20::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt20, Adt140>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt20, Adt140>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt20, Adt140>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f835(p2909: u64, p2910: i64) -> R<i64> {
    let m2908 = (p2909, p2910);
    {
        let mut m2908 = m2908;
        loop {
            let step = {
                let (v0, v1) = m2908;
                let m2905 = v0;
                if m2905 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2905 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2906 = v1;
                        let a2907 = 10i64;
                        int_mul(a2906, a2907)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2908 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f836(p2921: u64, p2922: i64, p2923: u64) -> R<u64> {
    let m2920 = (p2921, (p2922, p2923));
    {
        let mut m2920 = m2920;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2920;
                let m2911 = v0;
                if m2911 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2911 - 1;
                    let v4: u64 = {
                        let a2912 = v2;
                        let a2913 = 1u64;
                        nat_add(a2912, a2913)?
                    };
                    let m2916 = {
                        let a2914 = v1;
                        let a2915 = 10i64;
                        compare(a2914, a2915)
                    };
                    if match m2916 {
                        Ordering::Less => {
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
                            let a2917 = v1;
                            let a2918 = 10i64;
                            let a2919 = 0i64;
                            int_quot(a2917, a2918, a2919)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2920 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f837(v0: Option<Adt40>) -> List<(Str, Adt9)> {
    let m2924 = v0;
    match m2924 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt9::C5({
                let a2936 = f687(Str::lit("title"), match v1.clone() {
                    Adt40::C0(h2925, _, _, _, _) => {
                        h2925
                    }
                });
                let a2937 = {
                    let a2934 = f692(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt40::C0(_, h2926, _, _, _) => {
                            h2926
                        }
                    });
                    let a2935 = {
                        let a2932 = f692(Str::lit("destructiveHint"), match v1.clone() {
                            Adt40::C0(_, _, h2927, _, _) => {
                                h2927
                            }
                        });
                        let a2933 = {
                            let a2930 = f692(Str::lit("idempotentHint"), match v1.clone() {
                                Adt40::C0(_, _, _, h2928, _) => {
                                    h2928
                                }
                            });
                            let a2931 = f692(Str::lit("openWorldHint"), match v1 {
                                Adt40::C0(_, _, _, _, h2929) => {
                                    h2929
                                }
                            });
                            append_list(a2930, a2931)
                        };
                        append_list(a2932, a2933)
                    };
                    append_list(a2934, a2935)
                };
                append_list(a2936, a2937)
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f838(v0: Option<Adt8>) -> List<(Str, Adt9)> {
    f718(Str::lit("_meta"), v0)
}

pub fn f839(v0: Adt43) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt9::C3(match v0.clone() {
        Adt43::C0(h2938, _, _) => {
            h2938
        }
    })), {
        let a2941 = f843(match v0.clone() {
            Adt43::C0(_, h2939, _) => {
                h2939
            }
        });
        let a2942 = f838(match v0 {
            Adt43::C0(_, _, h2940) => {
                h2940
            }
        });
        append_list(a2941, a2942)
    })))
}

pub fn f840(v0: Str, v1: Str, v2: Str, v3: Option<Adt44>, v4: Option<Adt8>) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v0)), List::cons((Str::lit("data"), Adt9::C3(v1)), List::cons((Str::lit("mimeType"), Adt9::C3(v2)), {
        let a2943 = f843(v3);
        let a2944 = f838(v4);
        append_list(a2943, a2944)
    }))))
}

pub fn f841(v0: Adt48) -> R<Adt9> {
    Ok(Adt9::C5({
        let a2968 = List::cons((Str::lit("type"), Adt9::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt48::C0(h2945, _, _, _, _, _, _, _, _) => {
                h2945
            }
        })), List::<(Str, Adt9)>::nil()));
        let a2969 = {
            let a2966 = f687(Str::lit("title"), match v0.clone() {
                Adt48::C0(_, h2946, _, _, _, _, _, _, _) => {
                    h2946
                }
            });
            let a2967 = {
                let a2964 = f761(match v0.clone() {
                    Adt48::C0(_, _, h2947, _, _, _, _, _, _) => {
                        h2947
                    }
                });
                let a2965 = {
                    let a2962 = List::cons((Str::lit("uri"), Adt9::C3(match v0.clone() {
                        Adt48::C0(_, _, _, h2948, _, _, _, _, _) => {
                            h2948
                        }
                    })), List::<(Str, Adt9)>::nil());
                    let a2963 = {
                        let a2960 = f687(Str::lit("description"), match v0.clone() {
                            Adt48::C0(_, _, _, _, h2949, _, _, _, _) => {
                                h2949
                            }
                        });
                        let a2961 = {
                            let a2958 = f687(Str::lit("mimeType"), match v0.clone() {
                                Adt48::C0(_, _, _, _, _, h2950, _, _, _) => {
                                    h2950
                                }
                            });
                            let a2959 = {
                                let a2956 = f843(match v0.clone() {
                                    Adt48::C0(_, _, _, _, _, _, h2951, _, _) => {
                                        h2951
                                    }
                                });
                                let a2957 = {
                                    let a2954 = f844(match v0.clone() {
                                        Adt48::C0(_, _, _, _, _, _, _, h2952, _) => {
                                            h2952
                                        }
                                    })?;
                                    let a2955 = f838(match v0 {
                                        Adt48::C0(_, _, _, _, _, _, _, _, h2953) => {
                                            h2953
                                        }
                                    });
                                    append_list(a2954, a2955)
                                };
                                append_list(a2956, a2957)
                            };
                            append_list(a2958, a2959)
                        };
                        append_list(a2960, a2961)
                    };
                    append_list(a2962, a2963)
                };
                append_list(a2964, a2965)
            };
            append_list(a2966, a2967)
        };
        append_list(a2968, a2969)
    }))
}

pub fn f842(v0: Adt49) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f804(match v0.clone() {
        Adt49::C0(h2970, _, _) => {
            h2970
        }
    })), {
        let a2973 = f843(match v0.clone() {
            Adt49::C0(_, h2971, _) => {
                h2971
            }
        });
        let a2974 = f838(match v0 {
            Adt49::C0(_, _, h2972) => {
                h2972
            }
        });
        append_list(a2973, a2974)
    })))
}

pub fn f843(v0: Option<Adt44>) -> List<(Str, Adt9)> {
    let m2975 = v0;
    match m2975 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt9::C5({
                let a2981 = f869(match v1.clone() {
                    Adt44::C0(h2976, _, _) => {
                        h2976
                    }
                });
                let a2982 = {
                    let a2979 = f870(match v1.clone() {
                        Adt44::C0(_, h2977, _) => {
                            h2977
                        }
                    });
                    let a2980 = f687(Str::lit("lastModified"), match v1 {
                        Adt44::C0(_, _, h2978) => {
                            h2978
                        }
                    });
                    append_list(a2979, a2980)
                };
                append_list(a2981, a2982)
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f844(v0: Option<u64>) -> R<List<(Str, Adt9)>> {
    let m2983 = v0;
    match m2983 {
        None => {
            Ok(List::<(Str, Adt9)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f112(v1)?), List::<(Str, Adt9)>::nil()))
        }
    }
}

pub fn f845(v0: List<Adt61>) -> List<Adt9> {
    f553(f871(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f846(v0: Adt45) -> Str {
    let m2984 = v0;
    match m2984 {
        Adt45::C0 => {
            Str::lit("user")
        }
        Adt45::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f847(v0: Adt70) -> Adt9 {
    Adt9::C5({
        let a2991 = {
            let m2986 = match v0.clone() {
                Adt70::C0(h2985, _, _) => {
                    h2985
                }
            };
            match m2986 {
                None => {
                    List::<(Str, Adt9)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt9::C3(Str::lit("form"))), List::<(Str, Adt9)>::nil())
                }
            }
        };
        let a2992 = List::cons((Str::lit("message"), Adt9::C3(match v0.clone() {
            Adt70::C0(_, h2987, _) => {
                h2987
            }
        })), List::cons((Str::lit("requestedSchema"), Adt9::C5({
            let m2990 = match v0 {
                Adt70::C0(_, _, h2989) => {
                    h2989
                }
            };
            match m2990 {
                Adt8::C0(h2988) => {
                    h2988
                }
            }
        })), List::<(Str, Adt9)>::nil()));
        append_list(a2991, a2992)
    })
}

pub fn f848(v0: List<Adt74>) -> R<Option<List<Adt9>>> {
    let m2993 = f872(v0, List::<Adt9>::nil())?;
    match m2993 {
        None => {
            Ok(None::<List<Adt9>>)
        }
        Some(v1) => {
            Ok(Some(f553(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f849(v0: Adt73) -> List<(Str, Adt9)> {
    let a3004 = {
        let m2995 = match v0.clone() {
            Adt73::C0(_, h2994, _, _, _, _, _, _, _, _) => {
                h2994
            }
        };
        match m2995 {
            None => {
                List::<(Str, Adt9)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f873(v1)), List::<(Str, Adt9)>::nil())
            }
        }
    };
    let a3005 = {
        let a3002 = f687(Str::lit("systemPrompt"), match v0.clone() {
            Adt73::C0(_, _, h2996, _, _, _, _, _, _, _) => {
                h2996
            }
        });
        let a3003 = {
            let a3000 = {
                let m2998 = match v0.clone() {
                    Adt73::C0(_, _, _, h2997, _, _, _, _, _, _) => {
                        h2997
                    }
                };
                match m2998 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt9::C3(f874(v2))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            let a3001 = f875(Str::lit("temperature"), match v0 {
                Adt73::C0(_, _, _, _, h2999, _, _, _, _, _) => {
                    h2999
                }
            });
            append_list(a3000, a3001)
        };
        append_list(a3002, a3003)
    };
    append_list(a3004, a3005)
}

pub fn f850(v0: Adt73) -> List<(Str, Adt9)> {
    let a3017 = {
        let m3007 = match v0.clone() {
            Adt73::C0(_, _, _, _, _, _, h3006, _, _, _) => {
                h3006
            }
        };
        match m3007 {
            None => {
                List::<(Str, Adt9)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f685(v1)), List::<(Str, Adt9)>::nil())
            }
        }
    };
    let a3018 = {
        let a3015 = f718(Str::lit("metadata"), match v0.clone() {
            Adt73::C0(_, _, _, _, _, _, _, h3008, _, _) => {
                h3008
            }
        });
        let a3016 = {
            let a3013 = {
                let m3010 = match v0.clone() {
                    Adt73::C0(_, _, _, _, _, _, _, _, h3009, _) => {
                        h3009
                    }
                };
                match m3010 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt9::C4(f689(v2))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            let a3014 = {
                let m3012 = match v0 {
                    Adt73::C0(_, _, _, _, _, _, _, _, _, h3011) => {
                        h3011
                    }
                };
                match m3012 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt9::C5(f876(v3))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            append_list(a3013, a3014)
        };
        append_list(a3015, a3016)
    };
    append_list(a3017, a3018)
}

pub fn f851(p3021: List<Adt5>, p3022: List<Adt9>) -> List<Adt9> {
    let m3020 = (p3021, p3022);
    {
        let mut m3020 = m3020;
        loop {
            let step = {
                let (v0, v1) = m3020;
                let m3019 = v0;
                match m3019.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt5>, List<Adt9>), List<Adt9>>((v3, List::cons(f877(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3020 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f852() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f853() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f854(p3027: List<u64>, p3028: u64, p3029: List<u64>) -> List<u64> {
    let m3026 = (p3027, (p3028, p3029));
    {
        let mut m3026 = m3026;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3026;
                let m3023 = v0;
                match m3023.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f878(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt161 = f879(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt161::C0(h3024, _) => {
                                h3024
                            }
                        }, match v5 {
                            Adt161::C0(_, h3025) => {
                                h3025
                            }
                        })))
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

pub fn f855(v0: Adt148, v1: Adt36) -> bool {
    let m3030 = v1;
    match m3030 {
        Adt36::C0(_) => {
            f880(v0, Adt148::C0)
        }
        Adt36::C1(_) => {
            f880(v0, Adt148::C1)
        }
        Adt36::C2(_) => {
            f880(v0, Adt148::C2)
        }
        Adt36::C3(_) => {
            f880(v0, Adt148::C2)
        }
        Adt36::C4(_) => {
            f880(v0, Adt148::C3)
        }
        Adt36::C5(_) => {
            f880(v0, Adt148::C4)
        }
        Adt36::C6(_) => {
            f880(v0, Adt148::C5)
        }
        Adt36::C7(_) => {
            f880(v0, Adt148::C6)
        }
        Adt36::C8(_) => {
            f880(v0, Adt148::C7)
        }
        Adt36::C9(_) => {
            f880(v0, Adt148::C8)
        }
        Adt36::C10(_) => {
            f880(v0, Adt148::C9)
        }
        Adt36::C11 => {
            if f880(v0.clone(), Adt148::C10) {
                true
            } else {
                f880(v0, Adt148::C11)
            }
        }
        Adt36::C12(_) => {
            f881(v0)
        }
        Adt36::C13(_) => {
            f882(v0)
        }
    }
}

pub fn f856(v0: Adt3, v1: Adt18, v2: Adt36) -> Result<Adt136, Adt139> {
    let m3031 = v2.clone();
    match m3031 {
        Adt36::C0(_) => {
            Ok::<Adt136, Adt139>(f817(v0, v2))
        }
        Adt36::C1(_) => {
            Ok::<Adt136, Adt139>(f817(v0, v2))
        }
        Adt36::C2(_) => {
            Ok::<Adt136, Adt139>(f817(v0, v2))
        }
        Adt36::C3(v8) => {
            Ok::<Adt136, Adt139>(f886(v0, f884(v1), v8))
        }
        Adt36::C4(_) => {
            Ok::<Adt136, Adt139>(f817(v0, v2))
        }
        Adt36::C5(_) => {
            Ok::<Adt136, Adt139>(f817(v0, v2))
        }
        Adt36::C6(_) => {
            Ok::<Adt136, Adt139>(f817(v0, v2))
        }
        Adt36::C7(_) => {
            Ok::<Adt136, Adt139>(f817(v0, v2))
        }
        Adt36::C8(_) => {
            Ok::<Adt136, Adt139>(f817(v0, v2))
        }
        Adt36::C9(_) => {
            Ok::<Adt136, Adt139>(f817(v0, v2))
        }
        Adt36::C10(_) => {
            Ok::<Adt136, Adt139>(f817(v0, v2))
        }
        Adt36::C11 => {
            Ok::<Adt136, Adt139>(f817(v0, v2))
        }
        Adt36::C12(v3) => {
            f883(v0, f884(v1), v3)
        }
        Adt36::C13(v4) => {
            f885(v0, f884(v1), v4)
        }
    }
}

pub fn f857() -> Adt88 {
    Adt88::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f858(v0: Bytes, v1: u64, v2: u64, v3: Fn1, v4: Adt18, v5: Result<Adt18, Adt88>) -> R<Result<Adt36, Adt88>> {
    let m3032 = v5;
    match m3032 {
        Ok(v7) => {
            f887(v0, v4, f888(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt36, Adt88>(v6))
        }
    }
}

pub fn f859(v0: Bytes, v1: Adt18) -> R<Result<Adt18, Adt88>> {
    let m3033 = v1.clone();
    match m3033 {
        Adt18::C0(_) => {
            Ok(Ok::<Adt18, Adt88>(v1))
        }
        Adt18::C1(_) => {
            Ok(Ok::<Adt18, Adt88>(v1))
        }
        Adt18::C2(v2) => {
            f889(v0, v2)
        }
        Adt18::C3(_) => {
            Ok(Ok::<Adt18, Adt88>(v1))
        }
        Adt18::C4(v3) => {
            f890(v0, v3)
        }
        Adt18::C5(_) => {
            Ok(Ok::<Adt18, Adt88>(v1))
        }
        Adt18::C6(_) => {
            Ok(Ok::<Adt18, Adt88>(v1))
        }
        Adt18::C7(v4) => {
            f891(v0, v4)
        }
        Adt18::C8(_) => {
            Ok(Ok::<Adt18, Adt88>(v1))
        }
        Adt18::C9(_) => {
            Ok(Ok::<Adt18, Adt88>(v1))
        }
        Adt18::C10(_) => {
            Ok(Ok::<Adt18, Adt88>(v1))
        }
        Adt18::C11(_) => {
            Ok(Ok::<Adt18, Adt88>(v1))
        }
    }
}

pub fn f860(v0: Str, v1: List<(Str, Adt9)>) -> Result<List<(Str, Str)>, Adt140> {
    let m3034 = f892(v0, v1, List::<(Str, Str)>::nil());
    match m3034 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt140>(f893(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt140>(v3)
        }
    }
}

pub fn f861(p3038: List<Adt125>, v1: Adt8) -> R<Option<Adt123>> {
    let m3037 = p3038;
    {
        let mut m3037 = m3037;
        loop {
            let step = {
                let v0 = m3037;
                let m3035 = v0;
                match m3035.uncons() {
                    None => {
                        Err::<List<Adt125>, R<Option<Adt123>>>(Ok(None::<Adt123>))
                    }
                    Some((v2, v3)) => {
                        let m3036 = f894(v2, v1.clone())?;
                        match m3036 {
                            None => {
                                Ok::<List<Adt125>, R<Option<Adt123>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt125>, R<Option<Adt123>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f862(v0: Adt8) -> Adt8 {
    let m3039 = f225(Str::lit("arguments"), v0);
    match m3039 {
        Adt110::C0 => {
            f32()
        }
        Adt110::C1(v1) => {
            let m3040 = f226(v1);
            match m3040 {
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

pub fn f863(v0: Option<Str>, v1: Option<Str>) -> Option<Adt123> {
    let m3041 = v0;
    match m3041 {
        None => {
            Some(Adt123::C2)
        }
        Some(v2) => {
            let m3042 = v1;
            match m3042 {
                None => {
                    None::<Adt123>
                }
                Some(v3) => {
                    let a3043 = v2;
                    let a3044 = v3;
                    if equal(a3043, a3044) {
                        None::<Adt123>
                    } else {
                        Some(Adt123::C5)
                    }
                }
            }
        }
    }
}

pub fn f864(v0: Adt153, v1: Adt8) -> Option<Str> {
    let m3045 = v0;
    match m3045 {
        Adt153::C0 => {
            None::<Str>
        }
        Adt153::C1 => {
            let m3046 = f225(Str::lit("name"), v1);
            match m3046 {
                Adt110::C0 => {
                    None::<Str>
                }
                Adt110::C1(v2) => {
                    f275(v2)
                }
            }
        }
        Adt153::C2 => {
            let m3047 = f225(Str::lit("uri"), v1);
            match m3047 {
                Adt110::C0 => {
                    None::<Str>
                }
                Adt110::C1(v3) => {
                    f275(v3)
                }
            }
        }
        Adt153::C3 => {
            let m3048 = f225(Str::lit("taskId"), v1);
            match m3048 {
                Adt110::C0 => {
                    None::<Str>
                }
                Adt110::C1(v4) => {
                    f275(v4)
                }
            }
        }
    }
}

pub fn f865(v0: Adt9) -> Option<Adt21> {
    let m3049 = f226(v0);
    match m3049 {
        None => {
            None::<Adt21>
        }
        Some(v1) => {
            let m3050 = f895(Str::lit("experimental"), v1.clone());
            match m3050 {
                None => {
                    None::<Adt21>
                }
                Some(v2) => {
                    let m3051 = f895(Str::lit("roots"), v1.clone());
                    match m3051 {
                        None => {
                            None::<Adt21>
                        }
                        Some(v3) => {
                            let m3052 = f895(Str::lit("sampling"), v1.clone());
                            match m3052 {
                                None => {
                                    None::<Adt21>
                                }
                                Some(v4) => {
                                    let m3053 = f895(Str::lit("elicitation"), v1.clone());
                                    match m3053 {
                                        None => {
                                            None::<Adt21>
                                        }
                                        Some(v5) => {
                                            let m3054 = f895(Str::lit("extensions"), v1);
                                            match m3054 {
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

pub fn f866() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f867(v0: Adt8) -> Result<Option<Adt22>, Adt140> {
    let m3055 = f821(f896(), v0);
    match m3055 {
        Ok(v2) => {
            let m3056 = v2;
            match m3056 {
                None => {
                    Ok::<Option<Adt22>, Adt140>(None::<Adt22>)
                }
                Some(v3) => {
                    let m3057 = f897(v3);
                    match m3057 {
                        Adt162::C0 => {
                            Err::<Option<Adt22>, Adt140>(Adt140::C8(f896()))
                        }
                        Adt162::C1(v4) => {
                            Ok::<Option<Adt22>, Adt140>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt22>, Adt140>(v1)
        }
    }
}

pub fn f868(v0: Adt8) -> Result<Option<Adt23>, Adt140> {
    let m3058 = f225(f898(), v0);
    match m3058 {
        Adt110::C0 => {
            Ok::<Option<Adt23>, Adt140>(None::<Adt23>)
        }
        Adt110::C1(v1) => {
            let m3059 = f275(v1.clone());
            match m3059 {
                None => {
                    let m3060 = f675(v1);
                    match m3060 {
                        None => {
                            Err::<Option<Adt23>, Adt140>(Adt140::C8(f898()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt23>, Adt140>(Some(Adt23::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt23>, Adt140>(Some(Adt23::C0(v2)))
                }
            }
        }
    }
}

pub fn f869(v0: Option<List<Adt45>>) -> List<(Str, Adt9)> {
    let m3061 = v0;
    match m3061 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt9::C4(f899(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f870(v0: Option<Adt10>) -> List<(Str, Adt9)> {
    let m3062 = v0;
    match m3062 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt9::C2(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f871(p3065: List<Adt61>, p3066: List<Adt9>) -> List<Adt9> {
    let m3064 = (p3065, p3066);
    {
        let mut m3064 = m3064;
        loop {
            let step = {
                let (v0, v1) = m3064;
                let m3063 = v0;
                match m3063.uncons() {
                    None => {
                        Err::<(List<Adt61>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt61>, List<Adt9>), List<Adt9>>((v3, List::cons(f900(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3064 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f872(p3070: List<Adt74>, p3071: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m3069 = (p3070, p3071);
    {
        let mut m3069 = m3069;
        loop {
            let step = {
                let (v0, v1) = m3069;
                let m3067 = v0;
                match m3067.uncons() {
                    None => {
                        Err::<(List<Adt74>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3068 = f901(v2)?;
                        match m3068 {
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
                Ok(next) => { m3069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f873(v0: Adt79) -> Adt9 {
    Adt9::C5({
        let a3081 = {
            let m3073 = match v0.clone() {
                Adt79::C0(h3072, _, _, _) => {
                    h3072
                }
            };
            match m3073 {
                None => {
                    List::<(Str, Adt9)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt9::C4(f553(f902(v1, List::<Adt9>::nil()), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())
                }
            }
        };
        let a3082 = {
            let a3079 = f875(Str::lit("costPriority"), match v0.clone() {
                Adt79::C0(_, h3074, _, _) => {
                    h3074
                }
            });
            let a3080 = {
                let a3077 = f875(Str::lit("speedPriority"), match v0.clone() {
                    Adt79::C0(_, _, h3075, _) => {
                        h3075
                    }
                });
                let a3078 = f875(Str::lit("intelligencePriority"), match v0 {
                    Adt79::C0(_, _, _, h3076) => {
                        h3076
                    }
                });
                append_list(a3077, a3078)
            };
            append_list(a3079, a3080)
        };
        append_list(a3081, a3082)
    })
}

pub fn f874(v0: Adt81) -> Str {
    let m3083 = v0;
    match m3083 {
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

pub fn f875(v0: Str, v1: Option<Adt10>) -> List<(Str, Adt9)> {
    let m3084 = v1;
    match m3084 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C2(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f876(v0: Adt82) -> List<(Str, Adt9)> {
    let m3086 = match v0 {
        Adt82::C0(h3085) => {
            h3085
        }
    };
    match m3086 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt9::C3(f903(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f877(v0: Adt5) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("src"), Adt9::C3(match v0.clone() {
        Adt5::C0(h3087, _, _, _) => {
            h3087
        }
    })), {
        let a3093 = f687(Str::lit("mimeType"), match v0.clone() {
            Adt5::C0(_, h3088, _, _) => {
                h3088
            }
        });
        let a3094 = {
            let a3091 = f904(match v0.clone() {
                Adt5::C0(_, _, h3089, _) => {
                    h3089
                }
            });
            let a3092 = f905(match v0 {
                Adt5::C0(_, _, _, h3090) => {
                    h3090
                }
            });
            append_list(a3091, a3092)
        };
        append_list(a3093, a3094)
    }))
}

pub fn f878(v0: u64, v1: List<u64>) -> List<u64> {
    let a3095 = v0;
    let a3096 = 0u64;
    if nat_eq(a3095, a3096) {
        v1
    } else {
        let a3097 = v0;
        let a3098 = 1u64;
        if nat_eq(a3097, a3098) {
            List::cons(194u64, v1)
        } else {
            let a3099 = v0;
            let a3100 = 2u64;
            if nat_eq(a3099, a3100) {
                List::cons(226u64, v1)
            } else {
                let a3101 = v0;
                let a3102 = 3u64;
                if nat_eq(a3101, a3102) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f879(v0: u64, v1: u64, v2: List<u64>) -> Adt161 {
    let a3103 = v0;
    let a3104 = 0u64;
    if nat_eq(a3103, a3104) {
        f906(v1, v2)
    } else {
        let a3105 = v0;
        let a3106 = 1u64;
        if nat_eq(a3105, a3106) {
            let a3107 = 128u64;
            let a3108 = v1;
            if if nat_le(a3107, a3108) {
                let a3109 = v1;
                let a3110 = 159u64;
                nat_le(a3109, a3110)
            } else {
                false
            } {
                Adt161::C0(0u64, f907(v2))
            } else {
                f906(v1, List::cons(194u64, v2))
            }
        } else {
            let a3111 = v0;
            let a3112 = 2u64;
            if nat_eq(a3111, a3112) {
                let a3113 = v1;
                let a3114 = 128u64;
                if nat_eq(a3113, a3114) {
                    Adt161::C0(3u64, v2)
                } else {
                    let a3115 = v1;
                    let a3116 = 129u64;
                    if nat_eq(a3115, a3116) {
                        Adt161::C0(4u64, v2)
                    } else {
                        f906(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a3117 = v0;
                let a3118 = 3u64;
                if nat_eq(a3117, a3118) {
                    let a3119 = 170u64;
                    let a3120 = v1;
                    if if nat_le(a3119, a3120) {
                        let a3121 = v1;
                        let a3122 = 174u64;
                        nat_le(a3121, a3122)
                    } else {
                        false
                    } {
                        Adt161::C0(0u64, f907(v2))
                    } else {
                        f906(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a3123 = 166u64;
                    let a3124 = v1;
                    if if nat_le(a3123, a3124) {
                        let a3125 = v1;
                        let a3126 = 169u64;
                        nat_le(a3125, a3126)
                    } else {
                        false
                    } {
                        Adt161::C0(0u64, f907(v2))
                    } else {
                        f906(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f880(v0: Adt148, v1: Adt148) -> bool {
    let a3127 = f722(v0);
    let a3128 = f722(v1);
    equal(a3127, a3128)
}

pub fn f881(v0: Adt148) -> bool {
    let m3129 = v0;
    match m3129 {
        Adt148::C0 => {
            false
        }
        Adt148::C1 => {
            false
        }
        Adt148::C2 => {
            true
        }
        Adt148::C3 => {
            false
        }
        Adt148::C4 => {
            true
        }
        Adt148::C5 => {
            false
        }
        Adt148::C6 => {
            false
        }
        Adt148::C7 => {
            true
        }
        Adt148::C8 => {
            false
        }
        Adt148::C9 => {
            false
        }
        Adt148::C10 => {
            false
        }
        Adt148::C11 => {
            false
        }
    }
}

pub fn f882(v0: Adt148) -> bool {
    let m3130 = v0;
    match m3130 {
        Adt148::C0 => {
            false
        }
        Adt148::C1 => {
            false
        }
        Adt148::C2 => {
            true
        }
        Adt148::C3 => {
            false
        }
        Adt148::C4 => {
            false
        }
        Adt148::C5 => {
            false
        }
        Adt148::C6 => {
            false
        }
        Adt148::C7 => {
            false
        }
        Adt148::C8 => {
            false
        }
        Adt148::C9 => {
            false
        }
        Adt148::C10 => {
            false
        }
        Adt148::C11 => {
            false
        }
    }
}

pub fn f883(v0: Adt3, v1: Adt20, v2: Adt85) -> Result<Adt136, Adt139> {
    f908(v0, v2.clone(), f909(match v1 {
        Adt20::C0(_, h3131, _, _, _, _) => {
            h3131
        }
    }, f700(v2)))
}

pub fn f884(v0: Adt18) -> Adt20 {
    let m3132 = v0;
    match m3132 {
        Adt18::C0(v1) => {
            match v1 {
                Adt19::C0(h3133) => {
                    h3133
                }
            }
        }
        Adt18::C1(v2) => {
            match v2 {
                Adt24::C0(h3134, _) => {
                    h3134
                }
            }
        }
        Adt18::C2(v3) => {
            match v3 {
                Adt25::C0(h3135, _, _, _, _) => {
                    h3135
                }
            }
        }
        Adt18::C3(v4) => {
            match v4 {
                Adt24::C0(h3136, _) => {
                    h3136
                }
            }
        }
        Adt18::C4(v5) => {
            match v5 {
                Adt26::C0(h3137, _, _, _) => {
                    h3137
                }
            }
        }
        Adt18::C5(v6) => {
            match v6 {
                Adt24::C0(h3138, _) => {
                    h3138
                }
            }
        }
        Adt18::C6(v7) => {
            match v7 {
                Adt24::C0(h3139, _) => {
                    h3139
                }
            }
        }
        Adt18::C7(v8) => {
            match v8 {
                Adt27::C0(h3140, _, _, _, _) => {
                    h3140
                }
            }
        }
        Adt18::C8(v9) => {
            match v9 {
                Adt28::C0(h3141, _, _, _) => {
                    h3141
                }
            }
        }
        Adt18::C9(v10) => {
            match v10 {
                Adt33::C0(h3142, _) => {
                    h3142
                }
            }
        }
        Adt18::C10(v11) => {
            match v11 {
                Adt34::C0(h3143, _, _) => {
                    h3143
                }
            }
        }
        Adt18::C11(v12) => {
            match v12 {
                Adt35::C0(h3144, _) => {
                    h3144
                }
            }
        }
    }
}

pub fn f885(v0: Adt3, v1: Adt20, v2: Adt86) -> Result<Adt136, Adt139> {
    f910(v0, v2, f911(match v1 {
        Adt20::C0(_, h3145, _, _, _, _) => {
            h3145
        }
    }))
}

pub fn f886(v0: Adt3, v1: Adt20, v2: Adt53) -> Adt136 {
    Adt136::C0(Adt36::C2(match v2.clone() {
        Adt53::C0(h3146, _) => {
            h3146
        }
    }), {
        let m3148 = f817(v0, Adt36::C11);
        match m3148 {
            Adt136::C0(_, h3147, _) => {
                h3147
            }
        }
    }, f912(v1, match v2 {
        Adt53::C0(_, h3149) => {
            h3149
        }
    }))
}

pub fn f887(v0: Bytes, v1: Adt18, v2: Result<Adt36, Adt88>) -> R<Result<Adt36, Adt88>> {
    let m3150 = v2;
    match m3150 {
        Ok(v4) => {
            f913(v0, f816(v1.clone()), f914(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt36, Adt88>(v3))
        }
    }
}

pub fn f888(v0: u64, v1: u64, v2: Fn1, v3: Adt18) -> R<Result<Adt36, Adt88>> {
    let m3151 = v3.clone();
    match m3151 {
        Adt18::C0(_) => {
            let c3153: Fn1 = v2;
            c3153.apply(v3)
        }
        Adt18::C1(_) => {
            let c3154: Fn1 = v2;
            c3154.apply(v3)
        }
        Adt18::C2(_) => {
            if f915(v0, v1) {
                let c3152: Fn1 = v2;
                c3152.apply(v3)
            } else {
                Ok(Err::<Adt36, Adt88>(Adt88::C7(v0)))
            }
        }
        Adt18::C3(_) => {
            let c3155: Fn1 = v2;
            c3155.apply(v3)
        }
        Adt18::C4(_) => {
            let c3156: Fn1 = v2;
            c3156.apply(v3)
        }
        Adt18::C5(_) => {
            let c3157: Fn1 = v2;
            c3157.apply(v3)
        }
        Adt18::C6(_) => {
            let c3158: Fn1 = v2;
            c3158.apply(v3)
        }
        Adt18::C7(_) => {
            let c3159: Fn1 = v2;
            c3159.apply(v3)
        }
        Adt18::C8(_) => {
            let c3160: Fn1 = v2;
            c3160.apply(v3)
        }
        Adt18::C9(_) => {
            let c3161: Fn1 = v2;
            c3161.apply(v3)
        }
        Adt18::C10(_) => {
            let c3162: Fn1 = v2;
            c3162.apply(v3)
        }
        Adt18::C11(_) => {
            let c3163: Fn1 = v2;
            c3163.apply(v3)
        }
    }
}

pub fn f889(v0: Bytes, v1: Adt25) -> R<Result<Adt18, Adt88>> {
    let m3167 = f916(v0, Str::lit("tools/call"), match v1.clone() {
        Adt25::C0(_, h3164, _, _, _) => {
            h3164
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, _, h3165) => {
            h3165
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, h3166, _) => {
            h3166
        }
    })?;
    match m3167 {
        Ok(v3) => {
            Ok(Ok::<Adt18, Adt88>(Adt18::C2(Adt25::C0(match v1.clone() {
                Adt25::C0(h3168, _, _, _, _) => {
                    h3168
                }
            }, match v1.clone() {
                Adt25::C0(_, h3169, _, _, _) => {
                    h3169
                }
            }, match v1.clone() {
                Adt25::C0(_, _, h3170, _, _) => {
                    h3170
                }
            }, match v1 {
                Adt25::C0(_, _, _, h3171, _) => {
                    h3171
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt18, Adt88>(v2))
        }
    }
}

pub fn f890(v0: Bytes, v1: Adt26) -> R<Result<Adt18, Adt88>> {
    let m3175 = f916(v0, Str::lit("resources/read"), match v1.clone() {
        Adt26::C0(_, h3172, _, _) => {
            h3172
        }
    }, match v1.clone() {
        Adt26::C0(_, _, _, h3173) => {
            h3173
        }
    }, match v1.clone() {
        Adt26::C0(_, _, h3174, _) => {
            h3174
        }
    })?;
    match m3175 {
        Ok(v3) => {
            Ok(Ok::<Adt18, Adt88>(Adt18::C4(Adt26::C0(match v1.clone() {
                Adt26::C0(h3176, _, _, _) => {
                    h3176
                }
            }, match v1.clone() {
                Adt26::C0(_, h3177, _, _) => {
                    h3177
                }
            }, match v1 {
                Adt26::C0(_, _, h3178, _) => {
                    h3178
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt18, Adt88>(v2))
        }
    }
}

pub fn f891(v0: Bytes, v1: Adt27) -> R<Result<Adt18, Adt88>> {
    let m3182 = f916(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt27::C0(_, h3179, _, _, _) => {
            h3179
        }
    }, match v1.clone() {
        Adt27::C0(_, _, _, _, h3180) => {
            h3180
        }
    }, match v1.clone() {
        Adt27::C0(_, _, _, h3181, _) => {
            h3181
        }
    })?;
    match m3182 {
        Ok(v3) => {
            Ok(Ok::<Adt18, Adt88>(Adt18::C7(Adt27::C0(match v1.clone() {
                Adt27::C0(h3183, _, _, _, _) => {
                    h3183
                }
            }, match v1.clone() {
                Adt27::C0(_, h3184, _, _, _) => {
                    h3184
                }
            }, match v1.clone() {
                Adt27::C0(_, _, h3185, _, _) => {
                    h3185
                }
            }, match v1 {
                Adt27::C0(_, _, _, h3186, _) => {
                    h3186
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt18, Adt88>(v2))
        }
    }
}

pub fn f892(v0: Str, p3192: List<(Str, Adt9)>, p3193: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt140> {
    let m3191 = (p3192, p3193);
    {
        let mut m3191 = m3191;
        loop {
            let step = {
                let (v1, v2) = m3191;
                let m3187 = v1;
                match m3187.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt140>>(Ok::<List<(Str, Str)>, Adt140>(v2))
                    }
                    Some((v3, v4)) => {
                        let m3189 = f275({
                            let (_, h3188) = v3;
                            h3188
                        });
                        match m3189 {
                            None => {
                                Err::<(List<(Str, Adt9)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt140>>(Err::<List<(Str, Str)>, Adt140>(Adt140::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt9)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt140>>((v4, List::cons(({
                                    let (h3190, _) = v3;
                                    h3190
                                }, v5), v2)))
                            }
                        }
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

pub fn f893(p3196: List<(Str, Str)>, p3197: List<(Str, Str)>) -> List<(Str, Str)> {
    let m3195 = (p3196, p3197);
    {
        let mut m3195 = m3195;
        loop {
            let step = {
                let (v0, v1) = m3195;
                let m3194 = v0;
                match m3194.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3195 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f894(v0: Adt125, v1: Adt8) -> R<Option<Adt123>> {
    let m3199 = f917(match v0.clone() {
        Adt125::C0(_, h3198, _, _) => {
            h3198
        }
    }, v1);
    match m3199 {
        Adt110::C0 => {
            Ok(f918(v0))
        }
        Adt110::C1(v2) => {
            if f919(v2.clone()) {
                Ok(f918(v0))
            } else {
                f920(v0, v2)
            }
        }
    }
}

pub fn f895(v0: Str, v1: Adt8) -> Option<Option<Adt8>> {
    let m3200 = f225(v0, v1);
    match m3200 {
        Adt110::C0 => {
            Some(None::<Adt8>)
        }
        Adt110::C1(v2) => {
            let m3201 = f226(v2);
            match m3201 {
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

pub fn f896() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f897(v0: Str) -> Adt162 {
    let a3202 = v0.clone();
    let a3203 = Str::lit("debug");
    if equal(a3202, a3203) {
        Adt162::C1(Adt22::C0)
    } else {
        let a3204 = v0.clone();
        let a3205 = Str::lit("info");
        if equal(a3204, a3205) {
            Adt162::C1(Adt22::C1)
        } else {
            let a3206 = v0.clone();
            let a3207 = Str::lit("notice");
            if equal(a3206, a3207) {
                Adt162::C1(Adt22::C2)
            } else {
                let a3208 = v0.clone();
                let a3209 = Str::lit("warning");
                if equal(a3208, a3209) {
                    Adt162::C1(Adt22::C3)
                } else {
                    let a3210 = v0.clone();
                    let a3211 = Str::lit("error");
                    if equal(a3210, a3211) {
                        Adt162::C1(Adt22::C4)
                    } else {
                        let a3212 = v0.clone();
                        let a3213 = Str::lit("critical");
                        if equal(a3212, a3213) {
                            Adt162::C1(Adt22::C5)
                        } else {
                            let a3214 = v0.clone();
                            let a3215 = Str::lit("alert");
                            if equal(a3214, a3215) {
                                Adt162::C1(Adt22::C6)
                            } else {
                                let a3216 = v0;
                                let a3217 = Str::lit("emergency");
                                if equal(a3216, a3217) {
                                    Adt162::C1(Adt22::C7)
                                } else {
                                    Adt162::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f898() -> Str {
    Str::lit("progressToken")
}

pub fn f899(v0: List<Adt45>) -> List<Adt9> {
    f553(f921(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f900(v0: Adt61) -> Adt9 {
    Adt9::C5({
        let a3226 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt61::C0(h3218, _, _, _) => {
                h3218
            }
        })), List::<(Str, Adt9)>::nil());
        let a3227 = {
            let a3224 = f687(Str::lit("title"), match v0.clone() {
                Adt61::C0(_, h3219, _, _) => {
                    h3219
                }
            });
            let a3225 = {
                let a3222 = f687(Str::lit("description"), match v0.clone() {
                    Adt61::C0(_, _, h3220, _) => {
                        h3220
                    }
                });
                let a3223 = f692(Str::lit("required"), match v0 {
                    Adt61::C0(_, _, _, h3221) => {
                        h3221
                    }
                });
                append_list(a3222, a3223)
            };
            append_list(a3224, a3225)
        };
        append_list(a3226, a3227)
    })
}

pub fn f901(v0: Adt74) -> R<Option<Adt9>> {
    let m3229 = f922(match v0.clone() {
        Adt74::C0(_, h3228, _) => {
            h3228
        }
    })?;
    match m3229 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("role"), Adt9::C3(f846(match v0.clone() {
                Adt74::C0(h3230, _, _) => {
                    h3230
                }
            }))), List::cons((Str::lit("content"), v1), f838(match v0 {
                Adt74::C0(_, _, h3231) => {
                    h3231
                }
            }))))))
        }
    }
}

pub fn f902(p3235: List<Adt80>, p3236: List<Adt9>) -> List<Adt9> {
    let m3234 = (p3235, p3236);
    {
        let mut m3234 = m3234;
        loop {
            let step = {
                let (v0, v1) = m3234;
                let m3232 = v0;
                match m3232.uncons() {
                    None => {
                        Err::<(List<Adt80>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt80>, List<Adt9>), List<Adt9>>((v3, List::cons(Adt9::C5(f687(Str::lit("name"), match v2 {
                            Adt80::C0(h3233) => {
                                h3233
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3234 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f903(v0: Adt83) -> Str {
    let m3237 = v0;
    match m3237 {
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

pub fn f904(v0: Option<List<Str>>) -> List<(Str, Adt9)> {
    let m3238 = v0;
    match m3238 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt9::C4(f634(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f905(v0: Option<Adt6>) -> List<(Str, Adt9)> {
    let m3239 = v0;
    match m3239 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt9::C3(f923(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f906(v0: u64, v1: List<u64>) -> Adt161 {
    let a3240 = v0;
    let a3241 = 32u64;
    if nat_lt(a3240, a3241) {
        if f924(v0) {
            Adt161::C0(0u64, List::cons(v0, v1))
        } else {
            Adt161::C0(0u64, f907(v1))
        }
    } else {
        let a3242 = v0;
        let a3243 = 127u64;
        if nat_eq(a3242, a3243) {
            Adt161::C0(0u64, f907(v1))
        } else {
            let a3244 = v0;
            let a3245 = 194u64;
            if nat_eq(a3244, a3245) {
                Adt161::C0(1u64, v1)
            } else {
                let a3246 = v0;
                let a3247 = 226u64;
                if nat_eq(a3246, a3247) {
                    Adt161::C0(2u64, v1)
                } else {
                    Adt161::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f907(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f908(v0: Adt3, v1: Adt85, v2: bool) -> Result<Adt136, Adt139> {
    if v2 {
        Ok::<Adt136, Adt139>(f817(v0, Adt36::C12(v1)))
    } else {
        Err::<Adt136, Adt139>(Adt139::C4(Adt88::C8(f925(f700(v1)))))
    }
}

pub fn f909(v0: Adt21, v1: List<(Str, Adt68)>) -> bool {
    let a3248 = f926(v1.clone());
    if if if bool_not(a3248) {
        true
    } else {
        f286(match v0.clone() {
            Adt21::C0(_, _, _, h3249, _) => {
                h3249
            }
        })
    } {
        let a3250 = f927(v1.clone());
        if bool_not(a3250) {
            true
        } else {
            f286(match v0.clone() {
                Adt21::C0(_, _, h3251, _, _) => {
                    h3251
                }
            })
        }
    } else {
        false
    } {
        let a3252 = f928(v1);
        if bool_not(a3252) {
            true
        } else {
            f286(match v0 {
                Adt21::C0(_, h3253, _, _, _) => {
                    h3253
                }
            })
        }
    } else {
        false
    }
}

pub fn f910(v0: Adt3, v1: Adt86, v2: bool) -> Result<Adt136, Adt139> {
    if v2 {
        Ok::<Adt136, Adt139>(f817(v0, Adt36::C13(v1)))
    } else {
        Err::<Adt136, Adt139>(Adt139::C4(Adt88::C8(f929())))
    }
}

pub fn f911(v0: Adt21) -> bool {
    f725(match v0 {
        Adt21::C0(_, _, _, _, h3254) => {
            h3254
        }
    }, f726())
}

pub fn f912(v0: Adt20, v1: List<Adt54>) -> List<Adt138> {
    let m3256 = match v0 {
        Adt20::C0(_, _, _, _, h3255, _) => {
            h3255
        }
    };
    match m3256 {
        None => {
            List::<Adt138>::nil()
        }
        Some(v2) => {
            f930(f931(v2, v1, List::<Adt138>::nil()), List::<Adt138>::nil())
        }
    }
}

pub fn f913(v0: Bytes, v1: Adt148, v2: Str, v3: Adt36) -> R<Result<Adt36, Adt88>> {
    let m3257 = v3.clone();
    match m3257 {
        Adt36::C0(_) => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
        Adt36::C1(_) => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
        Adt36::C2(_) => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
        Adt36::C3(_) => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
        Adt36::C4(_) => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
        Adt36::C5(_) => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
        Adt36::C6(_) => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
        Adt36::C7(_) => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
        Adt36::C8(_) => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
        Adt36::C9(_) => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
        Adt36::C10(_) => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
        Adt36::C11 => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
        Adt36::C12(v4) => {
            f932(v0, v1, v2, v4)
        }
        Adt36::C13(_) => {
            Ok(Ok::<Adt36, Adt88>(v3))
        }
    }
}

pub fn f914(v0: Adt18) -> Str {
    let m3258 = v0;
    match m3258 {
        Adt18::C0(_) => {
            Str::lit("")
        }
        Adt18::C1(_) => {
            Str::lit("")
        }
        Adt18::C2(v1) => {
            match v1 {
                Adt25::C0(_, h3259, _, _, _) => {
                    h3259
                }
            }
        }
        Adt18::C3(_) => {
            Str::lit("")
        }
        Adt18::C4(v2) => {
            match v2 {
                Adt26::C0(_, h3260, _, _) => {
                    h3260
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
                Adt27::C0(_, h3261, _, _, _) => {
                    h3261
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

pub fn f915(v0: u64, v1: u64) -> bool {
    let a3262 = v1;
    let a3263 = v0;
    nat_lt(a3262, a3263)
}

pub fn f916(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt8>) -> R<Result<Option<Str>, Adt88>> {
    Ok(f933(f934(v0, v1, v2, v3)?, f935(v4)))
}

pub fn f917(p3269: List<Str>, p3270: Adt8) -> Adt110 {
    let m3268 = (p3269, p3270);
    {
        let mut m3268 = m3268;
        loop {
            let step = {
                let (v0, v1) = m3268;
                let m3264 = v0;
                match m3264.uncons() {
                    None => {
                        Err::<(List<Str>, Adt8), Adt110>(Adt110::C0)
                    }
                    Some((v2, v3)) => {
                        let m3265 = f225(v2, v1);
                        match m3265 {
                            Adt110::C0 => {
                                Err::<(List<Str>, Adt8), Adt110>(Adt110::C0)
                            }
                            Adt110::C1(v4) => {
                                let m3266 = v3.clone();
                                match m3266.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt8), Adt110>(Adt110::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3267 = f226(v4);
                                        match m3267 {
                                            None => {
                                                Err::<(List<Str>, Adt8), Adt110>(Adt110::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt8), Adt110>((v3, v7))
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
                Ok(next) => { m3268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f918(v0: Adt125) -> Option<Adt123> {
    let m3272 = match v0.clone() {
        Adt125::C0(_, _, _, h3271) => {
            h3271
        }
    };
    match m3272 {
        Adt126::C0 => {
            None::<Adt123>
        }
        Adt126::C1 => {
            Some(Adt123::C10(match v0 {
                Adt125::C0(h3273, _, _, _) => {
                    h3273
                }
            }))
        }
        Adt126::C2(_) => {
            None::<Adt123>
        }
    }
}

pub fn f919(v0: Adt9) -> bool {
    let m3274 = v0;
    match m3274 {
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

pub fn f920(v0: Adt125, v1: Adt9) -> R<Option<Adt123>> {
    let m3276 = match v0.clone() {
        Adt125::C0(_, _, _, h3275) => {
            h3275
        }
    };
    match m3276 {
        Adt126::C0 => {
            Ok(Some(Adt123::C8(match v0 {
                Adt125::C0(h3277, _, _, _) => {
                    h3277
                }
            })))
        }
        Adt126::C1 => {
            Ok(Some(Adt123::C10(match v0 {
                Adt125::C0(h3278, _, _, _) => {
                    h3278
                }
            })))
        }
        Adt126::C2(v2) => {
            if f936(match v0.clone() {
                Adt125::C0(_, _, h3279, _) => {
                    h3279
                }
            }, v2, v1)? {
                Ok(None::<Adt123>)
            } else {
                Ok(Some(Adt123::C9(match v0 {
                    Adt125::C0(h3280, _, _, _) => {
                        h3280
                    }
                })))
            }
        }
    }
}

pub fn f921(p3283: List<Adt45>, p3284: List<Adt9>) -> List<Adt9> {
    let m3282 = (p3283, p3284);
    {
        let mut m3282 = m3282;
        loop {
            let step = {
                let (v0, v1) = m3282;
                let m3281 = v0;
                match m3281.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt9>), List<Adt9>>((v3, List::cons(Adt9::C3(f846(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f922(v0: Adt75) -> R<Option<Adt9>> {
    let m3285 = v0;
    match m3285 {
        Adt75::C0(v1) => {
            f937(v1)
        }
        Adt75::C1(v2) => {
            let m3286 = f938(v2, List::<Adt9>::nil())?;
            match m3286 {
                None => {
                    Ok(None::<Adt9>)
                }
                Some(v3) => {
                    Ok(Some(Adt9::C4(f553(v3, List::<Adt9>::nil()))))
                }
            }
        }
    }
}

pub fn f923(v0: Adt6) -> Str {
    let m3287 = v0;
    match m3287 {
        Adt6::C0 => {
            Str::lit("light")
        }
        Adt6::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f924(v0: u64) -> bool {
    let a3288 = v0;
    let a3289 = 9u64;
    if nat_eq(a3288, a3289) {
        true
    } else {
        let a3290 = v0;
        let a3291 = 10u64;
        if nat_eq(a3290, a3291) {
            true
        } else {
            let a3292 = v0;
            let a3293 = 13u64;
            nat_eq(a3292, a3293)
        }
    }
}

pub fn f925(v0: List<(Str, Adt68)>) -> Adt21 {
    Adt21::C0(None::<Adt8>, if f928(v0.clone()) {
        Some(f32())
    } else {
        None::<Adt8>
    }, if f927(v0.clone()) {
        Some(f32())
    } else {
        None::<Adt8>
    }, if f926(v0) {
        Some(f32())
    } else {
        None::<Adt8>
    }, None::<Adt8>)
}

pub fn f926(p3298: List<(Str, Adt68)>) -> bool {
    let m3297 = p3298;
    {
        let mut m3297 = m3297;
        loop {
            let step = {
                let v0 = m3297;
                let m3294 = v0;
                match m3294.uncons() {
                    None => {
                        Err::<List<(Str, Adt68)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3296 = {
                            let (_, h3295) = v1;
                            h3295
                        };
                        match m3296 {
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
                Ok(next) => { m3297 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f927(p3303: List<(Str, Adt68)>) -> bool {
    let m3302 = p3303;
    {
        let mut m3302 = m3302;
        loop {
            let step = {
                let v0 = m3302;
                let m3299 = v0;
                match m3299.uncons() {
                    None => {
                        Err::<List<(Str, Adt68)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3301 = {
                            let (_, h3300) = v1;
                            h3300
                        };
                        match m3301 {
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
                Ok(next) => { m3302 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f928(p3308: List<(Str, Adt68)>) -> bool {
    let m3307 = p3308;
    {
        let mut m3307 = m3307;
        loop {
            let step = {
                let v0 = m3307;
                let m3304 = v0;
                match m3304.uncons() {
                    None => {
                        Err::<List<(Str, Adt68)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3306 = {
                            let (_, h3305) = v1;
                            h3305
                        };
                        match m3306 {
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
                Ok(next) => { m3307 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f929() -> Adt21 {
    Adt21::C0(None::<Adt8>, None::<Adt8>, None::<Adt8>, None::<Adt8>, Some(Adt8::C0(List::cons((f726(), Adt9::C5(List::<(Str, Adt9)>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f930(p3311: List<Adt138>, p3312: List<Adt138>) -> List<Adt138> {
    let m3310 = (p3311, p3312);
    {
        let mut m3310 = m3310;
        loop {
            let step = {
                let (v0, v1) = m3310;
                let m3309 = v0;
                match m3309.uncons() {
                    None => {
                        Err::<(List<Adt138>, List<Adt138>), List<Adt138>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt138>, List<Adt138>), List<Adt138>>((v3, List::cons(v2, v1)))
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

pub fn f931(v0: Adt23, p3315: List<Adt54>, p3316: List<Adt138>) -> List<Adt138> {
    let m3314 = (p3315, p3316);
    {
        let mut m3314 = m3314;
        loop {
            let step = {
                let (v1, v2) = m3314;
                let m3313 = v1;
                match m3313.uncons() {
                    None => {
                        Err::<(List<Adt54>, List<Adt138>), List<Adt138>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt54>, List<Adt138>), List<Adt138>>((v4, List::cons(Adt138::C0(v0.clone(), v3), v2)))
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

pub fn f932(v0: Bytes, v1: Adt148, v2: Str, v3: Adt85) -> R<Result<Adt36, Adt88>> {
    if f881(v1.clone()) {
        if f939(v3.clone()) {
            let m3317 = f940(v0, f722(v1), v2, v3)?;
            match m3317 {
                Ok(v5) => {
                    Ok(Ok::<Adt36, Adt88>(Adt36::C12(v5)))
                }
                Err(v4) => {
                    Ok(Err::<Adt36, Adt88>(v4))
                }
            }
        } else {
            Ok(Err::<Adt36, Adt88>(Adt88::C13))
        }
    } else {
        Ok(Err::<Adt36, Adt88>(Adt88::C12(f722(v1))))
    }
}

pub fn f933(v0: Result<Option<Str>, Adt88>, v1: bool) -> Result<Option<Str>, Adt88> {
    let m3318 = v0;
    match m3318 {
        Ok(v3) => {
            if v1 {
                Ok::<Option<Str>, Adt88>(v3)
            } else {
                Err::<Option<Str>, Adt88>(Adt88::C11)
            }
        }
        Err(v2) => {
            Err::<Option<Str>, Adt88>(v2)
        }
    }
}

pub fn f934(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt88>> {
    let m3319 = v3;
    match m3319 {
        None => {
            Ok(Ok::<Option<Str>, Adt88>(None::<Str>))
        }
        Some(v4) => {
            Ok(f941(f942(v0, v1, v2, v4)?))
        }
    }
}

pub fn f935(v0: Option<Adt8>) -> bool {
    let m3320 = v0;
    match m3320 {
        None => {
            true
        }
        Some(v1) => {
            f943(match v1 {
                Adt8::C0(h3321) => {
                    h3321
                }
            })
        }
    }
}

pub fn f936(v0: Adt16, v1: Str, v2: Adt9) -> R<bool> {
    let m3322 = v0;
    match m3322 {
        Adt16::C0 => {
            let m3323 = f275(v2);
            match m3323 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3324 = v1;
                    let a3325 = v3;
                    Ok(equal(a3324, a3325))
                }
            }
        }
        Adt16::C1 => {
            f944(v1, v2)
        }
        Adt16::C2 => {
            let m3326 = f346(v2);
            match m3326 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3327 = v1;
                        let a3328 = Str::lit("true");
                        Ok(equal(a3327, a3328))
                    } else {
                        let a3329 = v1;
                        let a3330 = Str::lit("false");
                        Ok(equal(a3329, a3330))
                    }
                }
            }
        }
    }
}

pub fn f937(v0: Adt76) -> R<Option<Adt9>> {
    let m3331 = v0;
    match m3331 {
        Adt76::C0(v1) => {
            Ok(Some(f839(v1)))
        }
        Adt76::C1(v2) => {
            Ok(Some(f840(Str::lit("image"), match v2.clone() {
                Adt46::C0(h3332, _, _, _) => {
                    h3332
                }
            }, match v2.clone() {
                Adt46::C0(_, h3333, _, _) => {
                    h3333
                }
            }, match v2.clone() {
                Adt46::C0(_, _, h3334, _) => {
                    h3334
                }
            }, match v2 {
                Adt46::C0(_, _, _, h3335) => {
                    h3335
                }
            })))
        }
        Adt76::C2(v3) => {
            Ok(Some(f840(Str::lit("audio"), match v3.clone() {
                Adt47::C0(h3336, _, _, _) => {
                    h3336
                }
            }, match v3.clone() {
                Adt47::C0(_, h3337, _, _) => {
                    h3337
                }
            }, match v3.clone() {
                Adt47::C0(_, _, h3338, _) => {
                    h3338
                }
            }, match v3 {
                Adt47::C0(_, _, _, h3339) => {
                    h3339
                }
            })))
        }
        Adt76::C3(v4) => {
            Ok(Some(f945(v4)))
        }
        Adt76::C4(v5) => {
            f946(v5)
        }
    }
}

pub fn f938(p3343: List<Adt76>, p3344: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m3342 = (p3343, p3344);
    {
        let mut m3342 = m3342;
        loop {
            let step = {
                let (v0, v1) = m3342;
                let m3340 = v0;
                match m3340.uncons() {
                    None => {
                        Err::<(List<Adt76>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3341 = f937(v2)?;
                        match m3341 {
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
                Ok(next) => { m3342 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f939(v0: Adt85) -> bool {
    let m3345 = v0;
    match m3345 {
        Adt85::C0(v1) => {
            let m3346 = v1;
            match m3346.uncons() {
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

pub fn f940(v0: Bytes, v1: Str, v2: Str, v3: Adt85) -> R<Result<Adt85, Adt88>> {
    let m3347 = v3.clone();
    match m3347 {
        Adt85::C0(_) => {
            Ok(Ok::<Adt85, Adt88>(v3))
        }
        Adt85::C1(v5) => {
            Ok(f947(f948(v0, v1, v2, v5)?, List::<(Str, Adt68)>::nil()))
        }
        Adt85::C2(v6, v7) => {
            Ok(f947(f948(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f941(v0: Option<Str>) -> Result<Option<Str>, Adt88> {
    let m3348 = v0;
    match m3348 {
        None => {
            Err::<Option<Str>, Adt88>(Adt88::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt88>(Some(v1))
        }
    }
}

pub fn f942(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3352 = f949(v0, {
        let a3349 = v1;
        utf8_encode(a3349)
    }, {
        let a3350 = v2;
        utf8_encode(a3350)
    }, {
        let a3351 = v3;
        utf8_encode(a3351)
    })?;
    match m3352 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3353 = v4;
            Ok(utf8_decode(a3353))
        }
    }
}

pub fn f943(p3358: List<(Str, Adt9)>) -> bool {
    let m3357 = p3358;
    {
        let mut m3357 = m3357;
        loop {
            let step = {
                let v0 = m3357;
                let m3354 = v0;
                match m3354.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3356 = f226({
                            let (_, h3355) = v1;
                            h3355
                        });
                        match m3356 {
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
                Ok(next) => { m3357 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f944(v0: Str, v1: Adt9) -> R<bool> {
    let m3359 = f585(v1);
    match m3359 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3360 = f476(v2)?;
            match m3360 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3362 = f950(f29({
                        let a3361 = v0;
                        utf8_encode(a3361)
                    })?)?;
                    match m3362 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f649({
                                let a3363 = v3;
                                let a3364 = v4;
                                int_sub(a3363, a3364)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f945(v0: Adt77) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt9::C3(match v0.clone() {
        Adt77::C0(h3365, _, _, _) => {
            h3365
        }
    })), List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
        Adt77::C0(_, h3366, _, _) => {
            h3366
        }
    })), List::cons((Str::lit("input"), Adt9::C5({
        let m3369 = match v0.clone() {
            Adt77::C0(_, _, h3368, _) => {
                h3368
            }
        };
        match m3369 {
            Adt8::C0(h3367) => {
                h3367
            }
        }
    })), f838(match v0 {
        Adt77::C0(_, _, _, h3370) => {
            h3370
        }
    }))))))
}

pub fn f946(v0: Adt78) -> R<Option<Adt9>> {
    let m3372 = f690(match v0.clone() {
        Adt78::C0(_, h3371, _, _, _) => {
            h3371
        }
    })?;
    match m3372 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt9::C3(match v0.clone() {
                Adt78::C0(h3373, _, _, _, _) => {
                    h3373
                }
            })), List::cons((Str::lit("content"), Adt9::C4(v1)), {
                let a3379 = f691(Str::lit("structuredContent"), match v0.clone() {
                    Adt78::C0(_, _, h3374, _, _) => {
                        h3374
                    }
                });
                let a3380 = {
                    let a3377 = f692(Str::lit("isError"), match v0.clone() {
                        Adt78::C0(_, _, _, h3375, _) => {
                            h3375
                        }
                    });
                    let a3378 = f838(match v0 {
                        Adt78::C0(_, _, _, _, h3376) => {
                            h3376
                        }
                    });
                    append_list(a3377, a3378)
                };
                append_list(a3379, a3380)
            }))))))
        }
    }
}

pub fn f947(v0: Option<Str>, v1: List<(Str, Adt68)>) -> Result<Adt85, Adt88> {
    let m3381 = v0;
    match m3381 {
        None => {
            Err::<Adt85, Adt88>(Adt88::C14)
        }
        Some(v2) => {
            let m3382 = v1.clone();
            match m3382.uncons() {
                None => {
                    Ok::<Adt85, Adt88>(Adt85::C1(v2))
                }
                Some((_, _)) => {
                    Ok::<Adt85, Adt88>(Adt85::C2(v1, v2))
                }
            }
        }
    }
}

pub fn f948(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3386 = f951(v0, {
        let a3383 = v1;
        utf8_encode(a3383)
    }, {
        let a3384 = v2;
        utf8_encode(a3384)
    }, {
        let a3385 = v3;
        utf8_encode(a3385)
    })?;
    match m3386 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3387 = v4;
            Ok(utf8_decode(a3387))
        }
    }
}

pub fn f949(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3389 = f952();
    let a3390 = {
        let a3388 = v0.clone();
        length_bytes(a3388)
    };
    if nat_le(a3389, a3390) {
        let a3392 = f953(v3.clone())?;
        let a3393 = {
            let a3391 = v3.clone();
            length_bytes(a3391)
        };
        if nat_eq(a3392, a3393) {
            Ok(None::<Bytes>)
        } else {
            f954(v0, v1, v2, v3.clone(), f953(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f950(v0: List<u64>) -> R<Option<i64>> {
    let m3394 = v0.clone();
    match m3394.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3395 = v1;
            let a3396 = 45u64;
            if nat_eq(a3395, a3396) {
                let m3397 = v2.clone();
                match m3397.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3398 = f955(v2, 0i64)?;
                        match m3398 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3399 = v5;
                                    int_neg(a3399)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f955(v0, 0i64)
            }
        }
    }
}

pub fn f951(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3401 = f952();
    let a3402 = {
        let a3400 = v0.clone();
        length_bytes(a3400)
    };
    if nat_le(a3401, a3402) {
        Ok(Some({
            let a3405 = f956(v3.clone())?;
            let a3406 = {
                let a3403 = Bytes::lit(&[0x2e]);
                let a3404 = f956(f957(v0, v1, v2, v3)?)?;
                append_bytes(a3403, a3404)
            };
            append_bytes(a3405, a3406)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f952() -> u64 {
    32u64
}

pub fn f953(v0: Bytes) -> R<u64> {
    let (_, h3407) = f958(Fn8::F959(v0.clone()), {
        let a3408 = v0.clone();
        length_bytes(a3408)
    }, (0u64, {
        let a3409 = v0;
        length_bytes(a3409)
    }))?;
    Ok(h3407)
}

pub fn f954(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3413 = {
        let a3410 = v3.clone();
        let a3411 = 0u64;
        let a3412 = v4;
        slice_bytes(a3410, a3411, a3412)
    };
    match m3413 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3424 = {
                let a3421 = v3.clone();
                let a3422 = {
                    let a3414 = v4;
                    let a3415 = 1u64;
                    nat_add(a3414, a3415)?
                };
                let a3423 = {
                    let a3419 = {
                        let a3416 = v3;
                        length_bytes(a3416)
                    };
                    let a3420 = {
                        let a3417 = v4;
                        let a3418 = 1u64;
                        nat_add(a3417, a3418)?
                    };
                    nat_sub(a3419, a3420)
                };
                slice_bytes(a3421, a3422, a3423)
            };
            match m3424 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f960(v0, v1, v2, f961(v5)?, f961(v6)?)
                }
            }
        }
    }
}

pub fn f955(p3430: List<u64>, p3431: i64) -> R<Option<i64>> {
    let m3429 = (p3430, p3431);
    {
        let mut m3429 = m3429;
        loop {
            let step = {
                let (v0, v1) = m3429;
                let m3425 = v0;
                match m3425.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3426 = f962(v2);
                        match m3426 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f345({
                                    let a3427 = v1;
                                    let a3428 = 10i64;
                                    int_mul(a3427, a3428)?
                                }, f552(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3429 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f956(v0: Bytes) -> R<Bytes> {
    let (_, h3432) = f963(Fn9::F964(v0.clone()), {
        let a3433 = v0;
        length_bytes(a3433)
    }, (0u64, f349()))?;
    Ok(h3432)
}

pub fn f957(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f965(v0, {
        let a3434 = f966(v1, v2);
        let a3435 = v3;
        append_bytes(a3434, a3435)
    })
}

pub fn f958(v0: Fn8, p3439: u64, p3440: (u64, u64)) -> R<(u64, u64)> {
    let m3438 = (p3439, p3440);
    {
        let mut m3438 = m3438;
        loop {
            let step = {
                let (v1, v2) = m3438;
                let m3436 = v1;
                if m3436 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3436 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3437: Fn8 = v0.clone();
                        c3437.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f959(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3442 = {
            let (h3441, _) = v1;
            h3441
        };
        let a3443 = 1u64;
        nat_add(a3442, a3443)?
    }, {
        let a3446 = {
            let (_, h3444) = v1;
            h3444
        };
        let a3447 = {
            let a3445 = v0.clone();
            length_bytes(a3445)
        };
        if nat_eq(a3446, a3447) {
            let a3449 = f59(v0, {
                let (h3448, _) = v1;
                h3448
            })?;
            let a3450 = 46u64;
            if nat_eq(a3449, a3450) {
                let (h3451, _) = v1;
                h3451
            } else {
                let (_, h3452) = v1;
                h3452
            }
        } else {
            let (_, h3453) = v1;
            h3453
        }
    }))
}

pub fn f960(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3454 = v3;
    match m3454 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3455 = v4;
            match m3455 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f967(f957(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f961(v0: Bytes) -> R<Option<Bytes>> {
    let a3460 = {
        let a3457 = {
            let a3456 = v0.clone();
            length_bytes(a3456)
        };
        let a3458 = 2u64;
        let a3459 = 0u64;
        nat_rem(a3457, a3458, a3459)
    };
    let a3461 = 0u64;
    if nat_eq(a3460, a3461) {
        let v1: (u64, (Bytes, u64)) = f968(Fn10::F969(v0.clone()), {
            let a3463 = {
                let a3462 = v0;
                length_bytes(a3462)
            };
            let a3464 = 2u64;
            let a3465 = 0u64;
            nat_quot(a3463, a3464, a3465)
        }, (0u64, (f349(), 0u64)))?;
        let a3468 = {
            let (_, h3466) = {
                let (_, h3467) = v1.clone();
                h3467
            };
            h3466
        };
        let a3469 = 0u64;
        if nat_eq(a3468, a3469) {
            Ok(Some({
                let (h3470, _) = {
                    let (_, h3471) = v1;
                    h3471
                };
                h3470
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f962(v0: u64) -> Option<u64> {
    let a3472 = v0;
    let a3473 = 48u64;
    if if nat_lt(a3472, a3473) {
        true
    } else {
        let a3474 = 57u64;
        let a3475 = v0;
        nat_lt(a3474, a3475)
    } {
        None::<u64>
    } else {
        Some({
            let a3476 = v0;
            let a3477 = 48u64;
            nat_sub(a3476, a3477)
        })
    }
}

pub fn f963(v0: Fn9, p3481: u64, p3482: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3480 = (p3481, p3482);
    {
        let mut m3480 = m3480;
        loop {
            let step = {
                let (v1, v2) = m3480;
                let m3478 = v1;
                if m3478 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3478 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3479: Fn9 = v0.clone();
                        c3479.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3480 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f964(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3484 = {
            let (h3483, _) = v1.clone();
            h3483
        };
        let a3485 = 1u64;
        nat_add(a3484, a3485)?
    }, {
        let a3488 = {
            let (_, h3486) = v1;
            h3486
        };
        let a3489 = f970(f59(v0, {
            let (h3487, _) = v1;
            h3487
        })?)?;
        append_bytes(a3488, a3489)
    }))
}

pub fn f965(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f971(f972(v0, v1)?)
}

pub fn f966(v0: Bytes, v1: Bytes) -> Bytes {
    let a3496 = f973({
        let a3490 = v0.clone();
        length_bytes(a3490)
    });
    let a3497 = {
        let a3494 = v0;
        let a3495 = {
            let a3492 = f973({
                let a3491 = v1.clone();
                length_bytes(a3491)
            });
            let a3493 = v1;
            append_bytes(a3492, a3493)
        };
        append_bytes(a3494, a3495)
    };
    append_bytes(a3496, a3497)
}

pub fn f967(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3500 = {
        let a3498 = v0.clone();
        length_bytes(a3498)
    };
    let a3501 = {
        let a3499 = v1.clone();
        length_bytes(a3499)
    };
    if nat_eq(a3500, a3501) {
        let a3502 = f974(v0, v1)?;
        let a3503 = 0u64;
        Ok(nat_eq(a3502, a3503))
    } else {
        Ok(false)
    }
}

pub fn f968(v0: Fn10, p3507: u64, p3508: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3506 = (p3507, p3508);
    {
        let mut m3506 = m3506;
        loop {
            let step = {
                let (v1, v2) = m3506;
                let m3504 = v1;
                if m3504 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3504 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3505: Fn10 = v0.clone();
                        c3505.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f969(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f975(v0, v1)
}

pub fn f970(v0: u64) -> R<Bytes> {
    let a3515 = f976({
        let a3509 = v0;
        let a3510 = 16u64;
        let a3511 = 0u64;
        nat_quot(a3509, a3510, a3511)
    })?;
    let a3516 = f976({
        let a3512 = v0;
        let a3513 = 16u64;
        let a3514 = 0u64;
        nat_rem(a3512, a3513, a3514)
    })?;
    Ok(append_bytes(a3515, a3516))
}

pub fn f971(v0: Adt163) -> R<Bytes> {
    let a3537 = f977(match v0.clone() {
        Adt163::C0(h3517, _, _, _, _, _, _, _) => {
            h3517
        }
    })?;
    let a3538 = {
        let a3535 = f977(match v0.clone() {
            Adt163::C0(_, h3518, _, _, _, _, _, _) => {
                h3518
            }
        })?;
        let a3536 = {
            let a3533 = f977(match v0.clone() {
                Adt163::C0(_, _, h3519, _, _, _, _, _) => {
                    h3519
                }
            })?;
            let a3534 = {
                let a3531 = f977(match v0.clone() {
                    Adt163::C0(_, _, _, h3520, _, _, _, _) => {
                        h3520
                    }
                })?;
                let a3532 = {
                    let a3529 = f977(match v0.clone() {
                        Adt163::C0(_, _, _, _, h3521, _, _, _) => {
                            h3521
                        }
                    })?;
                    let a3530 = {
                        let a3527 = f977(match v0.clone() {
                            Adt163::C0(_, _, _, _, _, h3522, _, _) => {
                                h3522
                            }
                        })?;
                        let a3528 = {
                            let a3525 = f977(match v0.clone() {
                                Adt163::C0(_, _, _, _, _, _, h3523, _) => {
                                    h3523
                                }
                            })?;
                            let a3526 = f977(match v0 {
                                Adt163::C0(_, _, _, _, _, _, _, h3524) => {
                                    h3524
                                }
                            })?;
                            append_bytes(a3525, a3526)
                        };
                        append_bytes(a3527, a3528)
                    };
                    append_bytes(a3529, a3530)
                };
                append_bytes(a3531, a3532)
            };
            append_bytes(a3533, a3534)
        };
        append_bytes(a3535, a3536)
    };
    Ok(append_bytes(a3537, a3538))
}

pub fn f972(v0: Bytes, v1: Bytes) -> R<Adt163> {
    let v2: Adt164 = f978(v0)?;
    let v3: Adt163 = f979(f980(f981(), f982(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f980(f980(f981(), f982(v2, 1549556828u32))?, f983(v3))
}

pub fn f973(v0: u64) -> Bytes {
    let a3564 = f452({
        let a3542 = {
            let a3539 = v0;
            let a3540 = 16777216u64;
            let a3541 = 0u64;
            nat_quot(a3539, a3540, a3541)
        };
        let a3543 = 256u64;
        let a3544 = 0u64;
        nat_rem(a3542, a3543, a3544)
    });
    let a3565 = {
        let a3562 = f452({
            let a3548 = {
                let a3545 = v0;
                let a3546 = 65536u64;
                let a3547 = 0u64;
                nat_quot(a3545, a3546, a3547)
            };
            let a3549 = 256u64;
            let a3550 = 0u64;
            nat_rem(a3548, a3549, a3550)
        });
        let a3563 = {
            let a3560 = f452({
                let a3554 = {
                    let a3551 = v0;
                    let a3552 = 256u64;
                    let a3553 = 0u64;
                    nat_quot(a3551, a3552, a3553)
                };
                let a3555 = 256u64;
                let a3556 = 0u64;
                nat_rem(a3554, a3555, a3556)
            });
            let a3561 = f452({
                let a3557 = v0;
                let a3558 = 256u64;
                let a3559 = 0u64;
                nat_rem(a3557, a3558, a3559)
            });
            append_bytes(a3560, a3561)
        };
        append_bytes(a3562, a3563)
    };
    append_bytes(a3564, a3565)
}

pub fn f974(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3566) = f958(Fn8::F984(v0.clone(), v1), {
        let a3567 = v0;
        length_bytes(a3567)
    }, (0u64, 0u64))?;
    Ok(h3566)
}

pub fn f975(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f985(f59(v0.clone(), {
        let (h3568, _) = v1.clone();
        h3568
    })?);
    let v3: u64 = f985(f59(v0, {
        let a3570 = {
            let (h3569, _) = v1.clone();
            h3569
        };
        let a3571 = 1u64;
        nat_add(a3570, a3571)?
    })?);
    let a3572 = v2;
    let a3573 = 16u64;
    if if nat_lt(a3572, a3573) {
        let a3574 = v3;
        let a3575 = 16u64;
        nat_lt(a3574, a3575)
    } else {
        false
    } {
        Ok(({
            let a3577 = {
                let (h3576, _) = v1.clone();
                h3576
            };
            let a3578 = 2u64;
            nat_add(a3577, a3578)?
        }, ({
            let a3585 = {
                let (h3579, _) = {
                    let (_, h3580) = v1.clone();
                    h3580
                };
                h3579
            };
            let a3586 = f452({
                let a3583 = {
                    let a3581 = v2;
                    let a3582 = 16u64;
                    nat_mul(a3581, a3582)?
                };
                let a3584 = v3;
                nat_add(a3583, a3584)?
            });
            append_bytes(a3585, a3586)
        }, {
            let (_, h3587) = {
                let (_, h3588) = v1;
                h3588
            };
            h3587
        })))
    } else {
        Ok(({
            let a3590 = {
                let (h3589, _) = v1.clone();
                h3589
            };
            let a3591 = 2u64;
            nat_add(a3590, a3591)?
        }, ({
            let (h3592, _) = {
                let (_, h3593) = v1.clone();
                h3593
            };
            h3592
        }, {
            let a3596 = {
                let (_, h3594) = {
                    let (_, h3595) = v1;
                    h3595
                };
                h3594
            };
            let a3597 = 1u64;
            nat_add(a3596, a3597)?
        })))
    }
}

pub fn f976(v0: u64) -> R<Bytes> {
    Ok(f452({
        let a3598 = v0;
        let a3599 = 10u64;
        if nat_lt(a3598, a3599) {
            let a3600 = 48u64;
            let a3601 = v0;
            nat_add(a3600, a3601)?
        } else {
            let a3602 = 87u64;
            let a3603 = v0;
            nat_add(a3602, a3603)?
        }
    }))
}

pub fn f977(v0: u32) -> R<Bytes> {
    let a3608 = f986(v0, 24u32)?;
    let a3609 = {
        let a3606 = f986(v0, 16u32)?;
        let a3607 = {
            let a3604 = f986(v0, 8u32)?;
            let a3605 = f986(v0, 0u32)?;
            append_bytes(a3604, a3605)
        };
        append_bytes(a3606, a3607)
    };
    Ok(append_bytes(a3608, a3609))
}

pub fn f978(v0: Bytes) -> R<Adt164> {
    let a3611 = f987();
    let a3612 = {
        let a3610 = v0.clone();
        length_bytes(a3610)
    };
    if nat_lt(a3611, a3612) {
        Ok(f988(f989(v0)?))
    } else {
        f990(v0)
    }
}

pub fn f979(v0: Adt163, v1: u64, v2: Bytes) -> R<Adt163> {
    let v3: u64 = {
        let a3613 = v2.clone();
        length_bytes(a3613)
    };
    let v4: u64 = {
        let a3619 = {
            let a3616 = {
                let a3614 = v3;
                let a3615 = 72u64;
                nat_add(a3614, a3615)?
            };
            let a3617 = 64u64;
            let a3618 = 0u64;
            nat_quot(a3616, a3617, a3618)
        };
        let a3620 = 64u64;
        nat_mul(a3619, a3620)?
    };
    let (_, h3621) = f991(Fn11::F992(v2, v1, v3, v4), {
        let a3622 = v4;
        let a3623 = 64u64;
        let a3624 = 0u64;
        nat_quot(a3622, a3623, a3624)
    }, (0u64, v0))?;
    Ok(h3621)
}

pub fn f980(v0: Adt163, v1: Adt164) -> R<Adt163> {
    let v2: Adt165 = f993(Fn12::F994, 64u64, Adt165::C0(match v0.clone() {
        Adt163::C0(h3625, _, _, _, _, _, _, _) => {
            h3625
        }
    }, match v0.clone() {
        Adt163::C0(_, h3626, _, _, _, _, _, _) => {
            h3626
        }
    }, match v0.clone() {
        Adt163::C0(_, _, h3627, _, _, _, _, _) => {
            h3627
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, h3628, _, _, _, _) => {
            h3628
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, _, h3629, _, _, _) => {
            h3629
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, _, _, h3630, _, _) => {
            h3630
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, _, _, _, h3631, _) => {
            h3631
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, _, _, _, _, h3632) => {
            h3632
        }
    }, v1, 0u64))?;
    Ok(Adt163::C0(f995(match v0.clone() {
        Adt163::C0(h3633, _, _, _, _, _, _, _) => {
            h3633
        }
    }, match v2.clone() {
        Adt165::C0(h3634, _, _, _, _, _, _, _, _, _) => {
            h3634
        }
    }), f995(match v0.clone() {
        Adt163::C0(_, h3635, _, _, _, _, _, _) => {
            h3635
        }
    }, match v2.clone() {
        Adt165::C0(_, h3636, _, _, _, _, _, _, _, _) => {
            h3636
        }
    }), f995(match v0.clone() {
        Adt163::C0(_, _, h3637, _, _, _, _, _) => {
            h3637
        }
    }, match v2.clone() {
        Adt165::C0(_, _, h3638, _, _, _, _, _, _, _) => {
            h3638
        }
    }), f995(match v0.clone() {
        Adt163::C0(_, _, _, h3639, _, _, _, _) => {
            h3639
        }
    }, match v2.clone() {
        Adt165::C0(_, _, _, h3640, _, _, _, _, _, _) => {
            h3640
        }
    }), f995(match v0.clone() {
        Adt163::C0(_, _, _, _, h3641, _, _, _) => {
            h3641
        }
    }, match v2.clone() {
        Adt165::C0(_, _, _, _, h3642, _, _, _, _, _) => {
            h3642
        }
    }), f995(match v0.clone() {
        Adt163::C0(_, _, _, _, _, h3643, _, _) => {
            h3643
        }
    }, match v2.clone() {
        Adt165::C0(_, _, _, _, _, h3644, _, _, _, _) => {
            h3644
        }
    }), f995(match v0.clone() {
        Adt163::C0(_, _, _, _, _, _, h3645, _) => {
            h3645
        }
    }, match v2.clone() {
        Adt165::C0(_, _, _, _, _, _, h3646, _, _, _) => {
            h3646
        }
    }), f995(match v0 {
        Adt163::C0(_, _, _, _, _, _, _, h3647) => {
            h3647
        }
    }, match v2 {
        Adt165::C0(_, _, _, _, _, _, _, h3648, _, _) => {
            h3648
        }
    })))
}

pub fn f981() -> Adt163 {
    Adt163::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f982(v0: Adt164, v1: u32) -> Adt164 {
    Adt164::C0({
        let a3650 = match v0.clone() {
            Adt164::C0(h3649, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3649
            }
        };
        let a3651 = v1;
        fixed_u32::bit_xor(a3650, a3651)
    }, {
        let a3653 = match v0.clone() {
            Adt164::C0(_, h3652, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3652
            }
        };
        let a3654 = v1;
        fixed_u32::bit_xor(a3653, a3654)
    }, {
        let a3656 = match v0.clone() {
            Adt164::C0(_, _, h3655, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3655
            }
        };
        let a3657 = v1;
        fixed_u32::bit_xor(a3656, a3657)
    }, {
        let a3659 = match v0.clone() {
            Adt164::C0(_, _, _, h3658, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3658
            }
        };
        let a3660 = v1;
        fixed_u32::bit_xor(a3659, a3660)
    }, {
        let a3662 = match v0.clone() {
            Adt164::C0(_, _, _, _, h3661, _, _, _, _, _, _, _, _, _, _, _) => {
                h3661
            }
        };
        let a3663 = v1;
        fixed_u32::bit_xor(a3662, a3663)
    }, {
        let a3665 = match v0.clone() {
            Adt164::C0(_, _, _, _, _, h3664, _, _, _, _, _, _, _, _, _, _) => {
                h3664
            }
        };
        let a3666 = v1;
        fixed_u32::bit_xor(a3665, a3666)
    }, {
        let a3668 = match v0.clone() {
            Adt164::C0(_, _, _, _, _, _, h3667, _, _, _, _, _, _, _, _, _) => {
                h3667
            }
        };
        let a3669 = v1;
        fixed_u32::bit_xor(a3668, a3669)
    }, {
        let a3671 = match v0.clone() {
            Adt164::C0(_, _, _, _, _, _, _, h3670, _, _, _, _, _, _, _, _) => {
                h3670
            }
        };
        let a3672 = v1;
        fixed_u32::bit_xor(a3671, a3672)
    }, {
        let a3674 = match v0.clone() {
            Adt164::C0(_, _, _, _, _, _, _, _, h3673, _, _, _, _, _, _, _) => {
                h3673
            }
        };
        let a3675 = v1;
        fixed_u32::bit_xor(a3674, a3675)
    }, {
        let a3677 = match v0.clone() {
            Adt164::C0(_, _, _, _, _, _, _, _, _, h3676, _, _, _, _, _, _) => {
                h3676
            }
        };
        let a3678 = v1;
        fixed_u32::bit_xor(a3677, a3678)
    }, {
        let a3680 = match v0.clone() {
            Adt164::C0(_, _, _, _, _, _, _, _, _, _, h3679, _, _, _, _, _) => {
                h3679
            }
        };
        let a3681 = v1;
        fixed_u32::bit_xor(a3680, a3681)
    }, {
        let a3683 = match v0.clone() {
            Adt164::C0(_, _, _, _, _, _, _, _, _, _, _, h3682, _, _, _, _) => {
                h3682
            }
        };
        let a3684 = v1;
        fixed_u32::bit_xor(a3683, a3684)
    }, {
        let a3686 = match v0.clone() {
            Adt164::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3685, _, _, _) => {
                h3685
            }
        };
        let a3687 = v1;
        fixed_u32::bit_xor(a3686, a3687)
    }, {
        let a3689 = match v0.clone() {
            Adt164::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3688, _, _) => {
                h3688
            }
        };
        let a3690 = v1;
        fixed_u32::bit_xor(a3689, a3690)
    }, {
        let a3692 = match v0.clone() {
            Adt164::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3691, _) => {
                h3691
            }
        };
        let a3693 = v1;
        fixed_u32::bit_xor(a3692, a3693)
    }, {
        let a3695 = match v0 {
            Adt164::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3694) => {
                h3694
            }
        };
        let a3696 = v1;
        fixed_u32::bit_xor(a3695, a3696)
    })
}

pub fn f983(v0: Adt163) -> Adt164 {
    Adt164::C0(match v0.clone() {
        Adt163::C0(h3697, _, _, _, _, _, _, _) => {
            h3697
        }
    }, match v0.clone() {
        Adt163::C0(_, h3698, _, _, _, _, _, _) => {
            h3698
        }
    }, match v0.clone() {
        Adt163::C0(_, _, h3699, _, _, _, _, _) => {
            h3699
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, h3700, _, _, _, _) => {
            h3700
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, _, h3701, _, _, _) => {
            h3701
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, _, _, h3702, _, _) => {
            h3702
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, _, _, _, h3703, _) => {
            h3703
        }
    }, match v0 {
        Adt163::C0(_, _, _, _, _, _, _, h3704) => {
            h3704
        }
    }, 2147483648u32, f996(), f996(), f996(), f996(), f996(), f996(), 768u32)
}

pub fn f984(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3706 = {
            let (h3705, _) = v2;
            h3705
        };
        let a3707 = 1u64;
        nat_add(a3706, a3707)?
    }, {
        let a3759 = {
            let (_, h3708) = v2;
            h3708
        };
        let a3760 = {
            let v3: u8 = {
                let a3711 = f997(v0, {
                    let (h3709, _) = v2;
                    h3709
                });
                let a3712 = f997(v1, {
                    let (h3710, _) = v2;
                    h3710
                });
                fixed_u8::bit_xor(a3711, a3712)
            };
            let a3757 = {
                let a3733 = {
                    let a3721 = {
                        let a3715 = {
                            let a3713 = v3;
                            let a3714 = 128u8;
                            fixed_u8::bit_and(a3713, a3714)
                        };
                        let a3716 = 128u8;
                        if equal(a3715, a3716) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a3722 = {
                        let a3719 = {
                            let a3717 = v3;
                            let a3718 = 64u8;
                            fixed_u8::bit_and(a3717, a3718)
                        };
                        let a3720 = 64u8;
                        if equal(a3719, a3720) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3721, a3722)?
                };
                let a3734 = {
                    let a3731 = {
                        let a3725 = {
                            let a3723 = v3;
                            let a3724 = 32u8;
                            fixed_u8::bit_and(a3723, a3724)
                        };
                        let a3726 = 32u8;
                        if equal(a3725, a3726) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a3732 = {
                        let a3729 = {
                            let a3727 = v3;
                            let a3728 = 16u8;
                            fixed_u8::bit_and(a3727, a3728)
                        };
                        let a3730 = 16u8;
                        if equal(a3729, a3730) {
                            16u64
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
                            let a3735 = v3;
                            let a3736 = 8u8;
                            fixed_u8::bit_and(a3735, a3736)
                        };
                        let a3738 = 8u8;
                        if equal(a3737, a3738) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a3744 = {
                        let a3741 = {
                            let a3739 = v3;
                            let a3740 = 4u8;
                            fixed_u8::bit_and(a3739, a3740)
                        };
                        let a3742 = 4u8;
                        if equal(a3741, a3742) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3743, a3744)?
                };
                let a3756 = {
                    let a3753 = {
                        let a3747 = {
                            let a3745 = v3;
                            let a3746 = 2u8;
                            fixed_u8::bit_and(a3745, a3746)
                        };
                        let a3748 = 2u8;
                        if equal(a3747, a3748) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a3754 = {
                        let a3751 = {
                            let a3749 = v3;
                            let a3750 = 1u8;
                            fixed_u8::bit_and(a3749, a3750)
                        };
                        let a3752 = 1u8;
                        if equal(a3751, a3752) {
                            1u64
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
        nat_add(a3759, a3760)?
    }))
}

pub fn f985(v0: u64) -> u64 {
    let a3761 = v0;
    let a3762 = 48u64;
    if nat_lt(a3761, a3762) {
        16u64
    } else {
        let a3763 = v0;
        let a3764 = 58u64;
        if nat_lt(a3763, a3764) {
            let a3765 = v0;
            let a3766 = 48u64;
            nat_sub(a3765, a3766)
        } else {
            let a3767 = v0;
            let a3768 = 97u64;
            if nat_lt(a3767, a3768) {
                16u64
            } else {
                let a3769 = v0;
                let a3770 = 103u64;
                if nat_lt(a3769, a3770) {
                    let a3771 = v0;
                    let a3772 = 87u64;
                    nat_sub(a3771, a3772)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f986(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f452({
        let v2: u32 = {
            let a3773 = f998(v0, v1);
            let a3774 = 255u32;
            fixed_u32::bit_and(a3773, a3774)
        };
        let a3963 = {
            let a3867 = {
                let a3819 = {
                    let a3795 = {
                        let a3783 = {
                            let a3777 = {
                                let a3775 = v2;
                                let a3776 = 2147483648u32;
                                fixed_u32::bit_and(a3775, a3776)
                            };
                            let a3778 = 2147483648u32;
                            if equal(a3777, a3778) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a3784 = {
                            let a3781 = {
                                let a3779 = v2;
                                let a3780 = 1073741824u32;
                                fixed_u32::bit_and(a3779, a3780)
                            };
                            let a3782 = 1073741824u32;
                            if equal(a3781, a3782) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3783, a3784)?
                    };
                    let a3796 = {
                        let a3793 = {
                            let a3787 = {
                                let a3785 = v2;
                                let a3786 = 536870912u32;
                                fixed_u32::bit_and(a3785, a3786)
                            };
                            let a3788 = 536870912u32;
                            if equal(a3787, a3788) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a3794 = {
                            let a3791 = {
                                let a3789 = v2;
                                let a3790 = 268435456u32;
                                fixed_u32::bit_and(a3789, a3790)
                            };
                            let a3792 = 268435456u32;
                            if equal(a3791, a3792) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3793, a3794)?
                    };
                    nat_add(a3795, a3796)?
                };
                let a3820 = {
                    let a3817 = {
                        let a3805 = {
                            let a3799 = {
                                let a3797 = v2;
                                let a3798 = 134217728u32;
                                fixed_u32::bit_and(a3797, a3798)
                            };
                            let a3800 = 134217728u32;
                            if equal(a3799, a3800) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a3806 = {
                            let a3803 = {
                                let a3801 = v2;
                                let a3802 = 67108864u32;
                                fixed_u32::bit_and(a3801, a3802)
                            };
                            let a3804 = 67108864u32;
                            if equal(a3803, a3804) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3805, a3806)?
                    };
                    let a3818 = {
                        let a3815 = {
                            let a3809 = {
                                let a3807 = v2;
                                let a3808 = 33554432u32;
                                fixed_u32::bit_and(a3807, a3808)
                            };
                            let a3810 = 33554432u32;
                            if equal(a3809, a3810) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a3816 = {
                            let a3813 = {
                                let a3811 = v2;
                                let a3812 = 16777216u32;
                                fixed_u32::bit_and(a3811, a3812)
                            };
                            let a3814 = 16777216u32;
                            if equal(a3813, a3814) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3815, a3816)?
                    };
                    nat_add(a3817, a3818)?
                };
                nat_add(a3819, a3820)?
            };
            let a3868 = {
                let a3865 = {
                    let a3841 = {
                        let a3829 = {
                            let a3823 = {
                                let a3821 = v2;
                                let a3822 = 8388608u32;
                                fixed_u32::bit_and(a3821, a3822)
                            };
                            let a3824 = 8388608u32;
                            if equal(a3823, a3824) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a3830 = {
                            let a3827 = {
                                let a3825 = v2;
                                let a3826 = 4194304u32;
                                fixed_u32::bit_and(a3825, a3826)
                            };
                            let a3828 = 4194304u32;
                            if equal(a3827, a3828) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3829, a3830)?
                    };
                    let a3842 = {
                        let a3839 = {
                            let a3833 = {
                                let a3831 = v2;
                                let a3832 = 2097152u32;
                                fixed_u32::bit_and(a3831, a3832)
                            };
                            let a3834 = 2097152u32;
                            if equal(a3833, a3834) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a3840 = {
                            let a3837 = {
                                let a3835 = v2;
                                let a3836 = 1048576u32;
                                fixed_u32::bit_and(a3835, a3836)
                            };
                            let a3838 = 1048576u32;
                            if equal(a3837, a3838) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3839, a3840)?
                    };
                    nat_add(a3841, a3842)?
                };
                let a3866 = {
                    let a3863 = {
                        let a3851 = {
                            let a3845 = {
                                let a3843 = v2;
                                let a3844 = 524288u32;
                                fixed_u32::bit_and(a3843, a3844)
                            };
                            let a3846 = 524288u32;
                            if equal(a3845, a3846) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a3852 = {
                            let a3849 = {
                                let a3847 = v2;
                                let a3848 = 262144u32;
                                fixed_u32::bit_and(a3847, a3848)
                            };
                            let a3850 = 262144u32;
                            if equal(a3849, a3850) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3851, a3852)?
                    };
                    let a3864 = {
                        let a3861 = {
                            let a3855 = {
                                let a3853 = v2;
                                let a3854 = 131072u32;
                                fixed_u32::bit_and(a3853, a3854)
                            };
                            let a3856 = 131072u32;
                            if equal(a3855, a3856) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a3862 = {
                            let a3859 = {
                                let a3857 = v2;
                                let a3858 = 65536u32;
                                fixed_u32::bit_and(a3857, a3858)
                            };
                            let a3860 = 65536u32;
                            if equal(a3859, a3860) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3861, a3862)?
                    };
                    nat_add(a3863, a3864)?
                };
                nat_add(a3865, a3866)?
            };
            nat_add(a3867, a3868)?
        };
        let a3964 = {
            let a3961 = {
                let a3913 = {
                    let a3889 = {
                        let a3877 = {
                            let a3871 = {
                                let a3869 = v2;
                                let a3870 = 32768u32;
                                fixed_u32::bit_and(a3869, a3870)
                            };
                            let a3872 = 32768u32;
                            if equal(a3871, a3872) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a3878 = {
                            let a3875 = {
                                let a3873 = v2;
                                let a3874 = 16384u32;
                                fixed_u32::bit_and(a3873, a3874)
                            };
                            let a3876 = 16384u32;
                            if equal(a3875, a3876) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3877, a3878)?
                    };
                    let a3890 = {
                        let a3887 = {
                            let a3881 = {
                                let a3879 = v2;
                                let a3880 = 8192u32;
                                fixed_u32::bit_and(a3879, a3880)
                            };
                            let a3882 = 8192u32;
                            if equal(a3881, a3882) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a3888 = {
                            let a3885 = {
                                let a3883 = v2;
                                let a3884 = 4096u32;
                                fixed_u32::bit_and(a3883, a3884)
                            };
                            let a3886 = 4096u32;
                            if equal(a3885, a3886) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3887, a3888)?
                    };
                    nat_add(a3889, a3890)?
                };
                let a3914 = {
                    let a3911 = {
                        let a3899 = {
                            let a3893 = {
                                let a3891 = v2;
                                let a3892 = 2048u32;
                                fixed_u32::bit_and(a3891, a3892)
                            };
                            let a3894 = 2048u32;
                            if equal(a3893, a3894) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a3900 = {
                            let a3897 = {
                                let a3895 = v2;
                                let a3896 = 1024u32;
                                fixed_u32::bit_and(a3895, a3896)
                            };
                            let a3898 = 1024u32;
                            if equal(a3897, a3898) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3899, a3900)?
                    };
                    let a3912 = {
                        let a3909 = {
                            let a3903 = {
                                let a3901 = v2;
                                let a3902 = 512u32;
                                fixed_u32::bit_and(a3901, a3902)
                            };
                            let a3904 = 512u32;
                            if equal(a3903, a3904) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a3910 = {
                            let a3907 = {
                                let a3905 = v2;
                                let a3906 = 256u32;
                                fixed_u32::bit_and(a3905, a3906)
                            };
                            let a3908 = 256u32;
                            if equal(a3907, a3908) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3909, a3910)?
                    };
                    nat_add(a3911, a3912)?
                };
                nat_add(a3913, a3914)?
            };
            let a3962 = {
                let a3959 = {
                    let a3935 = {
                        let a3923 = {
                            let a3917 = {
                                let a3915 = v2;
                                let a3916 = 128u32;
                                fixed_u32::bit_and(a3915, a3916)
                            };
                            let a3918 = 128u32;
                            if equal(a3917, a3918) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3924 = {
                            let a3921 = {
                                let a3919 = v2;
                                let a3920 = 64u32;
                                fixed_u32::bit_and(a3919, a3920)
                            };
                            let a3922 = 64u32;
                            if equal(a3921, a3922) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3923, a3924)?
                    };
                    let a3936 = {
                        let a3933 = {
                            let a3927 = {
                                let a3925 = v2;
                                let a3926 = 32u32;
                                fixed_u32::bit_and(a3925, a3926)
                            };
                            let a3928 = 32u32;
                            if equal(a3927, a3928) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3934 = {
                            let a3931 = {
                                let a3929 = v2;
                                let a3930 = 16u32;
                                fixed_u32::bit_and(a3929, a3930)
                            };
                            let a3932 = 16u32;
                            if equal(a3931, a3932) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3933, a3934)?
                    };
                    nat_add(a3935, a3936)?
                };
                let a3960 = {
                    let a3957 = {
                        let a3945 = {
                            let a3939 = {
                                let a3937 = v2;
                                let a3938 = 8u32;
                                fixed_u32::bit_and(a3937, a3938)
                            };
                            let a3940 = 8u32;
                            if equal(a3939, a3940) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3946 = {
                            let a3943 = {
                                let a3941 = v2;
                                let a3942 = 4u32;
                                fixed_u32::bit_and(a3941, a3942)
                            };
                            let a3944 = 4u32;
                            if equal(a3943, a3944) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3945, a3946)?
                    };
                    let a3958 = {
                        let a3955 = {
                            let a3949 = {
                                let a3947 = v2;
                                let a3948 = 2u32;
                                fixed_u32::bit_and(a3947, a3948)
                            };
                            let a3950 = 2u32;
                            if equal(a3949, a3950) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3956 = {
                            let a3953 = {
                                let a3951 = v2;
                                let a3952 = 1u32;
                                fixed_u32::bit_and(a3951, a3952)
                            };
                            let a3954 = 1u32;
                            if equal(a3953, a3954) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3955, a3956)?
                    };
                    nat_add(a3957, a3958)?
                };
                nat_add(a3959, a3960)?
            };
            nat_add(a3961, a3962)?
        };
        nat_add(a3963, a3964)?
    }))
}

pub fn f987() -> u64 {
    64u64
}

pub fn f988(v0: Adt163) -> Adt164 {
    Adt164::C0(match v0.clone() {
        Adt163::C0(h3965, _, _, _, _, _, _, _) => {
            h3965
        }
    }, match v0.clone() {
        Adt163::C0(_, h3966, _, _, _, _, _, _) => {
            h3966
        }
    }, match v0.clone() {
        Adt163::C0(_, _, h3967, _, _, _, _, _) => {
            h3967
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, h3968, _, _, _, _) => {
            h3968
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, _, h3969, _, _, _) => {
            h3969
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, _, _, h3970, _, _) => {
            h3970
        }
    }, match v0.clone() {
        Adt163::C0(_, _, _, _, _, _, h3971, _) => {
            h3971
        }
    }, match v0 {
        Adt163::C0(_, _, _, _, _, _, _, h3972) => {
            h3972
        }
    }, f996(), f996(), f996(), f996(), f996(), f996(), f996(), f996())
}

pub fn f989(v0: Bytes) -> R<Adt163> {
    let v1: u64 = {
        let a3973 = v0.clone();
        length_bytes(a3973)
    };
    let v2: u64 = {
        let a3979 = {
            let a3976 = {
                let a3974 = v1;
                let a3975 = 72u64;
                nat_add(a3974, a3975)?
            };
            let a3977 = 64u64;
            let a3978 = 0u64;
            nat_quot(a3976, a3977, a3978)
        };
        let a3980 = 64u64;
        nat_mul(a3979, a3980)?
    };
    let (_, h3981) = f991(Fn11::F999(v0, v1, v2), {
        let a3982 = v2;
        let a3983 = 64u64;
        let a3984 = 0u64;
        nat_quot(a3982, a3983, a3984)
    }, (0u64, f981()))?;
    Ok(h3981)
}

pub fn f990(v0: Bytes) -> R<Adt164> {
    Ok(Adt164::C0(f1000(v0.clone(), 0u64)?, f1000(v0.clone(), 4u64)?, f1000(v0.clone(), 8u64)?, f1000(v0.clone(), 12u64)?, f1000(v0.clone(), 16u64)?, f1000(v0.clone(), 20u64)?, f1000(v0.clone(), 24u64)?, f1000(v0.clone(), 28u64)?, f1000(v0.clone(), 32u64)?, f1000(v0.clone(), 36u64)?, f1000(v0.clone(), 40u64)?, f1000(v0.clone(), 44u64)?, f1000(v0.clone(), 48u64)?, f1000(v0.clone(), 52u64)?, f1000(v0.clone(), 56u64)?, f1000(v0, 60u64)?))
}

pub fn f991(v0: Fn11, p3988: u64, p3989: (u64, Adt163)) -> R<(u64, Adt163)> {
    let m3987 = (p3988, p3989);
    {
        let mut m3987 = m3987;
        loop {
            let step = {
                let (v1, v2) = m3987;
                let m3985 = v1;
                if m3985 == 0 {
                    Err::<(u64, (u64, Adt163)), R<(u64, Adt163)>>(Ok(v2))
                } else {
                    let v3 = m3985 - 1;
                    Ok::<(u64, (u64, Adt163)), R<(u64, Adt163)>>((v3, {
                        let c3986: Fn11 = v0.clone();
                        c3986.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3987 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f992(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt163)) -> R<(u64, Adt163)> {
    Ok(({
        let a3991 = {
            let (h3990, _) = v4.clone();
            h3990
        };
        let a3992 = 64u64;
        nat_add(a3991, a3992)?
    }, f980({
        let (_, h3993) = v4;
        h3993
    }, f1001(v0, v2, v3, {
        let a3996 = {
            let a3994 = v1;
            let a3995 = v2;
            nat_add(a3994, a3995)?
        };
        let a3997 = 8u64;
        nat_mul(a3996, a3997)?
    }, {
        let (h3998, _) = v4;
        h3998
    })?)?))
}

pub fn f993(v0: Fn12, p4002: u64, p4003: Adt165) -> R<Adt165> {
    let m4001 = (p4002, p4003);
    {
        let mut m4001 = m4001;
        loop {
            let step = {
                let (v1, v2) = m4001;
                let m3999 = v1;
                if m3999 == 0 {
                    Err::<(u64, Adt165), R<Adt165>>(Ok(v2))
                } else {
                    let v3 = m3999 - 1;
                    Ok::<(u64, Adt165), R<Adt165>>((v3, {
                        let c4000: Fn12 = v0.clone();
                        c4000.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4001 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f994(v0: Adt165) -> R<Adt165> {
    f1002(v0)
}

pub fn f995(v0: u32, v1: u32) -> u32 {
    let m4006 = {
        let a4004 = f1003(v0);
        let a4005 = f1003(v1);
        fixed_u64::checked_add(a4004, a4005)
    };
    match m4006 {
        None => {
            0u32
        }
        Some(v2) => {
            f1004(v2)
        }
    }
}

pub fn f996() -> u32 {
    0u32
}

pub fn f997(v0: Bytes, v1: u64) -> u8 {
    let m4009 = {
        let a4007 = v0;
        let a4008 = v1;
        index_bytes(a4007, a4008)
    };
    match m4009 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f998(v0: u32, v1: u32) -> u32 {
    let m4012 = {
        let a4010 = v0;
        let a4011 = v1;
        fixed_u32::shift_right(a4010, a4011)
    };
    match m4012 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f999(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt163)) -> R<(u64, Adt163)> {
    Ok(({
        let a4014 = {
            let (h4013, _) = v3.clone();
            h4013
        };
        let a4015 = 64u64;
        nat_add(a4014, a4015)?
    }, f980({
        let (_, h4016) = v3;
        h4016
    }, f1005(v0, v1, v2, {
        let (h4017, _) = v3;
        h4017
    })?)?))
}

pub fn f1000(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f1006(f1007(v0.clone(), v1), f1007(v0.clone(), {
        let a4018 = v1;
        let a4019 = 1u64;
        nat_add(a4018, a4019)?
    }), f1007(v0.clone(), {
        let a4020 = v1;
        let a4021 = 2u64;
        nat_add(a4020, a4021)?
    }), f1007(v0, {
        let a4022 = v1;
        let a4023 = 3u64;
        nat_add(a4022, a4023)?
    })))
}

pub fn f1001(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt164> {
    Ok(Adt164::C0(f1008(v0.clone(), v1, v2, v3, v4)?, f1008(v0.clone(), v1, v2, v3, {
        let a4024 = v4;
        let a4025 = 4u64;
        nat_add(a4024, a4025)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4026 = v4;
        let a4027 = 8u64;
        nat_add(a4026, a4027)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4028 = v4;
        let a4029 = 12u64;
        nat_add(a4028, a4029)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4030 = v4;
        let a4031 = 16u64;
        nat_add(a4030, a4031)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4032 = v4;
        let a4033 = 20u64;
        nat_add(a4032, a4033)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4034 = v4;
        let a4035 = 24u64;
        nat_add(a4034, a4035)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4036 = v4;
        let a4037 = 28u64;
        nat_add(a4036, a4037)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4038 = v4;
        let a4039 = 32u64;
        nat_add(a4038, a4039)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4040 = v4;
        let a4041 = 36u64;
        nat_add(a4040, a4041)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4042 = v4;
        let a4043 = 40u64;
        nat_add(a4042, a4043)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4044 = v4;
        let a4045 = 44u64;
        nat_add(a4044, a4045)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4046 = v4;
        let a4047 = 48u64;
        nat_add(a4046, a4047)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4048 = v4;
        let a4049 = 52u64;
        nat_add(a4048, a4049)?
    })?, f1008(v0.clone(), v1, v2, v3, {
        let a4050 = v4;
        let a4051 = 56u64;
        nat_add(a4050, a4051)?
    })?, f1008(v0, v1, v2, v3, {
        let a4052 = v4;
        let a4053 = 60u64;
        nat_add(a4052, a4053)?
    })?))
}

pub fn f1002(v0: Adt165) -> R<Adt165> {
    let v1: u32 = f1009(match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, _, _, _, h4054) => {
            h4054
        }
    });
    let v2: u32 = f995(match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, _, h4055, _, _) => {
            h4055
        }
    }, f995(f1010(match v0.clone() {
        Adt165::C0(_, _, _, _, h4056, _, _, _, _, _) => {
            h4056
        }
    }), f995(f1011(match v0.clone() {
        Adt165::C0(_, _, _, _, h4057, _, _, _, _, _) => {
            h4057
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, h4058, _, _, _, _) => {
            h4058
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, h4059, _, _, _) => {
            h4059
        }
    }), f995(v1, {
        let m4062 = match v0.clone() {
            Adt165::C0(_, _, _, _, _, _, _, _, h4061, _) => {
                h4061
            }
        };
        match m4062 {
            Adt164::C0(h4060, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4060
            }
        }
    }))));
    let v3: u32 = f995(f1012(match v0.clone() {
        Adt165::C0(h4063, _, _, _, _, _, _, _, _, _) => {
            h4063
        }
    }), f1013(match v0.clone() {
        Adt165::C0(h4064, _, _, _, _, _, _, _, _, _) => {
            h4064
        }
    }, match v0.clone() {
        Adt165::C0(_, h4065, _, _, _, _, _, _, _, _) => {
            h4065
        }
    }, match v0.clone() {
        Adt165::C0(_, _, h4066, _, _, _, _, _, _, _) => {
            h4066
        }
    }));
    Ok(Adt165::C0(f995(v2, v3), match v0.clone() {
        Adt165::C0(h4067, _, _, _, _, _, _, _, _, _) => {
            h4067
        }
    }, match v0.clone() {
        Adt165::C0(_, h4068, _, _, _, _, _, _, _, _) => {
            h4068
        }
    }, match v0.clone() {
        Adt165::C0(_, _, h4069, _, _, _, _, _, _, _) => {
            h4069
        }
    }, f995(match v0.clone() {
        Adt165::C0(_, _, _, h4070, _, _, _, _, _, _) => {
            h4070
        }
    }, v2), match v0.clone() {
        Adt165::C0(_, _, _, _, h4071, _, _, _, _, _) => {
            h4071
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, h4072, _, _, _, _) => {
            h4072
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, h4073, _, _, _) => {
            h4073
        }
    }, f1014(match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, _, _, h4074, _) => {
            h4074
        }
    }), {
        let a4076 = match v0 {
            Adt165::C0(_, _, _, _, _, _, _, _, _, h4075) => {
                h4075
            }
        };
        let a4077 = 1u64;
        nat_add(a4076, a4077)?
    }))
}

pub fn f1003(v0: u32) -> u64 {
    let m4079 = {
        let a4078 = v0;
        fixed_u64::convert(i128::from(a4078))
    };
    match m4079 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1004(v0: u64) -> u32 {
    let m4083 = {
        let a4082 = {
            let a4080 = v0;
            let a4081 = 4294967295u64;
            fixed_u64::bit_and(a4080, a4081)
        };
        fixed_u32::convert(i128::from(a4082))
    };
    match m4083 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1005(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt164> {
    Ok(Adt164::C0(f1015(v0.clone(), v1, v2, v3)?, f1015(v0.clone(), v1, v2, {
        let a4084 = v3;
        let a4085 = 4u64;
        nat_add(a4084, a4085)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4086 = v3;
        let a4087 = 8u64;
        nat_add(a4086, a4087)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4088 = v3;
        let a4089 = 12u64;
        nat_add(a4088, a4089)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4090 = v3;
        let a4091 = 16u64;
        nat_add(a4090, a4091)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4092 = v3;
        let a4093 = 20u64;
        nat_add(a4092, a4093)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4094 = v3;
        let a4095 = 24u64;
        nat_add(a4094, a4095)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4096 = v3;
        let a4097 = 28u64;
        nat_add(a4096, a4097)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4098 = v3;
        let a4099 = 32u64;
        nat_add(a4098, a4099)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4100 = v3;
        let a4101 = 36u64;
        nat_add(a4100, a4101)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4102 = v3;
        let a4103 = 40u64;
        nat_add(a4102, a4103)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4104 = v3;
        let a4105 = 44u64;
        nat_add(a4104, a4105)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4106 = v3;
        let a4107 = 48u64;
        nat_add(a4106, a4107)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4108 = v3;
        let a4109 = 52u64;
        nat_add(a4108, a4109)?
    })?, f1015(v0.clone(), v1, v2, {
        let a4110 = v3;
        let a4111 = 56u64;
        nat_add(a4110, a4111)?
    })?, f1015(v0, v1, v2, {
        let a4112 = v3;
        let a4113 = 60u64;
        nat_add(a4112, a4113)?
    })?))
}

pub fn f1006(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a4118 = f1016(v0, 24u32);
    let a4119 = {
        let a4116 = f1016(v1, 16u32);
        let a4117 = {
            let a4114 = f1016(v2, 8u32);
            let a4115 = v3;
            fixed_u32::bit_or(a4114, a4115)
        };
        fixed_u32::bit_or(a4116, a4117)
    };
    fixed_u32::bit_or(a4118, a4119)
}

pub fn f1007(v0: Bytes, v1: u64) -> u32 {
    let m4122 = {
        let a4120 = v0;
        let a4121 = v1;
        index_bytes(a4120, a4121)
    };
    match m4122 {
        None => {
            0u32
        }
        Some(v2) => {
            let m4124 = {
                let a4123 = v2;
                fixed_u32::convert(i128::from(a4123))
            };
            match m4124 {
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

pub fn f1008(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a4127 = {
        let a4125 = v4;
        let a4126 = 8u64;
        nat_add(a4125, a4126)?
    };
    let a4128 = v2;
    if nat_eq(a4127, a4128) {
        Ok(f1017(v3, true))
    } else {
        let a4131 = {
            let a4129 = v4;
            let a4130 = 4u64;
            nat_add(a4129, a4130)?
        };
        let a4132 = v2;
        if nat_eq(a4131, a4132) {
            Ok(f1017(v3, false))
        } else {
            Ok(f1006(f1018(v0.clone(), v1, v4), f1018(v0.clone(), v1, {
                let a4133 = v4;
                let a4134 = 1u64;
                nat_add(a4133, a4134)?
            }), f1018(v0.clone(), v1, {
                let a4135 = v4;
                let a4136 = 2u64;
                nat_add(a4135, a4136)?
            }), f1018(v0, v1, {
                let a4137 = v4;
                let a4138 = 3u64;
                nat_add(a4137, a4138)?
            })))
        }
    }
}

pub fn f1009(v0: u64) -> u32 {
    let a4139 = v0;
    let a4140 = 32u64;
    if nat_lt(a4139, a4140) {
        let a4141 = v0;
        let a4142 = 16u64;
        if nat_lt(a4141, a4142) {
            let a4143 = v0;
            let a4144 = 8u64;
            if nat_lt(a4143, a4144) {
                let a4145 = v0;
                let a4146 = 4u64;
                if nat_lt(a4145, a4146) {
                    let a4147 = v0;
                    let a4148 = 2u64;
                    if nat_lt(a4147, a4148) {
                        let a4149 = v0;
                        let a4150 = 1u64;
                        if nat_lt(a4149, a4150) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a4151 = v0;
                        let a4152 = 3u64;
                        if nat_lt(a4151, a4152) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a4153 = v0;
                    let a4154 = 6u64;
                    if nat_lt(a4153, a4154) {
                        let a4155 = v0;
                        let a4156 = 5u64;
                        if nat_lt(a4155, a4156) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4157 = v0;
                        let a4158 = 7u64;
                        if nat_lt(a4157, a4158) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4159 = v0;
                let a4160 = 12u64;
                if nat_lt(a4159, a4160) {
                    let a4161 = v0;
                    let a4162 = 10u64;
                    if nat_lt(a4161, a4162) {
                        let a4163 = v0;
                        let a4164 = 9u64;
                        if nat_lt(a4163, a4164) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4165 = v0;
                        let a4166 = 11u64;
                        if nat_lt(a4165, a4166) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4167 = v0;
                    let a4168 = 14u64;
                    if nat_lt(a4167, a4168) {
                        let a4169 = v0;
                        let a4170 = 13u64;
                        if nat_lt(a4169, a4170) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4171 = v0;
                        let a4172 = 15u64;
                        if nat_lt(a4171, a4172) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4173 = v0;
            let a4174 = 24u64;
            if nat_lt(a4173, a4174) {
                let a4175 = v0;
                let a4176 = 20u64;
                if nat_lt(a4175, a4176) {
                    let a4177 = v0;
                    let a4178 = 18u64;
                    if nat_lt(a4177, a4178) {
                        let a4179 = v0;
                        let a4180 = 17u64;
                        if nat_lt(a4179, a4180) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4181 = v0;
                        let a4182 = 19u64;
                        if nat_lt(a4181, a4182) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4183 = v0;
                    let a4184 = 22u64;
                    if nat_lt(a4183, a4184) {
                        let a4185 = v0;
                        let a4186 = 21u64;
                        if nat_lt(a4185, a4186) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4187 = v0;
                        let a4188 = 23u64;
                        if nat_lt(a4187, a4188) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4189 = v0;
                let a4190 = 28u64;
                if nat_lt(a4189, a4190) {
                    let a4191 = v0;
                    let a4192 = 26u64;
                    if nat_lt(a4191, a4192) {
                        let a4193 = v0;
                        let a4194 = 25u64;
                        if nat_lt(a4193, a4194) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4195 = v0;
                        let a4196 = 27u64;
                        if nat_lt(a4195, a4196) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4197 = v0;
                    let a4198 = 30u64;
                    if nat_lt(a4197, a4198) {
                        let a4199 = v0;
                        let a4200 = 29u64;
                        if nat_lt(a4199, a4200) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4201 = v0;
                        let a4202 = 31u64;
                        if nat_lt(a4201, a4202) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4203 = v0;
        let a4204 = 48u64;
        if nat_lt(a4203, a4204) {
            let a4205 = v0;
            let a4206 = 40u64;
            if nat_lt(a4205, a4206) {
                let a4207 = v0;
                let a4208 = 36u64;
                if nat_lt(a4207, a4208) {
                    let a4209 = v0;
                    let a4210 = 34u64;
                    if nat_lt(a4209, a4210) {
                        let a4211 = v0;
                        let a4212 = 33u64;
                        if nat_lt(a4211, a4212) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4213 = v0;
                        let a4214 = 35u64;
                        if nat_lt(a4213, a4214) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4215 = v0;
                    let a4216 = 38u64;
                    if nat_lt(a4215, a4216) {
                        let a4217 = v0;
                        let a4218 = 37u64;
                        if nat_lt(a4217, a4218) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4219 = v0;
                        let a4220 = 39u64;
                        if nat_lt(a4219, a4220) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4221 = v0;
                let a4222 = 44u64;
                if nat_lt(a4221, a4222) {
                    let a4223 = v0;
                    let a4224 = 42u64;
                    if nat_lt(a4223, a4224) {
                        let a4225 = v0;
                        let a4226 = 41u64;
                        if nat_lt(a4225, a4226) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4227 = v0;
                        let a4228 = 43u64;
                        if nat_lt(a4227, a4228) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4229 = v0;
                    let a4230 = 46u64;
                    if nat_lt(a4229, a4230) {
                        let a4231 = v0;
                        let a4232 = 45u64;
                        if nat_lt(a4231, a4232) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4233 = v0;
                        let a4234 = 47u64;
                        if nat_lt(a4233, a4234) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4235 = v0;
            let a4236 = 56u64;
            if nat_lt(a4235, a4236) {
                let a4237 = v0;
                let a4238 = 52u64;
                if nat_lt(a4237, a4238) {
                    let a4239 = v0;
                    let a4240 = 50u64;
                    if nat_lt(a4239, a4240) {
                        let a4241 = v0;
                        let a4242 = 49u64;
                        if nat_lt(a4241, a4242) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4243 = v0;
                        let a4244 = 51u64;
                        if nat_lt(a4243, a4244) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4245 = v0;
                    let a4246 = 54u64;
                    if nat_lt(a4245, a4246) {
                        let a4247 = v0;
                        let a4248 = 53u64;
                        if nat_lt(a4247, a4248) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4249 = v0;
                        let a4250 = 55u64;
                        if nat_lt(a4249, a4250) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4251 = v0;
                let a4252 = 60u64;
                if nat_lt(a4251, a4252) {
                    let a4253 = v0;
                    let a4254 = 58u64;
                    if nat_lt(a4253, a4254) {
                        let a4255 = v0;
                        let a4256 = 57u64;
                        if nat_lt(a4255, a4256) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4257 = v0;
                        let a4258 = 59u64;
                        if nat_lt(a4257, a4258) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4259 = v0;
                    let a4260 = 62u64;
                    if nat_lt(a4259, a4260) {
                        let a4261 = v0;
                        let a4262 = 61u64;
                        if nat_lt(a4261, a4262) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4263 = v0;
                        let a4264 = 63u64;
                        if nat_lt(a4263, a4264) {
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

pub fn f1010(v0: u32) -> u32 {
    let a4267 = {
        let a4265 = f1019(v0, 6u32, 26u32);
        let a4266 = f1019(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4265, a4266)
    };
    let a4268 = f1019(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4267, a4268)
}

pub fn f1011(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4274 = {
        let a4269 = v0;
        let a4270 = v1;
        fixed_u32::bit_and(a4269, a4270)
    };
    let a4275 = {
        let a4272 = {
            let a4271 = v0;
            fixed_u32::bit_not(a4271)
        };
        let a4273 = v2;
        fixed_u32::bit_and(a4272, a4273)
    };
    fixed_u32::bit_xor(a4274, a4275)
}

pub fn f1012(v0: u32) -> u32 {
    let a4278 = {
        let a4276 = f1019(v0, 2u32, 30u32);
        let a4277 = f1019(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4276, a4277)
    };
    let a4279 = f1019(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4278, a4279)
}

pub fn f1013(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4288 = {
        let a4284 = {
            let a4280 = v0;
            let a4281 = v1;
            fixed_u32::bit_and(a4280, a4281)
        };
        let a4285 = {
            let a4282 = v0;
            let a4283 = v2;
            fixed_u32::bit_and(a4282, a4283)
        };
        fixed_u32::bit_xor(a4284, a4285)
    };
    let a4289 = {
        let a4286 = v1;
        let a4287 = v2;
        fixed_u32::bit_and(a4286, a4287)
    };
    fixed_u32::bit_xor(a4288, a4289)
}

pub fn f1014(v0: Adt164) -> Adt164 {
    Adt164::C0(match v0.clone() {
        Adt164::C0(_, h4290, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4290
        }
    }, match v0.clone() {
        Adt164::C0(_, _, h4291, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4291
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, h4292, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4292
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, _, h4293, _, _, _, _, _, _, _, _, _, _, _) => {
            h4293
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, _, _, h4294, _, _, _, _, _, _, _, _, _, _) => {
            h4294
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, _, _, _, h4295, _, _, _, _, _, _, _, _, _) => {
            h4295
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, _, _, _, _, h4296, _, _, _, _, _, _, _, _) => {
            h4296
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, _, _, _, _, _, h4297, _, _, _, _, _, _, _) => {
            h4297
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, _, _, _, _, _, _, h4298, _, _, _, _, _, _) => {
            h4298
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, _, _, _, _, _, _, _, h4299, _, _, _, _, _) => {
            h4299
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, _, _, _, _, _, _, _, _, h4300, _, _, _, _) => {
            h4300
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4301, _, _, _) => {
            h4301
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4302, _, _) => {
            h4302
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4303, _) => {
            h4303
        }
    }, match v0.clone() {
        Adt164::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4304) => {
            h4304
        }
    }, f995(f1020(match v0.clone() {
        Adt164::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4305, _) => {
            h4305
        }
    }), f995(match v0.clone() {
        Adt164::C0(_, _, _, _, _, _, _, _, _, h4306, _, _, _, _, _, _) => {
            h4306
        }
    }, f995(f1021(match v0.clone() {
        Adt164::C0(_, h4307, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4307
        }
    }), match v0 {
        Adt164::C0(h4308, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4308
        }
    }))))
}

pub fn f1015(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4311 = {
        let a4309 = v3;
        let a4310 = 8u64;
        nat_add(a4309, a4310)?
    };
    let a4312 = v2;
    if nat_eq(a4311, a4312) {
        Ok(f1022({
            let a4315 = {
                let a4313 = v1;
                let a4314 = 8u64;
                nat_mul(a4313, a4314)?
            };
            let a4316 = 4294967296u64;
            let a4317 = 0u64;
            nat_quot(a4315, a4316, a4317)
        }))
    } else {
        let a4320 = {
            let a4318 = v3;
            let a4319 = 4u64;
            nat_add(a4318, a4319)?
        };
        let a4321 = v2;
        if nat_eq(a4320, a4321) {
            Ok(f1022({
                let a4324 = {
                    let a4322 = v1;
                    let a4323 = 8u64;
                    nat_mul(a4322, a4323)?
                };
                let a4325 = 4294967296u64;
                let a4326 = 0u64;
                nat_rem(a4324, a4325, a4326)
            }))
        } else {
            Ok(f1006(f1018(v0.clone(), v1, v3), f1018(v0.clone(), v1, {
                let a4327 = v3;
                let a4328 = 1u64;
                nat_add(a4327, a4328)?
            }), f1018(v0.clone(), v1, {
                let a4329 = v3;
                let a4330 = 2u64;
                nat_add(a4329, a4330)?
            }), f1018(v0, v1, {
                let a4331 = v3;
                let a4332 = 3u64;
                nat_add(a4331, a4332)?
            })))
        }
    }
}

pub fn f1016(v0: u32, v1: u32) -> u32 {
    let m4335 = {
        let a4333 = v0;
        let a4334 = v1;
        fixed_u32::shift_left(a4333, a4334)
    };
    match m4335 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1017(v0: u64, v1: bool) -> u32 {
    if v1 {
        f1022({
            let a4336 = v0;
            let a4337 = 4294967296u64;
            let a4338 = 0u64;
            nat_quot(a4336, a4337, a4338)
        })
    } else {
        f1022({
            let a4339 = v0;
            let a4340 = 4294967296u64;
            let a4341 = 0u64;
            nat_rem(a4339, a4340, a4341)
        })
    }
}

pub fn f1018(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4342 = v2;
    let a4343 = v1;
    if nat_lt(a4342, a4343) {
        let m4346 = {
            let a4344 = v0;
            let a4345 = v2;
            index_bytes(a4344, a4345)
        };
        match m4346 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4348 = {
                    let a4347 = v3;
                    fixed_u32::convert(i128::from(a4347))
                };
                match m4348 {
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
        let a4349 = v2;
        let a4350 = v1;
        if nat_eq(a4349, a4350) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f1019(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4351 = f998(v0, v1);
    let a4352 = f1016(v0, v2);
    fixed_u32::bit_or(a4351, a4352)
}

pub fn f1020(v0: u32) -> u32 {
    let a4355 = {
        let a4353 = f1019(v0, 17u32, 15u32);
        let a4354 = f1019(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4353, a4354)
    };
    let a4356 = f998(v0, 10u32);
    fixed_u32::bit_xor(a4355, a4356)
}

pub fn f1021(v0: u32) -> u32 {
    let a4359 = {
        let a4357 = f1019(v0, 7u32, 25u32);
        let a4358 = f1019(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4357, a4358)
    };
    let a4360 = f998(v0, 3u32);
    fixed_u32::bit_xor(a4359, a4360)
}

pub fn f1022(v0: u64) -> u32 {
    let (_, h4361) = {
        let (_, h4362) = f1023(Fn13::F1024, 32u64, (v0, (1u32, 0u32)));
        h4362
    };
    h4361
}

pub fn f1023(v0: Fn13, p4366: u64, p4367: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4365 = (p4366, p4367);
    {
        let mut m4365 = m4365;
        loop {
            let step = {
                let (v1, v2) = m4365;
                let m4363 = v1;
                if m4363 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4363 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4364: Fn13 = v0.clone();
                        c4364.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4365 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1024(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4368, _) = v0;
        h4368
    };
    let v2: u32 = {
        let (h4369, _) = {
            let (_, h4370) = v0;
            h4370
        };
        h4369
    };
    let v3: u32 = {
        let (_, h4371) = {
            let (_, h4372) = v0;
            h4372
        };
        h4371
    };
    ({
        let a4373 = v1;
        let a4374 = 2u64;
        let a4375 = 0u64;
        nat_quot(a4373, a4374, a4375)
    }, (f1016(v2, 1u32), {
        let a4379 = {
            let a4376 = v1;
            let a4377 = 2u64;
            let a4378 = 0u64;
            nat_rem(a4376, a4377, a4378)
        };
        let a4380 = 1u64;
        if nat_eq(a4379, a4380) {
            let a4381 = v3;
            let a4382 = v2;
            fixed_u32::bit_or(a4381, a4382)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
