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
    C0(Adt41),
    C1(Adt85),
    C2(Adt86),
    C3(Adt94),
}

#[derive(Clone)]
pub enum Adt103 {
    C0(u64, List<Adt100>, List<List<u64>>, List<u64>, Option<List<u64>>),
}

#[derive(Clone)]
pub enum Adt104 {
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
    C0(Adt9, Adt9, List<Adt111>),
    C1(List<Adt9>, Adt9, List<Adt111>),
    C2(List<Adt9>, Adt9, List<Adt111>, u64),
    C3(List<Adt111>),
    C4(Option<Adt9>, Option<Adt9>, Adt9, List<Adt111>),
    C5(Adt114, Adt9, Adt9, List<Adt111>),
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
    C0,
    C1(Adt9),
}

#[derive(Clone)]
pub enum Adt116 {
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
pub enum Adt117 {
    C0,
    C1(Option<Adt109>, Adt118),
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
    C8,
}

#[derive(Clone)]
pub enum Adt119 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt120 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt121 {
    C0(Adt9, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt122 {
    C0(Adt112, List<Adt113>),
}

#[derive(Clone)]
pub enum Adt123 {
    C0(Adt109, Str, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt124 {
    C0(List<Adt15>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt125 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt113>),
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
    C0(Adt116),
    C1(Adt135),
}

#[derive(Clone)]
pub enum Adt150 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt151 {
    C0(Adt116),
    C1(List<Adt127>, List<u64>),
}

#[derive(Clone)]
pub enum Adt152 {
    C0(Adt109, Adt153, Adt20, Adt8),
}

#[derive(Clone)]
pub enum Adt153 {
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
pub enum Adt154 {
    C0,
    C1,
    C2(Adt9),
}

#[derive(Clone)]
pub enum Adt155 {
    C0(Adt104),
    C1(Adt124),
}

#[derive(Clone)]
pub enum Adt156 {
    C0,
    C1(Adt153),
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
    F21,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt101, p1: Adt18) -> R<Result<Adt36, Adt93>> {
        match self {
            Fn1::F21 => f21(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F37(List<Adt1>),
    F81,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt25) -> R<Result<Adt53, Adt93>> {
        match self {
            Fn2::F37(k0) => Ok(f37(k0.clone(), p0)),
            Fn2::F81 => f81(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F61(Fn0, Adt0),
    F265(Fn1, Adt99),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt18) -> R<Result<Adt36, Adt93>> {
        match self {
            Fn3::F61(k0, k1) => f61(k0.clone(), k1.clone(), p0),
            Fn3::F265(k0, k1) => f265(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F80,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt25) -> R<Result<Adt105, Adt93>> {
        match self {
            Fn4::F80 => f80(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F118(Fn2, Adt39),
    F123(Fn4, Adt39),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt25) -> R<Result<Adt36, Adt93>> {
        match self {
            Fn5::F118(k0, k1) => f118(k0.clone(), k1.clone(), p0),
            Fn5::F123(k0, k1) => f123(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F151,
}

impl Fn6 {
    pub fn apply(&self, p0: Str) -> R<Option<List<Adt50>>> {
        match self {
            Fn6::F151 => f151(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F163,
}

impl Fn7 {
    pub fn apply(&self, p0: Str, p1: List<(Str, Str)>) -> Adt62 {
        match self {
            Fn7::F163 => f163(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F165,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt28) -> R<Result<List<Str>, Adt93>> {
        match self {
            Fn8::F165 => f165(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F1035(Bytes),
    F1060(Bytes, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn9::F1035(k0) => f1035(k0.clone(), p0),
            Fn9::F1060(k0, k1) => f1060(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F1040(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn10::F1040(k0) => f1040(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F1045(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn11::F1045(k0) => f1045(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F1068(Bytes, u64, u64, u64),
    F1075(Bytes, u64, u64),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, Adt169)) -> R<(u64, Adt169)> {
        match self {
            Fn12::F1068(k0, k1, k2, k3) => f1068(k0.clone(), *k1, *k2, *k3, p0),
            Fn12::F1075(k0, k1, k2) => f1075(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F1070,
}

impl Fn13 {
    pub fn apply(&self, p0: Adt171) -> R<Adt171> {
        match self {
            Fn13::F1070 => f1070(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F1100,
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn14::F1100 => f1100(p0),
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
    Ok(Adt3::C0({
        let m3 = f10()?;
        match m3 {
            Adt3::C0(h2, _, _, _, _, _, _) => {
                h2
            }
        }
    }, {
        let m5 = f10()?;
        match m5 {
            Adt3::C0(_, h4, _, _, _, _, _) => {
                h4
            }
        }
    }, f11(), {
        let m7 = f10()?;
        match m7 {
            Adt3::C0(_, _, _, h6, _, _, _) => {
                h6
            }
        }
    }, {
        let m9 = f10()?;
        match m9 {
            Adt3::C0(_, _, _, _, h8, _, _) => {
                h8
            }
        }
    }, {
        let m11 = f10()?;
        match m11 {
            Adt3::C0(_, _, _, _, _, h10, _) => {
                h10
            }
        }
    }, {
        let m13 = f10()?;
        match m13 {
            Adt3::C0(_, _, _, _, _, _, h12) => {
                h12
            }
        }
    }))
}

pub fn f4(v0: Adt17) -> R<Result<Adt36, Adt93>> {
    let m15 = match v0.clone() {
        Adt17::C0(_, h14) => {
            h14
        }
    };
    match m15 {
        Adt18::C0(_) => {
            f15(match v0 {
                Adt17::C0(_, h28) => {
                    h28
                }
            })
        }
        Adt18::C1(_) => {
            f15(match v0 {
                Adt17::C0(_, h29) => {
                    h29
                }
            })
        }
        Adt18::C2(v1) => {
            f12(match v0.clone() {
                Adt17::C0(h16, _) => {
                    h16
                }
            }, match v0 {
                Adt17::C0(_, h17) => {
                    h17
                }
            }, v1)
        }
        Adt18::C3(_) => {
            f15(match v0 {
                Adt17::C0(_, h30) => {
                    h30
                }
            })
        }
        Adt18::C4(v2) => {
            let a19 = match v2.clone() {
                Adt26::C0(_, h18, _, _) => {
                    h18
                }
            };
            let a20 = Str::lit("effect://item");
            if equal(a19, a20) {
                Ok(f13(f14(match v0 {
                    Adt17::C0(h21, _) => {
                        h21
                    }
                }, v2)))
            } else {
                f15(match v0 {
                    Adt17::C0(_, h22) => {
                        h22
                    }
                })
            }
        }
        Adt18::C5(_) => {
            f15(match v0 {
                Adt17::C0(_, h31) => {
                    h31
                }
            })
        }
        Adt18::C6(_) => {
            f15(match v0 {
                Adt17::C0(_, h32) => {
                    h32
                }
            })
        }
        Adt18::C7(v3) => {
            let a24 = match v3.clone() {
                Adt27::C0(_, h23, _, _, _) => {
                    h23
                }
            };
            let a25 = Str::lit("effect_prompt");
            if equal(a24, a25) {
                Ok(f16(f17(match v0 {
                    Adt17::C0(h26, _) => {
                        h26
                    }
                }, v3)))
            } else {
                f15(match v0 {
                    Adt17::C0(_, h27) => {
                        h27
                    }
                })
            }
        }
        Adt18::C8(_) => {
            f15(match v0 {
                Adt17::C0(_, h33) => {
                    h33
                }
            })
        }
        Adt18::C9(_) => {
            f15(match v0 {
                Adt17::C0(_, h34) => {
                    h34
                }
            })
        }
        Adt18::C10(_) => {
            f15(match v0 {
                Adt17::C0(_, h35) => {
                    h35
                }
            })
        }
        Adt18::C11(_) => {
            f15(match v0 {
                Adt17::C0(_, h36) => {
                    h36
                }
            })
        }
    }
}

pub fn f5(v0: Bytes, v1: Adt0) -> R<List<u64>> {
    if f18(f19(match v1.clone() {
        Adt0::C0(h37, _) => {
            h37
        }
    })?)? {
        f20(f3()?, Fn1::F21, match v1 {
            Adt0::C0(h38, _) => {
                h38
            }
        })
    } else {
        f2(f3()?, Fn0::F4, v0)
    }
}

pub fn f6(v0: Bytes, v1: Option<Adt95>) -> R<Result<Adt0, Adt2>> {
    let m39 = v1;
    match m39 {
        None => {
            Ok(Err::<Adt0, Adt2>(Adt2::C0(0u64)))
        }
        Some(v2) => {
            f22(v0.clone(), v2.clone(), f23(v0, match v2 {
                Adt95::C0(_, h40) => {
                    h40
                }
            })?)
        }
    }
}

pub fn f7(v0: Bytes, v1: u64) -> R<Option<Adt95>> {
    let m41 = f23(v0.clone(), v1)?;
    match m41 {
        None => {
            Ok(None::<Adt95>)
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
                    Ok(None::<Adt95>)
                }
                Some(v3) => {
                    Ok(Some(Adt95::C0(v3, {
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

pub fn f8(v0: Adt96) -> R<List<u64>> {
    let m52 = v0;
    match m52 {
        Adt96::C0(v1) => {
            let a53 = f24(0u64);
            let a54 = v1;
            Ok(append_list(a53, a54))
        }
        Adt96::C1(v2) => {
            let a59 = f24(1u64);
            let a60 = {
                let a57 = f24(match v2.clone() {
                    Adt94::C0(h55, _) => {
                        h55
                    }
                });
                let a58 = f25(match v2 {
                    Adt94::C0(_, h56) => {
                        h56
                    }
                })?;
                append_list(a57, a58)
            };
            Ok(append_list(a59, a60))
        }
    }
}

pub fn f9(v0: Adt3, v1: Fn0, v2: Bytes) -> R<Adt96> {
    let m61 = f1(v2)?;
    match m61 {
        Ok(v4) => {
            f27(v0, v1, v4.clone(), f19(match v4 {
                Adt0::C0(h62, _) => {
                    h62
                }
            })?)
        }
        Err(_) => {
            Ok(f26())
        }
    }
}

pub fn f10() -> R<Adt3> {
    Ok(Adt3::C0(f28(), None::<Str>, f29(), f30(), f31(f32()?)?, f33(), Bytes::lit(&[])))
}

pub fn f11() -> Adt7 {
    Adt7::C0({
        let m64 = f29();
        match m64 {
            Adt7::C0(h63, _, _, _, _, _, _) => {
                h63
            }
        }
    }, {
        let m66 = f29();
        match m66 {
            Adt7::C0(_, h65, _, _, _, _, _) => {
                h65
            }
        }
    }, {
        let m68 = f29();
        match m68 {
            Adt7::C0(_, _, h67, _, _, _, _) => {
                h67
            }
        }
    }, {
        let m70 = f29();
        match m70 {
            Adt7::C0(_, _, _, h69, _, _, _) => {
                h69
            }
        }
    }, {
        let m72 = f29();
        match m72 {
            Adt7::C0(_, _, _, _, h71, _, _) => {
                h71
            }
        }
    }, {
        let m74 = f29();
        match m74 {
            Adt7::C0(_, _, _, _, _, h73, _) => {
                h73
            }
        }
    }, None::<Adt8>)
}

pub fn f12(v0: List<Adt1>, v1: Adt18, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    if f34(v2.clone()) {
        f35(f36()?, Fn2::F37(v0), v2)
    } else {
        if f38(v2, Str::lit("provider_effect")) {
            Ok(f39(f40(v0)))
        } else {
            f15(v1)
        }
    }
}

pub fn f13(v0: Result<Adt97, Adt93>) -> Result<Adt36, Adt93> {
    let m75 = v0;
    match m75 {
        Ok(v1) => {
            let m76 = v1;
            match m76 {
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

pub fn f14(v0: List<Adt1>, v1: Adt26) -> Result<Adt97, Adt93> {
    let m77 = v0;
    match m77.uncons() {
        None => {
            Ok::<Adt97, Adt93>(Adt97::C2(f41(4u64, match v1 {
                Adt26::C0(_, h78, _, _) => {
                    h78
                }
            })))
        }
        Some((v2, v3)) => {
            let m79 = v3;
            match m79.uncons() {
                None => {
                    let a81 = match v2.clone() {
                        Adt1::C0(h80, _) => {
                            h80
                        }
                    };
                    let a82 = 4u64;
                    if nat_eq(a81, a82) {
                        Ok::<Adt97, Adt93>(Adt97::C0(f42(match v1 {
                            Adt26::C0(_, h83, _, _) => {
                                h83
                            }
                        }, f43(Str::lit("item:"), f44(match v2 {
                            Adt1::C0(_, h84) => {
                                h84
                            }
                        })))))
                    } else {
                        Err::<Adt97, Adt93>(f45())
                    }
                }
                Some((_, _)) => {
                    Err::<Adt97, Adt93>(f45())
                }
            }
        }
    }
}

pub fn f15(v0: Adt18) -> R<Result<Adt36, Adt93>> {
    let m85 = f46(f32()?)?;
    match m85 {
        None => {
            f47(v0)
        }
        Some(v1) => {
            Ok(Err::<Adt36, Adt93>(v1))
        }
    }
}

pub fn f16(v0: Result<Adt98, Adt93>) -> Result<Adt36, Adt93> {
    let m86 = v0;
    match m86 {
        Ok(v1) => {
            let m87 = v1;
            match m87 {
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

pub fn f17(v0: List<Adt1>, v1: Adt27) -> Result<Adt98, Adt93> {
    let m88 = v0;
    match m88.uncons() {
        None => {
            Ok::<Adt98, Adt93>(Adt98::C2(f41(5u64, match v1 {
                Adt27::C0(_, h89, _, _, _) => {
                    h89
                }
            })))
        }
        Some((v2, v3)) => {
            let m90 = v3;
            match m90.uncons() {
                None => {
                    let a92 = match v2.clone() {
                        Adt1::C0(h91, _) => {
                            h91
                        }
                    };
                    let a93 = 5u64;
                    if nat_eq(a92, a93) {
                        Ok::<Adt98, Adt93>(Adt98::C0(f48(f43(Str::lit("prompt:"), f44(match v2 {
                            Adt1::C0(_, h94) => {
                                h94
                            }
                        })))))
                    } else {
                        Err::<Adt98, Adt93>(f45())
                    }
                }
                Some((_, _)) => {
                    Err::<Adt98, Adt93>(f45())
                }
            }
        }
    }
}

pub fn f18(v0: Result<Adt99, Adt2>) -> R<bool> {
    let m95 = v0;
    match m95 {
        Ok(v2) => {
            Ok(f49(match v2 {
                Adt99::C0(_, h96, _, _, _, _, _, _) => {
                    h96
                }
            }, f50(Str::lit("byte-serve"))?))
        }
        Err(_) => {
            Ok(false)
        }
    }
}

pub fn f19(v0: Bytes) -> R<Result<Adt99, Adt2>> {
    let m97 = f7(v0.clone(), 0u64)?;
    match m97 {
        None => {
            Ok(Err::<Adt99, Adt2>(Adt2::C0(0u64)))
        }
        Some(v1) => {
            f51(v0.clone(), v1.clone(), f7(v0, match v1 {
                Adt95::C0(_, h98) => {
                    h98
                }
            })?)
        }
    }
}

pub fn f20(v0: Adt3, v1: Fn1, v2: Bytes) -> R<List<u64>> {
    let m99 = f19(v2)?;
    match m99 {
        Ok(v4) => {
            Ok(f52(f54(v0, v1, v4)?))
        }
        Err(_) => {
            Ok(f52(f53()))
        }
    }
}

pub fn f21(_: Adt101, v1: Adt18) -> R<Result<Adt36, Adt93>> {
    f55(v1)
}

pub fn f22(v0: Bytes, v1: Adt95, v2: Option<u64>) -> R<Result<Adt0, Adt2>> {
    let m100 = v2;
    match m100 {
        None => {
            Ok(Err::<Adt0, Adt2>(Adt2::C0(match v1 {
                Adt95::C0(_, h101) => {
                    h101
                }
            })))
        }
        Some(v3) => {
            let m105 = f56(v0.clone(), {
                let a103 = match v1.clone() {
                    Adt95::C0(_, h102) => {
                        h102
                    }
                };
                let a104 = 4u64;
                nat_add(a103, a104)?
            }, v3, List::<Adt1>::nil())?;
            match m105 {
                Ok(v5) => {
                    Ok(f57(v0, match v1 {
                        Adt95::C0(h106, _) => {
                            h106
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

pub fn f23(v0: Bytes, v1: u64) -> R<Option<u64>> {
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
                let a112 = f58(v0.clone(), v1)?;
                let a113 = 16777216u64;
                nat_mul(a112, a113)?
            };
            let a129 = {
                let a126 = {
                    let a116 = f58(v0.clone(), {
                        let a114 = v1;
                        let a115 = 1u64;
                        nat_add(a114, a115)?
                    })?;
                    let a117 = 65536u64;
                    nat_mul(a116, a117)?
                };
                let a127 = {
                    let a124 = {
                        let a120 = f58(v0.clone(), {
                            let a118 = v1;
                            let a119 = 2u64;
                            nat_add(a118, a119)?
                        })?;
                        let a121 = 256u64;
                        nat_mul(a120, a121)?
                    };
                    let a125 = f58(v0, {
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

pub fn f24(v0: u64) -> List<u64> {
    List::cons({
        let a133 = {
            let a130 = v0;
            let a131 = 16777216u64;
            let a132 = 0u64;
            nat_quot(a130, a131, a132)
        };
        let a134 = 256u64;
        let a135 = 0u64;
        nat_rem(a133, a134, a135)
    }, List::cons({
        let a139 = {
            let a136 = v0;
            let a137 = 65536u64;
            let a138 = 0u64;
            nat_quot(a136, a137, a138)
        };
        let a140 = 256u64;
        let a141 = 0u64;
        nat_rem(a139, a140, a141)
    }, List::cons({
        let a145 = {
            let a142 = v0;
            let a143 = 256u64;
            let a144 = 0u64;
            nat_quot(a142, a143, a144)
        };
        let a146 = 256u64;
        let a147 = 0u64;
        nat_rem(a145, a146, a147)
    }, List::cons({
        let a148 = v0;
        let a149 = 256u64;
        let a150 = 0u64;
        nat_rem(a148, a149, a150)
    }, List::<u64>::nil()))))
}

pub fn f25(v0: Bytes) -> R<List<u64>> {
    f59(v0.clone(), {
        let a151 = v0;
        length_bytes(a151)
    }, List::<u64>::nil())
}

pub fn f26() -> Adt96 {
    Adt96::C0(f52(f53()))
}

pub fn f27(v0: Adt3, v1: Fn0, v2: Adt0, v3: Result<Adt99, Adt2>) -> R<Adt96> {
    let m152 = v3;
    match m152 {
        Ok(v5) => {
            f60(v0, Fn3::F61(v1, v2), v5)
        }
        Err(_) => {
            Ok(f26())
        }
    }
}

pub fn f28() -> Adt4 {
    Adt4::C0(Str::lit("mcp-core-fixture"), None::<Str>, Str::lit("0.1.0"), None::<Str>, None::<Str>, None::<List<Adt5>>)
}

pub fn f29() -> Adt7 {
    Adt7::C0(None::<Adt8>, None::<Adt8>, Some(f62()), Some(Adt11::C0(None::<bool>)), Some(Adt12::C0(None::<bool>, None::<bool>)), Some(Adt11::C0(None::<bool>)), Some(Adt8::C0(List::cons((f63(), Adt9::C5(List::<(Str, Adt9)>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f30() -> Adt13 {
    Adt13::C0(0u64, Adt14::C1)
}

pub fn f31(v0: List<Adt39>) -> R<List<Adt15>> {
    let m153 = f64(v0)?;
    match m153 {
        Ok(v1) => {
            Ok(v1)
        }
        Err(_) => {
            Ok(List::<Adt15>::nil())
        }
    }
}

pub fn f32() -> R<List<Adt39>> {
    let a156 = {
        let a154 = f65()?;
        let a155 = f66();
        append_list(a154, a155)
    };
    let a157 = f67();
    Ok(append_list(a156, a157))
}

pub fn f33() -> u64 {
    100u64
}

pub fn f34(v0: Adt25) -> bool {
    if f38(v0.clone(), Str::lit("effect_read")) {
        true
    } else {
        if f38(v0.clone(), Str::lit("effect_chain")) {
            true
        } else {
            if f38(v0.clone(), Str::lit("effect_edge_ok")) {
                true
            } else {
                f38(v0, Str::lit("effect_edge_over"))
            }
        }
    }
}

pub fn f35(v0: List<Adt39>, v1: Fn2, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    let m159 = f68(match v2.clone() {
        Adt25::C0(_, h158, _, _, _) => {
            h158
        }
    }, v0);
    match m159 {
        None => {
            Ok(Err::<Adt36, Adt93>(Adt93::C0(match v2 {
                Adt25::C0(_, h160, _, _, _) => {
                    h160
                }
            })))
        }
        Some(v3) => {
            f69(v3, v1, v2)
        }
    }
}

pub fn f36() -> R<List<Adt39>> {
    Ok(List::cons(f70(Str::lit("effect_read"), Str::lit("Reads a key through the host"), f71()?), List::cons(f70(Str::lit("effect_chain"), Str::lit("Reads two keys through the host, the second after the first"), f72()?), List::cons(f70(Str::lit("effect_edge_ok"), Str::lit("Asks for the largest effect tag"), f73()), List::cons(f70(Str::lit("effect_edge_over"), Str::lit("Asks for a tag that does not fit thirty-two bits"), f73()), List::<Adt39>::nil())))))
}

pub fn f37(v0: List<Adt1>, v1: Adt25) -> Result<Adt53, Adt93> {
    f74(v0, v1)
}

pub fn f38(v0: Adt25, v1: Str) -> bool {
    let a162 = match v0 {
        Adt25::C0(_, h161, _, _, _) => {
            h161
        }
    };
    let a163 = v1;
    equal(a162, a163)
}

pub fn f39(v0: Result<Adt102, Adt93>) -> Result<Adt36, Adt93> {
    let m164 = v0;
    match m164 {
        Ok(v1) => {
            let m165 = v1;
            match m165 {
                Adt102::C0(v2) => {
                    Ok::<Adt36, Adt93>(Adt36::C2(v2))
                }
                Adt102::C1(v3) => {
                    Ok::<Adt36, Adt93>(Adt36::C12(v3))
                }
                Adt102::C2(v4) => {
                    Ok::<Adt36, Adt93>(Adt36::C13(v4))
                }
                Adt102::C3(v5) => {
                    Err::<Adt36, Adt93>(Adt93::C15(v5))
                }
            }
        }
        Err(v6) => {
            Err::<Adt36, Adt93>(v6)
        }
    }
}

pub fn f40(v0: List<Adt1>) -> Result<Adt102, Adt93> {
    let m166 = v0;
    match m166.uncons() {
        None => {
            Ok::<Adt102, Adt93>(Adt102::C3(f41(3u64, Str::lit("call"))))
        }
        Some((v1, v2)) => {
            let m167 = v2;
            match m167.uncons() {
                None => {
                    let a169 = match v1.clone() {
                        Adt1::C0(h168, _) => {
                            h168
                        }
                    };
                    let a170 = 3u64;
                    if nat_eq(a169, a170) {
                        Ok::<Adt102, Adt93>(Adt102::C0(f75(f43(Str::lit("call:"), f44(match v1 {
                            Adt1::C0(_, h171) => {
                                h171
                            }
                        })), false)))
                    } else {
                        Err::<Adt102, Adt93>(f45())
                    }
                }
                Some((_, _)) => {
                    Err::<Adt102, Adt93>(f45())
                }
            }
        }
    }
}

pub fn f41(v0: u64, v1: Str) -> Adt94 {
    Adt94::C0(v0, {
        let a172 = v1;
        utf8_encode(a172)
    })
}

pub fn f42(v0: Str, v1: Str) -> Adt56 {
    Adt56::C0(f30(), List::cons(Adt50::C0(Adt51::C0(v0, Some(Str::lit("text/plain")), v1, None::<Adt8>)), List::<Adt50>::nil()))
}

pub fn f43(v0: Str, v1: Str) -> Str {
    let a173 = List::cons(v0, List::cons(v1, List::<Str>::nil()));
    let a174 = Str::lit("");
    join(a173, a174)
}

pub fn f44(v0: Bytes) -> Str {
    let m176 = {
        let a175 = v0;
        utf8_decode(a175)
    };
    match m176 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f45() -> Adt93 {
    Adt93::C9(Str::lit("the outcomes of the effects do not match the request"))
}

pub fn f46(v0: List<Adt39>) -> R<Option<Adt93>> {
    let m177 = f64(v0)?;
    match m177 {
        Ok(_) => {
            Ok(None::<Adt93>)
        }
        Err(v1) => {
            Ok(Some(Adt93::C9(f76(v1))))
        }
    }
}

pub fn f47(v0: Adt18) -> R<Result<Adt36, Adt93>> {
    let m178 = v0;
    match m178 {
        Adt18::C0(_) => {
            Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt18::C1(v1) => {
            Ok(f77(f30(), f32()?, v1))
        }
        Adt18::C2(v2) => {
            if f78(match v2.clone() {
                Adt25::C0(_, h179, _, _, _) => {
                    h179
                }
            }) {
                f79(f32()?, Fn4::F80, v2)
            } else {
                f35(f32()?, Fn2::F81, v2)
            }
        }
        Adt18::C3(v4) => {
            Ok(f82(v4))
        }
        Adt18::C4(v5) => {
            f83(v5)
        }
        Adt18::C5(v6) => {
            Ok(f84(v6))
        }
        Adt18::C6(v7) => {
            Ok(f85(v7))
        }
        Adt18::C7(v8) => {
            if f86(match v8.clone() {
                Adt27::C0(_, h180, _, _, _) => {
                    h180
                }
            }) {
                Ok(f87(v8))
            } else {
                Ok(f88(v8))
            }
        }
        Adt18::C8(v9) => {
            f89(v9)
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

pub fn f48(v0: Str) -> Adt62 {
    Adt62::C0(None::<Str>, List::cons(Adt63::C0(Adt45::C0, f90(v0)), List::<Adt63>::nil()))
}

pub fn f49(p187: List<u64>, p188: List<u64>) -> bool {
    let m186 = (p187, p188);
    {
        let mut m186 = m186;
        loop {
            let step = {
                let (v0, v1) = m186;
                let m181 = v0;
                match m181.uncons() {
                    None => {
                        let m182 = v1;
                        match m182.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m183 = v1;
                        match m183.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a184 = v4;
                                let a185 = v6;
                                if nat_eq(a184, a185) {
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
                Ok(next) => { m186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: Str) -> R<List<u64>> {
    f25({
        let a189 = v0;
        utf8_encode(a189)
    })
}

pub fn f51(v0: Bytes, v1: Adt95, v2: Option<Adt95>) -> R<Result<Adt99, Adt2>> {
    let m190 = v2;
    match m190 {
        None => {
            Ok(Err::<Adt99, Adt2>(Adt2::C0(match v1 {
                Adt95::C0(_, h191) => {
                    h191
                }
            })))
        }
        Some(v3) => {
            f91(v0.clone(), v1, v3.clone(), f23(v0, match v3 {
                Adt95::C0(_, h192) => {
                    h192
                }
            })?)
        }
    }
}

pub fn f52(v0: Adt103) -> List<u64> {
    let a216 = f24(match v0.clone() {
        Adt103::C0(h193, _, _, _, _) => {
            h193
        }
    });
    let a217 = {
        let a214 = f24({
            let a195 = match v0.clone() {
                Adt103::C0(_, h194, _, _, _) => {
                    h194
                }
            };
            length_list(a195)
        });
        let a215 = {
            let a212 = f92(match v0.clone() {
                Adt103::C0(_, h196, _, _, _) => {
                    h196
                }
            });
            let a213 = {
                let a210 = f24({
                    let a198 = match v0.clone() {
                        Adt103::C0(_, _, h197, _, _) => {
                            h197
                        }
                    };
                    length_list(a198)
                });
                let a211 = {
                    let a208 = f93(match v0.clone() {
                        Adt103::C0(_, _, h199, _, _) => {
                            h199
                        }
                    });
                    let a209 = {
                        let a206 = f24({
                            let a201 = match v0.clone() {
                                Adt103::C0(_, _, _, h200, _) => {
                                    h200
                                }
                            };
                            length_list(a201)
                        });
                        let a207 = {
                            let a204 = match v0.clone() {
                                Adt103::C0(_, _, _, h202, _) => {
                                    h202
                                }
                            };
                            let a205 = f94(match v0 {
                                Adt103::C0(_, _, _, _, h203) => {
                                    h203
                                }
                            });
                            append_list(a204, a205)
                        };
                        append_list(a206, a207)
                    };
                    append_list(a208, a209)
                };
                append_list(a210, a211)
            };
            append_list(a212, a213)
        };
        append_list(a214, a215)
    };
    append_list(a216, a217)
}

pub fn f53() -> Adt103 {
    Adt103::C0(f95(), f96(), List::<List<u64>>::nil(), f97(), None::<List<u64>>)
}

pub fn f54(v0: Adt3, v1: Fn1, v2: Adt99) -> R<Adt103> {
    f98(v0, v1, v2.clone(), f99(match v2.clone() {
        Adt99::C0(_, _, h218, _, _, _, _, _) => {
            h218
        }
    }, match v2 {
        Adt99::C0(_, _, _, _, _, h219, _, _) => {
            h219
        }
    })?)
}

pub fn f55(v0: Adt18) -> R<Result<Adt36, Adt93>> {
    f4(Adt17::C0(List::<Adt1>::nil(), v0))
}

pub fn f56(v0: Bytes, p230: u64, p231: u64, p232: List<Adt1>) -> R<Result<(List<Adt1>, u64), Adt2>> {
    let m229 = (p230, (p231, p232));
    {
        let mut m229 = m229;
        loop {
            let step = {
                let (v1, (v2, v3)) = m229;
                let m220 = v2;
                if m220 == 0 {
                    Err::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>(Ok(Ok::<(List<Adt1>, u64), Adt2>((f100(v3, List::<Adt1>::nil()), v1))))
                } else {
                    let v4 = m220 - 1;
                    let m221 = f23(v0.clone(), v1)?;
                    match m221 {
                        None => {
                            Err::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>(Ok(Err::<(List<Adt1>, u64), Adt2>(Adt2::C0(v1))))
                        }
                        Some(v5) => {
                            let m224 = f7(v0.clone(), {
                                let a222 = v1;
                                let a223 = 4u64;
                                nat_add(a222, a223)?
                            })?;
                            match m224 {
                                None => {
                                    Err::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>(Ok(Err::<(List<Adt1>, u64), Adt2>(Adt2::C0({
                                        let a225 = v1;
                                        let a226 = 4u64;
                                        nat_add(a225, a226)?
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>((match v6.clone() {
                                        Adt95::C0(_, h227) => {
                                            h227
                                        }
                                    }, (v4, List::cons(Adt1::C0(v5, match v6 {
                                        Adt95::C0(h228, _) => {
                                            h228
                                        }
                                    }), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Bytes, v1: Bytes, v2: (List<Adt1>, u64)) -> Result<Adt0, Adt2> {
    f101({
        let a235 = {
            let (_, h233) = v2.clone();
            h233
        };
        let a236 = {
            let a234 = v0;
            length_bytes(a234)
        };
        nat_eq(a235, a236)
    }, v1, {
        let (h237, _) = v2;
        h237
    }, {
        let (_, h238) = v2;
        h238
    })
}

pub fn f58(v0: Bytes, v1: u64) -> R<u64> {
    let m241 = {
        let a239 = v0;
        let a240 = v1;
        index_bytes(a239, a240)
    };
    match m241 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a286 = {
                let a262 = {
                    let a250 = {
                        let a244 = {
                            let a242 = v3;
                            let a243 = 128u8;
                            fixed_u8::bit_and(a242, a243)
                        };
                        let a245 = 128u8;
                        if equal(a244, a245) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a251 = {
                        let a248 = {
                            let a246 = v3;
                            let a247 = 64u8;
                            fixed_u8::bit_and(a246, a247)
                        };
                        let a249 = 64u8;
                        if equal(a248, a249) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a250, a251)?
                };
                let a263 = {
                    let a260 = {
                        let a254 = {
                            let a252 = v3;
                            let a253 = 32u8;
                            fixed_u8::bit_and(a252, a253)
                        };
                        let a255 = 32u8;
                        if equal(a254, a255) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a261 = {
                        let a258 = {
                            let a256 = v3;
                            let a257 = 16u8;
                            fixed_u8::bit_and(a256, a257)
                        };
                        let a259 = 16u8;
                        if equal(a258, a259) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a260, a261)?
                };
                nat_add(a262, a263)?
            };
            let a287 = {
                let a284 = {
                    let a272 = {
                        let a266 = {
                            let a264 = v3;
                            let a265 = 8u8;
                            fixed_u8::bit_and(a264, a265)
                        };
                        let a267 = 8u8;
                        if equal(a266, a267) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a273 = {
                        let a270 = {
                            let a268 = v3;
                            let a269 = 4u8;
                            fixed_u8::bit_and(a268, a269)
                        };
                        let a271 = 4u8;
                        if equal(a270, a271) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a272, a273)?
                };
                let a285 = {
                    let a282 = {
                        let a276 = {
                            let a274 = v3;
                            let a275 = 2u8;
                            fixed_u8::bit_and(a274, a275)
                        };
                        let a277 = 2u8;
                        if equal(a276, a277) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a283 = {
                        let a280 = {
                            let a278 = v3;
                            let a279 = 1u8;
                            fixed_u8::bit_and(a278, a279)
                        };
                        let a281 = 1u8;
                        if equal(a280, a281) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a282, a283)?
                };
                nat_add(a284, a285)?
            };
            nat_add(a286, a287)
        }
    }
}

pub fn f59(v0: Bytes, p290: u64, p291: List<u64>) -> R<List<u64>> {
    let m289 = (p290, p291);
    {
        let mut m289 = m289;
        loop {
            let step = {
                let (v1, v2) = m289;
                let m288 = v1;
                if m288 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m288 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f58(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m289 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: Adt3, v1: Fn3, v2: Adt99) -> R<Adt96> {
    f102(v0, v1, v2.clone(), f99(match v2.clone() {
        Adt99::C0(_, _, h292, _, _, _, _, _) => {
            h292
        }
    }, match v2 {
        Adt99::C0(_, _, _, _, _, h293, _, _) => {
            h293
        }
    })?)
}

pub fn f61(v0: Fn0, v1: Adt0, v2: Adt18) -> R<Result<Adt36, Adt93>> {
    let c294: Fn0 = v0;
    c294.apply(Adt17::C0(match v1 {
        Adt0::C0(_, h295) => {
            h295
        }
    }, v2))
}

pub fn f62() -> Adt8 {
    Adt8::C0(List::<(Str, Adt9)>::nil())
}

pub fn f63() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f64(v0: List<Adt39>) -> R<Result<List<Adt15>, Adt104>> {
    let m296 = f103(v0.clone());
    match m296 {
        None => {
            f104(v0, List::<Adt15>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt15>, Adt104>(v1))
        }
    }
}

pub fn f65() -> R<List<Adt39>> {
    Ok(List::cons(f70(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f73()), List::cons(f70(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f105()), List::cons(f70(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f73()), List::cons(f70(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f106()), List::cons(f70(Str::lit("test_image_content"), Str::lit("Tests image content response"), f106()), List::cons(f70(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f106()), List::cons(f70(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f106()), List::cons(f70(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f106()), List::cons(f70(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f106()), List::cons(f70(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f106()), List::cons(f70(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f107()), List::cons(f70(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f108()?), List::cons(f70(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f109()), List::cons(f110(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f106(), f111()), List::cons(f110(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f106(), f111()), List::cons(f70(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f106()), List::cons(f70(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f106()), List::cons(f70(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f106()), List::cons(f70(Str::lit("test_streaming_elicitation"), Str::lit("Diagnostic tool validating response progress streams"), f106()), List::cons(f70(Str::lit("test_logging_tool"), Str::lit("Diagnostic logging validator tool"), f106()), List::<Adt39>::nil())))))))))))))))))))))
}

pub fn f66() -> List<Adt39> {
    List::cons(f112(Str::lit("test_input_required_result_elicitation"), Str::lit("MRTR: returns InputRequiredResult with elicitation request")), List::cons(f112(Str::lit("test_input_required_result_sampling"), Str::lit("MRTR: returns InputRequiredResult with sampling request")), List::cons(f112(Str::lit("test_input_required_result_list_roots"), Str::lit("MRTR: returns InputRequiredResult with roots/list request")), List::cons(f112(Str::lit("test_input_required_result_request_state"), Str::lit("MRTR: returns InputRequiredResult with requestState")), List::cons(f112(Str::lit("test_input_required_result_multiple_inputs"), Str::lit("MRTR: returns InputRequiredResult with multiple input requests")), List::cons(f112(Str::lit("test_input_required_result_multi_round"), Str::lit("MRTR: multi-round InputRequiredResult workflow")), List::cons(f112(Str::lit("test_input_required_result_tampered_state"), Str::lit("MRTR: HMAC-signed requestState integrity test")), List::cons(f112(Str::lit("test_input_required_result_capabilities"), Str::lit("MRTR: respects client capabilities in inputRequests")), List::<Adt39>::nil()))))))))
}

pub fn f67() -> List<Adt39> {
    List::cons(f113(Str::lit("greet"), Str::lit("Answers a greeting at once; never a task"), f114(Str::lit("name"))), List::cons(f113(Str::lit("slow_compute"), Str::lit("Computes for a number of seconds; a task for a client that declared the tasks extension"), f115()), List::cons(f113(Str::lit("failing_job"), Str::lit("Always fails; always a task, so the client must declare the tasks extension"), f116()), List::cons(f113(Str::lit("protocol_error_job"), Str::lit("Ends in a protocol error; a task for a client that declared the tasks extension"), f116()), List::cons(f113(Str::lit("confirm_delete"), Str::lit("Asks the client to confirm a deletion; a task that waits for input"), f114(Str::lit("filename"))), List::cons(f113(Str::lit("multi_input"), Str::lit("Asks the client two questions at once; a task that waits for both answers"), f116()), List::cons(f113(Str::lit("test_tool_with_task"), Str::lit("Asks for a name, then becomes a task; the client must declare the tasks extension"), f116()), List::<Adt39>::nil())))))))
}

pub fn f68(v0: Str, p302: List<Adt39>) -> Option<Adt39> {
    let m301 = p302;
    {
        let mut m301 = m301;
        loop {
            let step = {
                let v1 = m301;
                let m297 = v1;
                match m297.uncons() {
                    None => {
                        Err::<List<Adt39>, Option<Adt39>>(None::<Adt39>)
                    }
                    Some((v2, v3)) => {
                        let a299 = match v2.clone() {
                            Adt39::C0(h298, _, _, _, _, _, _, _) => {
                                h298
                            }
                        };
                        let a300 = v0.clone();
                        if equal(a299, a300) {
                            Err::<List<Adt39>, Option<Adt39>>(Some(v2))
                        } else {
                            Ok::<List<Adt39>, Option<Adt39>>(v3)
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

pub fn f69(v0: Adt39, v1: Fn2, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    f117(v0.clone(), v2, Fn5::F118(v1, v0))
}

pub fn f70(v0: Str, v1: Str, v2: Adt8) -> Adt39 {
    Adt39::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), v2, None::<Adt8>, None::<Adt40>, None::<Adt8>)
}

pub fn f71() -> R<Adt8> {
    Ok(Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("key"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f119(1u64)?), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("key")), List::<Adt9>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))
}

pub fn f72() -> R<Adt8> {
    Ok(Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("first"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f119(1u64)?), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("second"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f119(1u64)?), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("first")), List::cons(Adt9::C3(Str::lit("second")), List::<Adt9>::nil())))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))
}

pub fn f73() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::<(Str, Adt9)>::nil())), List::cons((Str::lit("required"), Adt9::C4(List::<Adt9>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f74(v0: List<Adt1>, v1: Adt25) -> Result<Adt53, Adt93> {
    if f38(v1.clone(), Str::lit("effect_read")) {
        f120(v0, v1)
    } else {
        if f38(v1.clone(), Str::lit("effect_chain")) {
            f121(v0, v1)
        } else {
            if f38(v1, Str::lit("effect_edge_ok")) {
                f122(v0, 4294967295u64)
            } else {
                f122(v0, 4294967296u64)
            }
        }
    }
}

pub fn f75(v0: Str, v1: bool) -> Adt41 {
    Adt41::C0(List::cons(Adt42::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>)), List::<Adt42>::nil()), None::<Adt9>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f76(v0: Adt104) -> Str {
    let m303 = v0;
    match m303 {
        Adt104::C0(v1) => {
            let a304 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a305 = Str::lit(" ");
            join(a304, a305)
        }
        Adt104::C1(v2) => {
            let a306 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a307 = Str::lit(" ");
            join(a306, a307)
        }
        Adt104::C2(v3) => {
            let a308 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a309 = Str::lit(" ");
            join(a308, a309)
        }
        Adt104::C3(v4) => {
            let a310 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a311 = Str::lit(" ");
            join(a310, a311)
        }
        Adt104::C4(v5, v6) => {
            let a312 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a313 = Str::lit(" ");
            join(a312, a313)
        }
        Adt104::C5(v7, v8) => {
            let a314 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a315 = Str::lit(" ");
            join(a314, a315)
        }
        Adt104::C6(v9, v10) => {
            let a316 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a317 = Str::lit(" ");
            join(a316, a317)
        }
        Adt104::C7(v11) => {
            let a318 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a319 = Str::lit(" ");
            join(a318, a319)
        }
        Adt104::C8(v12) => {
            let a320 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a321 = Str::lit(" ");
            join(a320, a321)
        }
        Adt104::C9(v13) => {
            let a322 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a323 = Str::lit(" ");
            join(a322, a323)
        }
        Adt104::C10(v14) => {
            let a324 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a325 = Str::lit(" ");
            join(a324, a325)
        }
        Adt104::C11(v15) => {
            let a326 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a327 = Str::lit(" ");
            join(a326, a327)
        }
        Adt104::C12(v16, v17) => {
            let a328 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a329 = Str::lit(" ");
            join(a328, a329)
        }
    }
}

pub fn f77(v0: Adt13, v1: List<Adt39>, v2: Adt24) -> Result<Adt36, Adt93> {
    let m331 = match v2 {
        Adt24::C0(_, h330) => {
            h330
        }
    };
    match m331 {
        None => {
            Ok::<Adt36, Adt93>(Adt36::C1(Adt38::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt36, Adt93>(Adt93::C5(v3))
        }
    }
}

pub fn f78(v0: Str) -> bool {
    let a332 = v0.clone();
    let a333 = Str::lit("test_input_required_result_elicitation");
    if equal(a332, a333) {
        true
    } else {
        let a334 = v0.clone();
        let a335 = Str::lit("test_input_required_result_sampling");
        if equal(a334, a335) {
            true
        } else {
            let a336 = v0.clone();
            let a337 = Str::lit("test_input_required_result_list_roots");
            if equal(a336, a337) {
                true
            } else {
                let a338 = v0.clone();
                let a339 = Str::lit("test_input_required_result_request_state");
                if equal(a338, a339) {
                    true
                } else {
                    let a340 = v0.clone();
                    let a341 = Str::lit("test_input_required_result_multiple_inputs");
                    if equal(a340, a341) {
                        true
                    } else {
                        let a342 = v0.clone();
                        let a343 = Str::lit("test_input_required_result_multi_round");
                        if equal(a342, a343) {
                            true
                        } else {
                            let a344 = v0.clone();
                            let a345 = Str::lit("test_input_required_result_tampered_state");
                            if equal(a344, a345) {
                                true
                            } else {
                                let a346 = v0;
                                let a347 = Str::lit("test_input_required_result_capabilities");
                                equal(a346, a347)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f79(v0: List<Adt39>, v1: Fn4, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    let m349 = f68(match v2.clone() {
        Adt25::C0(_, h348, _, _, _) => {
            h348
        }
    }, v0);
    match m349 {
        None => {
            Ok(Err::<Adt36, Adt93>(Adt93::C0(match v2 {
                Adt25::C0(_, h350, _, _, _) => {
                    h350
                }
            })))
        }
        Some(v3) => {
            f117(v3.clone(), v2, Fn5::F123(v1, v3))
        }
    }
}

pub fn f80(v0: Adt25) -> R<Result<Adt105, Adt93>> {
    if f124(v0.clone(), Str::lit("test_input_required_result_elicitation")) {
        Ok(f125(v0))
    } else {
        if f124(v0.clone(), Str::lit("test_input_required_result_sampling")) {
            Ok(f126(v0))
        } else {
            if f124(v0.clone(), Str::lit("test_input_required_result_list_roots")) {
                f127(v0)
            } else {
                if f124(v0.clone(), Str::lit("test_input_required_result_request_state")) {
                    Ok(f128(v0))
                } else {
                    if f124(v0.clone(), Str::lit("test_input_required_result_multiple_inputs")) {
                        f129(v0)
                    } else {
                        if f124(v0.clone(), Str::lit("test_input_required_result_multi_round")) {
                            f130(v0)
                        } else {
                            if f124(v0.clone(), Str::lit("test_input_required_result_tampered_state")) {
                                Ok(f131(v0))
                            } else {
                                if f124(v0.clone(), Str::lit("test_input_required_result_capabilities")) {
                                    Ok(f132(v0))
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

pub fn f81(v0: Adt25) -> R<Result<Adt53, Adt93>> {
    if f38(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt53, Adt93>(f133(List::cons(f90(Str::lit("This is a simple text response for testing.")), List::<Adt42>::nil()))))
    } else {
        if f38(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt53, Adt93>(f133(List::cons(f134(), List::<Adt42>::nil()))))
        } else {
            if f38(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt53, Adt93>(f133(List::cons(f135(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt42>::nil()))))
            } else {
                if f38(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt53, Adt93>(f133(List::cons(f136(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt42>::nil()))))
                } else {
                    if f38(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt53, Adt93>(f133(List::cons(f90(Str::lit("Multiple content types test:")), List::cons(f134(), List::cons(f136(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt42>::nil()))))))
                    } else {
                        if f38(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt53, Adt93>(f137(List::cons(f90(Str::lit("This tool intentionally returns an error for testing")), List::<Adt42>::nil()), None::<Adt9>, true, List::<Adt54>::nil())))
                        } else {
                            if f38(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt53, Adt93>(f138(v0, f139()?)))
                            } else {
                                if f38(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt53, Adt93>(f138(v0, f140()?)))
                                } else {
                                    if f38(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt53, Adt93>(f137(List::cons(f90(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt42>::nil()), Some(f141()?), false, List::<Adt54>::nil())))
                                    } else {
                                        if f38(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt53, Adt93>(f137(List::cons(f90(Str::lit("{\"sum\":\"three\"}")), List::<Adt42>::nil()), Some(f142()), false, List::<Adt54>::nil())))
                                        } else {
                                            if f38(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt53, Adt93>(f133(List::cons(f143(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt42>::nil()))))
                                            } else {
                                                if f38(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt53, Adt93>(f133(List::cons(f90(f144()), List::<Adt42>::nil()))))
                                                } else {
                                                    if f38(v0.clone(), Str::lit("test_streaming_elicitation")) {
                                                        Ok(Ok::<Adt53, Adt93>(f133(List::cons(f90(Str::lit("Streaming complete")), List::<Adt42>::nil()))))
                                                    } else {
                                                        if f38(v0.clone(), Str::lit("test_logging_tool")) {
                                                            Ok(Ok::<Adt53, Adt93>(f133(List::cons(f90(Str::lit("Logging evaluated")), List::<Adt42>::nil()))))
                                                        } else {
                                                            Ok(f145(v0))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f82(v0: Adt24) -> Result<Adt36, Adt93> {
    f146(f147(), f148(), v0)
}

pub fn f83(v0: Adt26) -> R<Result<Adt36, Adt93>> {
    f149(f150(), Fn6::F151, v0)
}

pub fn f84(v0: Adt24) -> Result<Adt36, Adt93> {
    f152(f147(), f153(), v0)
}

pub fn f85(v0: Adt24) -> Result<Adt36, Adt93> {
    f154(f147(), f155(), v0)
}

pub fn f86(v0: Str) -> bool {
    let a351 = v0;
    let a352 = f156();
    equal(a351, a352)
}

pub fn f87(v0: Adt27) -> Result<Adt36, Adt93> {
    let m354 = f157(Str::lit("user_context"), Adt106::C0, match v0 {
        Adt27::C0(_, _, _, h353, _) => {
            h353
        }
    });
    match m354 {
        Adt108::C0 => {
            Ok::<Adt36, Adt93>(Adt36::C12(Adt85::C0(f160(Str::lit("user_context"), f161(Str::lit("What context should the prompt use?"), Str::lit("context"), Str::lit("string"))))))
        }
        Adt108::C1 => {
            Err::<Adt36, Adt93>(Adt93::C11)
        }
        Adt108::C2(v1) => {
            Ok::<Adt36, Adt93>(Adt36::C8(f158(f159(v1, Str::lit("context"), Str::lit("unknown")))))
        }
    }
}

pub fn f88(v0: Adt27) -> Result<Adt36, Adt93> {
    f162(f155(), Fn7::F163, v0)
}

pub fn f89(v0: Adt28) -> R<Result<Adt36, Adt93>> {
    f164(Fn8::F165, v0)
}

pub fn f90(v0: Str) -> Adt42 {
    Adt42::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>))
}

pub fn f91(v0: Bytes, v1: Adt95, v2: Adt95, v3: Option<u64>) -> R<Result<Adt99, Adt2>> {
    let m355 = v3;
    match m355 {
        None => {
            Ok(Err::<Adt99, Adt2>(Adt2::C0(match v2 {
                Adt95::C0(_, h356) => {
                    h356
                }
            })))
        }
        Some(v4) => {
            let m360 = f166(v0.clone(), {
                let a358 = match v2.clone() {
                    Adt95::C0(_, h357) => {
                        h357
                    }
                };
                let a359 = 4u64;
                nat_add(a358, a359)?
            }, v4, List::<Adt100>::nil())?;
            match m360 {
                Ok(v6) => {
                    f167(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt99, Adt2>(v5))
                }
            }
        }
    }
}

pub fn f92(v0: List<Adt100>) -> List<u64> {
    f168(f169(v0, List::<Adt100>::nil()), List::<u64>::nil())
}

pub fn f93(v0: List<List<u64>>) -> List<u64> {
    f170(f171(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f94(v0: Option<List<u64>>) -> List<u64> {
    let m361 = v0;
    match m361 {
        None => {
            List::<u64>::nil()
        }
        Some(v1) => {
            let a365 = f24(1u64);
            let a366 = {
                let a363 = f24({
                    let a362 = v1.clone();
                    length_list(a362)
                });
                let a364 = v1;
                append_list(a363, a364)
            };
            append_list(a365, a366)
        }
    }
}

pub fn f95() -> u64 {
    500u64
}

pub fn f96() -> List<Adt100> {
    List::cons(Adt100::C0(f172(), f173()), List::<Adt100>::nil())
}

pub fn f97() -> List<u64> {
    let a369 = f174();
    let a370 = {
        let a367 = f175();
        let a368 = f176();
        append_list(a367, a368)
    };
    append_list(a369, a370)
}

pub fn f98(v0: Adt3, v1: Fn1, v2: Adt99, v3: Option<Adt107>) -> R<Adt103> {
    let m371 = v3;
    match m371 {
        None => {
            f178(v0, v1, v2)
        }
        Some(v4) => {
            f177(v4)
        }
    }
}

pub fn f99(v0: List<Adt100>, v1: List<List<u64>>) -> R<Option<Adt107>> {
    let m372 = f179(v0.clone(), v1.clone())?;
    match m372 {
        None => {
            f180(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f100(p375: List<Adt1>, p376: List<Adt1>) -> List<Adt1> {
    let m374 = (p375, p376);
    {
        let mut m374 = m374;
        loop {
            let step = {
                let (v0, v1) = m374;
                let m373 = v0;
                match m373.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m374 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: bool, v1: Bytes, v2: List<Adt1>, v3: u64) -> Result<Adt0, Adt2> {
    if v0 {
        Ok::<Adt0, Adt2>(Adt0::C0(v1, v2))
    } else {
        Err::<Adt0, Adt2>(Adt2::C1(v3))
    }
}

pub fn f102(v0: Adt3, v1: Fn3, v2: Adt99, v3: Option<Adt107>) -> R<Adt96> {
    let m377 = v3;
    match m377 {
        None => {
            f181(v0, v1, v2)
        }
        Some(v4) => {
            Ok(Adt96::C0(f52(f177(v4)?)))
        }
    }
}

pub fn f103(v0: List<Adt39>) -> Option<Adt104> {
    let m378 = f182(v0, List::<Str>::nil());
    match m378 {
        None => {
            None::<Adt104>
        }
        Some(v1) => {
            Some(Adt104::C1(v1))
        }
    }
}

pub fn f104(p382: List<Adt39>, p383: List<Adt15>) -> R<Result<List<Adt15>, Adt104>> {
    let m381 = (p382, p383);
    {
        let mut m381 = m381;
        loop {
            let step = {
                let (v0, v1) = m381;
                let m379 = v0;
                match m379.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt15>), R<Result<List<Adt15>, Adt104>>>(Ok(Ok::<List<Adt15>, Adt104>(f183(v1, List::<Adt15>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m380 = f184(v2)?;
                        match m380 {
                            Ok(v5) => {
                                Ok::<(List<Adt39>, List<Adt15>), R<Result<List<Adt15>, Adt104>>>((v3, f183(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt39>, List<Adt15>), R<Result<List<Adt15>, Adt104>>>(Ok(Err::<List<Adt15>, Adt104>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("region"), f185(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f186()), List::cons((Str::lit("limit"), f185(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f185(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt9)>::nil())))))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("region")), List::cons(Adt9::C3(Str::lit("query")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil()))))
}

pub fn f106() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil())))
}

pub fn f107() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("$schema"), Adt9::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt9::C5(List::cons((Str::lit("address"), Adt9::C5(List::cons((Str::lit("$anchor"), Adt9::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("street"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("city"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("name"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("address"), Adt9::C5(List::cons((Str::lit("$ref"), Adt9::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("contactMethod"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt9::C4(List::cons(Adt9::C3(Str::lit("phone")), List::cons(Adt9::C3(Str::lit("email")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("phone"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("email"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))))))), List::cons((Str::lit("allOf"), Adt9::C4(List::cons(Adt9::C5(List::cons((Str::lit("anyOf"), Adt9::C4(List::cons(Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("phone")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())), List::cons(Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("email")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil())), List::<Adt9>::nil()))), List::cons((Str::lit("if"), Adt9::C5(List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("contactMethod"), Adt9::C5(List::cons((Str::lit("const"), Adt9::C3(Str::lit("phone"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("contactMethod")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("then"), Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("phone")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("else"), Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("email")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))))))
}

pub fn f108() -> R<Adt8> {
    Ok(Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("name"), f186()), List::cons((Str::lit("label"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f119(1u64)?), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("count"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f119(0u64)?), List::cons((Str::lit("maximum"), f119(10u64)?), List::<(Str, Adt9)>::nil()))))), List::cons((Str::lit("mode"), Adt9::C5(List::cons((Str::lit("enum"), Adt9::C4(List::cons(Adt9::C3(Str::lit("fast")), List::cons(Adt9::C3(Str::lit("slow")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("tags"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("maxItems"), f119(3u64)?), List::<(Str, Adt9)>::nil()))))), List::cons((Str::lit("nested"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("flag"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("boolean"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))))))))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("name")), List::<Adt9>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))
}

pub fn f109() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("target"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("zone"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("x-mcp-header"), Adt9::C3(Str::lit("Zone"))), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::<Adt9>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f110(v0: Str, v1: Str, v2: Adt8, v3: Adt8) -> Adt39 {
    Adt39::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), v2, Some(v3), None::<Adt40>, None::<Adt8>)
}

pub fn f111() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("sum"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("label"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("sum")), List::cons(Adt9::C3(Str::lit("label")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil()))))
}

pub fn f112(v0: Str, v1: Str) -> Adt39 {
    Adt39::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::<(Str, Adt9)>::nil())), List::<(Str, Adt9)>::nil()))), None::<Adt8>, None::<Adt40>, None::<Adt8>)
}

pub fn f113(v0: Str, v1: Str, v2: Adt8) -> Adt39 {
    Adt39::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), v2, None::<Adt8>, None::<Adt40>, None::<Adt8>)
}

pub fn f114(v0: Str) -> Adt8 {
    f187(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((v0.clone(), f188(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(v0), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))))
}

pub fn f115() -> Adt8 {
    f187(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("seconds"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), Adt9::C2(Adt10::C0(0i64, 0i64))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("label"), f188(Str::lit("string"))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("seconds")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))))
}

pub fn f116() -> Adt8 {
    f187(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil())))
}

pub fn f117(v0: Adt39, v1: Adt25, v2: Fn5) -> R<Result<Adt36, Adt93>> {
    let m390 = f189(Adt9::C5({
        let m386 = match v0.clone() {
            Adt39::C0(_, _, _, _, h385, _, _, _) => {
                h385
            }
        };
        match m386 {
            Adt8::C0(h384) => {
                h384
            }
        }
    }), Adt9::C5({
        let m389 = match v0 {
            Adt39::C0(_, _, _, _, h388, _, _, _) => {
                h388
            }
        };
        match m389 {
            Adt8::C0(h387) => {
                h387
            }
        }
    }), f190(v1.clone()))?;
    match m390 {
        Adt110::C0 => {
            let c391: Fn5 = v2;
            c391.apply(v1)
        }
        Adt110::C1(v3, v4) => {
            Ok(Ok::<Adt36, Adt93>(Adt36::C2(f75(f191(v3, v4)?, true))))
        }
        Adt110::C2(_) => {
            Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt110::C3(_) => {
            Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt110::C4 => {
            Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f118(v0: Fn2, v1: Adt39, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    f192(v1, v0, v2)
}

pub fn f119(v0: u64) -> R<Adt9> {
    Ok(Adt9::C2(Adt10::C0(f193(v0, 0i64)?, 0i64)))
}

pub fn f120(v0: List<Adt1>, v1: Adt25) -> Result<Adt53, Adt93> {
    let m392 = v0;
    match m392.uncons() {
        None => {
            f194(1u64, f195(Str::lit("key"), v1))
        }
        Some((v2, v3)) => {
            let m393 = v3;
            match m393.uncons() {
                None => {
                    let a395 = match v2.clone() {
                        Adt1::C0(h394, _) => {
                            h394
                        }
                    };
                    let a396 = 1u64;
                    if nat_eq(a395, a396) {
                        f196(f43(Str::lit("read:"), f44(match v2 {
                            Adt1::C0(_, h397) => {
                                h397
                            }
                        })))
                    } else {
                        f197()
                    }
                }
                Some((_, _)) => {
                    f197()
                }
            }
        }
    }
}

pub fn f121(v0: List<Adt1>, v1: Adt25) -> Result<Adt53, Adt93> {
    let m398 = v0;
    match m398.uncons() {
        None => {
            f194(1u64, f195(Str::lit("first"), v1))
        }
        Some((v2, v3)) => {
            let m399 = v3;
            match m399.uncons() {
                None => {
                    let a401 = match v2.clone() {
                        Adt1::C0(h400, _) => {
                            h400
                        }
                    };
                    let a402 = 1u64;
                    if nat_eq(a401, a402) {
                        f194(2u64, f43(f44(match v2 {
                            Adt1::C0(_, h403) => {
                                h403
                            }
                        }), f195(Str::lit("second"), v1)))
                    } else {
                        f197()
                    }
                }
                Some((v4, v5)) => {
                    let m404 = v5;
                    match m404.uncons() {
                        None => {
                            let a406 = match v2.clone() {
                                Adt1::C0(h405, _) => {
                                    h405
                                }
                            };
                            let a407 = 1u64;
                            if nat_eq(a406, a407) {
                                let a409 = match v4.clone() {
                                    Adt1::C0(h408, _) => {
                                        h408
                                    }
                                };
                                let a410 = 2u64;
                                if nat_eq(a409, a410) {
                                    f196(f43(f43(Str::lit("chain:"), f44(match v2 {
                                        Adt1::C0(_, h411) => {
                                            h411
                                        }
                                    })), f43(Str::lit("|"), f44(match v4 {
                                        Adt1::C0(_, h412) => {
                                            h412
                                        }
                                    }))))
                                } else {
                                    f197()
                                }
                            } else {
                                f197()
                            }
                        }
                        Some((_, _)) => {
                            f197()
                        }
                    }
                }
            }
        }
    }
}

pub fn f122(v0: List<Adt1>, v1: u64) -> Result<Adt53, Adt93> {
    let m413 = v0;
    match m413.uncons() {
        None => {
            f194(v1, Str::lit("edge"))
        }
        Some((_, _)) => {
            f197()
        }
    }
}

pub fn f123(v0: Fn4, v1: Adt39, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    f198(v1, {
        let c414: Fn4 = v0;
        c414.apply(v2)?
    })
}

pub fn f124(v0: Adt25, v1: Str) -> bool {
    let a416 = match v0 {
        Adt25::C0(_, h415, _, _, _) => {
            h415
        }
    };
    let a417 = v1;
    equal(a416, a417)
}

pub fn f125(v0: Adt25) -> Result<Adt105, Adt93> {
    let m419 = f157(Str::lit("user_name"), Adt106::C0, match v0 {
        Adt25::C0(_, _, _, h418, _) => {
            h418
        }
    });
    match m419 {
        Adt108::C0 => {
            f201(f160(Str::lit("user_name"), f202()))
        }
        Adt108::C1 => {
            f203()
        }
        Adt108::C2(v1) => {
            f199(f200(List::cons(Str::lit("Hello, "), List::cons(f159(v1, Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("!"), List::<Str>::nil())))))
        }
    }
}

pub fn f126(v0: Adt25) -> Result<Adt105, Adt93> {
    let m421 = f157(Str::lit("capital_question"), Adt106::C1, match v0 {
        Adt25::C0(_, _, _, h420, _) => {
            h420
        }
    });
    match m421 {
        Adt108::C0 => {
            f201(f160(Str::lit("capital_question"), f204(Str::lit("What is the capital of France?"), 100u64)))
        }
        Adt108::C1 => {
            f203()
        }
        Adt108::C2(v1) => {
            f199(f200(List::cons(Str::lit("Sampling result: "), List::cons(f159(v1, Str::lit("text"), Str::lit("no response")), List::<Str>::nil()))))
        }
    }
}

pub fn f127(v0: Adt25) -> R<Result<Adt105, Adt93>> {
    let m423 = f157(Str::lit("client_roots"), Adt106::C2, match v0 {
        Adt25::C0(_, _, _, h422, _) => {
            h422
        }
    });
    match m423 {
        Adt108::C0 => {
            Ok(f201(f160(Str::lit("client_roots"), f207())))
        }
        Adt108::C1 => {
            Ok(f203())
        }
        Adt108::C2(v1) => {
            Ok(f199(f200(List::cons(Str::lit("Found "), List::cons(f205(f206(v1))?, List::cons(Str::lit(" root(s)"), List::<Str>::nil()))))))
        }
    }
}

pub fn f128(v0: Adt25) -> Result<Adt105, Adt93> {
    f208(v0, Str::lit("request-state"), Str::lit("state-ok: requestState validated"))
}

pub fn f129(v0: Adt25) -> R<Result<Adt105, Adt93>> {
    if f209(v0.clone(), Str::lit("multiple-inputs")) {
        f210(f157(Str::lit("user_name"), Adt106::C0, match v0.clone() {
            Adt25::C0(_, _, _, h424, _) => {
                h424
            }
        }), f157(Str::lit("greeting"), Adt106::C1, match v0.clone() {
            Adt25::C0(_, _, _, h425, _) => {
                h425
            }
        }), f157(Str::lit("client_roots"), Adt106::C2, match v0 {
            Adt25::C0(_, _, _, h426, _) => {
                h426
            }
        }))
    } else {
        Ok(f211(f212(Adt108::C0, Adt108::C0, Adt108::C0), Str::lit("multiple-inputs")))
    }
}

pub fn f130(v0: Adt25) -> R<Result<Adt105, Adt93>> {
    let v1: u64 = f213(f214(v0.clone()))?;
    let a427 = v1;
    let a428 = 49u64;
    if nat_eq(a427, a428) {
        Ok(f215(v0))
    } else {
        let a429 = v1;
        let a430 = 50u64;
        if nat_eq(a429, a430) {
            Ok(f216(v0))
        } else {
            Ok(f211(f160(Str::lit("step1"), f217()), Str::lit("1")))
        }
    }
}

pub fn f131(v0: Adt25) -> Result<Adt105, Adt93> {
    f208(v0, Str::lit("tamper-test"), Str::lit("integrity-ok: state verified"))
}

pub fn f132(v0: Adt25) -> Result<Adt105, Adt93> {
    let m432 = match v0.clone() {
        Adt25::C0(_, _, _, h431, _) => {
            h431
        }
    };
    match m432 {
        None => {
            f218(f219({
                let m435 = match v0 {
                    Adt25::C0(h434, _, _, _, _) => {
                        h434
                    }
                };
                match m435 {
                    Adt20::C0(_, h433, _, _, _, _) => {
                        h433
                    }
                }
            }))
        }
        Some(v1) => {
            let m437 = match v1.clone() {
                Adt8::C0(h436) => {
                    h436
                }
            };
            match m437.uncons() {
                None => {
                    f218(f219({
                        let m440 = match v0 {
                            Adt25::C0(h439, _, _, _, _) => {
                                h439
                            }
                        };
                        match m440 {
                            Adt20::C0(_, h438, _, _, _, _) => {
                                h438
                            }
                        }
                    }))
                }
                Some((_, _)) => {
                    f199(f200(List::cons(Str::lit("capabilities-ok: received "), List::cons({
                        let a442 = f220(match v1 {
                            Adt8::C0(h441) => {
                                h441
                            }
                        });
                        let a443 = Str::lit(",");
                        join(a442, a443)
                    }, List::<Str>::nil()))))
                }
            }
        }
    }
}

pub fn f133(v0: List<Adt42>) -> Adt53 {
    f137(v0, None::<Adt9>, false, List::<Adt54>::nil())
}

pub fn f134() -> Adt42 {
    f143(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f135(v0: Str, v1: Str) -> Adt42 {
    Adt42::C2(Adt47::C0(v0, v1, None::<Adt44>, None::<Adt8>))
}

pub fn f136(v0: Str, v1: Str, v2: Str) -> Adt42 {
    Adt42::C4(Adt49::C0(Adt50::C0(Adt51::C0(v0, Some(v1), v2, None::<Adt8>)), None::<Adt44>, None::<Adt8>))
}

pub fn f137(v0: List<Adt42>, v1: Option<Adt9>, v2: bool, v3: List<Adt54>) -> Adt53 {
    Adt53::C0(Adt41::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f138(v0: Adt25, v1: List<Adt54>) -> Adt53 {
    f137(List::cons(f90(f221({
        let m446 = match v0 {
            Adt25::C0(h445, _, _, _, _) => {
                h445
            }
        };
        match m446 {
            Adt20::C0(_, _, _, _, h444, _) => {
                h444
            }
        }
    })), List::<Adt42>::nil()), None::<Adt9>, false, v1)
}

pub fn f139() -> R<List<Adt54>> {
    Ok(List::cons(f222(0u64, 100u64)?, List::cons(f222(50u64, 100u64)?, List::cons(f222(100u64, 100u64)?, List::<Adt54>::nil()))))
}

pub fn f140() -> R<List<Adt54>> {
    Ok(List::cons(f222(50u64, 100u64)?, List::cons(f222(10u64, 100u64)?, List::<Adt54>::nil())))
}

pub fn f141() -> R<Adt9> {
    Ok(Adt9::C5(List::cons((Str::lit("sum"), f119(3u64)?), List::cons((Str::lit("label"), Adt9::C3(Str::lit("three"))), List::<(Str, Adt9)>::nil()))))
}

pub fn f142() -> Adt9 {
    Adt9::C5(List::cons((Str::lit("sum"), Adt9::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt9::C3(Str::lit("three"))), List::<(Str, Adt9)>::nil())))
}

pub fn f143(v0: Str, v1: Str) -> Adt42 {
    Adt42::C1(Adt46::C0(v0, v1, None::<Adt44>, None::<Adt8>))
}

pub fn f144() -> Str {
    let m448 = {
        let a447 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a447)
    };
    match m448 {
        None => {
            Str::lit("unreachable")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f145(v0: Adt25) -> Result<Adt53, Adt93> {
    if f38(v0.clone(), Str::lit("test_missing_capability")) {
        let m454 = {
            let m453 = {
                let m452 = match v0 {
                    Adt25::C0(h451, _, _, _, _) => {
                        h451
                    }
                };
                match m452 {
                    Adt20::C0(_, h450, _, _, _, _) => {
                        h450
                    }
                }
            };
            match m453 {
                Adt21::C0(_, _, h449, _, _) => {
                    h449
                }
            }
        };
        match m454 {
            None => {
                Err::<Adt53, Adt93>(Adt93::C8(f223()))
            }
            Some(_) => {
                Ok::<Adt53, Adt93>(f224())
            }
        }
    } else {
        if f38(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt53, Adt93>(f133(List::cons(f90(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt42>::nil())))
        } else {
            if f38(v0, Str::lit("test_arguments")) {
                Ok::<Adt53, Adt93>(f133(List::cons(f90(Str::lit("arguments accepted")), List::<Adt42>::nil())))
            } else {
                Ok::<Adt53, Adt93>(f224())
            }
        }
    }
}

pub fn f146(v0: Adt13, v1: List<Adt48>, v2: Adt24) -> Result<Adt36, Adt93> {
    let m456 = match v2 {
        Adt24::C0(_, h455) => {
            h455
        }
    };
    match m456 {
        None => {
            Ok::<Adt36, Adt93>(Adt36::C4(Adt55::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt36, Adt93>(Adt93::C5(v3))
        }
    }
}

pub fn f147() -> Adt13 {
    Adt13::C0(300000u64, Adt14::C0)
}

pub fn f148() -> List<Adt48> {
    f225(f226())
}

pub fn f149(v0: Adt13, v1: Fn6, v2: Adt26) -> R<Result<Adt36, Adt93>> {
    Ok(f227(v0, match v2.clone() {
        Adt26::C0(_, h457, _, _) => {
            h457
        }
    }, {
        let c458: Fn6 = v1;
        c458.apply(match v2 {
            Adt26::C0(_, h459, _, _) => {
                h459
            }
        })?
    }))
}

pub fn f150() -> Adt13 {
    Adt13::C0(300000u64, Adt14::C1)
}

pub fn f151(v0: Str) -> R<Option<List<Adt50>>> {
    let m460 = f228(v0.clone(), f226());
    match m460 {
        None => {
            f229(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f152(v0: Adt13, v1: List<Adt58>, v2: Adt24) -> Result<Adt36, Adt93> {
    let m462 = match v2 {
        Adt24::C0(_, h461) => {
            h461
        }
    };
    match m462 {
        None => {
            Ok::<Adt36, Adt93>(Adt36::C6(Adt57::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt36, Adt93>(Adt93::C5(v3))
        }
    }
}

pub fn f153() -> List<Adt58> {
    List::cons(Adt58::C0(Str::lit("template"), Some(Str::lit("Resource Template")), None::<List<Adt5>>, f230(), Some(Str::lit("A resource template with parameter substitution")), Some(Str::lit("application/json")), None::<Adt44>, None::<Adt8>), List::<Adt58>::nil())
}

pub fn f154(v0: Adt13, v1: List<Adt60>, v2: Adt24) -> Result<Adt36, Adt93> {
    let m464 = match v2 {
        Adt24::C0(_, h463) => {
            h463
        }
    };
    match m464 {
        None => {
            Ok::<Adt36, Adt93>(Adt36::C7(Adt59::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt36, Adt93>(Adt93::C5(v3))
        }
    }
}

pub fn f155() -> List<Adt60> {
    List::cons(f231(Str::lit("test_simple_prompt"), Str::lit("Simple Test Prompt"), Str::lit("A simple prompt without arguments"), None::<List<Adt61>>), List::cons(f231(Str::lit("test_prompt_with_arguments"), Str::lit("Prompt With Arguments"), Str::lit("A prompt with required arguments"), Some(List::cons(f232(Str::lit("arg1"), Str::lit("First test argument")), List::cons(f232(Str::lit("arg2"), Str::lit("Second test argument")), List::<Adt61>::nil())))), List::cons(f231(Str::lit("test_prompt_with_embedded_resource"), Str::lit("Prompt With Embedded Resource"), Str::lit("A prompt that includes an embedded resource"), Some(List::cons(f232(Str::lit("resourceUri"), Str::lit("URI of the resource to embed")), List::<Adt61>::nil()))), List::cons(f231(Str::lit("test_prompt_with_image"), Str::lit("Prompt With Image"), Str::lit("A prompt that includes image content"), None::<List<Adt61>>), List::<Adt60>::nil()))))
}

pub fn f156() -> Str {
    Str::lit("test_input_required_result_prompt")
}

pub fn f157(v0: Str, v1: Adt106, v2: Option<Adt8>) -> Adt108 {
    let m465 = v2;
    match m465 {
        None => {
            Adt108::C0
        }
        Some(v3) => {
            let m466 = f233(v0, v3);
            match m466 {
                Adt115::C0 => {
                    Adt108::C0
                }
                Adt115::C1(v4) => {
                    let m467 = f234(v4);
                    match m467 {
                        None => {
                            Adt108::C1
                        }
                        Some(v5) => {
                            if f235(v1, v5.clone()) {
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

pub fn f158(v0: Str) -> Adt62 {
    Adt62::C0(None::<Str>, List::cons(Adt63::C0(Adt45::C0, Adt42::C0(Adt43::C0(f200(List::cons(Str::lit("Prompt with context: "), List::cons(v0, List::<Str>::nil()))), None::<Adt44>, None::<Adt8>))), List::<Adt63>::nil()))
}

pub fn f159(v0: Adt8, v1: Str, v2: Str) -> Str {
    let m468 = f233(Str::lit("content"), v0);
    match m468 {
        Adt115::C0 => {
            v2
        }
        Adt115::C1(v3) => {
            let m469 = f234(v3);
            match m469 {
                None => {
                    v2
                }
                Some(v4) => {
                    f236(v1, v4, v2)
                }
            }
        }
    }
}

pub fn f160(v0: Str, v1: Adt68) -> List<(Str, Adt68)> {
    List::cons((v0, v1), List::<(Str, Adt68)>::nil())
}

pub fn f161(v0: Str, v1: Str, v2: Str) -> Adt68 {
    Adt68::C0(Adt69::C0(Adt70::C0(None::<Adt71>, v0, f237(v1, v2))))
}

pub fn f162(v0: List<Adt60>, v1: Fn7, v2: Adt27) -> Result<Adt36, Adt93> {
    let m471 = f238(match v2.clone() {
        Adt27::C0(_, h470, _, _, _) => {
            h470
        }
    }, v0);
    match m471 {
        None => {
            Err::<Adt36, Adt93>(Adt93::C1(match v2 {
                Adt27::C0(_, h472, _, _, _) => {
                    h472
                }
            }))
        }
        Some(v3) => {
            let m474 = f239(f240(v3), f241(match v2.clone() {
                Adt27::C0(_, _, h473, _, _) => {
                    h473
                }
            }));
            match m474 {
                None => {
                    Ok::<Adt36, Adt93>(Adt36::C8({
                        let c477: Fn7 = v1;
                        c477.apply(match v2.clone() {
                            Adt27::C0(_, h478, _, _, _) => {
                                h478
                            }
                        }, f241(match v2 {
                            Adt27::C0(_, _, h479, _, _) => {
                                h479
                            }
                        }))
                    }))
                }
                Some(v4) => {
                    Err::<Adt36, Adt93>(Adt93::C4({
                        let a475 = List::cons(Str::lit("missing required argument:"), List::cons(v4, List::<Str>::nil()));
                        let a476 = Str::lit(" ");
                        join(a475, a476)
                    }))
                }
            }
        }
    }
}

pub fn f163(v0: Str, v1: List<(Str, Str)>) -> Adt62 {
    let a480 = v0.clone();
    let a481 = Str::lit("test_simple_prompt");
    if equal(a480, a481) {
        f242(List::cons(f243(Str::lit("This is a simple prompt for testing.")), List::<Adt63>::nil()))
    } else {
        let a482 = v0.clone();
        let a483 = Str::lit("test_prompt_with_arguments");
        if equal(a482, a483) {
            f242(List::cons(f243({
                let a484 = List::cons(Str::lit("Prompt with arguments: arg1='"), List::cons(f244(Str::lit("arg1"), v1.clone()), List::cons(Str::lit("', arg2='"), List::cons(f244(Str::lit("arg2"), v1), List::cons(Str::lit("'"), List::<Str>::nil())))));
                let a485 = Str::lit("");
                join(a484, a485)
            }), List::<Adt63>::nil()))
        } else {
            let a486 = v0.clone();
            let a487 = Str::lit("test_prompt_with_embedded_resource");
            if equal(a486, a487) {
                f242(List::cons(Adt63::C0(Adt45::C0, Adt42::C4(Adt49::C0(f245(f244(Str::lit("resourceUri"), v1), Str::lit("text/plain"), Str::lit("Embedded resource content for testing.")), None::<Adt44>, None::<Adt8>))), List::cons(f243(Str::lit("Please process the embedded resource above.")), List::<Adt63>::nil())))
            } else {
                let a488 = v0;
                let a489 = Str::lit("test_prompt_with_image");
                if equal(a488, a489) {
                    f242(List::cons(Adt63::C0(Adt45::C0, Adt42::C1(Adt46::C0(f246(), Str::lit("image/png"), None::<Adt44>, None::<Adt8>))), List::cons(f243(Str::lit("Please analyze the image above.")), List::<Adt63>::nil())))
                } else {
                    f242(List::<Adt63>::nil())
                }
            }
        }
    }
}

pub fn f164(v0: Fn8, v1: Adt28) -> R<Result<Adt36, Adt93>> {
    let m491 = {
        let c490: Fn8 = v0;
        c490.apply(v1)?
    };
    match m491 {
        Ok(v3) => {
            Ok(Ok::<Adt36, Adt93>(Adt36::C9(f247(v3)?)))
        }
        Err(v2) => {
            Ok(Err::<Adt36, Adt93>(v2))
        }
    }
}

pub fn f165(v0: Adt28) -> R<Result<List<Str>, Adt93>> {
    let m493 = match v0.clone() {
        Adt28::C0(_, h492, _, _) => {
            h492
        }
    };
    match m493 {
        Adt29::C0(v1) => {
            let m495 = f238(match v1.clone() {
                Adt30::C0(h494, _) => {
                    h494
                }
            }, f155());
            match m495 {
                None => {
                    Ok(Err::<List<Str>, Adt93>(Adt93::C1(match v1 {
                        Adt30::C0(h496, _) => {
                            h496
                        }
                    })))
                }
                Some(v2) => {
                    if f248(f240(v2), {
                        let m499 = match v0.clone() {
                            Adt28::C0(_, _, h498, _) => {
                                h498
                            }
                        };
                        match m499 {
                            Adt31::C0(h497, _) => {
                                h497
                            }
                        }
                    }) {
                        Ok(Ok::<List<Str>, Adt93>(f249({
                            let m502 = match v0.clone() {
                                Adt28::C0(_, _, h501, _) => {
                                    h501
                                }
                            };
                            match m502 {
                                Adt31::C0(_, h500) => {
                                    h500
                                }
                            }
                        }, f250(match v1 {
                            Adt30::C0(h503, _) => {
                                h503
                            }
                        }, {
                            let m506 = match v0 {
                                Adt28::C0(_, _, h505, _) => {
                                    h505
                                }
                            };
                            match m506 {
                                Adt31::C0(h504, _) => {
                                    h504
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
            let a507 = v3.clone();
            let a508 = f230();
            if equal(a507, a508) {
                let a512 = {
                    let m511 = match v0.clone() {
                        Adt28::C0(_, _, h510, _) => {
                            h510
                        }
                    };
                    match m511 {
                        Adt31::C0(h509, _) => {
                            h509
                        }
                    }
                };
                let a513 = Str::lit("id");
                if equal(a512, a513) {
                    Ok(Ok::<List<Str>, Adt93>(f249({
                        let m516 = match v0 {
                            Adt28::C0(_, _, h515, _) => {
                                h515
                            }
                        };
                        match m516 {
                            Adt31::C0(_, h514) => {
                                h514
                            }
                        }
                    }, f251())?))
                } else {
                    Ok(Err::<List<Str>, Adt93>(Adt93::C4(Str::lit("the template declares no such argument"))))
                }
            } else {
                Ok(Err::<List<Str>, Adt93>(Adt93::C2(v3)))
            }
        }
    }
}

pub fn f166(v0: Bytes, p526: u64, p527: u64, p528: List<Adt100>) -> R<Result<(List<Adt100>, u64), Adt2>> {
    let m525 = (p526, (p527, p528));
    {
        let mut m525 = m525;
        loop {
            let step = {
                let (v1, (v2, v3)) = m525;
                let m517 = v2;
                if m517 == 0 {
                    Err::<(u64, (u64, List<Adt100>)), R<Result<(List<Adt100>, u64), Adt2>>>(Ok(Ok::<(List<Adt100>, u64), Adt2>((f252(v3, List::<Adt100>::nil()), v1))))
                } else {
                    let v4 = m517 - 1;
                    let m518 = f7(v0.clone(), v1)?;
                    match m518 {
                        None => {
                            Err::<(u64, (u64, List<Adt100>)), R<Result<(List<Adt100>, u64), Adt2>>>(Ok(Err::<(List<Adt100>, u64), Adt2>(Adt2::C0(v1))))
                        }
                        Some(v5) => {
                            let m520 = f7(v0.clone(), match v5.clone() {
                                Adt95::C0(_, h519) => {
                                    h519
                                }
                            })?;
                            match m520 {
                                None => {
                                    Err::<(u64, (u64, List<Adt100>)), R<Result<(List<Adt100>, u64), Adt2>>>(Ok(Err::<(List<Adt100>, u64), Adt2>(Adt2::C0(match v5 {
                                        Adt95::C0(_, h521) => {
                                            h521
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt100>)), R<Result<(List<Adt100>, u64), Adt2>>>((match v6.clone() {
                                        Adt95::C0(_, h522) => {
                                            h522
                                        }
                                    }, (v4, List::cons(Adt100::C0(f25(match v5 {
                                        Adt95::C0(h523, _) => {
                                            h523
                                        }
                                    })?, f25(match v6 {
                                        Adt95::C0(h524, _) => {
                                            h524
                                        }
                                    })?), v3))))
                                }
                            }
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

pub fn f167(v0: Bytes, v1: Adt95, v2: Adt95, v3: (List<Adt100>, u64)) -> R<Result<Adt99, Adt2>> {
    let m530 = f7(v0.clone(), {
        let (_, h529) = v3.clone();
        h529
    })?;
    match m530 {
        None => {
            Ok(Err::<Adt99, Adt2>(Adt2::C0({
                let (_, h531) = v3;
                h531
            })))
        }
        Some(v4) => {
            let m533 = f23(v0.clone(), match v4.clone() {
                Adt95::C0(_, h532) => {
                    h532
                }
            })?;
            match m533 {
                None => {
                    Ok(Err::<Adt99, Adt2>(Adt2::C0(match v4 {
                        Adt95::C0(_, h534) => {
                            h534
                        }
                    })))
                }
                Some(v5) => {
                    let m538 = f253(v0.clone(), {
                        let a536 = match v4.clone() {
                            Adt95::C0(_, h535) => {
                                h535
                            }
                        };
                        let a537 = 4u64;
                        nat_add(a536, a537)?
                    })?;
                    match m538 {
                        None => {
                            Ok(Err::<Adt99, Adt2>(Adt2::C0({
                                let a540 = match v4 {
                                    Adt95::C0(_, h539) => {
                                        h539
                                    }
                                };
                                let a541 = 4u64;
                                nat_add(a540, a541)?
                            })))
                        }
                        Some(v6) => {
                            f254(v0, Adt99::C0(f25(match v1 {
                                Adt95::C0(h542, _) => {
                                    h542
                                }
                            })?, f25(match v2 {
                                Adt95::C0(h543, _) => {
                                    h543
                                }
                            })?, {
                                let (h544, _) = v3;
                                h544
                            }, match v4 {
                                Adt95::C0(h545, _) => {
                                    h545
                                }
                            }, v5, List::<List<u64>>::nil(), match v6.clone() {
                                Adt95::C0(h546, _) => {
                                    h546
                                }
                            }, Adt101::C0(0i64, Str::lit(""), Str::lit(""), f255())), match v6 {
                                Adt95::C0(_, h547) => {
                                    h547
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

pub fn f168(p552: List<Adt100>, p553: List<u64>) -> List<u64> {
    let m551 = (p552, p553);
    {
        let mut m551 = m551;
        loop {
            let step = {
                let (v0, v1) = m551;
                let m548 = v0;
                match m548.uncons() {
                    None => {
                        Err::<(List<Adt100>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt100>, List<u64>), List<u64>>((v3, {
                            let a549 = f256(v2);
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

pub fn f169(p556: List<Adt100>, p557: List<Adt100>) -> List<Adt100> {
    let m555 = (p556, p557);
    {
        let mut m555 = m555;
        loop {
            let step = {
                let (v0, v1) = m555;
                let m554 = v0;
                match m554.uncons() {
                    None => {
                        Err::<(List<Adt100>, List<Adt100>), List<Adt100>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt100>, List<Adt100>), List<Adt100>>((v3, List::cons(v2, v1)))
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

pub fn f170(p562: List<List<u64>>, p563: List<u64>) -> List<u64> {
    let m561 = (p562, p563);
    {
        let mut m561 = m561;
        loop {
            let step = {
                let (v0, v1) = m561;
                let m558 = v0;
                match m558.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a559 = f257(v2);
                            let a560 = v1;
                            append_list(a559, a560)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m561 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(p566: List<List<u64>>, p567: List<List<u64>>) -> List<List<u64>> {
    let m565 = (p566, p567);
    {
        let mut m565 = m565;
        loop {
            let step = {
                let (v0, v1) = m565;
                let m564 = v0;
                match m564.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f172() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f173() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f174() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f175() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f176() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f177(v0: Adt107) -> R<Adt103> {
    f258(f259(v0.clone()), f260(None::<Adt109>, f261(v0)))
}

pub fn f178(v0: Adt3, v1: Fn1, v2: Adt99) -> R<Adt103> {
    if f262(match v2.clone() {
        Adt99::C0(h568, _, _, _, _, _, _, _) => {
            h568
        }
    }) {
        f263(match v2.clone() {
            Adt99::C0(_, _, _, _, _, _, _, h569) => {
                h569
            }
        }, f264(v0, match v2.clone() {
            Adt99::C0(_, _, _, _, _, _, h570, _) => {
                h570
            }
        }), Fn3::F265(v1, v2.clone()), match v2.clone() {
            Adt99::C0(_, _, _, _, h571, _, _, _) => {
                h571
            }
        }, match v2.clone() {
            Adt99::C0(_, _, h572, _, _, _, _, _) => {
                h572
            }
        }, f266(match v2 {
            Adt99::C0(_, _, _, h573, _, _, _, _) => {
                h573
            }
        })?)
    } else {
        Ok(f267())
    }
}

pub fn f179(v0: List<Adt100>, v1: List<List<u64>>) -> R<Option<Adt107>> {
    let m574 = f268(v0, f269())?;
    match m574 {
        Adt119::C0 => {
            Ok(None::<Adt107>)
        }
        Adt119::C1 => {
            Ok(Some(Adt107::C0))
        }
        Adt119::C2 => {
            Ok(Some(Adt107::C0))
        }
        Adt119::C3(v2) => {
            f270(f271(v2), v1, Adt107::C0)
        }
    }
}

pub fn f180(v0: List<Adt100>, v1: List<List<u64>>) -> R<Option<Adt107>> {
    let m575 = f268(v0, f272())?;
    match m575 {
        Adt119::C0 => {
            Ok(None::<Adt107>)
        }
        Adt119::C1 => {
            Ok(Some(Adt107::C1))
        }
        Adt119::C2 => {
            Ok(Some(Adt107::C1))
        }
        Adt119::C3(v2) => {
            f270(f273(v2), v1, Adt107::C1)
        }
    }
}

pub fn f181(v0: Adt3, v1: Fn3, v2: Adt99) -> R<Adt96> {
    if f262(match v2.clone() {
        Adt99::C0(h576, _, _, _, _, _, _, _) => {
            h576
        }
    }) {
        f274(f264(v0, match v2.clone() {
            Adt99::C0(_, _, _, _, _, _, h577, _) => {
                h577
            }
        }), v1, match v2.clone() {
            Adt99::C0(_, _, _, _, h578, _, _, _) => {
                h578
            }
        }, match v2.clone() {
            Adt99::C0(_, _, h579, _, _, _, _, _) => {
                h579
            }
        }, f266(match v2 {
            Adt99::C0(_, _, _, h580, _, _, _, _) => {
                h580
            }
        })?)
    } else {
        Ok(Adt96::C0(f52(f267())))
    }
}

pub fn f182(p586: List<Adt39>, p587: List<Str>) -> Option<Str> {
    let m585 = (p586, p587);
    {
        let mut m585 = m585;
        loop {
            let step = {
                let (v0, v1) = m585;
                let m581 = v0;
                match m581.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f275(match v2.clone() {
                            Adt39::C0(h582, _, _, _, _, _, _, _) => {
                                h582
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt39>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt39::C0(h583, _, _, _, _, _, _, _) => {
                                    h583
                                }
                            }))
                        } else {
                            Ok::<(List<Adt39>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt39::C0(h584, _, _, _, _, _, _, _) => {
                                    h584
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m585 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(p590: List<Adt15>, p591: List<Adt15>) -> List<Adt15> {
    let m589 = (p590, p591);
    {
        let mut m589 = m589;
        loop {
            let step = {
                let (v0, v1) = m589;
                let m588 = v0;
                match m588.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt15>), List<Adt15>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt15>, List<Adt15>), List<Adt15>>((v3, List::cons(v2, v1)))
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

pub fn f184(v0: Adt39) -> R<Result<List<Adt15>, Adt104>> {
    let m594 = f276(match v0.clone() {
        Adt39::C0(h592, _, _, _, _, _, _, _) => {
            h592
        }
    }, f277(match v0.clone() {
        Adt39::C0(h593, _, _, _, _, _, _, _) => {
            h593
        }
    })?);
    match m594 {
        None => {
            f278(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt15>, Adt104>(v1))
        }
    }
}

pub fn f185(v0: Str, v1: Str) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil())))
}

pub fn f186() -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))
}

pub fn f187(v0: List<(Str, Adt9)>) -> Adt8 {
    Adt8::C0(v0)
}

pub fn f188(v0: Str) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v0)), List::<(Str, Adt9)>::nil()))
}

pub fn f189(v0: Adt9, v1: Adt9, v2: Adt9) -> R<Adt110> {
    f279(v0, f280(), Adt112::C0, List::cons(Adt113::C0(v1, v2, List::<Adt111>::nil()), List::<Adt113>::nil()))
}

pub fn f190(v0: Adt25) -> Adt9 {
    let m596 = match v0 {
        Adt25::C0(_, _, h595, _, _) => {
            h595
        }
    };
    match m596 {
        None => {
            Adt9::C5(List::<(Str, Adt9)>::nil())
        }
        Some(v1) => {
            Adt9::C5(match v1 {
                Adt8::C0(h597) => {
                    h597
                }
            })
        }
    }
}

pub fn f191(v0: List<Adt111>, v1: Str) -> R<Str> {
    let v2: Str = f281(v0)?;
    let a600 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a598 = v2.clone();
        let a599 = Str::lit("");
        if equal(a598, a599) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a601 = Str::lit(" ");
    Ok(join(a600, a601))
}

pub fn f192(v0: Adt39, v1: Fn2, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    let m603 = {
        let c602: Fn2 = v1;
        c602.apply(v2)?
    };
    match m603 {
        Ok(v4) => {
            f282(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt36, Adt93>(v3))
        }
    }
}

pub fn f193(p608: u64, p609: i64) -> R<i64> {
    let m607 = (p608, p609);
    {
        let mut m607 = m607;
        loop {
            let step = {
                let (v0, v1) = m607;
                let m604 = v0;
                if m604 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m604 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a605 = v1;
                        let a606 = -1i64;
                        int_sub(a605, a606)?
                    }))
                }
            };
            match step {
                Ok(next) => { m607 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194(v0: u64, v1: Str) -> Result<Adt53, Adt93> {
    Err::<Adt53, Adt93>(Adt93::C15(f41(v0, v1)))
}

pub fn f195(v0: Str, v1: Adt25) -> Str {
    let m611 = match v1 {
        Adt25::C0(_, _, h610, _, _) => {
            h610
        }
    };
    match m611 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m612 = f233(v0, v2);
            match m612 {
                Adt115::C0 => {
                    Str::lit("")
                }
                Adt115::C1(v3) => {
                    let m613 = f283(v3);
                    match m613 {
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

pub fn f196(v0: Str) -> Result<Adt53, Adt93> {
    Ok::<Adt53, Adt93>(f133(List::cons(f90(v0), List::<Adt42>::nil())))
}

pub fn f197() -> Result<Adt53, Adt93> {
    Err::<Adt53, Adt93>(f45())
}

pub fn f198(v0: Adt39, v1: Result<Adt105, Adt93>) -> R<Result<Adt36, Adt93>> {
    let m614 = v1;
    match m614 {
        Ok(v3) => {
            let m615 = v3;
            match m615 {
                Adt105::C0(v4) => {
                    f282(v0, v4)
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

pub fn f199(v0: Str) -> Result<Adt105, Adt93> {
    Ok::<Adt105, Adt93>(Adt105::C0(Adt53::C0(Adt41::C0(List::cons(Adt42::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>)), List::<Adt42>::nil()), None::<Adt9>, None::<bool>), List::<Adt54>::nil())))
}

pub fn f200(v0: List<Str>) -> Str {
    let a616 = v0;
    let a617 = Str::lit("");
    join(a616, a617)
}

pub fn f201(v0: List<(Str, Adt68)>) -> Result<Adt105, Adt93> {
    Ok::<Adt105, Adt93>(Adt105::C1(Adt85::C0(v0)))
}

pub fn f202() -> Adt68 {
    f161(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f203() -> Result<Adt105, Adt93> {
    Err::<Adt105, Adt93>(Adt93::C11)
}

pub fn f204(v0: Str, v1: u64) -> Adt68 {
    Adt68::C1(Adt73::C0(List::cons(f284(v0), List::<Adt74>::nil()), None::<Adt79>, None::<Str>, None::<Adt81>, None::<Adt10>, v1, None::<List<Str>>, None::<Adt8>, None::<List<Adt39>>, None::<Adt82>))
}

pub fn f205(v0: u64) -> R<Str> {
    let a618 = f285(v0, 0i64)?;
    Ok(format_i64(a618))
}

pub fn f206(v0: Adt8) -> u64 {
    let m619 = f233(Str::lit("roots"), v0);
    match m619 {
        Adt115::C0 => {
            0u64
        }
        Adt115::C1(v1) => {
            let m620 = f286(v1);
            match m620 {
                None => {
                    0u64
                }
                Some(v2) => {
                    let a621 = v2;
                    length_list(a621)
                }
            }
        }
    }
}

pub fn f207() -> Adt68 {
    Adt68::C2(Some(f62()))
}

pub fn f208(v0: Adt25, v1: Str, v2: Str) -> Result<Adt105, Adt93> {
    if f209(v0.clone(), v1.clone()) {
        let m623 = f157(Str::lit("confirm"), Adt106::C0, match v0 {
            Adt25::C0(_, _, _, h622, _) => {
                h622
            }
        });
        match m623 {
            Adt108::C0 => {
                f211(f160(Str::lit("confirm"), f288()), v1)
            }
            Adt108::C1 => {
                f203()
            }
            Adt108::C2(v3) => {
                if f287(v3) {
                    f199(v2)
                } else {
                    f211(f160(Str::lit("confirm"), f288()), v1)
                }
            }
        }
    } else {
        f211(f160(Str::lit("confirm"), f288()), v1)
    }
}

pub fn f209(v0: Adt25, v1: Str) -> bool {
    let m625 = match v0 {
        Adt25::C0(_, _, _, _, h624) => {
            h624
        }
    };
    match m625 {
        None => {
            false
        }
        Some(v2) => {
            let a626 = v2;
            let a627 = v1;
            equal(a626, a627)
        }
    }
}

pub fn f210(v0: Adt108, v1: Adt108, v2: Adt108) -> R<Result<Adt105, Adt93>> {
    if if f289(v0.clone()) {
        true
    } else {
        if f289(v1.clone()) {
            true
        } else {
            f289(v2.clone())
        }
    } {
        Ok(f203())
    } else {
        let m628 = f212(v0.clone(), v1.clone(), v2.clone());
        match m628.uncons() {
            None => {
                Ok(f199(f200(List::cons(Str::lit("Name: "), List::cons(f159(f290(v0), Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("; Greeting: "), List::cons(f159(f290(v1), Str::lit("text"), Str::lit("Hello there!")), List::cons(Str::lit("; Roots: "), List::cons(f205(f206(f290(v2)))?, List::<Str>::nil())))))))))
            }
            Some((_, _)) => {
                Ok(f211(f212(v0, v1, v2), Str::lit("multiple-inputs")))
            }
        }
    }
}

pub fn f211(v0: List<(Str, Adt68)>, v1: Str) -> Result<Adt105, Adt93> {
    Ok::<Adt105, Adt93>(Adt105::C1(Adt85::C2(v0, v1)))
}

pub fn f212(v0: Adt108, v1: Adt108, v2: Adt108) -> List<(Str, Adt68)> {
    let a631 = if f291(v0) {
        f160(Str::lit("user_name"), f202())
    } else {
        List::<(Str, Adt68)>::nil()
    };
    let a632 = {
        let a629 = if f291(v1) {
            f160(Str::lit("greeting"), f204(Str::lit("Generate a greeting"), 50u64))
        } else {
            List::<(Str, Adt68)>::nil()
        };
        let a630 = if f291(v2) {
            f160(Str::lit("client_roots"), f207())
        } else {
            List::<(Str, Adt68)>::nil()
        };
        append_list(a629, a630)
    };
    append_list(a631, a632)
}

pub fn f213(v0: Str) -> R<u64> {
    f58({
        let a633 = v0;
        utf8_encode(a633)
    }, 0u64)
}

pub fn f214(v0: Adt25) -> Str {
    let m635 = match v0 {
        Adt25::C0(_, _, _, _, h634) => {
            h634
        }
    };
    match m635 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f215(v0: Adt25) -> Result<Adt105, Adt93> {
    let m637 = f157(Str::lit("step1"), Adt106::C0, match v0 {
        Adt25::C0(_, _, _, h636, _) => {
            h636
        }
    });
    match m637 {
        Adt108::C0 => {
            f211(f160(Str::lit("step1"), f217()), Str::lit("1"))
        }
        Adt108::C1 => {
            f203()
        }
        Adt108::C2(v1) => {
            f211(f160(Str::lit("step2"), f292()), f200(List::cons(Str::lit("2"), List::cons(f159(v1, Str::lit("name"), Str::lit("unknown")), List::<Str>::nil()))))
        }
    }
}

pub fn f216(v0: Adt25) -> Result<Adt105, Adt93> {
    let m639 = f157(Str::lit("step2"), Adt106::C0, match v0.clone() {
        Adt25::C0(_, _, _, h638, _) => {
            h638
        }
    });
    match m639 {
        Adt108::C0 => {
            f211(f160(Str::lit("step2"), f292()), f214(v0))
        }
        Adt108::C1 => {
            f203()
        }
        Adt108::C2(v1) => {
            f199(f200(List::cons(Str::lit("Multi-round complete for "), List::cons(f293(f214(v0)), List::cons(Str::lit(" who likes "), List::cons(f159(v1, Str::lit("color"), Str::lit("unknown")), List::<Str>::nil()))))))
        }
    }
}

pub fn f217() -> Adt68 {
    f161(Str::lit("Step 1: What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f218(v0: List<(Str, Adt68)>) -> Result<Adt105, Adt93> {
    let m640 = v0.clone();
    match m640.uncons() {
        None => {
            f199(Str::lit("No supported capabilities declared"))
        }
        Some((_, _)) => {
            f211(v0, Str::lit("capabilities-test"))
        }
    }
}

pub fn f219(v0: Adt21) -> List<(Str, Adt68)> {
    let a643 = if f294(match v0.clone() {
        Adt21::C0(_, _, _, h641, _) => {
            h641
        }
    }) {
        f160(Str::lit("elicit_input"), f161(Str::lit("Elicitation input"), Str::lit("value"), Str::lit("string")))
    } else {
        List::<(Str, Adt68)>::nil()
    };
    let a644 = if f294(match v0 {
        Adt21::C0(_, _, h642, _, _) => {
            h642
        }
    }) {
        f160(Str::lit("sample_input"), f204(Str::lit("Sample request"), 50u64))
    } else {
        List::<(Str, Adt68)>::nil()
    };
    append_list(a643, a644)
}

pub fn f220(v0: List<(Str, Adt9)>) -> List<Str> {
    f295(f296(v0, List::<Str>::nil()), List::<Str>::nil())
}

pub fn f221(v0: Option<Adt23>) -> Str {
    let m645 = v0;
    match m645 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m646 = v1;
            match m646 {
                Adt23::C0(v2) => {
                    v2
                }
                Adt23::C1(v3) => {
                    let a647 = v3;
                    format_i64(a647)
                }
            }
        }
    }
}

pub fn f222(v0: u64, v1: u64) -> R<Adt54> {
    Ok(Adt54::C0(f297(v0)?, Some(f297(v1)?), Some({
        let a650 = List::cons(Str::lit("Completed step"), List::cons({
            let a648 = f285(v0, 0i64)?;
            format_i64(a648)
        }, List::cons(Str::lit("of"), List::cons({
            let a649 = f285(v1, 0i64)?;
            format_i64(a649)
        }, List::<Str>::nil()))));
        let a651 = Str::lit(" ");
        join(a650, a651)
    })))
}

pub fn f223() -> Adt21 {
    Adt21::C0(None::<Adt8>, None::<Adt8>, Some(f62()), None::<Adt8>, None::<Adt8>)
}

pub fn f224() -> Adt53 {
    f133(List::cons(f90(Str::lit("ok")), List::<Adt42>::nil()))
}

pub fn f225(v0: List<(Adt48, List<Adt50>)>) -> List<Adt48> {
    f298(f299(v0, List::<Adt48>::nil()), List::<Adt48>::nil())
}

pub fn f226() -> List<(Adt48, List<Adt50>)> {
    List::cons(f300(f301(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f245(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f300(f301(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f302(Str::lit("test://static-binary"), Str::lit("image/png"), f246())), List::cons(f300(f301(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f245(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f300(f301(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f245(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt48, List<Adt50>)>::nil()))))
}

pub fn f227(v0: Adt13, v1: Str, v2: Option<List<Adt50>>) -> Result<Adt36, Adt93> {
    let m652 = v2;
    match m652 {
        None => {
            Err::<Adt36, Adt93>(Adt93::C2(v1))
        }
        Some(v3) => {
            let m653 = v3.clone();
            match m653.uncons() {
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

pub fn f228(v0: Str, p662: List<(Adt48, List<Adt50>)>) -> Option<List<Adt50>> {
    let m661 = p662;
    {
        let mut m661 = m661;
        loop {
            let step = {
                let v1 = m661;
                let m654 = v1;
                match m654.uncons() {
                    None => {
                        Err::<List<(Adt48, List<Adt50>)>, Option<List<Adt50>>>(None::<List<Adt50>>)
                    }
                    Some((v2, v3)) => {
                        let a658 = {
                            let m657 = {
                                let (h656, _) = v2;
                                h656
                            };
                            match m657 {
                                Adt48::C0(_, _, _, h655, _, _, _, _, _) => {
                                    h655
                                }
                            }
                        };
                        let a659 = v0.clone();
                        if equal(a658, a659) {
                            Err::<List<(Adt48, List<Adt50>)>, Option<List<Adt50>>>(Some({
                                let (_, h660) = v2;
                                h660
                            }))
                        } else {
                            Ok::<List<(Adt48, List<Adt50>)>, Option<List<Adt50>>>(v3)
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

pub fn f229(v0: Str) -> R<Option<List<Adt50>>> {
    let m663 = f303(v0.clone())?;
    match m663 {
        None => {
            Ok(None::<List<Adt50>>)
        }
        Some(v1) => {
            let m664 = f304(v1)?;
            match m664 {
                None => {
                    Ok(None::<List<Adt50>>)
                }
                Some(v2) => {
                    Ok(Some(List::cons(f245(v0, Str::lit("application/json"), v2), List::<Adt50>::nil())))
                }
            }
        }
    }
}

pub fn f230() -> Str {
    Str::lit("test://template/{id}/data")
}

pub fn f231(v0: Str, v1: Str, v2: Str, v3: Option<List<Adt61>>) -> Adt60 {
    Adt60::C0(v0, Some(v1), None::<List<Adt5>>, Some(v2), v3, None::<Adt8>)
}

pub fn f232(v0: Str, v1: Str) -> Adt61 {
    Adt61::C0(v0, None::<Str>, Some(v1), Some(true))
}

pub fn f233(v0: Str, v1: Adt8) -> Adt115 {
    f305(v0, match v1 {
        Adt8::C0(h665) => {
            h665
        }
    })
}

pub fn f234(v0: Adt9) -> Option<Adt8> {
    let m666 = v0;
    match m666 {
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

pub fn f235(v0: Adt106, v1: Adt8) -> bool {
    let m667 = v0;
    match m667 {
        Adt106::C0 => {
            f306(v1)
        }
        Adt106::C1 => {
            f307(v1)
        }
        Adt106::C2 => {
            f308(v1)
        }
    }
}

pub fn f236(v0: Str, v1: Adt8, v2: Str) -> Str {
    let m668 = f233(v0, v1);
    match m668 {
        Adt115::C0 => {
            v2
        }
        Adt115::C1(v3) => {
            let m669 = f283(v3);
            match m669 {
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

pub fn f237(v0: Str, v1: Str) -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((v0.clone(), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(v0), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))))
}

pub fn f238(v0: Str, p675: List<Adt60>) -> Option<Adt60> {
    let m674 = p675;
    {
        let mut m674 = m674;
        loop {
            let step = {
                let v1 = m674;
                let m670 = v1;
                match m670.uncons() {
                    None => {
                        Err::<List<Adt60>, Option<Adt60>>(None::<Adt60>)
                    }
                    Some((v2, v3)) => {
                        let a672 = match v2.clone() {
                            Adt60::C0(h671, _, _, _, _, _) => {
                                h671
                            }
                        };
                        let a673 = v0.clone();
                        if equal(a672, a673) {
                            Err::<List<Adt60>, Option<Adt60>>(Some(v2))
                        } else {
                            Ok::<List<Adt60>, Option<Adt60>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m674 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239(p681: List<Adt61>, v1: List<(Str, Str)>) -> Option<Str> {
    let m680 = p681;
    {
        let mut m680 = m680;
        loop {
            let step = {
                let v0 = m680;
                let m676 = v0;
                match m676.uncons() {
                    None => {
                        Err::<List<Adt61>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f309(v2.clone()) {
                            let m678 = f310(match v2.clone() {
                                Adt61::C0(h677, _, _, _) => {
                                    h677
                                }
                            }, v1.clone());
                            match m678 {
                                None => {
                                    Err::<List<Adt61>, Option<Str>>(Some(match v2 {
                                        Adt61::C0(h679, _, _, _) => {
                                            h679
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
                Ok(next) => { m680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240(v0: Adt60) -> List<Adt61> {
    let m683 = match v0 {
        Adt60::C0(_, _, _, _, h682, _) => {
            h682
        }
    };
    match m683 {
        None => {
            List::<Adt61>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f241(v0: Option<List<(Str, Str)>>) -> List<(Str, Str)> {
    let m684 = v0;
    match m684 {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f242(v0: List<Adt63>) -> Adt62 {
    Adt62::C0(None::<Str>, v0)
}

pub fn f243(v0: Str) -> Adt63 {
    Adt63::C0(Adt45::C0, Adt42::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>)))
}

pub fn f244(v0: Str, v1: List<(Str, Str)>) -> Str {
    let m685 = f310(v0, v1);
    match m685 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f245(v0: Str, v1: Str, v2: Str) -> Adt50 {
    Adt50::C0(Adt51::C0(v0, Some(v1), v2, None::<Adt8>))
}

pub fn f246() -> Str {
    Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==")
}

pub fn f247(v0: List<Str>) -> R<Adt64> {
    let v1: u64 = f311(v0.clone(), 0u64)?;
    Ok(Adt64::C0(Adt65::C0(f312(v0), Some(v1), Some({
        let a686 = f313();
        let a687 = v1;
        nat_lt(a686, a687)
    }))))
}

pub fn f248(p693: List<Adt61>, v1: Str) -> bool {
    let m692 = p693;
    {
        let mut m692 = m692;
        loop {
            let step = {
                let v0 = m692;
                let m688 = v0;
                match m688.uncons() {
                    None => {
                        Err::<List<Adt61>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a690 = match v2 {
                            Adt61::C0(h689, _, _, _) => {
                                h689
                            }
                        };
                        let a691 = v1.clone();
                        if equal(a690, a691) {
                            Err::<List<Adt61>, bool>(true)
                        } else {
                            Ok::<List<Adt61>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m692 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f249(v0: Str, v1: List<Str>) -> R<List<Str>> {
    Ok(f295(f314(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
}

pub fn f250(v0: Str, v1: Str) -> List<Str> {
    let a694 = v0;
    let a695 = Str::lit("test_prompt_with_embedded_resource");
    if if equal(a694, a695) {
        let a696 = v1;
        let a697 = Str::lit("resourceUri");
        equal(a696, a697)
    } else {
        false
    } {
        f315()
    } else {
        List::<Str>::nil()
    }
}

pub fn f251() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f252(p700: List<Adt100>, p701: List<Adt100>) -> List<Adt100> {
    let m699 = (p700, p701);
    {
        let mut m699 = m699;
        loop {
            let step = {
                let (v0, v1) = m699;
                let m698 = v0;
                match m698.uncons() {
                    None => {
                        Err::<(List<Adt100>, List<Adt100>), List<Adt100>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt100>, List<Adt100>), List<Adt100>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m699 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253(v0: Bytes, v1: u64) -> R<Option<Adt95>> {
    let a703 = v1;
    let a704 = {
        let a702 = v0.clone();
        length_bytes(a702)
    };
    if nat_eq(a703, a704) {
        Ok(Some(Adt95::C0(Bytes::lit(&[]), v1)))
    } else {
        f7(v0, v1)
    }
}

pub fn f254(v0: Bytes, v1: Adt99, v2: u64) -> R<Result<Adt99, Adt2>> {
    let a706 = v2;
    let a707 = {
        let a705 = v0.clone();
        length_bytes(a705)
    };
    if nat_eq(a706, a707) {
        Ok(Ok::<Adt99, Adt2>(v1))
    } else {
        let m708 = f23(v0.clone(), v2)?;
        match m708 {
            None => {
                Ok(Err::<Adt99, Adt2>(Adt2::C0(v2)))
            }
            Some(v3) => {
                let m711 = f316(v0.clone(), {
                    let a709 = v2;
                    let a710 = 4u64;
                    nat_add(a709, a710)?
                }, v3, List::<List<u64>>::nil())?;
                match m711 {
                    Ok(v5) => {
                        Ok(f317(v0.clone(), f318(v1, {
                            let (h712, _) = v5;
                            h712
                        }), f319(v0, {
                            let (_, h713) = v5;
                            h713
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

pub fn f255() -> Bytes {
    Bytes::lit(&[])
}

pub fn f256(v0: Adt100) -> List<u64> {
    let a724 = f24({
        let a715 = match v0.clone() {
            Adt100::C0(h714, _) => {
                h714
            }
        };
        length_list(a715)
    });
    let a725 = {
        let a722 = match v0.clone() {
            Adt100::C0(h716, _) => {
                h716
            }
        };
        let a723 = {
            let a720 = f24({
                let a718 = match v0.clone() {
                    Adt100::C0(_, h717) => {
                        h717
                    }
                };
                length_list(a718)
            });
            let a721 = match v0 {
                Adt100::C0(_, h719) => {
                    h719
                }
            };
            append_list(a720, a721)
        };
        append_list(a722, a723)
    };
    append_list(a724, a725)
}

pub fn f257(v0: List<u64>) -> List<u64> {
    let a727 = f24({
        let a726 = v0.clone();
        length_list(a726)
    });
    let a728 = v0;
    append_list(a727, a728)
}

pub fn f258(v0: u64, v1: Adt9) -> R<Adt103> {
    let m729 = f320(v1)?;
    match m729 {
        Ok(v3) => {
            Ok(Adt103::C0(v0, f96(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f53())
        }
    }
}

pub fn f259(v0: Adt107) -> u64 {
    let m730 = v0;
    match m730 {
        Adt107::C0 => {
            f321()
        }
        Adt107::C1 => {
            f321()
        }
    }
}

pub fn f260(v0: Option<Adt109>, v1: Adt84) -> Adt9 {
    Adt9::C5(f322(v0, v1))
}

pub fn f261(v0: Adt107) -> Adt84 {
    Adt84::C0(f323(), f324(v0), None::<Adt9>)
}

pub fn f262(v0: List<u64>) -> bool {
    f49(v0, f325())
}

pub fn f263(v0: Adt101, v1: Adt3, v2: Fn3, v3: u64, v4: List<Adt100>, v5: Result<Adt9, Adt116>) -> R<Adt103> {
    let m731 = v5;
    match m731 {
        Ok(v7) => {
            f328(v0, v1, v2, v3, v4, f329(v7)?)
        }
        Err(_) => {
            f258(f326(), f260(None::<Adt109>, f327(Adt117::C0)))
        }
    }
}

pub fn f264(v0: Adt3, v1: Bytes) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h732, _, _, _, _, _, _) => {
            h732
        }
    }, match v0.clone() {
        Adt3::C0(_, h733, _, _, _, _, _) => {
            h733
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h734, _, _, _, _) => {
            h734
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h735, _, _, _) => {
            h735
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h736, _, _) => {
            h736
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, h737, _) => {
            h737
        }
    }, v1)
}

pub fn f265(v0: Fn1, v1: Adt99, v2: Adt18) -> R<Result<Adt36, Adt93>> {
    let c738: Fn1 = v0;
    c738.apply(match v1 {
        Adt99::C0(_, _, _, _, _, _, _, h739) => {
            h739
        }
    }, v2)
}

pub fn f266(v0: Bytes) -> R<Result<Adt9, Adt116>> {
    f330(f25(v0)?)
}

pub fn f267() -> Adt103 {
    Adt103::C0(f331(), List::cons(Adt100::C0(f332(), f333()), List::<Adt100>::nil()), List::<List<u64>>::nil(), List::<u64>::nil(), None::<List<u64>>)
}

pub fn f268(v0: List<Adt100>, v1: List<u64>) -> R<Adt119> {
    let m740 = f334(v0, v1, Adt120::C0)?;
    match m740 {
        Adt120::C0 => {
            Ok(Adt119::C0)
        }
        Adt120::C1(v2) => {
            let m741 = f335(v2);
            match m741 {
                None => {
                    Ok(Adt119::C1)
                }
                Some(v3) => {
                    Ok(Adt119::C3(v3))
                }
            }
        }
        Adt120::C2 => {
            Ok(Adt119::C2)
        }
    }
}

pub fn f269() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f270(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt107) -> R<Option<Adt107>> {
    let m742 = v0;
    match m742 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f336(v3, v1)? {
                Ok(None::<Adt107>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f271(v0: List<u64>) -> Option<List<u64>> {
    let m743 = f337(v0);
    match m743 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f273(v1)
        }
    }
}

pub fn f272() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f273(v0: List<u64>) -> Option<List<u64>> {
    let m744 = v0.clone();
    match m744.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a745 = v1;
            let a746 = 91u64;
            if nat_eq(a745, a746) {
                f338(f339(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f338(f340(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f274(v0: Adt3, v1: Fn3, v2: u64, v3: List<Adt100>, v4: Result<Adt9, Adt116>) -> R<Adt96> {
    let m747 = v4;
    match m747 {
        Ok(v6) => {
            f341(v0, v1, v2, v3, f329(v6)?)
        }
        Err(_) => {
            Ok(Adt96::C0(f52(f258(f326(), f260(None::<Adt109>, f327(Adt117::C0)))?)))
        }
    }
}

pub fn f275(v0: Str, p752: List<Str>) -> bool {
    let m751 = p752;
    {
        let mut m751 = m751;
        loop {
            let step = {
                let v1 = m751;
                let m748 = v1;
                match m748.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a749 = v0.clone();
                        let a750 = v2;
                        if equal(a749, a750) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m751 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: Str, v1: List<u64>) -> Option<Adt104> {
    if f342(v1) {
        Some(Adt104::C0(v0))
    } else {
        None::<Adt104>
    }
}

pub fn f277(v0: Str) -> R<List<u64>> {
    f25({
        let a753 = v0;
        utf8_encode(a753)
    })
}

pub fn f278(v0: Adt39) -> R<Result<List<Adt15>, Adt104>> {
    let m754 = f343(v0.clone());
    match m754 {
        None => {
            f344(v0.clone(), f345(f346(), match v0.clone() {
                Adt39::C0(h755, _, _, _, _, _, _, _) => {
                    h755
                }
            }, Adt9::C5({
                let m758 = match v0.clone() {
                    Adt39::C0(_, _, _, _, h757, _, _, _) => {
                        h757
                    }
                };
                match m758 {
                    Adt8::C0(h756) => {
                        h756
                    }
                }
            }), List::cons(Adt121::C0(Adt9::C5({
                let m761 = match v0 {
                    Adt39::C0(_, _, _, _, h760, _, _, _) => {
                        h760
                    }
                };
                match m761 {
                    Adt8::C0(h759) => {
                        h759
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt121>::nil()), f347())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt15>, Adt104>(v1))
        }
    }
}

pub fn f279(v0: Adt9, p780: u64, p781: Adt112, p782: List<Adt113>) -> R<Adt110> {
    let m779 = (p780, (p781, p782));
    {
        let mut m779 = m779;
        loop {
            let step = {
                let (v1, (v2, v3)) = m779;
                let m762 = v1;
                if m762 == 0 {
                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C4))
                } else {
                    let v4 = m762 - 1;
                    let m763 = v3;
                    match m763.uncons() {
                        None => {
                            let m764 = v2;
                            match m764 {
                                Adt112::C0 => {
                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C0))
                                }
                                Adt112::C1(v5, v6) => {
                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m765 = v2.clone();
                            match m765 {
                                Adt112::C0 => {
                                    let m766 = v7.clone();
                                    match m766 {
                                        Adt113::C0(v9, v10, v11) => {
                                            let m767 = f348(v0.clone(), v9, v10, v11.clone())?;
                                            match m767 {
                                                Adt125::C0(v12) => {
                                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (Adt112::C1(v11, v12), v8)))
                                                }
                                                Adt125::C1(v13) => {
                                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C2(v13)))
                                                }
                                                Adt125::C2(v14) => {
                                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C3(v14)))
                                                }
                                                Adt125::C3(v15) => {
                                                    let v16: List<Adt113> = {
                                                        let a768 = v15;
                                                        let a769 = v8;
                                                        append_list(a768, a769)
                                                    };
                                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt113::C1(_, _, _) => {
                                            Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (v2, v8)))
                                        }
                                        Adt113::C2(_, _, _, _) => {
                                            let v24: Adt122 = f349(v7, v8)?;
                                            Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (match v24.clone() {
                                                Adt122::C0(h770, _) => {
                                                    h770
                                                }
                                            }, match v24 {
                                                Adt122::C0(_, h771) => {
                                                    h771
                                                }
                                            })))
                                        }
                                        Adt113::C3(v25) => {
                                            Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (Adt112::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt113::C4(_, _, _, _) => {
                                            let v39: Adt122 = f349(v7, v8)?;
                                            Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (match v39.clone() {
                                                Adt122::C0(h775, _) => {
                                                    h775
                                                }
                                            }, match v39 {
                                                Adt122::C0(_, h776) => {
                                                    h776
                                                }
                                            })))
                                        }
                                        Adt113::C5(v26, v27, v28, v29) => {
                                            let m772 = f350(v26, v27, v28, v29.clone());
                                            match m772 {
                                                Adt125::C0(v30) => {
                                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (Adt112::C1(v29, v30), v8)))
                                                }
                                                Adt125::C1(v31) => {
                                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C2(v31)))
                                                }
                                                Adt125::C2(v32) => {
                                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C3(v32)))
                                                }
                                                Adt125::C3(v33) => {
                                                    let v34: List<Adt113> = {
                                                        let a773 = v33;
                                                        let a774 = v8;
                                                        append_list(a773, a774)
                                                    };
                                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt112::C1(v40, v41) => {
                                    let v42: Adt122 = f351(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (match v42.clone() {
                                        Adt122::C0(h777, _) => {
                                            h777
                                        }
                                    }, match v42 {
                                        Adt122::C0(_, h778) => {
                                            h778
                                        }
                                    })))
                                }
                            }
                        }
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

pub fn f280() -> u64 {
    67108864u64
}

pub fn f281(v0: List<Adt111>) -> R<Str> {
    let a783 = List::cons(Str::lit(""), f352(v0, List::<Str>::nil())?);
    let a784 = Str::lit("/");
    Ok(join(a783, a784))
}

pub fn f282(v0: Adt39, v1: Adt53) -> R<Result<Adt36, Adt93>> {
    if f353({
        let m787 = match v1.clone() {
            Adt53::C0(h786, _) => {
                h786
            }
        };
        match m787 {
            Adt41::C0(h785, _, _) => {
                h785
            }
        }
    })? {
        f354(v0, v1)
    } else {
        Ok(Err::<Adt36, Adt93>(Adt93::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f283(v0: Adt9) -> Option<Str> {
    let m788 = v0;
    match m788 {
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

pub fn f284(v0: Str) -> Adt74 {
    Adt74::C0(Adt45::C0, Adt75::C0(Adt76::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>))), None::<Adt8>)
}

pub fn f285(p791: u64, p792: i64) -> R<i64> {
    let m790 = (p791, p792);
    {
        let mut m790 = m790;
        loop {
            let step = {
                let (v0, v1) = m790;
                let m789 = v0;
                if m789 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m789 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f355(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m790 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f286(v0: Adt9) -> Option<List<Adt9>> {
    let m793 = v0;
    match m793 {
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

pub fn f287(v0: Adt8) -> bool {
    let m794 = f233(Str::lit("content"), v0);
    match m794 {
        Adt115::C0 => {
            false
        }
        Adt115::C1(v1) => {
            let m795 = f234(v1);
            match m795 {
                None => {
                    false
                }
                Some(v2) => {
                    let m796 = f233(Str::lit("ok"), v2);
                    match m796 {
                        Adt115::C0 => {
                            false
                        }
                        Adt115::C1(v3) => {
                            let m797 = f356(v3);
                            match m797 {
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

pub fn f288() -> Adt68 {
    f161(Str::lit("Please confirm"), Str::lit("ok"), Str::lit("boolean"))
}

pub fn f289(v0: Adt108) -> bool {
    let m798 = v0;
    match m798 {
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

pub fn f290(v0: Adt108) -> Adt8 {
    let m799 = v0;
    match m799 {
        Adt108::C0 => {
            f62()
        }
        Adt108::C1 => {
            f62()
        }
        Adt108::C2(v1) => {
            v1
        }
    }
}

pub fn f291(v0: Adt108) -> bool {
    let m800 = v0;
    match m800 {
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

pub fn f292() -> Adt68 {
    f161(Str::lit("Step 2: What is your favorite color?"), Str::lit("color"), Str::lit("string"))
}

pub fn f293(v0: Str) -> Str {
    let v1: Bytes = {
        let a801 = v0;
        utf8_encode(a801)
    };
    let m808 = {
        let a805 = v1.clone();
        let a806 = 1u64;
        let a807 = {
            let a803 = {
                let a802 = v1;
                length_bytes(a802)
            };
            let a804 = 1u64;
            nat_sub(a803, a804)
        };
        slice_bytes(a805, a806, a807)
    };
    match m808 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m810 = {
                let a809 = v2;
                utf8_decode(a809)
            };
            match m810 {
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

pub fn f294(v0: Option<Adt8>) -> bool {
    let m811 = v0;
    match m811 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f295(p814: List<Str>, p815: List<Str>) -> List<Str> {
    let m813 = (p814, p815);
    {
        let mut m813 = m813;
        loop {
            let step = {
                let (v0, v1) = m813;
                let m812 = v0;
                match m812.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m813 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(p819: List<(Str, Adt9)>, p820: List<Str>) -> List<Str> {
    let m818 = (p819, p820);
    {
        let mut m818 = m818;
        loop {
            let step = {
                let (v0, v1) = m818;
                let m816 = v0;
                match m816.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt9)>, List<Str>), List<Str>>((v3, List::cons({
                            let (h817, _) = v2;
                            h817
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m818 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: u64) -> R<Adt10> {
    Ok(Adt10::C0(f285(v0, 0i64)?, 0i64))
}

pub fn f298(p823: List<Adt48>, p824: List<Adt48>) -> List<Adt48> {
    let m822 = (p823, p824);
    {
        let mut m822 = m822;
        loop {
            let step = {
                let (v0, v1) = m822;
                let m821 = v0;
                match m821.uncons() {
                    None => {
                        Err::<(List<Adt48>, List<Adt48>), List<Adt48>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt48>, List<Adt48>), List<Adt48>>((v3, List::cons(v2, v1)))
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

pub fn f299(p828: List<(Adt48, List<Adt50>)>, p829: List<Adt48>) -> List<Adt48> {
    let m827 = (p828, p829);
    {
        let mut m827 = m827;
        loop {
            let step = {
                let (v0, v1) = m827;
                let m825 = v0;
                match m825.uncons() {
                    None => {
                        Err::<(List<(Adt48, List<Adt50>)>, List<Adt48>), List<Adt48>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt48, List<Adt50>)>, List<Adt48>), List<Adt48>>((v3, List::cons({
                            let (h826, _) = v2;
                            h826
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m827 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: Adt48, v1: Adt50) -> (Adt48, List<Adt50>) {
    (v0, List::cons(v1, List::<Adt50>::nil()))
}

pub fn f301(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt48 {
    Adt48::C0(v0, v1, None::<List<Adt5>>, v2, Some(v3), Some(v4), None::<Adt44>, None::<u64>, None::<Adt8>)
}

pub fn f302(v0: Str, v1: Str, v2: Str) -> Adt50 {
    Adt50::C1(Adt52::C0(v0, Some(v1), v2, None::<Adt8>))
}

pub fn f303(v0: Str) -> R<Option<Str>> {
    let m830 = f357(v0)?;
    match m830 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a831 = f358(f255(), v1);
            Ok(utf8_decode(a831))
        }
    }
}

pub fn f304(v0: Str) -> R<Option<Str>> {
    let m834 = f359(Adt9::C5(List::cons((Str::lit("id"), Adt9::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt9::C1(true)), List::cons((Str::lit("data"), Adt9::C3({
        let a832 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a833 = Str::lit(" ");
        join(a832, a833)
    })), List::<(Str, Adt9)>::nil())))))?;
    match m834 {
        Ok(v1) => {
            let a835 = v1;
            Ok(utf8_decode(a835))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f305(v0: Str, p841: List<(Str, Adt9)>) -> Adt115 {
    let m840 = p841;
    {
        let mut m840 = m840;
        loop {
            let step = {
                let v1 = m840;
                let m836 = v1;
                match m836.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, Adt115>(Adt115::C0)
                    }
                    Some((v2, v3)) => {
                        let m838 = f360({
                            let (h837, _) = v2;
                            h837
                        }, v0.clone());
                        match m838 {
                            Adt126::C0 => {
                                Err::<List<(Str, Adt9)>, Adt115>(Adt115::C1({
                                    let (_, h839) = v2;
                                    h839
                                }))
                            }
                            Adt126::C1 => {
                                Ok::<List<(Str, Adt9)>, Adt115>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m840 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: Adt8) -> bool {
    if f361(v0.clone()) {
        f362(v0)
    } else {
        false
    }
}

pub fn f307(v0: Adt8) -> bool {
    if f363(v0.clone()) {
        if f364(f233(Str::lit("model"), v0.clone())) {
            f365(v0)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f308(v0: Adt8) -> bool {
    let m842 = f233(Str::lit("roots"), v0);
    match m842 {
        Adt115::C0 => {
            false
        }
        Adt115::C1(v1) => {
            let m843 = f286(v1);
            match m843 {
                None => {
                    false
                }
                Some(v2) => {
                    f366(v2)
                }
            }
        }
    }
}

pub fn f309(v0: Adt61) -> bool {
    let m845 = match v0 {
        Adt61::C0(_, _, _, h844) => {
            h844
        }
    };
    match m845 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f310(v0: Str, p852: List<(Str, Str)>) -> Option<Str> {
    let m851 = p852;
    {
        let mut m851 = m851;
        loop {
            let step = {
                let v1 = m851;
                let m846 = v1;
                match m846.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a848 = {
                            let (h847, _) = v2;
                            h847
                        };
                        let a849 = v0.clone();
                        if equal(a848, a849) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h850) = v2;
                                h850
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
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

pub fn f311(p857: List<Str>, p858: u64) -> R<u64> {
    let m856 = (p857, p858);
    {
        let mut m856 = m856;
        loop {
            let step = {
                let (v0, v1) = m856;
                let m853 = v0;
                match m853.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a854 = v1;
                            let a855 = 1u64;
                            nat_add(a854, a855)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m856 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: List<Str>) -> List<Str> {
    f295(f367(v0, f313(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f313() -> u64 {
    100u64
}

pub fn f314(v0: Str, p861: List<Str>, p862: List<Str>) -> R<List<Str>> {
    let m860 = (p861, p862);
    {
        let mut m860 = m860;
        loop {
            let step = {
                let (v1, v2) = m860;
                let m859 = v1;
                match m859.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f368(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
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

pub fn f315() -> List<Str> {
    f295(f369(f148(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f316(v0: Bytes, p868: u64, p869: u64, p870: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt2>> {
    let m867 = (p868, (p869, p870));
    {
        let mut m867 = m867;
        loop {
            let step = {
                let (v1, (v2, v3)) = m867;
                let m863 = v2;
                if m863 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt2>>>(Ok(Ok::<(List<List<u64>>, u64), Adt2>((f171(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m863 - 1;
                    let m864 = f7(v0.clone(), v1)?;
                    match m864 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt2>>>(Ok(Err::<(List<List<u64>>, u64), Adt2>(Adt2::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt2>>>((match v5.clone() {
                                Adt95::C0(_, h865) => {
                                    h865
                                }
                            }, (v4, List::cons(f25(match v5 {
                                Adt95::C0(h866, _) => {
                                    h866
                                }
                            })?, v3))))
                        }
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

pub fn f317(v0: Bytes, v1: Adt99, v2: Result<(Adt101, u64), Adt2>) -> Result<Adt99, Adt2> {
    let m871 = v2;
    match m871 {
        Ok(v4) => {
            let a874 = {
                let (_, h872) = v4.clone();
                h872
            };
            let a875 = {
                let a873 = v0;
                length_bytes(a873)
            };
            if nat_eq(a874, a875) {
                Ok::<Adt99, Adt2>(Adt99::C0(match v1.clone() {
                    Adt99::C0(h876, _, _, _, _, _, _, _) => {
                        h876
                    }
                }, match v1.clone() {
                    Adt99::C0(_, h877, _, _, _, _, _, _) => {
                        h877
                    }
                }, match v1.clone() {
                    Adt99::C0(_, _, h878, _, _, _, _, _) => {
                        h878
                    }
                }, match v1.clone() {
                    Adt99::C0(_, _, _, h879, _, _, _, _) => {
                        h879
                    }
                }, match v1.clone() {
                    Adt99::C0(_, _, _, _, h880, _, _, _) => {
                        h880
                    }
                }, match v1.clone() {
                    Adt99::C0(_, _, _, _, _, h881, _, _) => {
                        h881
                    }
                }, match v1 {
                    Adt99::C0(_, _, _, _, _, _, h882, _) => {
                        h882
                    }
                }, {
                    let (h883, _) = v4;
                    h883
                }))
            } else {
                Err::<Adt99, Adt2>(Adt2::C1({
                    let (_, h884) = v4;
                    h884
                }))
            }
        }
        Err(v3) => {
            Err::<Adt99, Adt2>(v3)
        }
    }
}

pub fn f318(v0: Adt99, v1: List<List<u64>>) -> Adt99 {
    Adt99::C0(match v0.clone() {
        Adt99::C0(h885, _, _, _, _, _, _, _) => {
            h885
        }
    }, match v0.clone() {
        Adt99::C0(_, h886, _, _, _, _, _, _) => {
            h886
        }
    }, match v0.clone() {
        Adt99::C0(_, _, h887, _, _, _, _, _) => {
            h887
        }
    }, match v0.clone() {
        Adt99::C0(_, _, _, h888, _, _, _, _) => {
            h888
        }
    }, match v0.clone() {
        Adt99::C0(_, _, _, _, h889, _, _, _) => {
            h889
        }
    }, v1, match v0.clone() {
        Adt99::C0(_, _, _, _, _, _, h890, _) => {
            h890
        }
    }, match v0 {
        Adt99::C0(_, _, _, _, _, _, _, h891) => {
            h891
        }
    })
}

pub fn f319(v0: Bytes, v1: u64) -> R<Result<(Adt101, u64), Adt2>> {
    let a893 = v1;
    let a894 = {
        let a892 = v0.clone();
        length_bytes(a892)
    };
    if nat_eq(a893, a894) {
        Ok(Ok::<(Adt101, u64), Adt2>((Adt101::C0(0i64, Str::lit(""), Str::lit(""), f255()), v1)))
    } else {
        f370(v0, v1)
    }
}

pub fn f320(v0: Adt9) -> R<Result<List<u64>, Adt116>> {
    f371(f372(), v0)
}

pub fn f321() -> u64 {
    403u64
}

pub fn f322(v0: Option<Adt109>, v1: Adt84) -> List<(Str, Adt9)> {
    List::cons((Str::lit("jsonrpc"), Adt9::C3(f373())), {
        let a895 = f374(v0);
        let a896 = List::cons((Str::lit("error"), f375(v1)), List::<(Str, Adt9)>::nil());
        append_list(a895, a896)
    })
}

pub fn f323() -> i64 {
    -32000i64
}

pub fn f324(v0: Adt107) -> Str {
    let m897 = v0;
    match m897 {
        Adt107::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt107::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f325() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f326() -> u64 {
    400u64
}

pub fn f327(v0: Adt117) -> Adt84 {
    Adt84::C0(f376(v0.clone()), f377(v0), None::<Adt9>)
}

pub fn f328(v0: Adt101, v1: Adt3, v2: Fn3, v3: u64, v4: List<Adt100>, v5: Result<Adt123, Adt117>) -> R<Adt103> {
    let m898 = v5;
    match m898 {
        Ok(v7) => {
            f380(v0, v1, v2, v3, v4, v7)
        }
        Err(v6) => {
            f258(f378(v6.clone()), f260(f379(v6.clone()), f327(v6)))
        }
    }
}

pub fn f329(v0: Adt9) -> R<Result<Adt123, Adt117>> {
    let m899 = v0;
    match m899 {
        Adt9::C0 => {
            Ok(f382(None::<Adt109>, Adt118::C0))
        }
        Adt9::C1(_) => {
            Ok(f382(None::<Adt109>, Adt118::C0))
        }
        Adt9::C2(_) => {
            Ok(f382(None::<Adt109>, Adt118::C0))
        }
        Adt9::C3(_) => {
            Ok(f382(None::<Adt109>, Adt118::C0))
        }
        Adt9::C4(_) => {
            Ok(f382(None::<Adt109>, Adt118::C1))
        }
        Adt9::C5(v1) => {
            f381(Adt8::C0(v1))
        }
    }
}

pub fn f330(v0: List<u64>) -> R<Result<Adt9, Adt116>> {
    let m900 = f383(v0.clone())?;
    match m900 {
        Adt133::C0 => {
            f384(f385(), 0u64, v0)
        }
        Adt133::C1(v1) => {
            Ok(Err::<Adt9, Adt116>(Adt116::C0(v1)))
        }
        Adt133::C2(v2) => {
            Ok(Err::<Adt9, Adt116>(Adt116::C1(v2)))
        }
    }
}

pub fn f331() -> u64 {
    405u64
}

pub fn f332() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f333() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f334(p905: List<Adt100>, v1: List<u64>, p906: Adt120) -> R<Adt120> {
    let m904 = (p905, p906);
    {
        let mut m904 = m904;
        loop {
            let step = {
                let (v0, v2) = m904;
                let m901 = v0;
                match m901.uncons() {
                    None => {
                        Err::<(List<Adt100>, Adt120), R<Adt120>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f386(match v3.clone() {
                            Adt100::C0(h902, _) => {
                                h902
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt100>, Adt120), R<Adt120>>((v4, f387(v2, match v3 {
                                Adt100::C0(_, h903) => {
                                    h903
                                }
                            })))
                        } else {
                            Ok::<(List<Adt100>, Adt120), R<Adt120>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m904 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f388(v0);
    if f389(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f336(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f386(v0.clone(), f390())? {
        Ok(true)
    } else {
        if f386(v0.clone(), f391())? {
            Ok(true)
        } else {
            if f386(v0.clone(), f392())? {
                Ok(true)
            } else {
                f393(v0, v1)
            }
        }
    }
}

pub fn f337(p911: List<u64>) -> Option<List<u64>> {
    let m910 = p911;
    {
        let mut m910 = m910;
        loop {
            let step = {
                let v0 = m910;
                let m907 = v0;
                match m907.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a908 = v1;
                        let a909 = 58u64;
                        if nat_eq(a908, a909) {
                            Err::<List<u64>, Option<List<u64>>>(f394(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
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

pub fn f338(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m912 = v0;
    match m912 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f395({
                let (_, h913) = v1;
                h913
            }) {
                Some({
                    let (h914, _) = v1;
                    h914
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f339(p919: List<u64>, p920: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m918 = (p919, p920);
    {
        let mut m918 = m918;
        loop {
            let step = {
                let (v0, v1) = m918;
                let m915 = v0;
                match m915.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a916 = v2;
                        let a917 = 93u64;
                        if nat_eq(a916, a917) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f396(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m918 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f340(p925: List<u64>, p926: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m924 = (p925, p926);
    {
        let mut m924 = m924;
        loop {
            let step = {
                let (v0, v1) = m924;
                let m921 = v0;
                match m921.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f396(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a922 = v2;
                        let a923 = 58u64;
                        if nat_eq(a922, a923) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f396(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
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

pub fn f341(v0: Adt3, v1: Fn3, v2: u64, v3: List<Adt100>, v4: Result<Adt123, Adt117>) -> R<Adt96> {
    let m927 = v4;
    match m927 {
        Ok(v6) => {
            f397(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            Ok(Adt96::C0(f52(f258(f378(v5.clone()), f260(f379(v5.clone()), f327(v5)))?)))
        }
    }
}

pub fn f342(v0: List<u64>) -> bool {
    let m928 = v0.clone();
    match m928.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a932 = if f398(v0.clone(), false) {
                let a930 = {
                    let a929 = v0;
                    length_list(a929)
                };
                let a931 = f399();
                nat_le(a930, a931)
            } else {
                false
            };
            bool_not(a932)
        }
    }
}

pub fn f343(v0: Adt39) -> Option<Adt104> {
    let m937 = f400(match v0.clone() {
        Adt39::C0(h933, _, _, _, _, _, _, _) => {
            h933
        }
    }, {
        let m936 = match v0.clone() {
            Adt39::C0(_, _, _, _, h935, _, _, _) => {
                h935
            }
        };
        match m936 {
            Adt8::C0(h934) => {
                h934
            }
        }
    });
    match m937 {
        None => {
            f401(match v0.clone() {
                Adt39::C0(h938, _, _, _, _, _, _, _) => {
                    h938
                }
            }, {
                let m941 = match v0 {
                    Adt39::C0(_, _, _, _, h940, _, _, _) => {
                        h940
                    }
                };
                match m941 {
                    Adt8::C0(h939) => {
                        h939
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f344(v0: Adt39, v1: Result<Adt124, Adt104>) -> R<Result<List<Adt15>, Adt104>> {
    let m942 = v1;
    match m942 {
        Ok(v3) => {
            let m943 = f402(v0)?;
            match m943 {
                None => {
                    Ok(Ok::<List<Adt15>, Adt104>(f183(match v3 {
                        Adt124::C0(h944, _) => {
                            h944
                        }
                    }, List::<Adt15>::nil())))
                }
                Some(v4) => {
                    Ok(Err::<List<Adt15>, Adt104>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<List<Adt15>, Adt104>(v2))
        }
    }
}

pub fn f345(p951: u64, v1: Str, v2: Adt9, p952: List<Adt121>, p953: Adt124) -> R<Result<Adt124, Adt104>> {
    let m950 = (p951, (p952, p953));
    {
        let mut m950 = m950;
        loop {
            let step = {
                let (v0, (v3, v4)) = m950;
                let m945 = v0;
                if m945 == 0 {
                    Err::<(u64, (List<Adt121>, Adt124)), R<Result<Adt124, Adt104>>>(Ok(Err::<Adt124, Adt104>(Adt104::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m945 - 1;
                    let m946 = v3;
                    match m946.uncons() {
                        None => {
                            Err::<(u64, (List<Adt121>, Adt124)), R<Result<Adt124, Adt104>>>(Ok(Ok::<Adt124, Adt104>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m947 = f403(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m947 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt121>, Adt124)), R<Result<Adt124, Adt104>>>((v5, ({
                                        let (h948, _) = v9;
                                        h948
                                    }, {
                                        let (_, h949) = v9;
                                        h949
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt121>, Adt124)), R<Result<Adt124, Adt104>>>(Ok(Err::<Adt124, Adt104>(v8)))
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

pub fn f346() -> u64 {
    67108864u64
}

pub fn f347() -> Adt124 {
    Adt124::C0(List::<Adt15>::nil(), List::<List<u64>>::nil())
}

pub fn f348(v0: Adt9, v1: Adt9, v2: Adt9, v3: List<Adt111>) -> R<Adt125> {
    let m954 = f356(v1.clone());
    match m954 {
        None => {
            let m955 = f234(v1);
            match m955 {
                None => {
                    Ok(Adt125::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f404(v0, match v5 {
                        Adt8::C0(h956) => {
                            h956
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt125::C3(List::<Adt113>::nil()))
            } else {
                Ok(Adt125::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f349(v0: Adt113, v1: List<Adt113>) -> R<Adt122> {
    let m957 = v0;
    match m957 {
        Adt113::C0(_, _, _) => {
            Ok(Adt122::C0(Adt112::C0, v1))
        }
        Adt113::C1(_, _, _) => {
            Ok(Adt122::C0(Adt112::C0, v1))
        }
        Adt113::C2(v8, v9, v10, v11) => {
            Ok(f405(v8, v9, v10, {
                let a958 = v11;
                let a959 = 1u64;
                nat_add(a958, a959)?
            }, v1))
        }
        Adt113::C3(v12) => {
            Ok(Adt122::C0(Adt112::C1(v12, Str::lit("not")), v1))
        }
        Adt113::C4(v13, _, v15, v16) => {
            Ok(f406(v13, v15, v16, v1))
        }
        Adt113::C5(_, _, _, _) => {
            Ok(Adt122::C0(Adt112::C0, v1))
        }
    }
}

pub fn f350(v0: Adt114, v1: Adt9, v2: Adt9, v3: List<Adt111>) -> Adt125 {
    let m960 = f234(v1);
    match m960 {
        None => {
            Adt125::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m961 = v0;
            match m961 {
                Adt114::C0 => {
                    f407(match v4 {
                        Adt8::C0(h962) => {
                            h962
                        }
                    }, v2, v3)
                }
                Adt114::C1 => {
                    f408(match v4 {
                        Adt8::C0(h963) => {
                            h963
                        }
                    }, v2, v3)
                }
                Adt114::C2 => {
                    f409(match v4 {
                        Adt8::C0(h964) => {
                            h964
                        }
                    }, v2, v3)
                }
                Adt114::C3 => {
                    f410(match v4 {
                        Adt8::C0(h965) => {
                            h965
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f351(v0: Adt113, v1: List<Adt113>, v2: List<Adt111>, v3: Str) -> Adt122 {
    let m966 = v0;
    match m966 {
        Adt113::C0(_, _, _) => {
            Adt122::C0(Adt112::C1(v2, v3), v1)
        }
        Adt113::C1(v7, v8, v9) => {
            f411(v7, v8, v9, v1)
        }
        Adt113::C2(v10, v11, v12, v13) => {
            f405(v10, v11, v12, v13, v1)
        }
        Adt113::C3(_) => {
            Adt122::C0(Adt112::C0, v1)
        }
        Adt113::C4(_, v16, v17, v18) => {
            f406(v16, v17, v18, v1)
        }
        Adt113::C5(_, _, _, _) => {
            Adt122::C0(Adt112::C1(v2, v3), v1)
        }
    }
}

pub fn f352(p969: List<Adt111>, p970: List<Str>) -> R<List<Str>> {
    let m968 = (p969, p970);
    {
        let mut m968 = m968;
        loop {
            let step = {
                let (v0, v1) = m968;
                let m967 = v0;
                match m967.uncons() {
                    None => {
                        Err::<(List<Adt111>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt111>, List<Str>), R<List<Str>>>((v3, List::cons(f412(v2)?, v1)))
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

pub fn f353(p979: List<Adt42>) -> R<bool> {
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
                                if f413(match v4 {
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
                                if f413(match v5 {
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
                                        if f413(match v9 {
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

pub fn f354(v0: Adt39, v1: Adt53) -> R<Result<Adt36, Adt93>> {
    if f414(match v1.clone() {
        Adt53::C0(h980, _) => {
            h980
        }
    }) {
        f415(v1)
    } else {
        let m982 = match v0 {
            Adt39::C0(_, _, _, _, _, h981, _, _) => {
                h981
            }
        };
        match m982 {
            None => {
                f415(v1)
            }
            Some(v2) => {
                f416(v2, v1)
            }
        }
    }
}

pub fn f355(v0: i64, v1: i64) -> R<i64> {
    let a984 = v0;
    let a985 = {
        let a983 = v1;
        int_neg(a983)?
    };
    int_sub(a984, a985)
}

pub fn f356(v0: Adt9) -> Option<bool> {
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

pub fn f357(v0: Str) -> R<Option<List<u64>>> {
    let m987 = f417(f418(f419())?, f418(v0)?);
    match m987 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m988 = f417(f420(List::cons(47u64, f418(f421())?)), f420(v1));
            match m988 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v2) => {
                    let v3: List<u64> = f420(v2);
                    if if f422(v3.clone()) {
                        f423(v3.clone())
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

pub fn f358(v0: Bytes, v1: List<u64>) -> Bytes {
    let a989 = v0;
    let a990 = f424(f255(), f425(List::<Bytes>::nil(), v1));
    append_bytes(a989, a990)
}

pub fn f359(v0: Adt9) -> R<Result<Bytes, Adt116>> {
    let m991 = f320(v0)?;
    match m991 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt116>(f358(f255(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt116>(v2))
        }
    }
}

pub fn f360(v0: Str, v1: Str) -> Adt126 {
    let a992 = v0;
    let a993 = v1;
    if equal(a992, a993) {
        Adt126::C0
    } else {
        Adt126::C1
    }
}

pub fn f361(v0: Adt8) -> bool {
    if f426(f233(Str::lit("action"), v0.clone()), Str::lit("accept")) {
        true
    } else {
        if f426(f233(Str::lit("action"), v0.clone()), Str::lit("decline")) {
            true
        } else {
            f426(f233(Str::lit("action"), v0), Str::lit("cancel"))
        }
    }
}

pub fn f362(v0: Adt8) -> bool {
    let m994 = f233(Str::lit("content"), v0);
    match m994 {
        Adt115::C0 => {
            true
        }
        Adt115::C1(v1) => {
            let m995 = f234(v1);
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

pub fn f363(v0: Adt8) -> bool {
    if f426(f233(Str::lit("role"), v0.clone()), Str::lit("user")) {
        true
    } else {
        f426(f233(Str::lit("role"), v0), Str::lit("assistant"))
    }
}

pub fn f364(v0: Adt115) -> bool {
    let m996 = v0;
    match m996 {
        Adt115::C0 => {
            false
        }
        Adt115::C1(v1) => {
            let m997 = f283(v1);
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

pub fn f365(v0: Adt8) -> bool {
    let m998 = f233(Str::lit("content"), v0);
    match m998 {
        Adt115::C0 => {
            false
        }
        Adt115::C1(v1) => {
            let m999 = f234(v1.clone());
            match m999 {
                None => {
                    let m1000 = f286(v1);
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

pub fn f366(p1004: List<Adt9>) -> bool {
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
                        let m1002 = f234(v1);
                        match m1002 {
                            None => {
                                Err::<List<Adt9>, bool>(false)
                            }
                            Some(v3) => {
                                if f364(f233(Str::lit("uri"), v3)) {
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

pub fn f367(p1008: List<Str>, p1009: u64, p1010: List<Str>) -> List<Str> {
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

pub fn f368(v0: Str, v1: Str) -> R<bool> {
    Ok(f427(f418(v0)?, f418(v1)?))
}

pub fn f369(p1014: List<Adt48>, p1015: List<Str>) -> List<Str> {
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

pub fn f370(v0: Bytes, v1: u64) -> R<Result<(Adt101, u64), Adt2>> {
    let m1016 = f428(v0.clone(), v1)?;
    match m1016 {
        None => {
            Ok(Err::<(Adt101, u64), Adt2>(Adt2::C0(v1)))
        }
        Some(v2) => {
            let m1019 = f429(v0.clone(), {
                let a1017 = v1;
                let a1018 = 8u64;
                nat_add(a1017, a1018)?
            })?;
            match m1019 {
                Ok(v4) => {
                    f430(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt101, u64), Adt2>(v3))
                }
            }
        }
    }
}

pub fn f371(v0: u64, v1: Adt9) -> R<Result<List<u64>, Adt116>> {
    let m1020 = f431(v0, List::cons(Adt127::C1(0u64, v1), List::<Adt127>::nil()), List::<u64>::nil())?;
    match m1020 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt116>(f432(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt116>(v3))
        }
    }
}

pub fn f372() -> u64 {
    67108864u64
}

pub fn f373() -> Str {
    Str::lit("2.0")
}

pub fn f374(v0: Option<Adt109>) -> List<(Str, Adt9)> {
    let m1021 = v0;
    match m1021 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f433(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f375(v0: Adt84) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("code"), Adt9::C2(Adt10::C0(match v0.clone() {
        Adt84::C0(h1022, _, _) => {
            h1022
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt9::C3(match v0.clone() {
        Adt84::C0(_, h1023, _) => {
            h1023
        }
    })), f434(match v0 {
        Adt84::C0(_, _, h1024) => {
            h1024
        }
    }))))
}

pub fn f376(v0: Adt117) -> i64 {
    let m1025 = v0;
    match m1025 {
        Adt117::C0 => {
            f435()
        }
        Adt117::C1(_, _) => {
            f436()
        }
    }
}

pub fn f377(v0: Adt117) -> Str {
    let m1026 = v0;
    match m1026 {
        Adt117::C0 => {
            Str::lit("Parse error")
        }
        Adt117::C1(_, v2) => {
            f437(v2)
        }
    }
}

pub fn f378(v0: Adt117) -> u64 {
    let m1027 = v0;
    match m1027 {
        Adt117::C0 => {
            f326()
        }
        Adt117::C1(_, _) => {
            f326()
        }
    }
}

pub fn f379(v0: Adt117) -> Option<Adt109> {
    let m1028 = v0;
    match m1028 {
        Adt117::C0 => {
            None::<Adt109>
        }
        Adt117::C1(v1, _) => {
            v1
        }
    }
}

pub fn f380(v0: Adt101, v1: Adt3, v2: Fn3, v3: u64, v4: List<Adt100>, v5: Adt123) -> R<Adt103> {
    f438(v0, f439(v4.clone())?, f440(v1.clone(), v2, v3, Some(f441(v4, match v1 {
        Adt3::C0(_, _, _, _, h1029, _, _) => {
            h1029
        }
    }, f442(v5.clone()), match v5.clone() {
        Adt123::C0(_, h1030, _) => {
            h1030
        }
    })?), f443(), v5)?)
}

pub fn f381(v0: Adt8) -> R<Result<Adt123, Adt117>> {
    let m1031 = f444(v0.clone())?;
    match m1031 {
        Adt148::C0 => {
            let m1032 = f233(Str::lit("method"), v0.clone());
            match m1032 {
                Adt115::C0 => {
                    Ok(f382(None::<Adt109>, f445(v0)))
                }
                Adt115::C1(_) => {
                    Ok(f382(None::<Adt109>, if f446(v0) {
                        Adt118::C8
                    } else {
                        Adt118::C7
                    }))
                }
            }
        }
        Adt148::C1(v2) => {
            Ok(f447(v2, v0))
        }
        Adt148::C2 => {
            Ok(f382(None::<Adt109>, Adt118::C5))
        }
    }
}

pub fn f382(v0: Option<Adt109>, v1: Adt118) -> Result<Adt123, Adt117> {
    Err::<Adt123, Adt117>(Adt117::C1(v0, v1))
}

pub fn f383(v0: List<u64>) -> R<Adt133> {
    Ok(f448({
        let a1033 = v0.clone();
        length_list(a1033)
    }, f449(Adt134::C0, 0u64, 0u64, v0)?))
}

pub fn f384(p1040: Adt135, p1041: u64, p1042: List<u64>) -> R<Result<Adt9, Adt116>> {
    let m1039 = (p1040, (p1041, p1042));
    {
        let mut m1039 = m1039;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1039;
                let m1034 = v2;
                match m1034.uncons() {
                    None => {
                        let m1035 = f450(v0, v1)?;
                        match m1035 {
                            Ok(v3) => {
                                Err::<(Adt135, (u64, List<u64>)), R<Result<Adt9, Adt116>>>(Ok(Ok::<Adt9, Adt116>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt135, (u64, List<u64>)), R<Result<Adt9, Adt116>>>(Ok(Err::<Adt9, Adt116>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m1036 = f451(v0, v1, v5)?;
                        match m1036 {
                            Adt149::C0(v7) => {
                                Err::<(Adt135, (u64, List<u64>)), R<Result<Adt9, Adt116>>>(Ok(Err::<Adt9, Adt116>(v7)))
                            }
                            Adt149::C1(v8) => {
                                Ok::<(Adt135, (u64, List<u64>)), R<Result<Adt9, Adt116>>>((v8, ({
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

pub fn f385() -> Adt135 {
    Adt135::C0(Adt136::C0, List::<Adt140>::nil(), 0u64)
}

pub fn f386(p1049: List<u64>, p1050: List<u64>) -> R<bool> {
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
                                let a1046 = f452(v4)?;
                                let a1047 = f452(v6)?;
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

pub fn f387(v0: Adt120, v1: List<u64>) -> Adt120 {
    let m1051 = v0;
    match m1051 {
        Adt120::C0 => {
            Adt120::C1(v1)
        }
        Adt120::C1(_) => {
            Adt120::C2
        }
        Adt120::C2 => {
            Adt120::C2
        }
    }
}

pub fn f388(v0: List<u64>) -> List<u64> {
    f396(f453(f396(f453(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f389(p1054: List<u64>) -> bool {
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
                        if f454(v1) {
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

pub fn f390() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f391() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f392() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f393(v0: List<u64>, p1057: List<List<u64>>) -> R<bool> {
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
                        if f386(v0.clone(), v2)? {
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

pub fn f394(v0: List<u64>) -> Option<List<u64>> {
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

pub fn f395(v0: List<u64>) -> bool {
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
                        f455(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f396(p1070: List<u64>, p1071: List<u64>) -> List<u64> {
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

pub fn f397(v0: Adt3, v1: Fn3, v2: u64, v3: List<Adt100>, v4: Adt123) -> R<Adt96> {
    f456(f439(v3.clone())?, f440(v0.clone(), v1, v2, Some(f441(v3, match v0 {
        Adt3::C0(_, _, _, _, h1072, _, _) => {
            h1072
        }
    }, f442(v4.clone()), match v4.clone() {
        Adt123::C0(_, h1073, _) => {
            h1073
        }
    })?), f443(), v4)?)
}

pub fn f398(p1076: List<u64>, v1: bool) -> bool {
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
                    Some((v2, v3)) => {
                        if if v1 {
                            f457(v2)
                        } else {
                            f458(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
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

pub fn f399() -> u64 {
    64u64
}

pub fn f400(v0: Str, v1: List<(Str, Adt9)>) -> Option<Adt104> {
    let m1077 = f305(Str::lit("type"), v1);
    match m1077 {
        Adt115::C0 => {
            Some(Adt104::C3(v0))
        }
        Adt115::C1(v2) => {
            let m1078 = f283(v2);
            match m1078 {
                None => {
                    Some(Adt104::C3(v0))
                }
                Some(v3) => {
                    let a1079 = v3;
                    let a1080 = Str::lit("object");
                    if equal(a1079, a1080) {
                        None::<Adt104>
                    } else {
                        Some(Adt104::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f401(v0: Str, v1: List<(Str, Adt9)>) -> Option<Adt104> {
    let m1081 = f305(Str::lit("$schema"), v1);
    match m1081 {
        Adt115::C0 => {
            None::<Adt104>
        }
        Adt115::C1(v2) => {
            let m1082 = f283(v2);
            match m1082 {
                None => {
                    Some(Adt104::C2(v0))
                }
                Some(v3) => {
                    let a1083 = v3;
                    let a1084 = f459();
                    if equal(a1083, a1084) {
                        None::<Adt104>
                    } else {
                        Some(Adt104::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f402(v0: Adt39) -> R<Option<Adt104>> {
    let m1086 = match v0.clone() {
        Adt39::C0(_, _, _, _, _, h1085, _, _) => {
            h1085
        }
    };
    match m1086 {
        None => {
            Ok(None::<Adt104>)
        }
        Some(v1) => {
            let m1089 = f401(match v0.clone() {
                Adt39::C0(h1087, _, _, _, _, _, _, _) => {
                    h1087
                }
            }, match v1.clone() {
                Adt8::C0(h1088) => {
                    h1088
                }
            });
            match m1089 {
                None => {
                    let m1093 = f345(f346(), match v0 {
                        Adt39::C0(h1090, _, _, _, _, _, _, _) => {
                            h1090
                        }
                    }, Adt9::C5(match v1.clone() {
                        Adt8::C0(h1091) => {
                            h1091
                        }
                    }), List::cons(Adt121::C0(Adt9::C5(match v1 {
                        Adt8::C0(h1092) => {
                            h1092
                        }
                    }), false, List::<Str>::nil()), List::<Adt121>::nil()), f347())?;
                    match m1093 {
                        Ok(_) => {
                            Ok(None::<Adt104>)
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

pub fn f403(v0: Str, v1: Adt9, v2: Adt121, v3: Adt124, v4: List<Adt121>) -> R<Result<(List<Adt121>, Adt124), Adt104>> {
    let m1094 = v2;
    match m1094 {
        Adt121::C0(v5, v6, v7) => {
            let m1095 = f356(v5.clone());
            match m1095 {
                None => {
                    let m1096 = f234(v5);
                    match m1096 {
                        None => {
                            Ok(Err::<(List<Adt121>, Adt124), Adt104>(Adt104::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f460(v0, v1, match v9 {
                                Adt8::C0(h1097) => {
                                    h1097
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt121>, Adt124), Adt104>((v4, v3)))
                }
            }
        }
    }
}

pub fn f404(v0: Adt9, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt111>) -> R<Adt125> {
    let m1098 = f461(v1.clone(), v2.clone())?;
    match m1098 {
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
            f462(v0, v1, v2, v3)
        }
    }
}

pub fn f405(v0: List<Adt9>, v1: Adt9, v2: List<Adt111>, v3: u64, v4: List<Adt113>) -> Adt122 {
    let m1099 = v0;
    match m1099.uncons() {
        None => {
            let a1100 = v3;
            let a1101 = 1u64;
            if nat_eq(a1100, a1101) {
                Adt122::C0(Adt112::C0, v4)
            } else {
                Adt122::C0(Adt112::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt122::C0(Adt112::C0, List::cons(Adt113::C0(v5, v1.clone(), v2.clone()), List::cons(Adt113::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f406(v0: Option<Adt9>, v1: Adt9, v2: List<Adt111>, v3: List<Adt113>) -> Adt122 {
    let m1102 = v0;
    match m1102 {
        None => {
            Adt122::C0(Adt112::C0, v3)
        }
        Some(v4) => {
            Adt122::C0(Adt112::C0, List::cons(Adt113::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f407(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt111>) -> Adt125 {
    let m1103 = f305(Str::lit("anyOf"), v0);
    match m1103 {
        Adt115::C0 => {
            Adt125::C3(List::<Adt113>::nil())
        }
        Adt115::C1(v3) => {
            let m1104 = f286(v3);
            match m1104 {
                None => {
                    Adt125::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1105 = v4;
                    match m1105.uncons() {
                        None => {
                            Adt125::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt125::C3(List::cons(Adt113::C0(v5, v1.clone(), v2.clone()), List::cons(Adt113::C1(v6, v1, v2), List::<Adt113>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f408(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt111>) -> Adt125 {
    let m1106 = f305(Str::lit("oneOf"), v0);
    match m1106 {
        Adt115::C0 => {
            Adt125::C3(List::<Adt113>::nil())
        }
        Adt115::C1(v3) => {
            let m1107 = f286(v3);
            match m1107 {
                None => {
                    Adt125::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1108 = v4;
                    match m1108.uncons() {
                        None => {
                            Adt125::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt125::C3(List::cons(Adt113::C0(v5, v1.clone(), v2.clone()), List::cons(Adt113::C2(v6, v1, v2, 0u64), List::<Adt113>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f409(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt111>) -> Adt125 {
    let m1109 = f305(Str::lit("not"), v0);
    match m1109 {
        Adt115::C0 => {
            Adt125::C3(List::<Adt113>::nil())
        }
        Adt115::C1(v3) => {
            Adt125::C3(List::cons(Adt113::C0(v3, v1, v2.clone()), List::cons(Adt113::C3(v2), List::<Adt113>::nil())))
        }
    }
}

pub fn f410(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt111>) -> Adt125 {
    let m1110 = f305(Str::lit("if"), v0.clone());
    match m1110 {
        Adt115::C0 => {
            Adt125::C3(List::<Adt113>::nil())
        }
        Adt115::C1(v3) => {
            Adt125::C3(List::cons(Adt113::C0(v3, v1.clone(), v2.clone()), List::cons(Adt113::C4(f463(Str::lit("then"), v0.clone()), f463(Str::lit("else"), v0), v1, v2), List::<Adt113>::nil())))
        }
    }
}

pub fn f411(v0: List<Adt9>, v1: Adt9, v2: List<Adt111>, v3: List<Adt113>) -> Adt122 {
    let m1111 = v0;
    match m1111.uncons() {
        None => {
            Adt122::C0(Adt112::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt122::C0(Adt112::C0, List::cons(Adt113::C0(v4, v1.clone(), v2.clone()), List::cons(Adt113::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f412(v0: Adt111) -> R<Str> {
    let m1112 = v0;
    match m1112 {
        Adt111::C0(v1) => {
            f464(v1)
        }
        Adt111::C1(v2) => {
            let a1113 = f285(v2, 0i64)?;
            Ok(format_i64(a1113))
        }
    }
}

pub fn f413(v0: Str) -> R<bool> {
    let m1115 = f465(f25({
        let a1114 = v0;
        utf8_encode(a1114)
    })?)?;
    match m1115 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f414(v0: Adt41) -> bool {
    let m1117 = match v0 {
        Adt41::C0(_, _, h1116) => {
            h1116
        }
    };
    match m1117 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f415(v0: Adt53) -> R<Result<Adt36, Adt93>> {
    let m1119 = match v0.clone() {
        Adt53::C0(_, h1118) => {
            h1118
        }
    };
    match m1119.uncons() {
        None => {
            Ok(Ok::<Adt36, Adt93>(Adt36::C2(match v0 {
                Adt53::C0(h1120, _) => {
                    h1120
                }
            })))
        }
        Some((_, _)) => {
            if f466(match v0.clone() {
                Adt53::C0(_, h1121) => {
                    h1121
                }
            })? {
                Ok(Ok::<Adt36, Adt93>(Adt36::C3(v0)))
            } else {
                Ok(Err::<Adt36, Adt93>(Adt93::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f416(v0: Adt8, v1: Adt53) -> R<Result<Adt36, Adt93>> {
    let m1125 = {
        let m1124 = match v1.clone() {
            Adt53::C0(h1123, _) => {
                h1123
            }
        };
        match m1124 {
            Adt41::C0(_, h1122, _) => {
                h1122
            }
        }
    };
    match m1125 {
        None => {
            Ok(Err::<Adt36, Adt93>(Adt93::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1128 = f189(Adt9::C5(match v0.clone() {
                Adt8::C0(h1126) => {
                    h1126
                }
            }), Adt9::C5(match v0 {
                Adt8::C0(h1127) => {
                    h1127
                }
            }), v2)?;
            match m1128 {
                Adt110::C0 => {
                    f415(v1)
                }
                Adt110::C1(_, _) => {
                    Ok(Err::<Adt36, Adt93>(Adt93::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt110::C2(_) => {
                    Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt110::C3(_) => {
                    Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt110::C4 => {
                    Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f417(p1134: List<u64>, p1135: List<u64>) -> Option<List<u64>> {
    let m1133 = (p1134, p1135);
    {
        let mut m1133 = m1133;
        loop {
            let step = {
                let (v0, v1) = m1133;
                let m1129 = v0;
                match m1129.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1130 = v1;
                        match m1130.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1131 = v2;
                                let a1132 = v4;
                                if nat_eq(a1131, a1132) {
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
                Ok(next) => { m1133 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f418(v0: Str) -> R<List<u64>> {
    f25({
        let a1136 = v0;
        utf8_encode(a1136)
    })
}

pub fn f419() -> Str {
    Str::lit("test://template/")
}

pub fn f420(v0: List<u64>) -> List<u64> {
    f432(v0, List::<u64>::nil())
}

pub fn f421() -> Str {
    Str::lit("data")
}

pub fn f422(v0: List<u64>) -> bool {
    let m1137 = v0;
    match m1137.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f423(p1142: List<u64>) -> bool {
    let m1141 = p1142;
    {
        let mut m1141 = m1141;
        loop {
            let step = {
                let v0 = m1141;
                let m1138 = v0;
                match m1138.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1139 = v1;
                        let a1140 = 47u64;
                        if nat_eq(a1139, a1140) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f424(p1147: Bytes, p1148: List<Bytes>) -> Bytes {
    let m1146 = (p1147, p1148);
    {
        let mut m1146 = m1146;
        loop {
            let step = {
                let (v0, v1) = m1146;
                let m1143 = v1;
                match m1143.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1144 = v2;
                            let a1145 = v0;
                            append_bytes(a1144, a1145)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425(p1151: List<Bytes>, p1152: List<u64>) -> List<Bytes> {
    let m1150 = (p1151, p1152);
    {
        let mut m1150 = m1150;
        loop {
            let step = {
                let (v0, v1) = m1150;
                let m1149 = v1;
                match m1149.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f467(f468(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(v0: Adt115, v1: Str) -> bool {
    let m1153 = v0;
    match m1153 {
        Adt115::C0 => {
            false
        }
        Adt115::C1(v2) => {
            let m1154 = f283(v2);
            match m1154 {
                None => {
                    false
                }
                Some(v3) => {
                    let a1155 = v3;
                    let a1156 = v1;
                    equal(a1155, a1156)
                }
            }
        }
    }
}

pub fn f427(p1162: List<u64>, p1163: List<u64>) -> bool {
    let m1161 = (p1162, p1163);
    {
        let mut m1161 = m1161;
        loop {
            let step = {
                let (v0, v1) = m1161;
                let m1157 = v0;
                match m1157.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m1158 = v1;
                        match m1158.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a1159 = v2;
                                let a1160 = v4;
                                if nat_eq(a1159, a1160) {
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
                Ok(next) => { m1161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f428(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a1167 = {
        let a1164 = v1;
        let a1165 = 8u64;
        nat_add(a1164, a1165)?
    };
    let a1168 = {
        let a1166 = v0.clone();
        length_bytes(a1166)
    };
    if nat_le(a1167, a1168) {
        Ok(Some(f469(f469(f469(f469(f469(f469(f469(f469(0i64, f58(v0.clone(), v1)?)?, f58(v0.clone(), {
            let a1169 = v1;
            let a1170 = 1u64;
            nat_add(a1169, a1170)?
        })?)?, f58(v0.clone(), {
            let a1171 = v1;
            let a1172 = 2u64;
            nat_add(a1171, a1172)?
        })?)?, f58(v0.clone(), {
            let a1173 = v1;
            let a1174 = 3u64;
            nat_add(a1173, a1174)?
        })?)?, f58(v0.clone(), {
            let a1175 = v1;
            let a1176 = 4u64;
            nat_add(a1175, a1176)?
        })?)?, f58(v0.clone(), {
            let a1177 = v1;
            let a1178 = 5u64;
            nat_add(a1177, a1178)?
        })?)?, f58(v0.clone(), {
            let a1179 = v1;
            let a1180 = 6u64;
            nat_add(a1179, a1180)?
        })?)?, f58(v0, {
            let a1181 = v1;
            let a1182 = 7u64;
            nat_add(a1181, a1182)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f429(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt2>> {
    let m1183 = f7(v0, v1)?;
    match m1183 {
        None => {
            Ok(Err::<(Str, u64), Adt2>(Adt2::C0(v1)))
        }
        Some(v2) => {
            let m1185 = f470(f25(match v2.clone() {
                Adt95::C0(h1184, _) => {
                    h1184
                }
            })?);
            match m1185 {
                None => {
                    Ok(Err::<(Str, u64), Adt2>(Adt2::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt2>((v3, match v2 {
                        Adt95::C0(_, h1186) => {
                            h1186
                        }
                    })))
                }
            }
        }
    }
}

pub fn f430(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt101, u64), Adt2>> {
    let m1188 = f429(v0.clone(), {
        let (_, h1187) = v2;
        h1187
    })?;
    match m1188 {
        Ok(v4) => {
            f471(v0, v1, {
                let (h1189, _) = v2;
                h1189
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt101, u64), Adt2>(v3))
        }
    }
}

pub fn f431(p1194: u64, p1195: List<Adt127>, p1196: List<u64>) -> R<Result<List<u64>, Adt116>> {
    let m1193 = (p1194, (p1195, p1196));
    {
        let mut m1193 = m1193;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1193;
                let m1190 = v0;
                if m1190 == 0 {
                    Err::<(u64, (List<Adt127>, List<u64>)), R<Result<List<u64>, Adt116>>>(Ok(Err::<List<u64>, Adt116>(Adt116::C13)))
                } else {
                    let v3 = m1190 - 1;
                    let m1191 = v1;
                    match m1191.uncons() {
                        None => {
                            Err::<(u64, (List<Adt127>, List<u64>)), R<Result<List<u64>, Adt116>>>(Ok(Ok::<List<u64>, Adt116>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m1192 = f472(v4, v5, v2)?;
                            match m1192 {
                                Adt151::C0(v6) => {
                                    Err::<(u64, (List<Adt127>, List<u64>)), R<Result<List<u64>, Adt116>>>(Ok(Err::<List<u64>, Adt116>(v6)))
                                }
                                Adt151::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt127>, List<u64>)), R<Result<List<u64>, Adt116>>>((v3, (v7, v8)))
                                }
                            }
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

pub fn f432(p1199: List<u64>, p1200: List<u64>) -> List<u64> {
    let m1198 = (p1199, p1200);
    {
        let mut m1198 = m1198;
        loop {
            let step = {
                let (v0, v1) = m1198;
                let m1197 = v0;
                match m1197.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1198 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: Adt109) -> Adt9 {
    let m1201 = v0;
    match m1201 {
        Adt109::C0(v1) => {
            Adt9::C3(v1)
        }
        Adt109::C1(v2) => {
            Adt9::C2(Adt10::C0(v2, 0i64))
        }
    }
}

pub fn f434(v0: Option<Adt9>) -> List<(Str, Adt9)> {
    let m1202 = v0;
    match m1202 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f435() -> i64 {
    -32700i64
}

pub fn f436() -> i64 {
    -32600i64
}

pub fn f437(v0: Adt118) -> Str {
    let m1203 = v0;
    match m1203 {
        Adt118::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt118::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt118::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt118::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt118::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt118::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt118::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt118::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt118::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f438(v0: Adt101, v1: bool, v2: Adt141) -> R<Adt103> {
    let m1205 = match v2.clone() {
        Adt141::C0(_, h1204) => {
            h1204
        }
    };
    match m1205 {
        Ok(v3) => {
            let m1206 = f473(v3.clone())?;
            match m1206 {
                None => {
                    Ok(f53())
                }
                Some(v4) => {
                    f474(v0, f475(match v3.clone() {
                        Adt142::C0(h1207, _, _) => {
                            h1207
                        }
                    }), f476(v1, f258(f477(), f478(match v2 {
                        Adt141::C0(h1208, _) => {
                            h1208
                        }
                    }, v4))?, f479(f480(match v3.clone() {
                        Adt142::C0(_, _, h1209) => {
                            h1209
                        }
                    }, List::<List<u64>>::nil())?, f481(f475(match v3 {
                        Adt142::C0(h1210, _, _) => {
                            h1210
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f258(f482(v5.clone()), f260(Some(match v2 {
                Adt141::C0(h1211, _) => {
                    h1211
                }
            }), f483(v5)?))
        }
    }
}

pub fn f439(p1216: List<Adt100>) -> R<bool> {
    let m1215 = p1216;
    {
        let mut m1215 = m1215;
        loop {
            let step = {
                let v0 = m1215;
                let m1212 = v0;
                match m1212.uncons() {
                    None => {
                        Err::<List<Adt100>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f386(match v1.clone() {
                            Adt100::C0(h1213, _) => {
                                h1213
                            }
                        }, f484())? {
                            if f485(f486(44u64, match v1 {
                                Adt100::C0(_, h1214) => {
                                    h1214
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
                Ok(next) => { m1215 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f440(v0: Adt3, v1: Fn3, v2: u64, v3: Option<Adt128>, v4: List<Str>, v5: Adt123) -> R<Adt141> {
    Ok(Adt141::C0(match v5.clone() {
        Adt123::C0(h1217, _, _) => {
            h1217
        }
    }, f487(v0, v1, v2, f488(v3, v4, v5)?)?))
}

pub fn f441(v0: List<Adt100>, v1: List<Adt15>, v2: Option<Str>, v3: Str) -> R<Adt128> {
    let v4: Adt147 = f489(Adt130::C0, f268(v0.clone(), f490())?);
    let v5: Adt147 = f489(Adt130::C1, f268(v0.clone(), f491())?);
    let v6: Adt147 = if f492(v3) {
        f489(Adt130::C2, f493(f268(v0.clone(), f494())?)?)
    } else {
        f495()
    };
    Ok(Adt128::C0(match v4.clone() {
        Adt147::C0(h1218, _) => {
            h1218
        }
    }, match v5.clone() {
        Adt147::C0(h1219, _) => {
            h1219
        }
    }, match v6.clone() {
        Adt147::C0(h1220, _) => {
            h1220
        }
    }, f496(match v4 {
        Adt147::C0(_, h1221) => {
            h1221
        }
    }, f496(match v5 {
        Adt147::C0(_, h1222) => {
            h1222
        }
    }, match v6 {
        Adt147::C0(_, h1223) => {
            h1223
        }
    })), f497(v0, v1, v2)?))
}

pub fn f442(v0: Adt123) -> Option<Str> {
    let a1225 = match v0.clone() {
        Adt123::C0(_, h1224, _) => {
            h1224
        }
    };
    let a1226 = Str::lit("tools/call");
    if equal(a1225, a1226) {
        let m1228 = match v0 {
            Adt123::C0(_, _, h1227) => {
                h1227
            }
        };
        match m1228 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m1229 = f233(Str::lit("name"), v1);
                match m1229 {
                    Adt115::C0 => {
                        None::<Str>
                    }
                    Adt115::C1(v2) => {
                        f283(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f443() -> List<Str> {
    List::cons(f498(), List::<Str>::nil())
}

pub fn f444(v0: Adt8) -> R<Adt148> {
    let m1230 = f233(Str::lit("id"), v0);
    match m1230 {
        Adt115::C0 => {
            Ok(Adt148::C0)
        }
        Adt115::C1(v1) => {
            let m1231 = v1;
            match m1231 {
                Adt9::C0 => {
                    Ok(Adt148::C2)
                }
                Adt9::C1(_) => {
                    Ok(Adt148::C2)
                }
                Adt9::C2(v3) => {
                    let m1232 = f499(v3)?;
                    match m1232 {
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

pub fn f445(v0: Adt8) -> Adt118 {
    if f446(v0) {
        Adt118::C8
    } else {
        Adt118::C3
    }
}

pub fn f446(v0: Adt8) -> bool {
    if f500(Str::lit("result"), v0.clone()) {
        true
    } else {
        f500(Str::lit("error"), v0)
    }
}

pub fn f447(v0: Adt109, v1: Adt8) -> Result<Adt123, Adt117> {
    let m1233 = f233(Str::lit("jsonrpc"), v1.clone());
    match m1233 {
        Adt115::C0 => {
            f382(Some(v0), Adt118::C2)
        }
        Adt115::C1(v2) => {
            let m1234 = f283(v2);
            match m1234 {
                None => {
                    f382(Some(v0), Adt118::C2)
                }
                Some(v3) => {
                    let a1235 = v3;
                    let a1236 = f373();
                    if equal(a1235, a1236) {
                        f501(v0, v1)
                    } else {
                        f382(Some(v0), Adt118::C2)
                    }
                }
            }
        }
    }
}

pub fn f448(v0: u64, v1: u64) -> Adt133 {
    let a1237 = v1;
    let a1238 = 0u64;
    if nat_eq(a1237, a1238) {
        Adt133::C0
    } else {
        let a1239 = v1;
        let a1240 = 2u64;
        if nat_eq(a1239, a1240) {
            Adt133::C2(v0)
        } else {
            Adt133::C1({
                let a1244 = v0;
                let a1245 = {
                    let a1241 = v1;
                    let a1242 = 3u64;
                    let a1243 = 0u64;
                    nat_quot(a1241, a1242, a1243)
                };
                nat_sub(a1244, a1245)
            })
        }
    }
}

pub fn f449(p1273: Adt134, p1274: u64, p1275: u64, p1276: List<u64>) -> R<u64> {
    let m1272 = (p1273, (p1274, (p1275, p1276)));
    {
        let mut m1272 = m1272;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1272;
                let m1246 = v3;
                match m1246.uncons() {
                    None => {
                        let m1247 = v0;
                        match m1247 {
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
                        let m1248 = v0;
                        match m1248 {
                            Adt134::C0 => {
                                let a1249 = v4;
                                let a1250 = 128u64;
                                if nat_lt(a1249, a1250) {
                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a1251 = v4;
                                    let a1252 = 194u64;
                                    if nat_lt(a1251, a1252) {
                                        let v6: u64 = f502({
                                            let a1253 = List::cons(v4, v5);
                                            length_list(a1253)
                                        })?;
                                        Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a1254 = v4;
                                        let a1255 = 224u64;
                                        if nat_lt(a1254, a1255) {
                                            Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a1256 = v4;
                                            let a1257 = 224u64;
                                            if nat_eq(a1256, a1257) {
                                                Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a1258 = v4;
                                                let a1259 = 237u64;
                                                if nat_eq(a1258, a1259) {
                                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a1260 = v4;
                                                    let a1261 = 240u64;
                                                    if nat_lt(a1260, a1261) {
                                                        Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a1262 = v4;
                                                        let a1263 = 240u64;
                                                        if nat_eq(a1262, a1263) {
                                                            Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a1264 = v4;
                                                            let a1265 = 244u64;
                                                            if nat_eq(a1264, a1265) {
                                                                Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a1266 = v4;
                                                                let a1267 = 244u64;
                                                                if nat_lt(a1266, a1267) {
                                                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f502({
                                                                        let a1268 = List::cons(v4, v5);
                                                                        length_list(a1268)
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
                                if f503(v4, v1, v2) {
                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f502({
                                        let a1269 = List::cons(v4, v5);
                                        length_list(a1269)
                                    })?;
                                    Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt134::C2 => {
                                if f503(v4, v1, v2) {
                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f502({
                                        let a1270 = List::cons(v4, v5);
                                        length_list(a1270)
                                    })?;
                                    Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt134::C3 => {
                                if f503(v4, v1, v2) {
                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f502({
                                        let a1271 = List::cons(v4, v5);
                                        length_list(a1271)
                                    })?;
                                    Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f450(v0: Adt135, v1: u64) -> R<Result<Adt9, Adt116>> {
    let m1278 = match v0.clone() {
        Adt135::C0(h1277, _, _) => {
            h1277
        }
    };
    match m1278 {
        Adt136::C0 => {
            Ok(Err::<Adt9, Adt116>(Adt116::C3))
        }
        Adt136::C1 => {
            Ok(Err::<Adt9, Adt116>(Adt116::C3))
        }
        Adt136::C2 => {
            Ok(Err::<Adt9, Adt116>(Adt116::C3))
        }
        Adt136::C3 => {
            Ok(Err::<Adt9, Adt116>(Adt116::C3))
        }
        Adt136::C4 => {
            Ok(Err::<Adt9, Adt116>(Adt116::C3))
        }
        Adt136::C5 => {
            Ok(Err::<Adt9, Adt116>(Adt116::C3))
        }
        Adt136::C6(_, _, _) => {
            Ok(Err::<Adt9, Adt116>(Adt116::C3))
        }
        Adt136::C7(_, _) => {
            Ok(Err::<Adt9, Adt116>(Adt116::C3))
        }
        Adt136::C8(v3) => {
            let m1279 = f504(v3)?;
            match m1279 {
                Adt154::C0 => {
                    Ok(Err::<Adt9, Adt116>(Adt116::C7(v1)))
                }
                Adt154::C1 => {
                    Ok(Err::<Adt9, Adt116>(Adt116::C8(v1)))
                }
                Adt154::C2(v4) => {
                    let m1284 = {
                        let m1283 = f505(v4, match v0.clone() {
                            Adt135::C0(_, h1281, _) => {
                                h1281
                            }
                        }, match v0 {
                            Adt135::C0(_, _, h1282) => {
                                h1282
                            }
                        });
                        match m1283 {
                            Adt135::C0(h1280, _, _) => {
                                h1280
                            }
                        }
                    };
                    match m1284 {
                        Adt136::C0 => {
                            Ok(Err::<Adt9, Adt116>(Adt116::C3))
                        }
                        Adt136::C1 => {
                            Ok(Err::<Adt9, Adt116>(Adt116::C3))
                        }
                        Adt136::C2 => {
                            Ok(Err::<Adt9, Adt116>(Adt116::C3))
                        }
                        Adt136::C3 => {
                            Ok(Err::<Adt9, Adt116>(Adt116::C3))
                        }
                        Adt136::C4 => {
                            Ok(Err::<Adt9, Adt116>(Adt116::C3))
                        }
                        Adt136::C5 => {
                            Ok(Err::<Adt9, Adt116>(Adt116::C3))
                        }
                        Adt136::C6(_, _, _) => {
                            Ok(Err::<Adt9, Adt116>(Adt116::C3))
                        }
                        Adt136::C7(_, _) => {
                            Ok(Err::<Adt9, Adt116>(Adt116::C3))
                        }
                        Adt136::C8(_) => {
                            Ok(Err::<Adt9, Adt116>(Adt116::C3))
                        }
                        Adt136::C9(v5) => {
                            Ok(Ok::<Adt9, Adt116>(v5))
                        }
                    }
                }
            }
        }
        Adt136::C9(v2) => {
            Ok(Ok::<Adt9, Adt116>(v2))
        }
    }
}

pub fn f451(v0: Adt135, v1: u64, v2: u64) -> R<Adt149> {
    let m1286 = match v0.clone() {
        Adt135::C0(h1285, _, _) => {
            h1285
        }
    };
    match m1286 {
        Adt136::C0 => {
            if f506(v2) {
                Ok(Adt149::C1(v0))
            } else {
                f507(v0, v1, v2)
            }
        }
        Adt136::C1 => {
            if f506(v2) {
                Ok(Adt149::C1(v0))
            } else {
                let a1287 = v2;
                let a1288 = 93u64;
                if nat_eq(a1287, a1288) {
                    Ok(f508(v0, v1))
                } else {
                    f507(v0, v1, v2)
                }
            }
        }
        Adt136::C2 => {
            Ok(f509(v0, v1, v2))
        }
        Adt136::C3 => {
            if f506(v2) {
                Ok(Adt149::C1(v0))
            } else {
                let a1289 = v2;
                let a1290 = 125u64;
                if nat_eq(a1289, a1290) {
                    Ok(f510(v0, v1))
                } else {
                    let a1291 = v2;
                    let a1292 = 34u64;
                    if nat_eq(a1291, a1292) {
                        Ok(f511(v0, Adt136::C6(Adt137::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f512(Adt116::C2(v1)))
                    }
                }
            }
        }
        Adt136::C4 => {
            if f506(v2) {
                Ok(Adt149::C1(v0))
            } else {
                let a1293 = v2;
                let a1294 = 34u64;
                if nat_eq(a1293, a1294) {
                    Ok(f511(v0, Adt136::C6(Adt137::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f512(Adt116::C2(v1)))
                }
            }
        }
        Adt136::C5 => {
            if f506(v2) {
                Ok(Adt149::C1(v0))
            } else {
                let a1295 = v2;
                let a1296 = 58u64;
                if nat_eq(a1295, a1296) {
                    Ok(f511(v0, Adt136::C0))
                } else {
                    Ok(f512(Adt116::C2(v1)))
                }
            }
        }
        Adt136::C6(v3, v4, v5) => {
            f513(v0, v1, v2, v3, v4, v5)
        }
        Adt136::C7(v6, v7) => {
            f514(v0, v1, v2, v6, v7)
        }
        Adt136::C8(v8) => {
            f515(v0, v1, v2, v8)
        }
        Adt136::C9(_) => {
            Ok(f509(v0, v1, v2))
        }
    }
}

pub fn f452(v0: u64) -> R<u64> {
    let a1297 = 64u64;
    let a1298 = v0;
    if if nat_lt(a1297, a1298) {
        let a1299 = v0;
        let a1300 = 91u64;
        nat_lt(a1299, a1300)
    } else {
        false
    } {
        let a1301 = v0;
        let a1302 = 32u64;
        nat_add(a1301, a1302)
    } else {
        Ok(v0)
    }
}

pub fn f453(p1305: List<u64>) -> List<u64> {
    let m1304 = p1305;
    {
        let mut m1304 = m1304;
        loop {
            let step = {
                let v0 = m1304;
                let m1303 = v0;
                match m1303.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f516(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1304 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: u64) -> bool {
    let a1306 = v0;
    let a1307 = 9u64;
    if nat_eq(a1306, a1307) {
        true
    } else {
        let a1308 = 31u64;
        let a1309 = v0;
        if nat_lt(a1308, a1309) {
            let a1310 = v0;
            let a1311 = 127u64;
            nat_lt(a1310, a1311)
        } else {
            false
        }
    }
}

pub fn f455(p1314: List<u64>) -> bool {
    let m1313 = p1314;
    {
        let mut m1313 = m1313;
        loop {
            let step = {
                let v0 = m1313;
                let m1312 = v0;
                match m1312.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f517(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: bool, v1: Adt141) -> R<Adt96> {
    let m1316 = f518(match v1.clone() {
        Adt141::C0(_, h1315) => {
            h1315
        }
    });
    match m1316 {
        None => {
            Ok(Adt96::C0(f52(f520(v0, v1)?)))
        }
        Some(v2) => {
            f519(v0, v1, v2)
        }
    }
}

pub fn f457(v0: u64) -> bool {
    let a1317 = 48u64;
    let a1318 = v0;
    if if nat_le(a1317, a1318) {
        let a1319 = v0;
        let a1320 = 57u64;
        nat_le(a1319, a1320)
    } else {
        false
    } {
        true
    } else {
        let a1321 = 65u64;
        let a1322 = v0;
        if if nat_le(a1321, a1322) {
            let a1323 = v0;
            let a1324 = 90u64;
            nat_le(a1323, a1324)
        } else {
            false
        } {
            true
        } else {
            let a1325 = 97u64;
            let a1326 = v0;
            if if nat_le(a1325, a1326) {
                let a1327 = v0;
                let a1328 = 122u64;
                nat_le(a1327, a1328)
            } else {
                false
            } {
                true
            } else {
                let a1329 = v0;
                let a1330 = 33u64;
                if nat_eq(a1329, a1330) {
                    true
                } else {
                    let a1331 = v0;
                    let a1332 = 35u64;
                    if nat_eq(a1331, a1332) {
                        true
                    } else {
                        let a1333 = v0;
                        let a1334 = 36u64;
                        if nat_eq(a1333, a1334) {
                            true
                        } else {
                            let a1335 = v0;
                            let a1336 = 37u64;
                            if nat_eq(a1335, a1336) {
                                true
                            } else {
                                let a1337 = v0;
                                let a1338 = 38u64;
                                if nat_eq(a1337, a1338) {
                                    true
                                } else {
                                    let a1339 = v0;
                                    let a1340 = 39u64;
                                    if nat_eq(a1339, a1340) {
                                        true
                                    } else {
                                        let a1341 = v0;
                                        let a1342 = 42u64;
                                        if nat_eq(a1341, a1342) {
                                            true
                                        } else {
                                            let a1343 = v0;
                                            let a1344 = 43u64;
                                            if nat_eq(a1343, a1344) {
                                                true
                                            } else {
                                                let a1345 = v0;
                                                let a1346 = 45u64;
                                                if nat_eq(a1345, a1346) {
                                                    true
                                                } else {
                                                    let a1347 = v0;
                                                    let a1348 = 46u64;
                                                    if nat_eq(a1347, a1348) {
                                                        true
                                                    } else {
                                                        let a1349 = v0;
                                                        let a1350 = 94u64;
                                                        if nat_eq(a1349, a1350) {
                                                            true
                                                        } else {
                                                            let a1351 = v0;
                                                            let a1352 = 95u64;
                                                            if nat_eq(a1351, a1352) {
                                                                true
                                                            } else {
                                                                let a1353 = v0;
                                                                let a1354 = 96u64;
                                                                if nat_eq(a1353, a1354) {
                                                                    true
                                                                } else {
                                                                    let a1355 = v0;
                                                                    let a1356 = 124u64;
                                                                    if nat_eq(a1355, a1356) {
                                                                        true
                                                                    } else {
                                                                        let a1357 = v0;
                                                                        let a1358 = 126u64;
                                                                        nat_eq(a1357, a1358)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f458(v0: u64) -> bool {
    let a1359 = 48u64;
    let a1360 = v0;
    if if nat_le(a1359, a1360) {
        let a1361 = v0;
        let a1362 = 57u64;
        nat_le(a1361, a1362)
    } else {
        false
    } {
        true
    } else {
        let a1363 = 65u64;
        let a1364 = v0;
        if if nat_le(a1363, a1364) {
            let a1365 = v0;
            let a1366 = 90u64;
            nat_le(a1365, a1366)
        } else {
            false
        } {
            true
        } else {
            let a1367 = 97u64;
            let a1368 = v0;
            if if nat_le(a1367, a1368) {
                let a1369 = v0;
                let a1370 = 122u64;
                nat_le(a1369, a1370)
            } else {
                false
            } {
                true
            } else {
                let a1371 = v0;
                let a1372 = 95u64;
                if nat_eq(a1371, a1372) {
                    true
                } else {
                    let a1373 = v0;
                    let a1374 = 45u64;
                    if nat_eq(a1373, a1374) {
                        true
                    } else {
                        let a1375 = v0;
                        let a1376 = 46u64;
                        nat_eq(a1375, a1376)
                    }
                }
            }
        }
    }
}

pub fn f459() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f460(v0: Str, v1: Adt9, v2: List<(Str, Adt9)>, v3: bool, v4: List<Str>, v5: Adt124, v6: List<Adt121>) -> R<Result<(List<Adt121>, Adt124), Adt104>> {
    let m1377 = f521(v0.clone(), v1, v2.clone());
    match m1377 {
        None => {
            let m1378 = f522(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1378 {
                Adt155::C0(v8) => {
                    Ok(Err::<(List<Adt121>, Adt124), Adt104>(v8))
                }
                Adt155::C1(v9) => {
                    Ok(Ok::<(List<Adt121>, Adt124), Adt104>((f523(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt121>, Adt124), Adt104>(v7))
        }
    }
}

pub fn f461(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt125> {
    Ok(f524(List::cons((Str::lit("type"), f525(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f526(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f527(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f528(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f528(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f528(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f528(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1379 = f529(v0.clone(), v1.clone())?;
        let a1380 = List::cons((Str::lit("required"), f530(v0, v1)), List::<(Str, Adt150)>::nil());
        append_list(a1379, a1380)
    })))))))))
}

pub fn f462(v0: Adt9, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt111>) -> R<Adt125> {
    Ok(f531(f532(v0, v1.clone(), v2.clone(), v3.clone()), f531(f533(v1.clone(), v2.clone(), v3.clone()), f531(f534(v1.clone(), v2.clone(), v3.clone()), f531(f535(v1.clone(), v2.clone(), v3.clone())?, f531(f536(v1.clone(), v2.clone(), v3.clone()), f531(f537(Adt114::C0, f407(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f531(f537(Adt114::C1, f408(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f531(f537(Adt114::C2, f409(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f537(Adt114::C3, f410(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f463(v0: Str, v1: List<(Str, Adt9)>) -> Option<Adt9> {
    let m1381 = f305(v0, v1);
    match m1381 {
        Adt115::C0 => {
            None::<Adt9>
        }
        Adt115::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f464(v0: Str) -> R<Str> {
    let m1384 = {
        let a1383 = f358(f255(), f432(f538(f25({
            let a1382 = v0.clone();
            utf8_encode(a1382)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1383)
    };
    match m1384 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f465(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1386 = f539({
        let a1385 = v0.clone();
        length_list(a1385)
    }, v0, List::<u64>::nil())?;
    match m1386 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f432(v1, List::<u64>::nil())))
        }
    }
}

pub fn f466(v0: List<Adt54>) -> R<bool> {
    let m1387 = v0;
    match m1387.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f540(v2, match v1 {
                Adt54::C0(h1388, _, _) => {
                    h1388
                }
            })
        }
    }
}

pub fn f467(p1397: Bytes, p1398: List<Bytes>) -> List<Bytes> {
    let m1396 = (p1397, p1398);
    {
        let mut m1396 = m1396;
        loop {
            let step = {
                let (v0, v1) = m1396;
                let m1389 = v1;
                match m1389.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1392 = {
                            let a1390 = v2.clone();
                            length_bytes(a1390)
                        };
                        let a1393 = {
                            let a1391 = v0.clone();
                            length_bytes(a1391)
                        };
                        if nat_eq(a1392, a1393) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1394 = v2;
                                let a1395 = v0;
                                append_bytes(a1394, a1395)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1396 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f468(v0: u64) -> Bytes {
    let m1402 = {
        let a1399 = f541();
        let a1400 = v0;
        let a1401 = 1u64;
        slice_bytes(a1399, a1400, a1401)
    };
    match m1402 {
        None => {
            f255()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f469(v0: i64, v1: u64) -> R<i64> {
    f355({
        let a1403 = v0;
        let a1404 = 256i64;
        int_mul(a1403, a1404)?
    }, f285(v1, 0i64)?)
}

pub fn f470(v0: List<u64>) -> Option<Str> {
    let a1405 = f358(f255(), v0);
    utf8_decode(a1405)
}

pub fn f471(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt101, u64), Adt2>> {
    let m1407 = f7(v0, {
        let (_, h1406) = v3.clone();
        h1406
    })?;
    match m1407 {
        None => {
            Ok(Err::<(Adt101, u64), Adt2>(Adt2::C0({
                let (_, h1408) = v3;
                h1408
            })))
        }
        Some(v4) => {
            Ok(f542(v1, v2, {
                let (h1409, _) = v3;
                h1409
            }, v4))
        }
    }
}

pub fn f472(v0: Adt127, v1: List<Adt127>, v2: List<u64>) -> R<Adt151> {
    let m1410 = v0;
    match m1410 {
        Adt127::C0(v3) => {
            Ok(Adt151::C1(v1, f432(v3, v2)))
        }
        Adt127::C1(v4, v5) => {
            f543(v4, v5, v1, v2)
        }
        Adt127::C2(v6, v7) => {
            let m1411 = v7;
            match m1411.uncons() {
                None => {
                    Ok(Adt151::C1(List::cons(Adt127::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt151::C1(List::cons(Adt127::C1(v6, v8), List::cons(Adt127::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt127::C3(v10, v11) => {
            let m1412 = v11.clone();
            match m1412.uncons() {
                None => {
                    Ok(Adt151::C1(List::cons(Adt127::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt151::C1(List::cons(Adt127::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt127::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt127::C4(v14, v15) => {
            let m1413 = v15;
            match m1413.uncons() {
                None => {
                    Ok(Adt151::C1(List::cons(Adt127::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt151::C1(List::cons(Adt127::C0({
                        let a1415 = f544({
                            let (h1414, _) = v16;
                            h1414
                        })?;
                        let a1416 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1415, a1416)
                    }), List::cons(Adt127::C1(v14, {
                        let (_, h1417) = v16;
                        h1417
                    }), List::cons(Adt127::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt127::C5(v18, v19) => {
            let m1418 = v19.clone();
            match m1418.uncons() {
                None => {
                    Ok(Adt151::C1(List::cons(Adt127::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt151::C1(List::cons(Adt127::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt127::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f473(v0: Adt142) -> R<Option<Adt8>> {
    let m1420 = f545(match v0.clone() {
        Adt142::C0(h1419, _, _) => {
            h1419
        }
    })?;
    match m1420 {
        None => {
            Ok(None::<Adt8>)
        }
        Some(v1) => {
            Ok(Some(Adt8::C0({
                let a1423 = v1;
                let a1424 = {
                    let m1422 = f546(v0);
                    match m1422 {
                        Adt8::C0(h1421) => {
                            h1421
                        }
                    }
                };
                append_list(a1423, a1424)
            })))
        }
    }
}

pub fn f474(v0: Adt101, v1: Option<Adt91>, v2: Adt103) -> R<Adt103> {
    let m1425 = v1;
    match m1425 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a1427 = match v2.clone() {
                Adt103::C0(h1426, _, _, _, _) => {
                    h1426
                }
            };
            let a1428 = f477();
            if nat_eq(a1427, a1428) {
                Ok(f547(f548(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f475(v0: Adt36) -> Option<Adt91> {
    let m1429 = v0;
    match m1429 {
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
                Adt86::C0(_, _, h1430) => {
                    h1430
                }
            }, Adt92::C0, false))
        }
        Adt36::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f476(v0: bool, v1: Adt103, v2: Option<List<List<u64>>>) -> Adt103 {
    let m1431 = v2;
    match m1431 {
        None => {
            f53()
        }
        Some(v3) => {
            f549(v0, v1, f171(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f477() -> u64 {
    200u64
}

pub fn f478(v0: Adt109, v1: Adt8) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("jsonrpc"), Adt9::C3(f373())), List::cons((Str::lit("id"), f433(v0)), List::cons((Str::lit("result"), Adt9::C5(match v1 {
        Adt8::C0(h1432) => {
            h1432
        }
    })), List::<(Str, Adt9)>::nil()))))
}

pub fn f479(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
    let m1433 = v0;
    match m1433 {
        None => {
            None::<List<List<u64>>>
        }
        Some(v2) => {
            let m1434 = v1;
            match m1434 {
                None => {
                    None::<List<List<u64>>>
                }
                Some(v3) => {
                    Some(f171(v3, v2))
                }
            }
        }
    }
}

pub fn f480(p1438: List<Adt144>, p1439: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1437 = (p1438, p1439);
    {
        let mut m1437 = m1437;
        loop {
            let step = {
                let (v0, v1) = m1437;
                let m1435 = v0;
                match m1435.uncons() {
                    None => {
                        Err::<(List<Adt144>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1436 = f550(v2)?;
                        match m1436 {
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
                Ok(next) => { m1437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f481(v0: Option<Adt91>) -> R<Option<List<List<u64>>>> {
    let m1440 = v0;
    match m1440 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f551(v1)
        }
    }
}

pub fn f482(v0: Adt145) -> u64 {
    let m1441 = v0;
    match m1441 {
        Adt145::C0(_) => {
            f326()
        }
        Adt145::C1(_, _) => {
            f326()
        }
        Adt145::C2(_) => {
            f326()
        }
        Adt145::C3(_) => {
            f552()
        }
        Adt145::C4(v6) => {
            f553(v6)
        }
    }
}

pub fn f483(v0: Adt145) -> R<Adt84> {
    Ok(Adt84::C0(f554(v0.clone()), f555(v0.clone()), f556(v0)?))
}

pub fn f484() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f485(p1444: List<List<u64>>) -> R<bool> {
    let m1443 = p1444;
    {
        let mut m1443 = m1443;
        loop {
            let step = {
                let v0 = m1443;
                let m1442 = v0;
                match m1442.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f557(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1443 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f486(v0: u64, p1449: List<u64>, p1450: List<u64>, p1451: List<List<u64>>) -> List<List<u64>> {
    let m1448 = (p1449, (p1450, p1451));
    {
        let mut m1448 = m1448;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1448;
                let m1445 = v1;
                match m1445.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f171(List::cons(f396(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1446 = v4;
                        let a1447 = v0;
                        if nat_eq(a1446, a1447) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f396(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
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

pub fn f487(v0: Adt3, v1: Fn3, v2: u64, v3: Result<Adt152, Adt145>) -> R<Result<Adt142, Adt145>> {
    let m1452 = v3;
    match m1452 {
        Ok(v5) => {
            f558(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt142, Adt145>(v4))
        }
    }
}

pub fn f488(v0: Option<Adt128>, v1: List<Str>, v2: Adt123) -> R<Result<Adt152, Adt145>> {
    Ok(f559(f560(match v2.clone() {
        Adt123::C0(_, h1453, _) => {
            h1453
        }
    }), f561(v0, v1, v2)?))
}

pub fn f489(v0: Adt130, v1: Adt119) -> Adt147 {
    let m1454 = v1;
    match m1454 {
        Adt119::C0 => {
            f495()
        }
        Adt119::C1 => {
            Adt147::C0(None::<Str>, Some(Adt129::C6(v0)))
        }
        Adt119::C2 => {
            Adt147::C0(None::<Str>, Some(Adt129::C7(v0)))
        }
        Adt119::C3(v2) => {
            let m1455 = f470(v2);
            match m1455 {
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

pub fn f490() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f491() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f492(v0: Str) -> bool {
    let m1456 = f562(v0);
    match m1456 {
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

pub fn f493(v0: Adt119) -> R<Adt119> {
    let m1457 = v0.clone();
    match m1457 {
        Adt119::C0 => {
            Ok(v0)
        }
        Adt119::C1 => {
            Ok(v0)
        }
        Adt119::C2 => {
            Ok(v0)
        }
        Adt119::C3(v1) => {
            let m1458 = f563(v1);
            match m1458 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1459 = f465(v2)?;
                    match m1459 {
                        None => {
                            Ok(Adt119::C1)
                        }
                        Some(v3) => {
                            Ok(Adt119::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f494() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f495() -> Adt147 {
    Adt147::C0(None::<Str>, None::<Adt129>)
}

pub fn f496(v0: Option<Adt129>, v1: Option<Adt129>) -> Option<Adt129> {
    let m1460 = v0.clone();
    match m1460 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f497(v0: List<Adt100>, v1: List<Adt15>, v2: Option<Str>) -> R<List<Adt131>> {
    Ok(f564(f565(v0, v1, v2, List::<Adt131>::nil())?, List::<Adt131>::nil()))
}

pub fn f498() -> Str {
    Str::lit("2026-07-28")
}

pub fn f499(v0: Adt10) -> R<Option<i64>> {
    let m1463 = f566(40u64, match v0.clone() {
        Adt10::C0(h1461, _) => {
            h1461
        }
    }, match v0 {
        Adt10::C0(_, h1462) => {
            h1462
        }
    })?;
    match m1463 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f567(v1))
        }
    }
}

pub fn f500(v0: Str, v1: Adt8) -> bool {
    f568(f233(v0, v1))
}

pub fn f501(v0: Adt109, v1: Adt8) -> Result<Adt123, Adt117> {
    let m1464 = f233(Str::lit("method"), v1.clone());
    match m1464 {
        Adt115::C0 => {
            f382(Some(v0), f445(v1))
        }
        Adt115::C1(v2) => {
            let m1465 = f283(v2);
            match m1465 {
                None => {
                    f382(Some(v0), Adt118::C4)
                }
                Some(v3) => {
                    f569(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f502(v0: u64) -> R<u64> {
    let a1468 = {
        let a1466 = v0;
        let a1467 = 3u64;
        nat_mul(a1466, a1467)?
    };
    let a1469 = 1u64;
    nat_add(a1468, a1469)
}

pub fn f503(v0: u64, v1: u64, v2: u64) -> bool {
    let a1470 = v1;
    let a1471 = v0;
    if nat_le(a1470, a1471) {
        let a1472 = v0;
        let a1473 = v2;
        nat_le(a1472, a1473)
    } else {
        false
    }
}

pub fn f504(v0: Adt138) -> R<Adt154> {
    if f570(match v0.clone() {
        Adt138::C0(_, h1474, _, _, _, _, _) => {
            h1474
        }
    }) {
        let v1: i64 = {
            let a1480 = if match v0.clone() {
                Adt138::C0(_, _, _, _, _, h1475, _) => {
                    h1475
                }
            } {
                let a1477 = match v0.clone() {
                    Adt138::C0(_, _, _, _, _, _, h1476) => {
                        h1476
                    }
                };
                int_neg(a1477)?
            } else {
                match v0.clone() {
                    Adt138::C0(_, _, _, _, _, _, h1478) => {
                        h1478
                    }
                }
            };
            let a1481 = match v0.clone() {
                Adt138::C0(_, _, _, _, h1479, _, _) => {
                    h1479
                }
            };
            int_sub(a1480, a1481)?
        };
        let m1484 = {
            let a1482 = v1;
            let a1483 = -9999i64;
            compare(a1482, a1483)
        };
        if if match m1484 {
            Ordering::Less => {
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
            let m1487 = {
                let a1485 = 9999i64;
                let a1486 = v1;
                compare(a1485, a1486)
            };
            match m1487 {
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
            Ok(Adt154::C1)
        } else {
            Ok(Adt154::C2(Adt9::C2(Adt10::C0(if match v0.clone() {
                Adt138::C0(h1488, _, _, _, _, _, _) => {
                    h1488
                }
            } {
                let a1490 = match v0 {
                    Adt138::C0(_, _, h1489, _, _, _, _) => {
                        h1489
                    }
                };
                int_neg(a1490)?
            } else {
                match v0 {
                    Adt138::C0(_, _, h1491, _, _, _, _) => {
                        h1491
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt154::C0)
    }
}

pub fn f505(v0: Adt9, v1: List<Adt140>, v2: u64) -> Adt135 {
    let m1492 = v1.clone();
    match m1492.uncons() {
        None => {
            Adt135::C0(Adt136::C9(v0), List::<Adt140>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1493 = v3;
            match m1493 {
                Adt140::C0(v5) => {
                    Adt135::C0(Adt136::C2, List::cons(Adt140::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt140::C1(v6, v7) => {
                    let m1494 = v7;
                    match m1494 {
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

pub fn f506(v0: u64) -> bool {
    let a1495 = v0;
    let a1496 = 32u64;
    if nat_eq(a1495, a1496) {
        true
    } else {
        let a1497 = v0;
        let a1498 = 10u64;
        if nat_eq(a1497, a1498) {
            true
        } else {
            let a1499 = v0;
            let a1500 = 13u64;
            if nat_eq(a1499, a1500) {
                true
            } else {
                let a1501 = v0;
                let a1502 = 9u64;
                nat_eq(a1501, a1502)
            }
        }
    }
}

pub fn f507(v0: Adt135, v1: u64, v2: u64) -> R<Adt149> {
    let a1503 = v2;
    let a1504 = 34u64;
    if nat_eq(a1503, a1504) {
        Ok(f511(v0, Adt136::C6(Adt137::C0, List::<u64>::nil(), false)))
    } else {
        let a1505 = v2;
        let a1506 = 123u64;
        if nat_eq(a1505, a1506) {
            f571(v0, Adt140::C1(List::<(Str, Adt9)>::nil(), None::<Str>), Adt136::C3, v1)
        } else {
            let a1507 = v2;
            let a1508 = 91u64;
            if nat_eq(a1507, a1508) {
                f571(v0, Adt140::C0(List::<Adt9>::nil()), Adt136::C1, v1)
            } else {
                let a1509 = v2;
                let a1510 = 116u64;
                if nat_eq(a1509, a1510) {
                    Ok(f511(v0, Adt136::C7(0u64, 1u64)))
                } else {
                    let a1511 = v2;
                    let a1512 = 102u64;
                    if nat_eq(a1511, a1512) {
                        Ok(f511(v0, Adt136::C7(1u64, 1u64)))
                    } else {
                        let a1513 = v2;
                        let a1514 = 110u64;
                        if nat_eq(a1513, a1514) {
                            Ok(f511(v0, Adt136::C7(2u64, 1u64)))
                        } else {
                            let a1515 = v2;
                            let a1516 = 45u64;
                            if nat_eq(a1515, a1516) {
                                Ok(f511(v0, f572(true, Adt139::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1517 = v2;
                                let a1518 = 48u64;
                                if nat_eq(a1517, a1518) {
                                    Ok(f511(v0, f572(false, Adt139::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f503(v2, 49u64, 57u64) {
                                        Ok(f511(v0, f572(false, Adt139::C2, f573({
                                            let a1519 = v2;
                                            let a1520 = 48u64;
                                            nat_sub(a1519, a1520)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f512(Adt116::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f508(v0: Adt135, v1: u64) -> Adt149 {
    let m1522 = match v0.clone() {
        Adt135::C0(_, h1521, _) => {
            h1521
        }
    };
    match m1522.uncons() {
        None => {
            f512(Adt116::C2(v1))
        }
        Some((v2, v3)) => {
            let m1523 = v2;
            match m1523 {
                Adt140::C0(v4) => {
                    Adt149::C1(f505(Adt9::C4(f574(v4, List::<Adt9>::nil())), v3, {
                        let a1525 = match v0 {
                            Adt135::C0(_, _, h1524) => {
                                h1524
                            }
                        };
                        let a1526 = 1u64;
                        nat_sub(a1525, a1526)
                    }))
                }
                Adt140::C1(_, _) => {
                    f512(Adt116::C2(v1))
                }
            }
        }
    }
}

pub fn f509(v0: Adt135, v1: u64, v2: u64) -> Adt149 {
    let m1528 = match v0.clone() {
        Adt135::C0(h1527, _, _) => {
            h1527
        }
    };
    match m1528 {
        Adt136::C0 => {
            f512(Adt116::C2(v1))
        }
        Adt136::C1 => {
            f512(Adt116::C2(v1))
        }
        Adt136::C2 => {
            if f506(v2) {
                Adt149::C1(v0)
            } else {
                let a1529 = v2;
                let a1530 = 44u64;
                if nat_eq(a1529, a1530) {
                    f575(v0, v1)
                } else {
                    let a1531 = v2;
                    let a1532 = 93u64;
                    if nat_eq(a1531, a1532) {
                        f508(v0, v1)
                    } else {
                        let a1533 = v2;
                        let a1534 = 125u64;
                        if nat_eq(a1533, a1534) {
                            f510(v0, v1)
                        } else {
                            f512(Adt116::C2(v1))
                        }
                    }
                }
            }
        }
        Adt136::C3 => {
            f512(Adt116::C2(v1))
        }
        Adt136::C4 => {
            f512(Adt116::C2(v1))
        }
        Adt136::C5 => {
            f512(Adt116::C2(v1))
        }
        Adt136::C6(_, _, _) => {
            f512(Adt116::C2(v1))
        }
        Adt136::C7(_, _) => {
            f512(Adt116::C2(v1))
        }
        Adt136::C8(_) => {
            f512(Adt116::C2(v1))
        }
        Adt136::C9(_) => {
            if f506(v2) {
                Adt149::C1(v0)
            } else {
                f512(Adt116::C10(v1))
            }
        }
    }
}

pub fn f510(v0: Adt135, v1: u64) -> Adt149 {
    let m1536 = match v0.clone() {
        Adt135::C0(_, h1535, _) => {
            h1535
        }
    };
    match m1536.uncons() {
        None => {
            f512(Adt116::C2(v1))
        }
        Some((v2, v3)) => {
            let m1537 = v2;
            match m1537 {
                Adt140::C0(_) => {
                    f512(Adt116::C2(v1))
                }
                Adt140::C1(v5, _) => {
                    Adt149::C1(f505(Adt9::C5(f576(v5, List::<(Str, Adt9)>::nil())), v3, {
                        let a1539 = match v0 {
                            Adt135::C0(_, _, h1538) => {
                                h1538
                            }
                        };
                        let a1540 = 1u64;
                        nat_sub(a1539, a1540)
                    }))
                }
            }
        }
    }
}

pub fn f511(v0: Adt135, v1: Adt136) -> Adt149 {
    Adt149::C1(f577(v0, v1))
}

pub fn f512(v0: Adt116) -> Adt149 {
    Adt149::C0(v0)
}

pub fn f513(v0: Adt135, v1: u64, v2: u64, v3: Adt137, v4: List<u64>, v5: bool) -> R<Adt149> {
    let m1541 = v3;
    match m1541 {
        Adt137::C0 => {
            let a1542 = v2;
            let a1543 = 34u64;
            if nat_eq(a1542, a1543) {
                Ok(f578(v0, v1, v4, v5))
            } else {
                let a1544 = v2;
                let a1545 = 92u64;
                if nat_eq(a1544, a1545) {
                    Ok(f511(v0, Adt136::C6(Adt137::C1, v4, v5)))
                } else {
                    let a1546 = v2;
                    let a1547 = 32u64;
                    if nat_lt(a1546, a1547) {
                        Ok(f512(Adt116::C4(v1)))
                    } else {
                        Ok(f511(v0, Adt136::C6(Adt137::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt137::C1 => {
            let a1548 = v2;
            let a1549 = 117u64;
            if nat_eq(a1548, a1549) {
                Ok(f511(v0, Adt136::C6(Adt137::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1550 = f579(v2);
                let a1551 = 0u64;
                if nat_eq(a1550, a1551) {
                    Ok(f512(Adt116::C5(v1)))
                } else {
                    Ok(f511(v0, Adt136::C6(Adt137::C0, List::cons(f579(v2), v4), v5)))
                }
            }
        }
        Adt137::C2(v6, v7, v8) => {
            let a1552 = 15u64;
            let a1553 = f580(v2);
            if nat_lt(a1552, a1553) {
                Ok(f512(Adt116::C5(v1)))
            } else {
                let a1556 = {
                    let a1554 = v6;
                    let a1555 = 1u64;
                    nat_add(a1554, a1555)?
                };
                let a1557 = 4u64;
                if nat_eq(a1556, a1557) {
                    f581(v0, v1, {
                        let a1560 = {
                            let a1558 = v7;
                            let a1559 = 16u64;
                            nat_mul(a1558, a1559)?
                        };
                        let a1561 = f580(v2);
                        nat_add(a1560, a1561)?
                    }, v8, v4, v5)
                } else {
                    Ok(f511(v0, Adt136::C6(Adt137::C2({
                        let a1562 = v6;
                        let a1563 = 1u64;
                        nat_add(a1562, a1563)?
                    }, {
                        let a1566 = {
                            let a1564 = v7;
                            let a1565 = 16u64;
                            nat_mul(a1564, a1565)?
                        };
                        let a1567 = f580(v2);
                        nat_add(a1566, a1567)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt137::C3(v9) => {
            let a1568 = v2;
            let a1569 = 92u64;
            if nat_eq(a1568, a1569) {
                Ok(f511(v0, Adt136::C6(Adt137::C4(v9), v4, v5)))
            } else {
                Ok(f512(Adt116::C6(v1)))
            }
        }
        Adt137::C4(v10) => {
            let a1570 = v2;
            let a1571 = 117u64;
            if nat_eq(a1570, a1571) {
                Ok(f511(v0, Adt136::C6(Adt137::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f512(Adt116::C6(v1)))
            }
        }
    }
}

pub fn f514(v0: Adt135, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt149> {
    let a1572 = v2;
    let a1573 = f582(v3, v4);
    if nat_eq(a1572, a1573) {
        let a1577 = {
            let a1574 = v4;
            let a1575 = 1u64;
            nat_add(a1574, a1575)?
        };
        let a1578 = {
            let a1576 = f583(v3);
            length_list(a1576)
        };
        if nat_eq(a1577, a1578) {
            Ok(f584(v0, f585(v3)))
        } else {
            Ok(f511(v0, Adt136::C7(v3, {
                let a1579 = v4;
                let a1580 = 1u64;
                nat_add(a1579, a1580)?
            })))
        }
    } else {
        Ok(f512(Adt116::C2(v1)))
    }
}

pub fn f515(v0: Adt135, v1: u64, v2: u64, v3: Adt138) -> R<Adt149> {
    let m1582 = match v3.clone() {
        Adt138::C0(_, h1581, _, _, _, _, _) => {
            h1581
        }
    };
    match m1582 {
        Adt139::C0 => {
            let a1583 = v2;
            let a1584 = 48u64;
            if nat_eq(a1583, a1584) {
                Ok(f586(v0, v3, Adt139::C1))
            } else {
                if f503(v2, 49u64, 57u64) {
                    f587(v0, v1, v2, v3, false)
                } else {
                    Ok(f512(Adt116::C7(v1)))
                }
            }
        }
        Adt139::C1 => {
            let a1585 = v2;
            let a1586 = 46u64;
            if nat_eq(a1585, a1586) {
                Ok(f586(v0, v3, Adt139::C3))
            } else {
                if f588(v2) {
                    Ok(f586(v0, v3, Adt139::C5))
                } else {
                    if f589(v2) {
                        Ok(f512(Adt116::C7(v1)))
                    } else {
                        f590(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt139::C2 => {
            if f589(v2) {
                f587(v0, v1, v2, v3, false)
            } else {
                let a1587 = v2;
                let a1588 = 46u64;
                if nat_eq(a1587, a1588) {
                    Ok(f586(v0, v3, Adt139::C3))
                } else {
                    if f588(v2) {
                        Ok(f586(v0, v3, Adt139::C5))
                    } else {
                        f590(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt139::C3 => {
            if f589(v2) {
                f587(v0, v1, v2, v3, true)
            } else {
                Ok(f512(Adt116::C7(v1)))
            }
        }
        Adt139::C4 => {
            if f589(v2) {
                f587(v0, v1, v2, v3, true)
            } else {
                if f588(v2) {
                    Ok(f586(v0, v3, Adt139::C5))
                } else {
                    f590(v0, v1, v2, v3)
                }
            }
        }
        Adt139::C5 => {
            let a1589 = v2;
            let a1590 = 43u64;
            if nat_eq(a1589, a1590) {
                Ok(f511(v0, f572(match v3.clone() {
                    Adt138::C0(h1591, _, _, _, _, _, _) => {
                        h1591
                    }
                }, Adt139::C6, match v3.clone() {
                    Adt138::C0(_, _, h1592, _, _, _, _) => {
                        h1592
                    }
                }, match v3.clone() {
                    Adt138::C0(_, _, _, h1593, _, _, _) => {
                        h1593
                    }
                }, match v3 {
                    Adt138::C0(_, _, _, _, h1594, _, _) => {
                        h1594
                    }
                }, false, 0i64)))
            } else {
                let a1595 = v2;
                let a1596 = 45u64;
                if nat_eq(a1595, a1596) {
                    Ok(f511(v0, f572(match v3.clone() {
                        Adt138::C0(h1597, _, _, _, _, _, _) => {
                            h1597
                        }
                    }, Adt139::C6, match v3.clone() {
                        Adt138::C0(_, _, h1598, _, _, _, _) => {
                            h1598
                        }
                    }, match v3.clone() {
                        Adt138::C0(_, _, _, h1599, _, _, _) => {
                            h1599
                        }
                    }, match v3 {
                        Adt138::C0(_, _, _, _, h1600, _, _) => {
                            h1600
                        }
                    }, true, 0i64)))
                } else {
                    if f589(v2) {
                        f591(v0, v1, v2, Adt138::C0(match v3.clone() {
                            Adt138::C0(h1601, _, _, _, _, _, _) => {
                                h1601
                            }
                        }, match v3.clone() {
                            Adt138::C0(_, h1602, _, _, _, _, _) => {
                                h1602
                            }
                        }, match v3.clone() {
                            Adt138::C0(_, _, h1603, _, _, _, _) => {
                                h1603
                            }
                        }, match v3.clone() {
                            Adt138::C0(_, _, _, h1604, _, _, _) => {
                                h1604
                            }
                        }, match v3 {
                            Adt138::C0(_, _, _, _, h1605, _, _) => {
                                h1605
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f512(Adt116::C7(v1)))
                    }
                }
            }
        }
        Adt139::C6 => {
            if f589(v2) {
                f591(v0, v1, v2, v3.clone(), match v3 {
                    Adt138::C0(_, _, _, _, _, h1606, _) => {
                        h1606
                    }
                })
            } else {
                Ok(f512(Adt116::C7(v1)))
            }
        }
        Adt139::C7 => {
            if f589(v2) {
                f591(v0, v1, v2, v3.clone(), match v3 {
                    Adt138::C0(_, _, _, _, _, h1607, _) => {
                        h1607
                    }
                })
            } else {
                f590(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f516(v0: u64) -> bool {
    let a1608 = v0;
    let a1609 = 9u64;
    if nat_eq(a1608, a1609) {
        true
    } else {
        let a1610 = v0;
        let a1611 = 32u64;
        nat_eq(a1610, a1611)
    }
}

pub fn f517(v0: u64) -> bool {
    let a1612 = 47u64;
    let a1613 = v0;
    if nat_lt(a1612, a1613) {
        let a1614 = v0;
        let a1615 = 58u64;
        nat_lt(a1614, a1615)
    } else {
        false
    }
}

pub fn f518(v0: Result<Adt142, Adt145>) -> Option<Adt94> {
    let m1616 = v0;
    match m1616 {
        Ok(_) => {
            None::<Adt94>
        }
        Err(v2) => {
            let m1617 = v2;
            match m1617 {
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
                    f592(v8)
                }
            }
        }
    }
}

pub fn f519(v0: bool, v1: Adt141, v2: Adt94) -> R<Adt96> {
    if f593(match v2.clone() {
        Adt94::C0(h1618, _) => {
            h1618
        }
    }) {
        Ok(Adt96::C1(v2))
    } else {
        Ok(Adt96::C0(f52(f520(v0, Adt141::C0(match v1 {
            Adt141::C0(h1619, _) => {
                h1619
            }
        }, Err::<Adt142, Adt145>(f594())))?)))
    }
}

pub fn f520(v0: bool, v1: Adt141) -> R<Adt103> {
    f438(Adt101::C0(0i64, Str::lit(""), Str::lit(""), f255()), v0, v1)
}

pub fn f521(v0: Str, v1: Adt9, v2: List<(Str, Adt9)>) -> Option<Adt104> {
    let m1620 = f595(f596(), v2.clone());
    match m1620 {
        None => {
            let m1621 = f597(f598(), v2.clone());
            match m1621 {
                None => {
                    f599(v0, v1, v2)
                }
                Some(v4) => {
                    Some(Adt104::C5(v0, v4))
                }
            }
        }
        Some(v3) => {
            Some(Adt104::C4(v0, v3))
        }
    }
}

pub fn f522(v0: Str, v1: List<(Str, Adt9)>, v2: bool, v3: List<Str>, v4: Adt124) -> R<Adt155> {
    let m1622 = f305(Str::lit("x-mcp-header"), v1.clone());
    match m1622 {
        Adt115::C0 => {
            Ok(Adt155::C1(v4))
        }
        Adt115::C1(v5) => {
            f600(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f523(v0: List<(Str, Adt9)>, v1: bool, v2: List<Str>, v3: List<Adt121>) -> List<Adt121> {
    f601(Str::lit("properties"), v0.clone(), v1, v2, f601(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f602(Str::lit("items"), v0.clone(), f602(Str::lit("additionalProperties"), v0.clone(), f603(Str::lit("allOf"), v0.clone(), f603(Str::lit("anyOf"), v0.clone(), f603(Str::lit("oneOf"), v0.clone(), f602(Str::lit("not"), v0.clone(), f602(Str::lit("if"), v0.clone(), f602(Str::lit("then"), v0.clone(), f602(Str::lit("else"), v0, v3)))))))))))
}

pub fn f524(p1629: List<(Str, Adt150)>) -> Adt125 {
    let m1628 = p1629;
    {
        let mut m1628 = m1628;
        loop {
            let step = {
                let v0 = m1628;
                let m1623 = v0;
                match m1623.uncons() {
                    None => {
                        Err::<List<(Str, Adt150)>, Adt125>(Adt125::C3(List::<Adt113>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1625 = {
                            let (_, h1624) = v1;
                            h1624
                        };
                        match m1625 {
                            Adt150::C0 => {
                                Ok::<List<(Str, Adt150)>, Adt125>(v2)
                            }
                            Adt150::C1 => {
                                Err::<List<(Str, Adt150)>, Adt125>(Adt125::C0({
                                    let (h1626, _) = v1;
                                    h1626
                                }))
                            }
                            Adt150::C2 => {
                                Err::<List<(Str, Adt150)>, Adt125>(Adt125::C1({
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

pub fn f525(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt150> {
    let m1630 = f305(Str::lit("type"), v0);
    match m1630 {
        Adt115::C0 => {
            Ok(Adt150::C0)
        }
        Adt115::C1(v2) => {
            let m1631 = f283(v2.clone());
            match m1631 {
                None => {
                    let m1632 = f286(v2);
                    match m1632 {
                        None => {
                            Ok(Adt150::C2)
                        }
                        Some(v4) => {
                            f606(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f604(v3.clone()) {
                        if f605(v3, v1)? {
                            Ok(Adt150::C0)
                        } else {
                            Ok(Adt150::C1)
                        }
                    } else {
                        Ok(Adt150::C2)
                    }
                }
            }
        }
    }
}

pub fn f526(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt150> {
    let m1633 = f305(Str::lit("enum"), v0);
    match m1633 {
        Adt115::C0 => {
            Ok(Adt150::C0)
        }
        Adt115::C1(v2) => {
            let m1634 = f286(v2);
            match m1634 {
                None => {
                    Ok(Adt150::C2)
                }
                Some(v3) => {
                    f607(v3, v1)
                }
            }
        }
    }
}

pub fn f527(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt150> {
    let m1635 = f305(Str::lit("const"), v0);
    match m1635 {
        Adt115::C0 => {
            Ok(Adt150::C0)
        }
        Adt115::C1(v2) => {
            let m1636 = f608(v2, v1)?;
            match m1636 {
                Adt160::C0 => {
                    Ok(Adt150::C0)
                }
                Adt160::C1 => {
                    Ok(Adt150::C1)
                }
                Adt160::C2 => {
                    Ok(Adt150::C2)
                }
            }
        }
    }
}

pub fn f528(v0: Str, v1: u64, v2: List<(Str, Adt9)>, v3: Adt9) -> R<Adt150> {
    let m1637 = f305(v0, v2);
    match m1637 {
        Adt115::C0 => {
            Ok(Adt150::C0)
        }
        Adt115::C1(v4) => {
            let m1638 = f609(v4);
            match m1638 {
                None => {
                    Ok(Adt150::C2)
                }
                Some(v5) => {
                    let m1639 = f609(v3);
                    match m1639 {
                        None => {
                            Ok(Adt150::C0)
                        }
                        Some(v6) => {
                            if f610(v1, f611(v6, v5)?) {
                                Ok(Adt150::C0)
                            } else {
                                Ok(Adt150::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f529(v0: List<(Str, Adt9)>, v1: Adt9) -> R<List<(Str, Adt150)>> {
    let m1640 = v1;
    match m1640 {
        Adt9::C0 => {
            Ok(List::<(Str, Adt150)>::nil())
        }
        Adt9::C1(_) => {
            Ok(List::<(Str, Adt150)>::nil())
        }
        Adt9::C2(_) => {
            Ok(List::<(Str, Adt150)>::nil())
        }
        Adt9::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f612(Str::lit("minLength"), true, v0.clone(), f613(v4.clone())?)?), List::cons((Str::lit("maxLength"), f612(Str::lit("maxLength"), false, v0, f613(v4)?)?), List::<(Str, Adt150)>::nil())))
        }
        Adt9::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f612(Str::lit("minItems"), true, v0.clone(), f614(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f612(Str::lit("maxItems"), false, v0, f614(v5, 0u64)?)?), List::<(Str, Adt150)>::nil())))
        }
        Adt9::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f612(Str::lit("minProperties"), true, v0.clone(), f615(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f612(Str::lit("maxProperties"), false, v0, f615(v6, 0u64)?)?), List::<(Str, Adt150)>::nil())))
        }
    }
}

pub fn f530(v0: List<(Str, Adt9)>, v1: Adt9) -> Adt150 {
    let m1641 = f305(Str::lit("required"), v0);
    match m1641 {
        Adt115::C0 => {
            Adt150::C0
        }
        Adt115::C1(v2) => {
            let m1642 = f286(v2);
            match m1642 {
                None => {
                    Adt150::C2
                }
                Some(v3) => {
                    let m1643 = f234(v1);
                    match m1643 {
                        None => {
                            Adt150::C0
                        }
                        Some(v4) => {
                            f616(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f531(v0: Adt125, v1: Adt125) -> Adt125 {
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

pub fn f532(v0: Adt9, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt111>) -> Adt125 {
    let m1648 = f305(Str::lit("$ref"), v1);
    match m1648 {
        Adt115::C0 => {
            Adt125::C3(List::<Adt113>::nil())
        }
        Adt115::C1(v4) => {
            let m1649 = f283(v4);
            match m1649 {
                None => {
                    Adt125::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1650 = f617(v0, v5.clone());
                    match m1650 {
                        None => {
                            Adt125::C2(v5)
                        }
                        Some(v6) => {
                            Adt125::C3(List::cons(Adt113::C0(v6, v2, v3), List::<Adt113>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f533(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt111>) -> Adt125 {
    let m1651 = f305(Str::lit("properties"), v0);
    match m1651 {
        Adt115::C0 => {
            Adt125::C3(List::<Adt113>::nil())
        }
        Adt115::C1(v3) => {
            let m1652 = f234(v3);
            match m1652 {
                None => {
                    Adt125::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1653 = f234(v1);
                    match m1653 {
                        None => {
                            Adt125::C3(List::<Adt113>::nil())
                        }
                        Some(v5) => {
                            Adt125::C3(f618(match v4 {
                                Adt8::C0(h1654) => {
                                    h1654
                                }
                            }, match v5 {
                                Adt8::C0(h1655) => {
                                    h1655
                                }
                            }, v2, List::<Adt113>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f534(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt111>) -> Adt125 {
    let m1656 = f305(Str::lit("additionalProperties"), v0.clone());
    match m1656 {
        Adt115::C0 => {
            Adt125::C3(List::<Adt113>::nil())
        }
        Adt115::C1(v3) => {
            let m1657 = f234(v1);
            match m1657 {
                None => {
                    Adt125::C3(List::<Adt113>::nil())
                }
                Some(v4) => {
                    let m1658 = f356(v3.clone());
                    match m1658 {
                        None => {
                            Adt125::C3(f621(match v4 {
                                Adt8::C0(h1660) => {
                                    h1660
                                }
                            }, f620(v0), v3, v2, List::<Adt113>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt125::C3(List::<Adt113>::nil())
                            } else {
                                if f619(match v4 {
                                    Adt8::C0(h1659) => {
                                        h1659
                                    }
                                }, f620(v0)) {
                                    Adt125::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt125::C3(List::<Adt113>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f535(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt111>) -> R<Adt125> {
    let m1661 = f305(Str::lit("items"), v0);
    match m1661 {
        Adt115::C0 => {
            Ok(Adt125::C3(List::<Adt113>::nil()))
        }
        Adt115::C1(v3) => {
            let m1662 = f286(v1);
            match m1662 {
                None => {
                    Ok(Adt125::C3(List::<Adt113>::nil()))
                }
                Some(v4) => {
                    Ok(Adt125::C3(f622(v4, 0u64, v3, v2, List::<Adt113>::nil())?))
                }
            }
        }
    }
}

pub fn f536(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt111>) -> Adt125 {
    let m1663 = f305(Str::lit("allOf"), v0);
    match m1663 {
        Adt115::C0 => {
            Adt125::C3(List::<Adt113>::nil())
        }
        Adt115::C1(v3) => {
            let m1664 = f286(v3);
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
                            Adt125::C3(f623(v4, v1, v2, List::<Adt113>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f537(v0: Adt114, v1: Adt125, v2: List<(Str, Adt9)>, v3: Adt9, v4: List<Adt111>) -> Adt125 {
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
                    Adt125::C3(List::cons(Adt113::C5(v0, Adt9::C5(v2), v3, v4), List::<Adt113>::nil()))
                }
            }
        }
    }
}

pub fn f538(p1674: List<u64>, p1675: List<u64>) -> List<u64> {
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

pub fn f539(p1684: u64, p1685: List<u64>, p1686: List<u64>) -> R<Option<List<u64>>> {
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
                                                    let m1682 = f624(v6, v8, v10, v12, f625(v13.clone()))?;
                                                    match m1682 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f432(v14, v2))))
                                                        }
                                                    }
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

pub fn f540(p1691: List<Adt54>, p1692: Adt10) -> R<bool> {
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
                        if f626(f611(match v2.clone() {
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

pub fn f541() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f542(v0: i64, v1: Str, v2: Str, v3: Adt95) -> Result<(Adt101, u64), Adt2> {
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

pub fn f543(v0: u64, v1: Adt9, v2: List<Adt127>, v3: List<u64>) -> R<Adt151> {
    let m1695 = v1;
    match m1695 {
        Adt9::C0 => {
            Ok(Adt151::C1(v2, f432(f627(2u64), v3)))
        }
        Adt9::C1(v4) => {
            Ok(Adt151::C1(v2, f432(if v4 {
                f627(0u64)
            } else {
                f627(1u64)
            }, v3)))
        }
        Adt9::C2(v5) => {
            if f628(v5.clone())? {
                Ok(Adt151::C1(v2, f432(f629(v5)?, v3)))
            } else {
                Ok(Adt151::C0(Adt116::C11))
            }
        }
        Adt9::C3(v6) => {
            Ok(Adt151::C1(v2, f432(f544(v6)?, v3)))
        }
        Adt9::C4(v7) => {
            let a1696 = v0;
            let a1697 = f630();
            if nat_lt(a1696, a1697) {
                Ok(Adt151::C1(f631(91u64, Adt127::C2({
                    let a1698 = v0;
                    let a1699 = 1u64;
                    nat_add(a1698, a1699)?
                }, v7), v2), v3))
            } else {
                Ok(Adt151::C0(Adt116::C12))
            }
        }
        Adt9::C5(v8) => {
            let a1700 = v0;
            let a1701 = f630();
            if nat_lt(a1700, a1701) {
                Ok(Adt151::C1(f631(123u64, Adt127::C4({
                    let a1702 = v0;
                    let a1703 = 1u64;
                    nat_add(a1702, a1703)?
                }, v8), v2), v3))
            } else {
                Ok(Adt151::C0(Adt116::C12))
            }
        }
    }
}

pub fn f544(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1705 = f432(f632(f25({
            let a1704 = v0;
            utf8_encode(a1704)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1706 = List::cons(34u64, List::<u64>::nil());
        append_list(a1705, a1706)
    }))
}

pub fn f545(v0: Adt36) -> R<Option<List<(Str, Adt9)>>> {
    let m1707 = v0;
    match m1707 {
        Adt36::C0(v1) => {
            Ok(Some(f633(v1)?))
        }
        Adt36::C1(v2) => {
            Ok(Some(f634(v2)?))
        }
        Adt36::C2(v3) => {
            f635(f636(v3)?)
        }
        Adt36::C3(v4) => {
            f635(f636(match v4 {
                Adt53::C0(h1708, _) => {
                    h1708
                }
            })?)
        }
        Adt36::C4(v5) => {
            Ok(Some(f637(v5)?))
        }
        Adt36::C5(v6) => {
            Ok(Some(f638(v6)?))
        }
        Adt36::C6(v7) => {
            Ok(Some(f639(v7)?))
        }
        Adt36::C7(v8) => {
            Ok(Some(f640(v8)?))
        }
        Adt36::C8(v9) => {
            f635(f641(v9)?)
        }
        Adt36::C9(v10) => {
            Ok(Some(f642(v10)?))
        }
        Adt36::C10(_) => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Adt36::C11 => {
            Ok(Some(List::<(Str, Adt9)>::nil()))
        }
        Adt36::C12(v12) => {
            f635(f643(v12)?)
        }
        Adt36::C13(v13) => {
            f635(f644({
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
            Ok(f645(v14))
        }
    }
}

pub fn f546(v0: Adt142) -> Adt8 {
    Adt8::C0(List::cons((Str::lit("resultType"), Adt9::C3(f646(f647(match v0.clone() {
        Adt142::C0(h1712, _, _) => {
            h1712
        }
    })))), List::cons((Str::lit("_meta"), Adt9::C5(f648({
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

pub fn f547(v0: Option<Option<List<u64>>>, v1: Adt103) -> Adt103 {
    let m1716 = v0;
    match m1716 {
        None => {
            f53()
        }
        Some(v2) => {
            Adt103::C0(match v1.clone() {
                Adt103::C0(h1717, _, _, _, _) => {
                    h1717
                }
            }, match v1.clone() {
                Adt103::C0(_, h1718, _, _, _) => {
                    h1718
                }
            }, match v1.clone() {
                Adt103::C0(_, _, h1719, _, _) => {
                    h1719
                }
            }, match v1 {
                Adt103::C0(_, _, _, h1720, _) => {
                    h1720
                }
            }, v2)
        }
    }
}

pub fn f548(v0: Adt101, v1: Adt91) -> R<Option<Option<List<u64>>>> {
    let m1722 = f649(match v0.clone() {
        Adt101::C0(_, _, _, h1721) => {
            h1721
        }
    })?;
    match m1722 {
        None => {
            Ok(Some(None::<List<u64>>))
        }
        Some(v2) => {
            let m1725 = f650(f651(match v1 {
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

pub fn f549(v0: bool, v1: Adt103, v2: List<List<u64>>) -> Adt103 {
    if f652(v0, match v1.clone() {
        Adt103::C0(h1726, _, _, _, _) => {
            h1726
        }
    }, v2.clone()) {
        f653(match v1.clone() {
            Adt103::C0(h1727, _, _, _, _) => {
                h1727
            }
        }, {
            let a1729 = v2;
            let a1730 = List::cons(match v1 {
                Adt103::C0(_, _, _, h1728, _) => {
                    h1728
                }
            }, List::<List<u64>>::nil());
            append_list(a1729, a1730)
        })
    } else {
        v1
    }
}

pub fn f550(v0: Adt144) -> R<Option<List<u64>>> {
    let m1731 = f654(f655(v0))?;
    match m1731 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1732 = f320(v1)?;
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

pub fn f551(v0: Adt91) -> R<Option<List<List<u64>>>> {
    if f656(v0.clone()) {
        let m1736 = f657({
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
                let m1737 = f654(v1)?;
                match m1737 {
                    None => {
                        Ok(None::<List<List<u64>>>)
                    }
                    Some(v2) => {
                        let m1738 = f320(v2)?;
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

pub fn f552() -> u64 {
    404u64
}

pub fn f553(v0: Adt93) -> u64 {
    let m1739 = v0;
    match m1739 {
        Adt93::C0(_) => {
            f326()
        }
        Adt93::C1(_) => {
            f326()
        }
        Adt93::C2(_) => {
            f326()
        }
        Adt93::C3(_) => {
            f326()
        }
        Adt93::C4(_) => {
            f326()
        }
        Adt93::C5(_) => {
            f326()
        }
        Adt93::C6(_) => {
            f95()
        }
        Adt93::C7(_) => {
            f658()
        }
        Adt93::C8(_) => {
            f326()
        }
        Adt93::C9(_) => {
            f95()
        }
        Adt93::C10 => {
            f326()
        }
        Adt93::C11 => {
            f326()
        }
        Adt93::C12(_) => {
            f95()
        }
        Adt93::C13 => {
            f95()
        }
        Adt93::C14 => {
            f95()
        }
        Adt93::C15(_) => {
            f95()
        }
    }
}

pub fn f554(v0: Adt145) -> i64 {
    let m1740 = v0;
    match m1740 {
        Adt145::C0(_) => {
            f659()
        }
        Adt145::C1(_, _) => {
            f660()
        }
        Adt145::C2(_) => {
            f661()
        }
        Adt145::C3(_) => {
            f662()
        }
        Adt145::C4(v6) => {
            f663(v6)
        }
    }
}

pub fn f555(v0: Adt145) -> Str {
    let m1741 = v0;
    match m1741 {
        Adt145::C0(v1) => {
            f664(v1)
        }
        Adt145::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt145::C2(v4) => {
            f665(v4)
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

pub fn f556(v0: Adt145) -> R<Option<Adt9>> {
    let m1743 = v0;
    match m1743 {
        Adt145::C0(_) => {
            Ok(None::<Adt9>)
        }
        Adt145::C1(v2, v3) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("supported"), Adt9::C4(f666(v3))), List::cons((Str::lit("requested"), Adt9::C3(v2)), List::<(Str, Adt9)>::nil())))))
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
                    Ok(Some(Adt9::C5(List::cons((Str::lit("limit"), Adt9::C2(Adt10::C0(f285(v14, 0i64)?, 0i64))), List::<(Str, Adt9)>::nil()))))
                }
                Adt93::C8(v15) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("requiredCapabilities"), f667(v15)), List::<(Str, Adt9)>::nil()))))
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

pub fn f557(v0: List<u64>) -> R<bool> {
    let m1745 = f486(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1745.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f668(v2)? {
                Ok(false)
            } else {
                f386(f388(v1), f669())
            }
        }
    }
}

pub fn f558(v0: Adt3, v1: Fn3, v2: u64, v3: Adt152) -> R<Result<Adt142, Adt145>> {
    f670(v0.clone(), v1, v2, v3.clone(), f671(match v0 {
        Adt3::C0(_, _, h1746, _, _, _, _) => {
            h1746
        }
    }, match v3 {
        Adt152::C0(_, h1747, _, _) => {
            h1747
        }
    }))
}

pub fn f559(v0: Adt156, v1: Adt157) -> Result<Adt152, Adt145> {
    let m1748 = v1;
    match m1748 {
        Adt157::C0(v3) => {
            f672(v3, v0)
        }
        Adt157::C1(v2) => {
            Err::<Adt152, Adt145>(v2)
        }
    }
}

pub fn f560(v0: Str) -> Adt156 {
    let a1749 = v0.clone();
    let a1750 = Str::lit("server/discover");
    if equal(a1749, a1750) {
        Adt156::C1(Adt153::C0)
    } else {
        let a1751 = v0.clone();
        let a1752 = Str::lit("tools/list");
        if equal(a1751, a1752) {
            Adt156::C1(Adt153::C1)
        } else {
            let a1753 = v0.clone();
            let a1754 = Str::lit("tools/call");
            if equal(a1753, a1754) {
                Adt156::C1(Adt153::C2)
            } else {
                let a1755 = v0.clone();
                let a1756 = Str::lit("resources/list");
                if equal(a1755, a1756) {
                    Adt156::C1(Adt153::C3)
                } else {
                    let a1757 = v0.clone();
                    let a1758 = Str::lit("resources/read");
                    if equal(a1757, a1758) {
                        Adt156::C1(Adt153::C4)
                    } else {
                        let a1759 = v0.clone();
                        let a1760 = Str::lit("resources/templates/list");
                        if equal(a1759, a1760) {
                            Adt156::C1(Adt153::C5)
                        } else {
                            let a1761 = v0.clone();
                            let a1762 = Str::lit("prompts/list");
                            if equal(a1761, a1762) {
                                Adt156::C1(Adt153::C6)
                            } else {
                                let a1763 = v0.clone();
                                let a1764 = Str::lit("prompts/get");
                                if equal(a1763, a1764) {
                                    Adt156::C1(Adt153::C7)
                                } else {
                                    let a1765 = v0.clone();
                                    let a1766 = Str::lit("completion/complete");
                                    if equal(a1765, a1766) {
                                        Adt156::C1(Adt153::C8)
                                    } else {
                                        let a1767 = v0.clone();
                                        let a1768 = Str::lit("tasks/get");
                                        if equal(a1767, a1768) {
                                            Adt156::C1(Adt153::C9)
                                        } else {
                                            let a1769 = v0.clone();
                                            let a1770 = Str::lit("tasks/update");
                                            if equal(a1769, a1770) {
                                                Adt156::C1(Adt153::C10)
                                            } else {
                                                let a1771 = v0;
                                                let a1772 = Str::lit("tasks/cancel");
                                                if equal(a1771, a1772) {
                                                    Adt156::C1(Adt153::C11)
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

pub fn f561(v0: Option<Adt128>, v1: List<Str>, v2: Adt123) -> R<Adt157> {
    f673(v0, v1, v2.clone(), f674(match v2 {
        Adt123::C0(_, _, h1773) => {
            h1773
        }
    }))
}

pub fn f562(v0: Str) -> Adt159 {
    let m1774 = f560(v0);
    match m1774 {
        Adt156::C0 => {
            Adt159::C0
        }
        Adt156::C1(v1) => {
            f675(v1)
        }
    }
}

pub fn f563(v0: List<u64>) -> Option<List<u64>> {
    let m1775 = f676(f677(), v0);
    match m1775 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1776 = f676(f678(), f396(v1, List::<u64>::nil()));
            match m1776 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f396(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f564(p1779: List<Adt131>, p1780: List<Adt131>) -> List<Adt131> {
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

pub fn f565(v0: List<Adt100>, p1787: List<Adt15>, v2: Option<Str>, p1788: List<Adt131>) -> R<List<Adt131>> {
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
                        if f679(v4.clone(), v2.clone()) {
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
                            }, f680(v0.clone(), match v4 {
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

pub fn f566(p1804: u64, p1805: i64, p1806: i64) -> R<Option<i64>> {
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
                    if f681(v2) {
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
                            if f681({
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
                                }, f355(v2, 1i64)?)))
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

pub fn f567(v0: i64) -> Option<i64> {
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

pub fn f568(v0: Adt115) -> bool {
    let m1813 = v0;
    match m1813 {
        Adt115::C0 => {
            false
        }
        Adt115::C1(_) => {
            true
        }
    }
}

pub fn f569(v0: Adt109, v1: Str, v2: Adt8) -> Result<Adt123, Adt117> {
    let m1814 = f233(Str::lit("params"), v2);
    match m1814 {
        Adt115::C0 => {
            Ok::<Adt123, Adt117>(Adt123::C0(v0, v1, None::<Adt8>))
        }
        Adt115::C1(v3) => {
            let m1815 = f234(v3);
            match m1815 {
                None => {
                    f382(Some(v0), Adt118::C6)
                }
                Some(v4) => {
                    Ok::<Adt123, Adt117>(Adt123::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f570(v0: Adt139) -> bool {
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

pub fn f571(v0: Adt135, v1: Adt140, v2: Adt136, v3: u64) -> R<Adt149> {
    let a1818 = match v0.clone() {
        Adt135::C0(_, _, h1817) => {
            h1817
        }
    };
    let a1819 = f630();
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
        Ok(f512(Adt116::C9(v3)))
    }
}

pub fn f572(v0: bool, v1: Adt139, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt136 {
    Adt136::C8(Adt138::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f573(v0: u64) -> i64 {
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

pub fn f574(p1844: List<Adt9>, p1845: List<Adt9>) -> List<Adt9> {
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

pub fn f575(v0: Adt135, v1: u64) -> Adt149 {
    let m1847 = match v0.clone() {
        Adt135::C0(_, h1846, _) => {
            h1846
        }
    };
    match m1847.uncons() {
        None => {
            f512(Adt116::C2(v1))
        }
        Some((v2, _)) => {
            let m1848 = v2;
            match m1848 {
                Adt140::C0(_) => {
                    f511(v0, Adt136::C0)
                }
                Adt140::C1(_, _) => {
                    f511(v0, Adt136::C4)
                }
            }
        }
    }
}

pub fn f576(p1851: List<(Str, Adt9)>, p1852: List<(Str, Adt9)>) -> List<(Str, Adt9)> {
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

pub fn f577(v0: Adt135, v1: Adt136) -> Adt135 {
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

pub fn f578(v0: Adt135, v1: u64, v2: List<u64>, v3: bool) -> Adt149 {
    let m1856 = {
        let a1855 = f358(f255(), f432(v2, List::<u64>::nil()));
        utf8_decode(a1855)
    };
    match m1856 {
        None => {
            f512(Adt116::C14)
        }
        Some(v4) => {
            if v3 {
                f682(v0, v1, v4)
            } else {
                f584(v0, Adt9::C3(v4))
            }
        }
    }
}

pub fn f579(v0: u64) -> u64 {
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

pub fn f580(v0: u64) -> u64 {
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

pub fn f581(v0: Adt135, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt149> {
    let a1891 = v3;
    let a1892 = 0u64;
    if nat_eq(a1891, a1892) {
        if f503(v2, 55296u64, 56319u64) {
            Ok(f511(v0, Adt136::C6(Adt137::C3(v2), v4, v5)))
        } else {
            if f503(v2, 56320u64, 57343u64) {
                Ok(f512(Adt116::C6(v1)))
            } else {
                Ok(f511(v0, Adt136::C6(Adt137::C0, f683(v2, v4)?, v5)))
            }
        }
    } else {
        if f503(v2, 56320u64, 57343u64) {
            Ok(f511(v0, Adt136::C6(Adt137::C0, f683({
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
            Ok(f512(Adt116::C6(v1)))
        }
    }
}

pub fn f582(v0: u64, v1: u64) -> u64 {
    let m1905 = {
        let a1903 = f583(v0);
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

pub fn f583(v0: u64) -> List<u64> {
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

pub fn f584(v0: Adt135, v1: Adt9) -> Adt149 {
    Adt149::C1(f505(v1, match v0.clone() {
        Adt135::C0(_, h1910, _) => {
            h1910
        }
    }, match v0 {
        Adt135::C0(_, _, h1911) => {
            h1911
        }
    }))
}

pub fn f585(v0: u64) -> Adt9 {
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

pub fn f586(v0: Adt135, v1: Adt138, v2: Adt139) -> Adt149 {
    f511(v0, Adt136::C8(Adt138::C0(match v1.clone() {
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

pub fn f587(v0: Adt135, v1: u64, v2: u64, v3: Adt138, v4: bool) -> R<Adt149> {
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
        f355(match v3.clone() {
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
    let a1932 = f684();
    let a1933 = v5;
    if nat_lt(a1932, a1933) {
        Ok(f512(Adt116::C8(v1)))
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
            Ok(f512(Adt116::C8(v1)))
        } else {
            Ok(f511(v0, f572(match v3.clone() {
                Adt138::C0(h1937, _, _, _, _, _, _) => {
                    h1937
                }
            }, if v4 {
                Adt139::C4
            } else {
                Adt139::C2
            }, f355({
                let a1939 = match v3.clone() {
                    Adt138::C0(_, _, h1938, _, _, _, _) => {
                        h1938
                    }
                };
                let a1940 = 10i64;
                int_mul(a1939, a1940)?
            }, f573({
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

pub fn f588(v0: u64) -> bool {
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

pub fn f589(v0: u64) -> bool {
    f503(v0, 48u64, 57u64)
}

pub fn f590(v0: Adt135, v1: u64, v2: u64, v3: Adt138) -> R<Adt149> {
    let m1949 = f504(v3)?;
    match m1949 {
        Adt154::C0 => {
            Ok(f512(Adt116::C7(v1)))
        }
        Adt154::C1 => {
            Ok(f512(Adt116::C8(v1)))
        }
        Adt154::C2(v4) => {
            Ok(f509(f505(v4, match v0.clone() {
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

pub fn f591(v0: Adt135, v1: u64, v2: u64, v3: Adt138, v4: bool) -> R<Adt149> {
    let v5: i64 = f355({
        let a1953 = match v3.clone() {
            Adt138::C0(_, _, _, _, _, _, h1952) => {
                h1952
            }
        };
        let a1954 = 10i64;
        int_mul(a1953, a1954)?
    }, f573({
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
        Ok(f512(Adt116::C8(v1)))
    } else {
        Ok(f511(v0, f572(match v3.clone() {
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

pub fn f592(v0: Adt93) -> Option<Adt94> {
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

pub fn f593(v0: u64) -> bool {
    let a1965 = v0;
    let a1966 = f685();
    nat_lt(a1965, a1966)
}

pub fn f594() -> Adt145 {
    Adt145::C4(Adt93::C9(Str::lit("the effect tag does not fit thirty-two bits")))
}

pub fn f595(p1970: List<Str>, v1: List<(Str, Adt9)>) -> Option<Str> {
    let m1969 = p1970;
    {
        let mut m1969 = m1969;
        loop {
            let step = {
                let v0 = m1969;
                let m1967 = v0;
                match m1967.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1968 = f305(v2.clone(), v1.clone());
                        match m1968 {
                            Adt115::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt115::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
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

pub fn f596() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f597(p1977: List<(Str, u64)>, v1: List<(Str, Adt9)>) -> Option<Str> {
    let m1976 = p1977;
    {
        let mut m1976 = m1976;
        loop {
            let step = {
                let v0 = m1976;
                let m1971 = v0;
                match m1971.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1973 = f305({
                            let (h1972, _) = v2.clone();
                            h1972
                        }, v1.clone());
                        match m1973 {
                            Adt115::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt115::C1(v4) => {
                                if f686({
                                    let (_, h1974) = v2;
                                    h1974
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1975, _) = v2;
                                        h1975
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1976 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f598() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f599(v0: Str, v1: Adt9, v2: List<(Str, Adt9)>) -> Option<Adt104> {
    let m1978 = f305(Str::lit("$ref"), v2);
    match m1978 {
        Adt115::C0 => {
            None::<Adt104>
        }
        Adt115::C1(v3) => {
            let m1979 = f283(v3);
            match m1979 {
                None => {
                    None::<Adt104>
                }
                Some(v4) => {
                    let m1980 = f617(v1, v4.clone());
                    match m1980 {
                        None => {
                            Some(Adt104::C6(v0, v4))
                        }
                        Some(_) => {
                            None::<Adt104>
                        }
                    }
                }
            }
        }
    }
}

pub fn f600(v0: Str, v1: List<(Str, Adt9)>, v2: Adt9, v3: bool, v4: List<Str>, v5: Adt124) -> R<Adt155> {
    if v3 {
        let m1981 = f283(v2);
        match m1981 {
            None => {
                Ok(Adt155::C0(Adt104::C7(v0)))
            }
            Some(v6) => {
                f687(v0, v1, v6.clone(), f277(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt155::C0(Adt104::C10(v0)))
    }
}

pub fn f601(v0: Str, v1: List<(Str, Adt9)>, v2: bool, v3: List<Str>, v4: List<Adt121>) -> List<Adt121> {
    let m1982 = f305(v0.clone(), v1);
    match m1982 {
        Adt115::C0 => {
            v4
        }
        Adt115::C1(v5) => {
            let m1983 = f234(v5);
            match m1983 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1984 = v0;
                    let a1985 = Str::lit("properties");
                    if equal(a1984, a1985) {
                        f688(match v6 {
                            Adt8::C0(h1986) => {
                                h1986
                            }
                        }, v2, v3, v4)
                    } else {
                        f689(match v6 {
                            Adt8::C0(h1987) => {
                                h1987
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f602(v0: Str, v1: List<(Str, Adt9)>, v2: List<Adt121>) -> List<Adt121> {
    let m1988 = f305(v0, v1);
    match m1988 {
        Adt115::C0 => {
            v2
        }
        Adt115::C1(v3) => {
            List::cons(Adt121::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f603(v0: Str, v1: List<(Str, Adt9)>, v2: List<Adt121>) -> List<Adt121> {
    let m1989 = f305(v0, v1);
    match m1989 {
        Adt115::C0 => {
            v2
        }
        Adt115::C1(v3) => {
            let m1990 = f286(v3);
            match m1990 {
                None => {
                    v2
                }
                Some(v4) => {
                    f690(v4, v2)
                }
            }
        }
    }
}

pub fn f604(v0: Str) -> bool {
    let a1991 = v0.clone();
    let a1992 = Str::lit("null");
    if equal(a1991, a1992) {
        true
    } else {
        let a1993 = v0.clone();
        let a1994 = Str::lit("boolean");
        if equal(a1993, a1994) {
            true
        } else {
            let a1995 = v0.clone();
            let a1996 = Str::lit("object");
            if equal(a1995, a1996) {
                true
            } else {
                let a1997 = v0.clone();
                let a1998 = Str::lit("array");
                if equal(a1997, a1998) {
                    true
                } else {
                    let a1999 = v0.clone();
                    let a2000 = Str::lit("number");
                    if equal(a1999, a2000) {
                        true
                    } else {
                        let a2001 = v0.clone();
                        let a2002 = Str::lit("string");
                        if equal(a2001, a2002) {
                            true
                        } else {
                            let a2003 = v0;
                            let a2004 = Str::lit("integer");
                            equal(a2003, a2004)
                        }
                    }
                }
            }
        }
    }
}

pub fn f605(v0: Str, v1: Adt9) -> R<bool> {
    let a2005 = v0.clone();
    let a2006 = f691(v1.clone());
    if equal(a2005, a2006) {
        Ok(true)
    } else {
        let a2007 = v0;
        let a2008 = Str::lit("integer");
        if equal(a2007, a2008) {
            let m2009 = f609(v1);
            match m2009 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f692(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f606(p2012: List<Adt9>, v1: Adt9, p2013: bool) -> R<Adt150> {
    let m2011 = (p2012, p2013);
    {
        let mut m2011 = m2011;
        loop {
            let step = {
                let (v0, v2) = m2011;
                let m2010 = v0;
                match m2010.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt9>, bool), R<Adt150>>(Ok(Adt150::C0))
                        } else {
                            Err::<(List<Adt9>, bool), R<Adt150>>(Ok(Adt150::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f604(f693(v3.clone())) {
                            Ok::<(List<Adt9>, bool), R<Adt150>>((v4, if v2 {
                                true
                            } else {
                                f605(f693(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt9>, bool), R<Adt150>>(Ok(Adt150::C2))
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

pub fn f607(p2017: List<Adt9>, v1: Adt9) -> R<Adt150> {
    let m2016 = p2017;
    {
        let mut m2016 = m2016;
        loop {
            let step = {
                let v0 = m2016;
                let m2014 = v0;
                match m2014.uncons() {
                    None => {
                        Err::<List<Adt9>, R<Adt150>>(Ok(Adt150::C1))
                    }
                    Some((v2, v3)) => {
                        let m2015 = f608(v2, v1.clone())?;
                        match m2015 {
                            Adt160::C0 => {
                                Err::<List<Adt9>, R<Adt150>>(Ok(Adt150::C0))
                            }
                            Adt160::C1 => {
                                Ok::<List<Adt9>, R<Adt150>>(v3)
                            }
                            Adt160::C2 => {
                                Err::<List<Adt9>, R<Adt150>>(Ok(Adt150::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2016 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f608(v0: Adt9, v1: Adt9) -> R<Adt160> {
    f694(f695(), List::cons((v0, v1), List::<(Adt9, Adt9)>::nil()))
}

pub fn f609(v0: Adt9) -> Option<Adt10> {
    let m2018 = v0;
    match m2018 {
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

pub fn f610(v0: u64, v1: Adt161) -> bool {
    let a2019 = v0;
    let a2020 = 0u64;
    if nat_eq(a2019, a2020) {
        let a2021 = f696(v1);
        bool_not(a2021)
    } else {
        let a2022 = v0;
        let a2023 = 1u64;
        if nat_eq(a2022, a2023) {
            let a2024 = f626(v1);
            bool_not(a2024)
        } else {
            let a2025 = v0;
            let a2026 = 2u64;
            if nat_eq(a2025, a2026) {
                f626(v1)
            } else {
                f696(v1)
            }
        }
    }
}

pub fn f611(v0: Adt10, v1: Adt10) -> R<Adt161> {
    f697(v0.clone(), v1.clone(), f698(match v0 {
        Adt10::C0(h2027, _) => {
            h2027
        }
    }), f698(match v1 {
        Adt10::C0(h2028, _) => {
            h2028
        }
    }))
}

pub fn f612(v0: Str, v1: bool, v2: List<(Str, Adt9)>, v3: u64) -> R<Adt150> {
    let m2029 = f305(v0, v2);
    match m2029 {
        Adt115::C0 => {
            Ok(Adt150::C0)
        }
        Adt115::C1(v4) => {
            let m2030 = f699(v4);
            match m2030 {
                None => {
                    Ok(Adt150::C2)
                }
                Some(v5) => {
                    let m2033 = {
                        let a2031 = v5;
                        let a2032 = 0i64;
                        compare(a2031, a2032)
                    };
                    if match m2033 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Ok(Adt150::C2)
                    } else {
                        if v1 {
                            if f696(f700(f285(v3, 0i64)?, v5)) {
                                Ok(Adt150::C1)
                            } else {
                                Ok(Adt150::C0)
                            }
                        } else {
                            if f626(f700(f285(v3, 0i64)?, v5)) {
                                Ok(Adt150::C1)
                            } else {
                                Ok(Adt150::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f613(v0: Str) -> R<u64> {
    f701(f25({
        let a2034 = v0;
        utf8_encode(a2034)
    })?, 0u64)
}

pub fn f614(p2039: List<Adt9>, p2040: u64) -> R<u64> {
    let m2038 = (p2039, p2040);
    {
        let mut m2038 = m2038;
        loop {
            let step = {
                let (v0, v1) = m2038;
                let m2035 = v0;
                match m2035.uncons() {
                    None => {
                        Err::<(List<Adt9>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt9>, u64), R<u64>>((v3, {
                            let a2036 = v1;
                            let a2037 = 1u64;
                            nat_add(a2036, a2037)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2038 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f615(p2045: List<(Str, Adt9)>, p2046: u64) -> R<u64> {
    let m2044 = (p2045, p2046);
    {
        let mut m2044 = m2044;
        loop {
            let step = {
                let (v0, v1) = m2044;
                let m2041 = v0;
                match m2041.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt9)>, u64), R<u64>>((v3, {
                            let a2042 = v1;
                            let a2043 = 1u64;
                            nat_add(a2042, a2043)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2044 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f616(p2050: List<Adt9>, v1: Adt8) -> Adt150 {
    let m2049 = p2050;
    {
        let mut m2049 = m2049;
        loop {
            let step = {
                let v0 = m2049;
                let m2047 = v0;
                match m2047.uncons() {
                    None => {
                        Err::<List<Adt9>, Adt150>(Adt150::C0)
                    }
                    Some((v2, v3)) => {
                        let m2048 = f283(v2);
                        match m2048 {
                            None => {
                                Err::<List<Adt9>, Adt150>(Adt150::C2)
                            }
                            Some(v4) => {
                                if f500(v4, v1.clone()) {
                                    Ok::<List<Adt9>, Adt150>(v3)
                                } else {
                                    Err::<List<Adt9>, Adt150>(Adt150::C1)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2049 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f617(v0: Adt9, v1: Str) -> Option<Adt9> {
    let a2051 = v1.clone();
    let a2052 = Str::lit("#");
    if equal(a2051, a2052) {
        Some(v0)
    } else {
        let m2053 = f234(v0);
        match m2053 {
            None => {
                None::<Adt9>
            }
            Some(v2) => {
                let m2054 = f233(Str::lit("$defs"), v2);
                match m2054 {
                    Adt115::C0 => {
                        None::<Adt9>
                    }
                    Adt115::C1(v3) => {
                        let m2055 = f234(v3);
                        match m2055 {
                            None => {
                                None::<Adt9>
                            }
                            Some(v4) => {
                                f702(v1, match v4 {
                                    Adt8::C0(h2056) => {
                                        h2056
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

pub fn f618(p2063: List<(Str, Adt9)>, v1: List<(Str, Adt9)>, v2: List<Adt111>, p2064: List<Adt113>) -> List<Adt113> {
    let m2062 = (p2063, p2064);
    {
        let mut m2062 = m2062;
        loop {
            let step = {
                let (v0, v3) = m2062;
                let m2057 = v0;
                match m2057.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt113>), List<Adt113>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m2059 = f305({
                            let (h2058, _) = v4.clone();
                            h2058
                        }, v1.clone());
                        match m2059 {
                            Adt115::C0 => {
                                Ok::<(List<(Str, Adt9)>, List<Adt113>), List<Adt113>>((v5, v3))
                            }
                            Adt115::C1(v6) => {
                                Ok::<(List<(Str, Adt9)>, List<Adt113>), List<Adt113>>((v5, List::cons(Adt113::C0({
                                    let (_, h2060) = v4;
                                    h2060
                                }, v6, List::cons(Adt111::C0({
                                    let (h2061, _) = v4;
                                    h2061
                                }), v2.clone())), v3)))
                            }
                        }
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

pub fn f619(p2069: List<(Str, Adt9)>, v1: List<(Str, Adt9)>) -> bool {
    let m2068 = p2069;
    {
        let mut m2068 = m2068;
        loop {
            let step = {
                let v0 = m2068;
                let m2065 = v0;
                match m2065.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m2067 = f305({
                            let (h2066, _) = v2;
                            h2066
                        }, v1.clone());
                        match m2067 {
                            Adt115::C0 => {
                                Err::<List<(Str, Adt9)>, bool>(true)
                            }
                            Adt115::C1(_) => {
                                Ok::<List<(Str, Adt9)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f620(v0: List<(Str, Adt9)>) -> List<(Str, Adt9)> {
    let m2070 = f305(Str::lit("properties"), v0);
    match m2070 {
        Adt115::C0 => {
            List::<(Str, Adt9)>::nil()
        }
        Adt115::C1(v1) => {
            let m2071 = f234(v1);
            match m2071 {
                None => {
                    List::<(Str, Adt9)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt8::C0(h2072) => {
                            h2072
                        }
                    }
                }
            }
        }
    }
}

pub fn f621(p2079: List<(Str, Adt9)>, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt111>, p2080: List<Adt113>) -> List<Adt113> {
    let m2078 = (p2079, p2080);
    {
        let mut m2078 = m2078;
        loop {
            let step = {
                let (v0, v4) = m2078;
                let m2073 = v0;
                match m2073.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt113>), List<Adt113>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m2075 = f305({
                            let (h2074, _) = v5.clone();
                            h2074
                        }, v1.clone());
                        match m2075 {
                            Adt115::C0 => {
                                Ok::<(List<(Str, Adt9)>, List<Adt113>), List<Adt113>>((v6, List::cons(Adt113::C0(v2.clone(), {
                                    let (_, h2076) = v5;
                                    h2076
                                }, List::cons(Adt111::C0({
                                    let (h2077, _) = v5;
                                    h2077
                                }), v3.clone())), v4)))
                            }
                            Adt115::C1(_) => {
                                Ok::<(List<(Str, Adt9)>, List<Adt113>), List<Adt113>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2078 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f622(p2085: List<Adt9>, p2086: u64, v2: Adt9, v3: List<Adt111>, p2087: List<Adt113>) -> R<List<Adt113>> {
    let m2084 = (p2085, (p2086, p2087));
    {
        let mut m2084 = m2084;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2084;
                let m2081 = v0;
                match m2081.uncons() {
                    None => {
                        Err::<(List<Adt9>, (u64, List<Adt113>)), R<List<Adt113>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt9>, (u64, List<Adt113>)), R<List<Adt113>>>((v6, ({
                            let a2082 = v1;
                            let a2083 = 1u64;
                            nat_add(a2082, a2083)?
                        }, List::cons(Adt113::C0(v2.clone(), v5, List::cons(Adt111::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m2084 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f623(p2090: List<Adt9>, v1: Adt9, v2: List<Adt111>, p2091: List<Adt113>) -> List<Adt113> {
    let m2089 = (p2090, p2091);
    {
        let mut m2089 = m2089;
        loop {
            let step = {
                let (v0, v3) = m2089;
                let m2088 = v0;
                match m2088.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt113>), List<Adt113>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt9>, List<Adt113>), List<Adt113>>((v5, List::cons(Adt113::C0(v4, v1.clone(), v2.clone()), v3)))
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

pub fn f624(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m2092 = f703(v0)?;
    match m2092 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m2093 = f703(v1)?;
            match m2093 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a2094 = v2;
                    let a2095 = 61u64;
                    if nat_eq(a2094, a2095) {
                        f704(v5, v6, v4, v3)
                    } else {
                        let m2096 = f703(v2)?;
                        match m2096 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a2097 = v3;
                                let a2098 = 61u64;
                                if nat_eq(a2097, a2098) {
                                    f705(v5, v6, v7, v4)
                                } else {
                                    let m2099 = f703(v3)?;
                                    match m2099 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f706(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f625(v0: List<u64>) -> bool {
    let m2100 = v0;
    match m2100.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f626(v0: Adt161) -> bool {
    let m2101 = v0;
    match m2101 {
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

pub fn f627(v0: u64) -> List<u64> {
    f583(v0)
}

pub fn f628(v0: Adt10) -> R<bool> {
    let a2119 = {
        let m2105 = {
            let a2103 = f707();
            let a2104 = match v0.clone() {
                Adt10::C0(h2102, _) => {
                    h2102
                }
            };
            compare(a2103, a2104)
        };
        if match m2105 {
            Ordering::Less => {
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
            let m2110 = {
                let a2108 = match v0.clone() {
                    Adt10::C0(h2106, _) => {
                        h2106
                    }
                };
                let a2109 = {
                    let a2107 = f707();
                    int_neg(a2107)?
                };
                compare(a2108, a2109)
            };
            if match m2110 {
                Ordering::Less => {
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
                let m2114 = {
                    let a2112 = 9999i64;
                    let a2113 = match v0.clone() {
                        Adt10::C0(_, h2111) => {
                            h2111
                        }
                    };
                    compare(a2112, a2113)
                };
                if match m2114 {
                    Ordering::Less => {
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
                    let m2118 = {
                        let a2116 = match v0 {
                            Adt10::C0(_, h2115) => {
                                h2115
                            }
                        };
                        let a2117 = -9999i64;
                        compare(a2116, a2117)
                    };
                    match m2118 {
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
    Ok(bool_not(a2119))
}

pub fn f629(v0: Adt10) -> R<List<u64>> {
    let m2123 = {
        let a2121 = match v0.clone() {
            Adt10::C0(_, h2120) => {
                h2120
            }
        };
        let a2122 = 0i64;
        compare(a2121, a2122)
    };
    if if match m2123 {
        Ordering::Less => {
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
        let m2127 = {
            let a2125 = 0i64;
            let a2126 = match v0.clone() {
                Adt10::C0(_, h2124) => {
                    h2124
                }
            };
            compare(a2125, a2126)
        };
        match m2127 {
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
        let a2130 = f708(match v0.clone() {
            Adt10::C0(h2128, _) => {
                h2128
            }
        })?;
        let a2131 = List::cons(101u64, f708(match v0 {
            Adt10::C0(_, h2129) => {
                h2129
            }
        })?);
        Ok(append_list(a2130, a2131))
    } else {
        f708(match v0 {
            Adt10::C0(h2132, _) => {
                h2132
            }
        })
    }
}

pub fn f630() -> u64 {
    512u64
}

pub fn f631(v0: u64, v1: Adt127, v2: List<Adt127>) -> List<Adt127> {
    List::cons(Adt127::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f632(p2135: List<u64>, p2136: List<u64>) -> R<List<u64>> {
    let m2134 = (p2135, p2136);
    {
        let mut m2134 = m2134;
        loop {
            let step = {
                let (v0, v1) = m2134;
                let m2133 = v0;
                match m2133.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f432(f709(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2134 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f633(v0: Adt37) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f710(match v0.clone() {
        Adt37::C0(_, h2137, _, _) => {
            h2137
        }
    })), List::cons((Str::lit("capabilities"), f711(match v0.clone() {
        Adt37::C0(_, _, h2138, _) => {
            h2138
        }
    })), {
        let a2141 = f712(Str::lit("instructions"), match v0.clone() {
            Adt37::C0(_, _, _, h2139) => {
                h2139
            }
        });
        let a2142 = f713(match v0 {
            Adt37::C0(h2140, _, _, _) => {
                h2140
            }
        })?;
        append_list(a2141, a2142)
    })))
}

pub fn f634(v0: Adt38) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("tools"), Adt9::C4(f714(match v0.clone() {
        Adt38::C0(_, _, h2143) => {
            h2143
        }
    }))), {
        let a2146 = f712(Str::lit("nextCursor"), match v0.clone() {
            Adt38::C0(_, h2144, _) => {
                h2144
            }
        });
        let a2147 = f713(match v0 {
            Adt38::C0(h2145, _, _) => {
                h2145
            }
        })?;
        append_list(a2146, a2147)
    }))
}

pub fn f635(v0: Option<List<(Str, Adt9)>>) -> R<Option<List<(Str, Adt9)>>> {
    let m2148 = v0;
    match m2148 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            let m2149 = f654(Adt9::C5(v1))?;
            match m2149 {
                None => {
                    Ok(None::<List<(Str, Adt9)>>)
                }
                Some(v2) => {
                    let m2150 = f234(v2);
                    match m2150 {
                        None => {
                            Ok(None::<List<(Str, Adt9)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt8::C0(h2151) => {
                                    h2151
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f636(v0: Adt41) -> R<Option<List<(Str, Adt9)>>> {
    let m2153 = f715(match v0.clone() {
        Adt41::C0(h2152, _, _) => {
            h2152
        }
    })?;
    match m2153 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt9::C4(v1)), {
                let a2156 = f716(Str::lit("structuredContent"), match v0.clone() {
                    Adt41::C0(_, h2154, _) => {
                        h2154
                    }
                });
                let a2157 = f717(Str::lit("isError"), match v0 {
                    Adt41::C0(_, _, h2155) => {
                        h2155
                    }
                });
                append_list(a2156, a2157)
            })))
        }
    }
}

pub fn f637(v0: Adt55) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("resources"), Adt9::C4(f718(match v0.clone() {
        Adt55::C0(_, _, h2158) => {
            h2158
        }
    })?)), {
        let a2161 = f712(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h2159, _) => {
                h2159
            }
        });
        let a2162 = f713(match v0 {
            Adt55::C0(h2160, _, _) => {
                h2160
            }
        })?;
        append_list(a2161, a2162)
    }))
}

pub fn f638(v0: Adt56) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("contents"), Adt9::C4(f719(match v0.clone() {
        Adt56::C0(_, h2163) => {
            h2163
        }
    }))), f713(match v0 {
        Adt56::C0(h2164, _) => {
            h2164
        }
    })?))
}

pub fn f639(v0: Adt57) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt9::C4(f720(match v0.clone() {
        Adt57::C0(_, _, h2165) => {
            h2165
        }
    }))), {
        let a2168 = f712(Str::lit("nextCursor"), match v0.clone() {
            Adt57::C0(_, h2166, _) => {
                h2166
            }
        });
        let a2169 = f713(match v0 {
            Adt57::C0(h2167, _, _) => {
                h2167
            }
        })?;
        append_list(a2168, a2169)
    }))
}

pub fn f640(v0: Adt59) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("prompts"), Adt9::C4(f721(match v0.clone() {
        Adt59::C0(_, _, h2170) => {
            h2170
        }
    }))), {
        let a2173 = f712(Str::lit("nextCursor"), match v0.clone() {
            Adt59::C0(_, h2171, _) => {
                h2171
            }
        });
        let a2174 = f713(match v0 {
            Adt59::C0(h2172, _, _) => {
                h2172
            }
        })?;
        append_list(a2173, a2174)
    }))
}

pub fn f641(v0: Adt62) -> R<Option<List<(Str, Adt9)>>> {
    let m2176 = f722(match v0.clone() {
        Adt62::C0(_, h2175) => {
            h2175
        }
    })?;
    match m2176 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2178 = f712(Str::lit("description"), match v0 {
                    Adt62::C0(h2177, _) => {
                        h2177
                    }
                });
                let a2179 = List::cons((Str::lit("messages"), Adt9::C4(v1)), List::<(Str, Adt9)>::nil());
                append_list(a2178, a2179)
            }))
        }
    }
}

pub fn f642(v0: Adt64) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("completion"), Adt9::C5(List::cons((Str::lit("values"), Adt9::C4(f666({
        let m2182 = match v0.clone() {
            Adt64::C0(h2181) => {
                h2181
            }
        };
        match m2182 {
            Adt65::C0(h2180, _, _) => {
                h2180
            }
        }
    }))), {
        let a2189 = f723(Str::lit("total"), {
            let m2185 = match v0.clone() {
                Adt64::C0(h2184) => {
                    h2184
                }
            };
            match m2185 {
                Adt65::C0(_, h2183, _) => {
                    h2183
                }
            }
        })?;
        let a2190 = f717(Str::lit("hasMore"), {
            let m2188 = match v0 {
                Adt64::C0(h2187) => {
                    h2187
                }
            };
            match m2188 {
                Adt65::C0(_, _, h2186) => {
                    h2186
                }
            }
        });
        append_list(a2189, a2190)
    }))), List::<(Str, Adt9)>::nil()))
}

pub fn f643(v0: Adt85) -> R<Option<List<(Str, Adt9)>>> {
    let m2191 = f724(f725(v0.clone()), List::<(Str, Adt9)>::nil())?;
    match m2191 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2192 = f726(v1);
                let a2193 = f727(f728(v0));
                append_list(a2192, a2193)
            }))
        }
    }
}

pub fn f644(v0: Adt66) -> Option<List<(Str, Adt9)>> {
    let m2194 = f729(v0.clone());
    match m2194 {
        None => {
            None::<List<(Str, Adt9)>>
        }
        Some(v1) => {
            Some({
                let a2195 = f730(f731(v0.clone()), f732(f733(v0)));
                let a2196 = v1;
                append_list(a2195, a2196)
            })
        }
    }
}

pub fn f645(v0: Adt91) -> Option<List<(Str, Adt9)>> {
    let m2198 = match v0.clone() {
        Adt91::C0(_, h2197, _) => {
            h2197
        }
    };
    match m2198 {
        Adt92::C0 => {
            f644({
                let m2201 = match v0 {
                    Adt91::C0(h2200, _, _) => {
                        h2200
                    }
                };
                match m2201 {
                    Adt88::C0(h2199, _, _) => {
                        h2199
                    }
                }
            })
        }
        Adt92::C1 => {
            Some(List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f646(v0: Adt162) -> Str {
    let m2202 = v0;
    match m2202 {
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

pub fn f647(v0: Adt36) -> Adt162 {
    let m2203 = v0;
    match m2203 {
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

pub fn f648(v0: Option<Adt4>) -> List<(Str, Adt9)> {
    let m2204 = v0;
    match m2204 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((f734(), f735(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f649(v0: Bytes) -> R<Option<List<Adt88>>> {
    let a2206 = {
        let a2205 = v0.clone();
        length_bytes(a2205)
    };
    let a2207 = 0u64;
    if nat_eq(a2206, a2207) {
        Ok(Some(List::<Adt88>::nil()))
    } else {
        let m2208 = f266(v0)?;
        match m2208 {
            Ok(v2) => {
                let m2209 = f286(v2);
                match m2209 {
                    None => {
                        Ok(None::<List<Adt88>>)
                    }
                    Some(v3) => {
                        Ok(f736(v3, List::<Adt88>::nil()))
                    }
                }
            }
            Err(_) => {
                Ok(None::<List<Adt88>>)
            }
        }
    }
}

pub fn f650(v0: List<Adt88>) -> R<Option<List<u64>>> {
    let m2210 = f737(v0);
    match m2210 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m2211 = f320(v1)?;
            match m2211 {
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

pub fn f651(v0: Adt88, v1: List<Adt88>, v2: i64) -> R<List<Adt88>> {
    Ok(List::cons(v0.clone(), f738(f739(f740(v0), v1, v2, List::<Adt88>::nil())?, List::<Adt88>::nil())))
}

pub fn f652(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f741(v2) {
            let a2212 = v1;
            let a2213 = 200u64;
            nat_eq(a2212, a2213)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f653(v0: u64, v1: List<List<u64>>) -> Adt103 {
    Adt103::C0(v0, f742(), List::<List<u64>>::nil(), f743(v1), None::<List<u64>>)
}

pub fn f654(v0: Adt9) -> R<Option<Adt9>> {
    f744(f745(), Adt163::C0(v0), List::<Adt164>::nil())
}

pub fn f655(v0: Adt144) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("jsonrpc"), Adt9::C3(f373())), List::cons((Str::lit("method"), Adt9::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt9::C5(List::cons((Str::lit("progressToken"), f746(match v0.clone() {
        Adt144::C0(h2214, _) => {
            h2214
        }
    })), List::cons((Str::lit("progress"), Adt9::C2({
        let m2217 = match v0.clone() {
            Adt144::C0(_, h2216) => {
                h2216
            }
        };
        match m2217 {
            Adt54::C0(h2215, _, _) => {
                h2215
            }
        }
    })), {
        let a2224 = f747({
            let m2220 = match v0.clone() {
                Adt144::C0(_, h2219) => {
                    h2219
                }
            };
            match m2220 {
                Adt54::C0(_, h2218, _) => {
                    h2218
                }
            }
        });
        let a2225 = f748({
            let m2223 = match v0 {
                Adt144::C0(_, h2222) => {
                    h2222
                }
            };
            match m2223 {
                Adt54::C0(_, _, h2221) => {
                    h2221
                }
            }
        });
        append_list(a2224, a2225)
    })))), List::<(Str, Adt9)>::nil()))))
}

pub fn f656(v0: Adt91) -> bool {
    if match v0.clone() {
        Adt91::C0(_, _, h2226) => {
            h2226
        }
    } {
        let m2228 = match v0 {
            Adt91::C0(_, h2227, _) => {
                h2227
            }
        };
        match m2228 {
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

pub fn f657(v0: Adt66) -> Option<Adt9> {
    let m2229 = f644(v0);
    match m2229 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            Some(Adt9::C5(List::cons((Str::lit("jsonrpc"), Adt9::C3(Str::lit("2.0"))), List::cons((Str::lit("method"), Adt9::C3(Str::lit("notifications/tasks"))), List::cons((Str::lit("params"), Adt9::C5(v1)), List::<(Str, Adt9)>::nil())))))
        }
    }
}

pub fn f658() -> u64 {
    429u64
}

pub fn f659() -> i64 {
    -32602i64
}

pub fn f660() -> i64 {
    -32022i64
}

pub fn f661() -> i64 {
    -32020i64
}

pub fn f662() -> i64 {
    -32601i64
}

pub fn f663(v0: Adt93) -> i64 {
    let m2230 = v0;
    match m2230 {
        Adt93::C0(_) => {
            f659()
        }
        Adt93::C1(_) => {
            f659()
        }
        Adt93::C2(_) => {
            f659()
        }
        Adt93::C3(_) => {
            f659()
        }
        Adt93::C4(_) => {
            f659()
        }
        Adt93::C5(_) => {
            f659()
        }
        Adt93::C6(_) => {
            f749()
        }
        Adt93::C7(_) => {
            f750()
        }
        Adt93::C8(_) => {
            f751()
        }
        Adt93::C9(_) => {
            f749()
        }
        Adt93::C10 => {
            f659()
        }
        Adt93::C11 => {
            f659()
        }
        Adt93::C12(_) => {
            f749()
        }
        Adt93::C13 => {
            f749()
        }
        Adt93::C14 => {
            f749()
        }
        Adt93::C15(_) => {
            f749()
        }
    }
}

pub fn f664(v0: Adt146) -> Str {
    let m2231 = v0;
    match m2231 {
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

pub fn f665(v0: Adt129) -> Str {
    let m2232 = v0;
    match m2232 {
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
            let m2233 = v1;
            match m2233 {
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
            let m2234 = v2;
            match m2234 {
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

pub fn f666(v0: List<Str>) -> List<Adt9> {
    f574(f752(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f667(v0: Adt21) -> Adt9 {
    Adt9::C5({
        let a2246 = f753(Str::lit("experimental"), match v0.clone() {
            Adt21::C0(h2235, _, _, _, _) => {
                h2235
            }
        });
        let a2247 = {
            let a2244 = f753(Str::lit("roots"), match v0.clone() {
                Adt21::C0(_, h2236, _, _, _) => {
                    h2236
                }
            });
            let a2245 = {
                let a2242 = f753(Str::lit("sampling"), match v0.clone() {
                    Adt21::C0(_, _, h2237, _, _) => {
                        h2237
                    }
                });
                let a2243 = {
                    let a2240 = f753(Str::lit("elicitation"), match v0.clone() {
                        Adt21::C0(_, _, _, h2238, _) => {
                            h2238
                        }
                    });
                    let a2241 = f753(Str::lit("extensions"), match v0 {
                        Adt21::C0(_, _, _, _, h2239) => {
                            h2239
                        }
                    });
                    append_list(a2240, a2241)
                };
                append_list(a2242, a2243)
            };
            append_list(a2244, a2245)
        };
        append_list(a2246, a2247)
    })
}

pub fn f668(p2250: List<List<u64>>) -> R<bool> {
    let m2249 = p2250;
    {
        let mut m2249 = m2249;
        loop {
            let step = {
                let v0 = m2249;
                let m2248 = v0;
                match m2248.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f754(f388(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
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

pub fn f669() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f670(v0: Adt3, v1: Fn3, v2: u64, v3: Adt152, v4: bool) -> R<Result<Adt142, Adt145>> {
    if v4 {
        f755(v0, v1, v2, f756(match v3.clone() {
            Adt152::C0(_, h2251, _, _) => {
                h2251
            }
        }, match v3.clone() {
            Adt152::C0(_, _, h2252, _) => {
                h2252
            }
        }, match v3 {
            Adt152::C0(_, _, _, h2253) => {
                h2253
            }
        }))
    } else {
        Ok(Err::<Adt142, Adt145>(Adt145::C3(f757(match v3 {
            Adt152::C0(_, h2254, _, _) => {
                h2254
            }
        }))))
    }
}

pub fn f671(v0: Adt7, v1: Adt153) -> bool {
    let m2255 = v1;
    match m2255 {
        Adt153::C0 => {
            true
        }
        Adt153::C1 => {
            f758(match v0 {
                Adt7::C0(_, _, _, _, _, h2256, _) => {
                    h2256
                }
            })
        }
        Adt153::C2 => {
            f758(match v0 {
                Adt7::C0(_, _, _, _, _, h2257, _) => {
                    h2257
                }
            })
        }
        Adt153::C3 => {
            f759(match v0 {
                Adt7::C0(_, _, _, _, h2258, _, _) => {
                    h2258
                }
            })
        }
        Adt153::C4 => {
            f759(match v0 {
                Adt7::C0(_, _, _, _, h2259, _, _) => {
                    h2259
                }
            })
        }
        Adt153::C5 => {
            f759(match v0 {
                Adt7::C0(_, _, _, _, h2260, _, _) => {
                    h2260
                }
            })
        }
        Adt153::C6 => {
            f758(match v0 {
                Adt7::C0(_, _, _, h2261, _, _, _) => {
                    h2261
                }
            })
        }
        Adt153::C7 => {
            f758(match v0 {
                Adt7::C0(_, _, _, h2262, _, _, _) => {
                    h2262
                }
            })
        }
        Adt153::C8 => {
            f294(match v0 {
                Adt7::C0(_, _, h2263, _, _, _, _) => {
                    h2263
                }
            })
        }
        Adt153::C9 => {
            f760(match v0 {
                Adt7::C0(_, _, _, _, _, _, h2264) => {
                    h2264
                }
            }, f63())
        }
        Adt153::C10 => {
            f760(match v0 {
                Adt7::C0(_, _, _, _, _, _, h2265) => {
                    h2265
                }
            }, f63())
        }
        Adt153::C11 => {
            f760(match v0 {
                Adt7::C0(_, _, _, _, _, _, h2266) => {
                    h2266
                }
            }, f63())
        }
    }
}

pub fn f672(v0: Adt158, v1: Adt156) -> Result<Adt152, Adt145> {
    let m2267 = v1;
    match m2267 {
        Adt156::C0 => {
            Err::<Adt152, Adt145>(Adt145::C3(match v0 {
                Adt158::C0(_, h2268, _, _) => {
                    h2268
                }
            }))
        }
        Adt156::C1(v2) => {
            Ok::<Adt152, Adt145>(Adt152::C0(match v0.clone() {
                Adt158::C0(h2269, _, _, _) => {
                    h2269
                }
            }, v2, match v0.clone() {
                Adt158::C0(_, _, h2270, _) => {
                    h2270
                }
            }, match v0 {
                Adt158::C0(_, _, _, h2271) => {
                    h2271
                }
            }))
        }
    }
}

pub fn f673(v0: Option<Adt128>, v1: List<Str>, v2: Adt123, v3: Result<Adt20, Adt146>) -> R<Adt157> {
    let m2272 = v3;
    match m2272 {
        Ok(v5) => {
            Ok(f761(v1, v2.clone(), v5.clone(), f762(v0, f562(match v2.clone() {
                Adt123::C0(_, h2273, _) => {
                    h2273
                }
            }), match v2.clone() {
                Adt123::C0(_, h2274, _) => {
                    h2274
                }
            }, match v5 {
                Adt20::C0(h2275, _, _, _, _, _) => {
                    h2275
                }
            }, f763(match v2 {
                Adt123::C0(_, _, h2276) => {
                    h2276
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt157::C1(Adt145::C0(v4)))
        }
    }
}

pub fn f674(v0: Option<Adt8>) -> Result<Adt20, Adt146> {
    let m2277 = v0;
    match m2277 {
        None => {
            Err::<Adt20, Adt146>(Adt146::C0)
        }
        Some(v1) => {
            let m2278 = f233(Str::lit("_meta"), v1);
            match m2278 {
                Adt115::C0 => {
                    Err::<Adt20, Adt146>(Adt146::C1)
                }
                Adt115::C1(v2) => {
                    let m2279 = f234(v2);
                    match m2279 {
                        None => {
                            Err::<Adt20, Adt146>(Adt146::C4)
                        }
                        Some(v3) => {
                            f764(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f675(v0: Adt153) -> Adt159 {
    let m2280 = v0;
    match m2280 {
        Adt153::C0 => {
            Adt159::C0
        }
        Adt153::C1 => {
            Adt159::C0
        }
        Adt153::C2 => {
            Adt159::C1
        }
        Adt153::C3 => {
            Adt159::C0
        }
        Adt153::C4 => {
            Adt159::C2
        }
        Adt153::C5 => {
            Adt159::C0
        }
        Adt153::C6 => {
            Adt159::C0
        }
        Adt153::C7 => {
            Adt159::C1
        }
        Adt153::C8 => {
            Adt159::C0
        }
        Adt153::C9 => {
            Adt159::C3
        }
        Adt153::C10 => {
            Adt159::C3
        }
        Adt153::C11 => {
            Adt159::C3
        }
    }
}

pub fn f676(p2286: List<u64>, p2287: List<u64>) -> Option<List<u64>> {
    let m2285 = (p2286, p2287);
    {
        let mut m2285 = m2285;
        loop {
            let step = {
                let (v0, v1) = m2285;
                let m2281 = v0;
                match m2281.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2282 = v1;
                        match m2282.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a2283 = v4;
                                let a2284 = v2;
                                if nat_eq(a2283, a2284) {
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
                Ok(next) => { m2285 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f677() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f678() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f679(v0: Adt15, v1: Option<Str>) -> bool {
    let m2288 = v1;
    match m2288 {
        None => {
            false
        }
        Some(v2) => {
            let a2290 = match v0 {
                Adt15::C0(h2289, _, _, _) => {
                    h2289
                }
            };
            let a2291 = v2;
            equal(a2290, a2291)
        }
    }
}

pub fn f680(v0: List<Adt100>, v1: Str) -> R<Adt132> {
    let m2294 = f493(f268(v0, {
        let a2292 = f765();
        let a2293 = f50(v1)?;
        append_list(a2292, a2293)
    })?)?;
    match m2294 {
        Adt119::C0 => {
            Ok(Adt132::C0)
        }
        Adt119::C1 => {
            Ok(Adt132::C1)
        }
        Adt119::C2 => {
            Ok(Adt132::C1)
        }
        Adt119::C3(v2) => {
            let m2295 = f470(v2);
            match m2295 {
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

pub fn f681(v0: i64) -> bool {
    let a2302 = {
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
            true
        } else {
            let m2301 = {
                let a2299 = 0i64;
                let a2300 = v0;
                compare(a2299, a2300)
            };
            match m2301 {
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
    bool_not(a2302)
}

pub fn f682(v0: Adt135, v1: u64, v2: Str) -> Adt149 {
    let m2304 = match v0.clone() {
        Adt135::C0(_, h2303, _) => {
            h2303
        }
    };
    match m2304.uncons() {
        None => {
            f512(Adt116::C2(v1))
        }
        Some((v3, v4)) => {
            let m2305 = v3;
            match m2305 {
                Adt140::C0(_) => {
                    f512(Adt116::C2(v1))
                }
                Adt140::C1(v6, _) => {
                    Adt149::C1(Adt135::C0(Adt136::C5, List::cons(Adt140::C1(v6, Some(v2)), v4), match v0 {
                        Adt135::C0(_, _, h2306) => {
                            h2306
                        }
                    }))
                }
            }
        }
    }
}

pub fn f683(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a2307 = v0;
    let a2308 = 128u64;
    if nat_lt(a2307, a2308) {
        Ok(List::cons(v0, v1))
    } else {
        let a2309 = v0;
        let a2310 = 2048u64;
        if nat_lt(a2309, a2310) {
            Ok(List::cons({
                let a2311 = 128u64;
                let a2312 = f766(v0, 64u64);
                nat_add(a2311, a2312)?
            }, List::cons({
                let a2313 = 192u64;
                let a2314 = f767(v0, 64u64);
                nat_add(a2313, a2314)?
            }, v1)))
        } else {
            let a2315 = v0;
            let a2316 = 65536u64;
            if nat_lt(a2315, a2316) {
                Ok(List::cons({
                    let a2317 = 128u64;
                    let a2318 = f766(v0, 64u64);
                    nat_add(a2317, a2318)?
                }, List::cons({
                    let a2319 = 128u64;
                    let a2320 = f766(f767(v0, 64u64), 64u64);
                    nat_add(a2319, a2320)?
                }, List::cons({
                    let a2321 = 224u64;
                    let a2322 = f767(v0, 4096u64);
                    nat_add(a2321, a2322)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a2323 = 128u64;
                    let a2324 = f766(v0, 64u64);
                    nat_add(a2323, a2324)?
                }, List::cons({
                    let a2325 = 128u64;
                    let a2326 = f766(f767(v0, 64u64), 64u64);
                    nat_add(a2325, a2326)?
                }, List::cons({
                    let a2327 = 128u64;
                    let a2328 = f766(f767(v0, 4096u64), 64u64);
                    nat_add(a2327, a2328)?
                }, List::cons({
                    let a2329 = 240u64;
                    let a2330 = f767(v0, 262144u64);
                    nat_add(a2329, a2330)?
                }, v1)))))
            }
        }
    }
}

pub fn f684() -> u64 {
    18u64
}

pub fn f685() -> u64 {
    4294967296u64
}

pub fn f686(v0: u64, v1: Adt9) -> bool {
    let a2331 = v0;
    let a2332 = 0u64;
    if nat_eq(a2331, a2332) {
        f768(v1)
    } else {
        let a2333 = v0;
        let a2334 = 1u64;
        if nat_eq(a2333, a2334) {
            f769(v1)
        } else {
            let a2335 = v0;
            let a2336 = 2u64;
            if nat_eq(a2335, a2336) {
                f770(v1)
            } else {
                let a2337 = v0;
                let a2338 = 3u64;
                if nat_eq(a2337, a2338) {
                    f771(v1)
                } else {
                    let a2339 = v0;
                    let a2340 = 4u64;
                    if nat_eq(a2339, a2340) {
                        f772(v1)
                    } else {
                        let a2341 = v0;
                        let a2342 = 5u64;
                        if nat_eq(a2341, a2342) {
                            f773(v1)
                        } else {
                            let a2343 = v0;
                            let a2344 = 6u64;
                            if nat_eq(a2343, a2344) {
                                f774(v1)
                            } else {
                                let a2345 = v0;
                                let a2346 = 7u64;
                                if nat_eq(a2345, a2346) {
                                    f775(v1)
                                } else {
                                    f776(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f687(v0: Str, v1: List<(Str, Adt9)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt124) -> R<Adt155> {
    let m2347 = v3.clone();
    match m2347.uncons() {
        None => {
            Ok(Adt155::C0(Adt104::C8(v0)))
        }
        Some((_, _)) => {
            if f398(v3.clone(), true) {
                let m2348 = f777(v1);
                match m2348 {
                    None => {
                        Ok(Adt155::C0(Adt104::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f778(v3)?;
                        if f779(v9.clone(), match v5.clone() {
                            Adt124::C0(_, h2349) => {
                                h2349
                            }
                        }) {
                            Ok(Adt155::C0(Adt104::C12(v0, v2)))
                        } else {
                            Ok(Adt155::C1(Adt124::C0(List::cons(Adt15::C0(v0, v2, f295(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt124::C0(h2350, _) => {
                                    h2350
                                }
                            }), List::cons(v9, match v5 {
                                Adt124::C0(_, h2351) => {
                                    h2351
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt155::C0(Adt104::C9(v0)))
            }
        }
    }
}

pub fn f688(p2356: List<(Str, Adt9)>, v1: bool, v2: List<Str>, p2357: List<Adt121>) -> List<Adt121> {
    let m2355 = (p2356, p2357);
    {
        let mut m2355 = m2355;
        loop {
            let step = {
                let (v0, v3) = m2355;
                let m2352 = v0;
                match m2352.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt121>), List<Adt121>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt9)>, List<Adt121>), List<Adt121>>((v5, List::cons(Adt121::C0({
                            let (_, h2353) = v4;
                            h2353
                        }, v1, List::cons({
                            let (h2354, _) = v4;
                            h2354
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f689(p2361: List<(Str, Adt9)>, p2362: List<Adt121>) -> List<Adt121> {
    let m2360 = (p2361, p2362);
    {
        let mut m2360 = m2360;
        loop {
            let step = {
                let (v0, v1) = m2360;
                let m2358 = v0;
                match m2358.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt121>), List<Adt121>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt9)>, List<Adt121>), List<Adt121>>((v3, List::cons(Adt121::C0({
                            let (_, h2359) = v2;
                            h2359
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2360 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f690(p2365: List<Adt9>, p2366: List<Adt121>) -> List<Adt121> {
    let m2364 = (p2365, p2366);
    {
        let mut m2364 = m2364;
        loop {
            let step = {
                let (v0, v1) = m2364;
                let m2363 = v0;
                match m2363.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt121>), List<Adt121>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt9>, List<Adt121>), List<Adt121>>((v3, List::cons(Adt121::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2364 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f691(v0: Adt9) -> Str {
    let m2367 = v0;
    match m2367 {
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

pub fn f692(v0: Adt10) -> R<bool> {
    if f681(match v0.clone() {
        Adt10::C0(h2368, _) => {
            h2368
        }
    }) {
        Ok(true)
    } else {
        f780(40u64, match v0.clone() {
            Adt10::C0(h2369, _) => {
                h2369
            }
        }, match v0 {
            Adt10::C0(_, h2370) => {
                h2370
            }
        })
    }
}

pub fn f693(v0: Adt9) -> Str {
    let m2371 = f283(v0);
    match m2371 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f694(p2378: u64, p2379: List<(Adt9, Adt9)>) -> R<Adt160> {
    let m2377 = (p2378, p2379);
    {
        let mut m2377 = m2377;
        loop {
            let step = {
                let (v0, v1) = m2377;
                let m2372 = v0;
                if m2372 == 0 {
                    Err::<(u64, List<(Adt9, Adt9)>), R<Adt160>>(Ok(Adt160::C2))
                } else {
                    let v2 = m2372 - 1;
                    let m2373 = v1;
                    match m2373.uncons() {
                        None => {
                            Err::<(u64, List<(Adt9, Adt9)>), R<Adt160>>(Ok(Adt160::C0))
                        }
                        Some((v3, v4)) => {
                            let m2376 = f781({
                                let (h2374, _) = v3;
                                h2374
                            }, {
                                let (_, h2375) = v3;
                                h2375
                            }, v4)?;
                            match m2376 {
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
                Ok(next) => { m2377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f695() -> u64 {
    67108864u64
}

pub fn f696(v0: Adt161) -> bool {
    let m2380 = v0;
    match m2380 {
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

pub fn f697(v0: Adt10, v1: Adt10, v2: Adt161, v3: Adt161) -> R<Adt161> {
    let m2381 = v2;
    match m2381 {
        Adt161::C0 => {
            let m2383 = v3;
            match m2383 {
                Adt161::C0 => {
                    Ok(f782(Adt161::C0, f783(f784(match v0.clone() {
                        Adt10::C0(h2384, _) => {
                            h2384
                        }
                    })?, match v0 {
                        Adt10::C0(_, h2385) => {
                            h2385
                        }
                    }, f784(match v1.clone() {
                        Adt10::C0(h2386, _) => {
                            h2386
                        }
                    })?, match v1 {
                        Adt10::C0(_, h2387) => {
                            h2387
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
            let m2382 = v3;
            match m2382 {
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
            let m2388 = v3;
            match m2388 {
                Adt161::C0 => {
                    Ok(Adt161::C2)
                }
                Adt161::C1 => {
                    Ok(Adt161::C2)
                }
                Adt161::C2 => {
                    f783(match v0.clone() {
                        Adt10::C0(h2389, _) => {
                            h2389
                        }
                    }, match v0 {
                        Adt10::C0(_, h2390) => {
                            h2390
                        }
                    }, match v1.clone() {
                        Adt10::C0(h2391, _) => {
                            h2391
                        }
                    }, match v1 {
                        Adt10::C0(_, h2392) => {
                            h2392
                        }
                    })
                }
            }
        }
    }
}

pub fn f698(v0: i64) -> Adt161 {
    f700(v0, 0i64)
}

pub fn f699(v0: Adt9) -> Option<i64> {
    let m2393 = v0;
    match m2393 {
        Adt9::C0 => {
            None::<i64>
        }
        Adt9::C1(_) => {
            None::<i64>
        }
        Adt9::C2(v2) => {
            if f681(match v2.clone() {
                Adt10::C0(_, h2394) => {
                    h2394
                }
            }) {
                Some(match v2 {
                    Adt10::C0(h2395, _) => {
                        h2395
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

pub fn f700(v0: i64, v1: i64) -> Adt161 {
    let m2398 = {
        let a2396 = v0;
        let a2397 = v1;
        compare(a2396, a2397)
    };
    if match m2398 {
        Ordering::Less => {
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
        let m2401 = {
            let a2399 = v1;
            let a2400 = v0;
            compare(a2399, a2400)
        };
        if match m2401 {
            Ordering::Less => {
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

pub fn f701(p2410: List<u64>, p2411: u64) -> R<u64> {
    let m2409 = (p2410, p2411);
    {
        let mut m2409 = m2409;
        loop {
            let step = {
                let (v0, v1) = m2409;
                let m2402 = v0;
                match m2402.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2403 = v2;
                        let a2404 = 128u64;
                        if if nat_lt(a2403, a2404) {
                            true
                        } else {
                            let a2405 = 191u64;
                            let a2406 = v2;
                            nat_lt(a2405, a2406)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2407 = v1;
                                let a2408 = 1u64;
                                nat_add(a2407, a2408)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f702(v0: Str, p2429: List<(Str, Adt9)>) -> Option<Adt9> {
    let m2428 = p2429;
    {
        let mut m2428 = m2428;
        loop {
            let step = {
                let v1 = m2428;
                let m2412 = v1;
                match m2412.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, Option<Adt9>>(None::<Adt9>)
                    }
                    Some((v2, v3)) => {
                        let a2416 = v0.clone();
                        let a2417 = {
                            let a2414 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2413, _) = v2.clone();
                                h2413
                            }, List::<Str>::nil()));
                            let a2415 = Str::lit("");
                            join(a2414, a2415)
                        };
                        if equal(a2416, a2417) {
                            Err::<List<(Str, Adt9)>, Option<Adt9>>(Some({
                                let (_, h2418) = v2;
                                h2418
                            }))
                        } else {
                            let m2420 = f234({
                                let (_, h2419) = v2.clone();
                                h2419
                            });
                            match m2420 {
                                None => {
                                    Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                }
                                Some(v4) => {
                                    let m2421 = f233(Str::lit("$anchor"), v4);
                                    match m2421 {
                                        Adt115::C0 => {
                                            Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                        }
                                        Adt115::C1(v5) => {
                                            let m2422 = f283(v5);
                                            match m2422 {
                                                None => {
                                                    Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2425 = v0.clone();
                                                    let a2426 = {
                                                        let a2423 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2424 = Str::lit("");
                                                        join(a2423, a2424)
                                                    };
                                                    if equal(a2425, a2426) {
                                                        Err::<List<(Str, Adt9)>, Option<Adt9>>(Some({
                                                            let (_, h2427) = v2;
                                                            h2427
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
                Ok(next) => { m2428 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f703(v0: u64) -> R<Option<u64>> {
    let a2430 = 64u64;
    let a2431 = v0;
    if if nat_lt(a2430, a2431) {
        let a2432 = v0;
        let a2433 = 91u64;
        nat_lt(a2432, a2433)
    } else {
        false
    } {
        Ok(Some({
            let a2434 = v0;
            let a2435 = 65u64;
            nat_sub(a2434, a2435)
        }))
    } else {
        let a2436 = 96u64;
        let a2437 = v0;
        if if nat_lt(a2436, a2437) {
            let a2438 = v0;
            let a2439 = 123u64;
            nat_lt(a2438, a2439)
        } else {
            false
        } {
            Ok(Some({
                let a2440 = v0;
                let a2441 = 71u64;
                nat_sub(a2440, a2441)
            }))
        } else {
            let a2442 = 47u64;
            let a2443 = v0;
            if if nat_lt(a2442, a2443) {
                let a2444 = v0;
                let a2445 = 58u64;
                nat_lt(a2444, a2445)
            } else {
                false
            } {
                Ok(Some({
                    let a2446 = v0;
                    let a2447 = 4u64;
                    nat_add(a2446, a2447)?
                }))
            } else {
                let a2448 = v0;
                let a2449 = 43u64;
                if nat_eq(a2448, a2449) {
                    Ok(Some(62u64))
                } else {
                    let a2450 = v0;
                    let a2451 = 47u64;
                    if nat_eq(a2450, a2451) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f704(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2452 = v3;
        let a2453 = 61u64;
        if nat_eq(a2452, a2453) {
            let a2454 = f785(v1, 16u64);
            let a2455 = 0u64;
            nat_eq(a2454, a2455)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f786(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f705(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2456 = f785(v2, 4u64);
        let a2457 = 0u64;
        nat_eq(a2456, a2457)
    } else {
        false
    } {
        Ok(Some(f787(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f706(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2460 = {
            let a2458 = v0;
            let a2459 = 4u64;
            nat_mul(a2458, a2459)?
        };
        let a2461 = f788(v1, 16u64);
        nat_add(a2460, a2461)?
    }, List::cons({
        let a2464 = {
            let a2462 = f785(v1, 16u64);
            let a2463 = 16u64;
            nat_mul(a2462, a2463)?
        };
        let a2465 = f788(v2, 4u64);
        nat_add(a2464, a2465)?
    }, List::cons({
        let a2468 = {
            let a2466 = f785(v2, 4u64);
            let a2467 = 64u64;
            nat_mul(a2466, a2467)?
        };
        let a2469 = v3;
        nat_add(a2468, a2469)?
    }, List::<u64>::nil()))))
}

pub fn f707() -> i64 {
    999999999999999999i64
}

pub fn f708(v0: i64) -> R<List<u64>> {
    let m2472 = {
        let a2470 = v0;
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
        Ok(List::cons(45u64, f789(20u64, {
            let a2473 = v0;
            int_neg(a2473)?
        }, List::<u64>::nil())?))
    } else {
        f789(20u64, v0, List::<u64>::nil())
    }
}

pub fn f709(v0: u64) -> R<List<u64>> {
    let a2474 = v0;
    let a2475 = 34u64;
    if nat_eq(a2474, a2475) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2476 = v0;
        let a2477 = 92u64;
        if nat_eq(a2476, a2477) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2478 = v0;
            let a2479 = 8u64;
            if nat_eq(a2478, a2479) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2480 = v0;
                let a2481 = 9u64;
                if nat_eq(a2480, a2481) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2482 = v0;
                    let a2483 = 10u64;
                    if nat_eq(a2482, a2483) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2484 = v0;
                        let a2485 = 12u64;
                        if nat_eq(a2484, a2485) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2486 = v0;
                            let a2487 = 13u64;
                            if nat_eq(a2486, a2487) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2488 = v0;
                                let a2489 = 32u64;
                                if nat_lt(a2488, a2489) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f790(f767(v0, 16u64))?, List::cons(f790(f766(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f710(v0: List<Str>) -> Adt9 {
    Adt9::C4(f666(v0))
}

pub fn f711(v0: Adt7) -> Adt9 {
    Adt9::C5({
        let a2507 = f753(Str::lit("experimental"), match v0.clone() {
            Adt7::C0(h2490, _, _, _, _, _, _) => {
                h2490
            }
        });
        let a2508 = {
            let a2505 = f753(Str::lit("logging"), match v0.clone() {
                Adt7::C0(_, h2491, _, _, _, _, _) => {
                    h2491
                }
            });
            let a2506 = {
                let a2503 = f753(Str::lit("completions"), match v0.clone() {
                    Adt7::C0(_, _, h2492, _, _, _, _) => {
                        h2492
                    }
                });
                let a2504 = {
                    let a2501 = f791(Str::lit("prompts"), match v0.clone() {
                        Adt7::C0(_, _, _, h2493, _, _, _) => {
                            h2493
                        }
                    });
                    let a2502 = {
                        let a2499 = f792(match v0.clone() {
                            Adt7::C0(_, _, _, _, h2494, _, _) => {
                                h2494
                            }
                        });
                        let a2500 = {
                            let a2497 = f791(Str::lit("tools"), match v0.clone() {
                                Adt7::C0(_, _, _, _, _, h2495, _) => {
                                    h2495
                                }
                            });
                            let a2498 = f753(Str::lit("extensions"), match v0 {
                                Adt7::C0(_, _, _, _, _, _, h2496) => {
                                    h2496
                                }
                            });
                            append_list(a2497, a2498)
                        };
                        append_list(a2499, a2500)
                    };
                    append_list(a2501, a2502)
                };
                append_list(a2503, a2504)
            };
            append_list(a2505, a2506)
        };
        append_list(a2507, a2508)
    })
}

pub fn f712(v0: Str, v1: Option<Str>) -> List<(Str, Adt9)> {
    let m2509 = v1;
    match m2509 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C3(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f713(v0: Adt13) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("ttlMs"), f119(match v0.clone() {
        Adt13::C0(h2510, _) => {
            h2510
        }
    })?), List::cons((Str::lit("cacheScope"), Adt9::C3(f793(match v0 {
        Adt13::C0(_, h2511) => {
            h2511
        }
    }))), List::<(Str, Adt9)>::nil())))
}

pub fn f714(v0: List<Adt39>) -> List<Adt9> {
    f574(f794(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f715(v0: List<Adt42>) -> R<Option<List<Adt9>>> {
    let m2512 = f795(v0, List::<Adt9>::nil())?;
    match m2512 {
        None => {
            Ok(None::<List<Adt9>>)
        }
        Some(v1) => {
            Ok(Some(f574(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f716(v0: Str, v1: Option<Adt9>) -> List<(Str, Adt9)> {
    let m2513 = v1;
    match m2513 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f717(v0: Str, v1: Option<bool>) -> List<(Str, Adt9)> {
    let m2514 = v1;
    match m2514 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C1(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f718(v0: List<Adt48>) -> R<List<Adt9>> {
    Ok(f574(f796(v0, List::<Adt9>::nil())?, List::<Adt9>::nil()))
}

pub fn f719(v0: List<Adt50>) -> List<Adt9> {
    f574(f797(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f720(v0: List<Adt58>) -> List<Adt9> {
    f574(f798(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f721(v0: List<Adt60>) -> List<Adt9> {
    f574(f799(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f722(v0: List<Adt63>) -> R<Option<List<Adt9>>> {
    let m2515 = f800(v0, List::<Adt9>::nil())?;
    match m2515 {
        None => {
            Ok(None::<List<Adt9>>)
        }
        Some(v1) => {
            Ok(Some(f574(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f723(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt9)>> {
    let m2516 = v1;
    match m2516 {
        None => {
            Ok(List::<(Str, Adt9)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f119(v2)?), List::<(Str, Adt9)>::nil()))
        }
    }
}

pub fn f724(p2522: List<(Str, Adt68)>, p2523: List<(Str, Adt9)>) -> R<Option<List<(Str, Adt9)>>> {
    let m2521 = (p2522, p2523);
    {
        let mut m2521 = m2521;
        loop {
            let step = {
                let (v0, v1) = m2521;
                let m2517 = v0;
                match m2517.uncons() {
                    None => {
                        Err::<(List<(Str, Adt68)>, List<(Str, Adt9)>), R<Option<List<(Str, Adt9)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2519 = f801({
                            let (_, h2518) = v2;
                            h2518
                        })?;
                        match m2519 {
                            None => {
                                Err::<(List<(Str, Adt68)>, List<(Str, Adt9)>), R<Option<List<(Str, Adt9)>>>>(Ok(None::<List<(Str, Adt9)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt68)>, List<(Str, Adt9)>), R<Option<List<(Str, Adt9)>>>>((v3, List::cons(({
                                    let (h2520, _) = v2;
                                    h2520
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2521 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f725(v0: Adt85) -> List<(Str, Adt68)> {
    let m2524 = v0;
    match m2524 {
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

pub fn f726(v0: List<(Str, Adt9)>) -> List<(Str, Adt9)> {
    let m2525 = v0.clone();
    match m2525.uncons() {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt9::C5(f576(v0, List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f727(v0: Option<Str>) -> List<(Str, Adt9)> {
    let m2526 = v0;
    match m2526 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f728(v0: Adt85) -> Option<Str> {
    let m2527 = v0;
    match m2527 {
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

pub fn f729(v0: Adt66) -> Option<List<(Str, Adt9)>> {
    let m2528 = v0;
    match m2528 {
        Adt66::C0(_) => {
            Some(List::<(Str, Adt9)>::nil())
        }
        Adt66::C1(_, v3) => {
            let m2529 = f802(v3);
            match m2529 {
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
                Adt8::C0(h2530) => {
                    h2530
                }
            })), List::<(Str, Adt9)>::nil()))
        }
        Adt66::C3(_, v8) => {
            Some(List::cons((Str::lit("error"), f375(v8)), List::<(Str, Adt9)>::nil()))
        }
        Adt66::C4(_) => {
            Some(List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f730(v0: Adt67, v1: Str) -> List<(Str, Adt9)> {
    List::cons((Str::lit("taskId"), Adt9::C3(match v0.clone() {
        Adt67::C0(h2531, _, _, _, _, _) => {
            h2531
        }
    })), List::cons((Str::lit("status"), Adt9::C3(v1)), {
        let a2537 = f803(Str::lit("statusMessage"), match v0.clone() {
            Adt67::C0(_, h2532, _, _, _, _) => {
                h2532
            }
        });
        let a2538 = List::cons((Str::lit("createdAt"), Adt9::C3(match v0.clone() {
            Adt67::C0(_, _, h2533, _, _, _) => {
                h2533
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt9::C3(match v0.clone() {
            Adt67::C0(_, _, _, h2534, _, _) => {
                h2534
            }
        })), List::cons((Str::lit("ttlMs"), f804(match v0.clone() {
            Adt67::C0(_, _, _, _, h2535, _) => {
                h2535
            }
        })), f805(Str::lit("pollIntervalMs"), match v0 {
            Adt67::C0(_, _, _, _, _, h2536) => {
                h2536
            }
        }))));
        append_list(a2537, a2538)
    }))
}

pub fn f731(v0: Adt66) -> Adt67 {
    let m2539 = v0;
    match m2539 {
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

pub fn f732(v0: Adt87) -> Str {
    let m2540 = v0;
    match m2540 {
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

pub fn f733(v0: Adt66) -> Adt87 {
    let m2541 = v0;
    match m2541 {
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

pub fn f734() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f735(v0: Adt4) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
        Adt4::C0(h2542, _, _, _, _, _) => {
            h2542
        }
    })), List::cons((Str::lit("version"), Adt9::C3(match v0.clone() {
        Adt4::C0(_, _, h2543, _, _, _) => {
            h2543
        }
    })), {
        let a2552 = f712(Str::lit("title"), match v0.clone() {
            Adt4::C0(_, h2544, _, _, _, _) => {
                h2544
            }
        });
        let a2553 = {
            let a2550 = f712(Str::lit("description"), match v0.clone() {
                Adt4::C0(_, _, _, h2545, _, _) => {
                    h2545
                }
            });
            let a2551 = {
                let a2548 = f712(Str::lit("websiteUrl"), match v0.clone() {
                    Adt4::C0(_, _, _, _, h2546, _) => {
                        h2546
                    }
                });
                let a2549 = f806(match v0 {
                    Adt4::C0(_, _, _, _, _, h2547) => {
                        h2547
                    }
                });
                append_list(a2548, a2549)
            };
            append_list(a2550, a2551)
        };
        append_list(a2552, a2553)
    })))
}

pub fn f736(p2557: List<Adt9>, p2558: List<Adt88>) -> Option<List<Adt88>> {
    let m2556 = (p2557, p2558);
    {
        let mut m2556 = m2556;
        loop {
            let step = {
                let (v0, v1) = m2556;
                let m2554 = v0;
                match m2554.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt88>), Option<List<Adt88>>>(Some(f738(v1, List::<Adt88>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2555 = f807(v2);
                        match m2555 {
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
                Ok(next) => { m2556 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f737(v0: List<Adt88>) -> Option<Adt9> {
    let m2559 = f808(v0, List::<Adt9>::nil());
    match m2559 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            Some(Adt9::C4(f574(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f738(p2562: List<Adt88>, p2563: List<Adt88>) -> List<Adt88> {
    let m2561 = (p2562, p2563);
    {
        let mut m2561 = m2561;
        loop {
            let step = {
                let (v0, v1) = m2561;
                let m2560 = v0;
                match m2560.uncons() {
                    None => {
                        Err::<(List<Adt88>, List<Adt88>), List<Adt88>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt88>, List<Adt88>), List<Adt88>>((v3, List::cons(v2, v1)))
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

pub fn f739(v0: Str, p2568: List<Adt88>, v2: i64, p2569: List<Adt88>) -> R<List<Adt88>> {
    let m2567 = (p2568, p2569);
    {
        let mut m2567 = m2567;
        loop {
            let step = {
                let (v1, v3) = m2567;
                let m2564 = v1;
                match m2564.uncons() {
                    None => {
                        Err::<(List<Adt88>, List<Adt88>), R<List<Adt88>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a2565 = f740(v4.clone());
                        let a2566 = v0.clone();
                        if if equal(a2565, a2566) {
                            true
                        } else {
                            f809(v4.clone(), v2)?
                        } {
                            Ok::<(List<Adt88>, List<Adt88>), R<List<Adt88>>>((v5, v3))
                        } else {
                            Ok::<(List<Adt88>, List<Adt88>), R<List<Adt88>>>((v5, List::cons(v4, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f740(v0: Adt88) -> Str {
    let m2572 = f731(match v0 {
        Adt88::C0(h2571, _, _) => {
            h2571
        }
    });
    match m2572 {
        Adt67::C0(h2570, _, _, _, _, _) => {
            h2570
        }
    }
}

pub fn f741(v0: List<List<u64>>) -> bool {
    let m2573 = v0;
    match m2573.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f742() -> List<Adt100> {
    List::cons(Adt100::C0(f810(), f669()), List::cons(Adt100::C0(f811(), f812()), List::cons(Adt100::C0(f813(), f814()), List::<Adt100>::nil())))
}

pub fn f743(v0: List<List<u64>>) -> List<u64> {
    f815(f171(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f744(p2588: u64, p2589: Adt163, p2590: List<Adt164>) -> R<Option<Adt9>> {
    let m2587 = (p2588, (p2589, p2590));
    {
        let mut m2587 = m2587;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2587;
                let m2574 = v0;
                if m2574 == 0 {
                    Err::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>(Ok(None::<Adt9>))
                } else {
                    let v3 = m2574 - 1;
                    let m2575 = v1;
                    match m2575 {
                        Adt163::C0(v4) => {
                            let m2576 = v4.clone();
                            match m2576 {
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
                                    let v8: Str = f816(v7)?;
                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(Adt9::C3(v8)), v2)))
                                }
                                Adt9::C4(v9) => {
                                    let m2577 = v9;
                                    match m2577.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C0(v10), List::cons(Adt164::C0(List::<Adt9>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt9::C5(v12) => {
                                    let m2578 = v12;
                                    match m2578.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f816({
                                                let (h2579, _) = v13;
                                                h2579
                                            })?;
                                            Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C0({
                                                let (_, h2580) = v13;
                                                h2580
                                            }), List::cons(Adt164::C1(v15, List::<(Str, Adt9)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt163::C1(v16) => {
                            let m2581 = v2;
                            match m2581.uncons() {
                                None => {
                                    Err::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m2582 = v17;
                                    match m2582 {
                                        Adt164::C0(v19, v20) => {
                                            let v21: List<Adt9> = List::cons(v16, v19);
                                            let m2583 = v20;
                                            match m2583.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(Adt9::C4(f574(v21, List::<Adt9>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C0(v22), List::cons(Adt164::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt164::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt9)> = List::cons((v24, v16), v25);
                                            let m2584 = v26;
                                            match m2584.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(Adt9::C5(f576(v27, List::<(Str, Adt9)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f816({
                                                        let (h2585, _) = v28;
                                                        h2585
                                                    })?;
                                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C0({
                                                        let (_, h2586) = v28;
                                                        h2586
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
                Ok(next) => { m2587 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f745() -> u64 {
    67108864u64
}

pub fn f746(v0: Adt23) -> Adt9 {
    let m2591 = v0;
    match m2591 {
        Adt23::C0(v1) => {
            Adt9::C3(v1)
        }
        Adt23::C1(v2) => {
            Adt9::C2(Adt10::C0(v2, 0i64))
        }
    }
}

pub fn f747(v0: Option<Adt10>) -> List<(Str, Adt9)> {
    let m2592 = v0;
    match m2592 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt9::C2(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f748(v0: Option<Str>) -> List<(Str, Adt9)> {
    let m2593 = v0;
    match m2593 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f749() -> i64 {
    -32603i64
}

pub fn f750() -> i64 {
    429i64
}

pub fn f751() -> i64 {
    -32021i64
}

pub fn f752(p2596: List<Str>, p2597: List<Adt9>) -> List<Adt9> {
    let m2595 = (p2596, p2597);
    {
        let mut m2595 = m2595;
        loop {
            let step = {
                let (v0, v1) = m2595;
                let m2594 = v0;
                match m2594.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt9>), List<Adt9>>((v3, List::cons(Adt9::C3(v2), v1)))
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

pub fn f753(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt9)> {
    let m2598 = v1;
    match m2598 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C5(match v2 {
                Adt8::C0(h2599) => {
                    h2599
                }
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f754(v0: List<u64>) -> R<bool> {
    let m2600 = v0;
    match m2600.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2601 = v2;
            match m2601.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2602 = f452(v1)?;
                    let a2603 = 113u64;
                    if if nat_eq(a2602, a2603) {
                        let a2604 = v3;
                        let a2605 = 61u64;
                        nat_eq(a2604, a2605)
                    } else {
                        false
                    } {
                        Ok(f817(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f755(v0: Adt3, v1: Fn3, v2: u64, v3: Result<Adt18, Adt146>) -> R<Result<Adt142, Adt145>> {
    let m2606 = v3;
    match m2606 {
        Ok(v5) => {
            f818(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt142, Adt145>(Adt145::C0(v4)))
        }
    }
}

pub fn f756(v0: Adt153, v1: Adt20, v2: Adt8) -> Result<Adt18, Adt146> {
    let m2607 = v0;
    match m2607 {
        Adt153::C0 => {
            f819(v1, v2)
        }
        Adt153::C1 => {
            f820(v1, v2)
        }
        Adt153::C2 => {
            f821(v1, v2)
        }
        Adt153::C3 => {
            f822(v1, v2)
        }
        Adt153::C4 => {
            f823(v1, v2)
        }
        Adt153::C5 => {
            f824(v1, v2)
        }
        Adt153::C6 => {
            f825(v1, v2)
        }
        Adt153::C7 => {
            f826(v1, v2)
        }
        Adt153::C8 => {
            f827(v1, v2)
        }
        Adt153::C9 => {
            f828(v1, v2)
        }
        Adt153::C10 => {
            f829(v1, v2)
        }
        Adt153::C11 => {
            f830(v1, v2)
        }
    }
}

pub fn f757(v0: Adt153) -> Str {
    let m2608 = v0;
    match m2608 {
        Adt153::C0 => {
            Str::lit("server/discover")
        }
        Adt153::C1 => {
            Str::lit("tools/list")
        }
        Adt153::C2 => {
            Str::lit("tools/call")
        }
        Adt153::C3 => {
            Str::lit("resources/list")
        }
        Adt153::C4 => {
            Str::lit("resources/read")
        }
        Adt153::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt153::C6 => {
            Str::lit("prompts/list")
        }
        Adt153::C7 => {
            Str::lit("prompts/get")
        }
        Adt153::C8 => {
            Str::lit("completion/complete")
        }
        Adt153::C9 => {
            Str::lit("tasks/get")
        }
        Adt153::C10 => {
            Str::lit("tasks/update")
        }
        Adt153::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f758(v0: Option<Adt11>) -> bool {
    let m2609 = v0;
    match m2609 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f759(v0: Option<Adt12>) -> bool {
    let m2610 = v0;
    match m2610 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f760(v0: Option<Adt8>, v1: Str) -> bool {
    let m2611 = v0;
    match m2611 {
        None => {
            false
        }
        Some(v2) => {
            f500(v1, v2)
        }
    }
}

pub fn f761(v0: List<Str>, v1: Adt123, v2: Adt20, v3: Option<Adt129>) -> Adt157 {
    let m2612 = v3;
    match m2612 {
        None => {
            f831(v1, v2.clone(), f832(match v2 {
                Adt20::C0(h2613, _, _, _, _, _) => {
                    h2613
                }
            }, v0))
        }
        Some(v4) => {
            Adt157::C1(Adt145::C2(v4))
        }
    }
}

pub fn f762(v0: Option<Adt128>, v1: Adt159, v2: Str, v3: Str, v4: Adt8) -> R<Option<Adt129>> {
    let m2614 = v0;
    match m2614 {
        None => {
            Ok(None::<Adt129>)
        }
        Some(v5) => {
            let m2616 = match v5.clone() {
                Adt128::C0(_, _, _, h2615, _) => {
                    h2615
                }
            };
            match m2616 {
                None => {
                    f833(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f763(v0: Option<Adt8>) -> Adt8 {
    let m2617 = v0;
    match m2617 {
        None => {
            f62()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f764(v0: Adt8) -> Result<Adt20, Adt146> {
    let m2618 = f834(v0.clone());
    match m2618 {
        Adt166::C0 => {
            f835(v0.clone(), f233(f836(), v0.clone()), f233(f837(), v0))
        }
        Adt166::C1 => {
            Err::<Adt20, Adt146>(Adt146::C2)
        }
        Adt166::C2 => {
            Err::<Adt20, Adt146>(Adt146::C3)
        }
    }
}

pub fn f765() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f766(v0: u64, v1: u64) -> u64 {
    let a2619 = v0;
    let a2620 = v1;
    let a2621 = 0u64;
    nat_rem(a2619, a2620, a2621)
}

pub fn f767(v0: u64, v1: u64) -> u64 {
    let a2622 = v0;
    let a2623 = v1;
    let a2624 = 0u64;
    nat_quot(a2622, a2623, a2624)
}

pub fn f768(v0: Adt9) -> bool {
    let m2625 = f283(v0);
    match m2625 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f769(v0: Adt9) -> bool {
    let m2626 = f609(v0);
    match m2626 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f770(v0: Adt9) -> bool {
    let m2627 = f699(v0);
    match m2627 {
        None => {
            false
        }
        Some(v1) => {
            let a2631 = {
                let m2630 = {
                    let a2628 = v1;
                    let a2629 = 0i64;
                    compare(a2628, a2629)
                };
                match m2630 {
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
            bool_not(a2631)
        }
    }
}

pub fn f771(v0: Adt9) -> bool {
    let m2632 = f286(v0);
    match m2632 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f772(v0: Adt9) -> bool {
    let m2633 = f234(v0);
    match m2633 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f773(v0: Adt9) -> bool {
    let m2634 = f286(v0);
    match m2634 {
        None => {
            false
        }
        Some(v1) => {
            f838(v1)
        }
    }
}

pub fn f774(v0: Adt9) -> bool {
    let m2635 = f356(v0);
    match m2635 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f775(v0: Adt9) -> bool {
    let m2636 = f283(v0.clone());
    match m2636 {
        None => {
            let m2637 = f286(v0);
            match m2637 {
                None => {
                    false
                }
                Some(v2) => {
                    f839(v2)
                }
            }
        }
        Some(v1) => {
            f604(v1)
        }
    }
}

pub fn f776(v0: Adt9) -> bool {
    let m2638 = f286(v0);
    match m2638 {
        None => {
            false
        }
        Some(v1) => {
            let m2639 = v1;
            match m2639.uncons() {
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

pub fn f777(v0: List<(Str, Adt9)>) -> Option<Adt16> {
    let m2640 = f305(Str::lit("type"), v0);
    match m2640 {
        Adt115::C0 => {
            None::<Adt16>
        }
        Adt115::C1(v1) => {
            let m2641 = f283(v1);
            match m2641 {
                None => {
                    None::<Adt16>
                }
                Some(v2) => {
                    f840(v2)
                }
            }
        }
    }
}

pub fn f778(v0: List<u64>) -> R<List<u64>> {
    Ok(f432(f841(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f779(v0: List<u64>, p2644: List<List<u64>>) -> bool {
    let m2643 = p2644;
    {
        let mut m2643 = m2643;
        loop {
            let step = {
                let v1 = m2643;
                let m2642 = v1;
                match m2642.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f842(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2643 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f780(p2656: u64, p2657: i64, p2658: i64) -> R<bool> {
    let m2655 = (p2656, (p2657, p2658));
    {
        let mut m2655 = m2655;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2655;
                let m2645 = v0;
                if m2645 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2645 - 1;
                    let m2648 = {
                        let a2646 = v2;
                        let a2647 = 0i64;
                        compare(a2646, a2647)
                    };
                    if match m2648 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f681({
                            let a2649 = v1;
                            let a2650 = 10i64;
                            let a2651 = 0i64;
                            int_rem(a2649, a2650, a2651)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2652 = v1;
                                let a2653 = 10i64;
                                let a2654 = 0i64;
                                int_quot(a2652, a2653, a2654)?
                            }, f355(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
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

pub fn f781(v0: Adt9, v1: Adt9, v2: List<(Adt9, Adt9)>) -> R<Adt165> {
    let a2659 = f843(v0.clone());
    let a2660 = f843(v1.clone());
    if nat_eq(a2659, a2660) {
        let m2661 = v0;
        match m2661 {
            Adt9::C0 => {
                Ok(Adt165::C1(v2))
            }
            Adt9::C1(v3) => {
                let m2662 = f356(v1);
                match m2662 {
                    None => {
                        Ok(Adt165::C0)
                    }
                    Some(v4) => {
                        let a2663 = v3;
                        let a2664 = v4;
                        if equal(a2663, a2664) {
                            Ok(Adt165::C1(v2))
                        } else {
                            Ok(Adt165::C0)
                        }
                    }
                }
            }
            Adt9::C2(v5) => {
                let m2665 = f609(v1);
                match m2665 {
                    None => {
                        Ok(Adt165::C0)
                    }
                    Some(v6) => {
                        if f844(f611(v5, v6)?) {
                            Ok(Adt165::C1(v2))
                        } else {
                            Ok(Adt165::C0)
                        }
                    }
                }
            }
            Adt9::C3(v7) => {
                let m2666 = f283(v1);
                match m2666 {
                    None => {
                        Ok(Adt165::C0)
                    }
                    Some(v8) => {
                        let a2667 = v7;
                        let a2668 = v8;
                        if equal(a2667, a2668) {
                            Ok(Adt165::C1(v2))
                        } else {
                            Ok(Adt165::C0)
                        }
                    }
                }
            }
            Adt9::C4(v9) => {
                let m2669 = f286(v1);
                match m2669 {
                    None => {
                        Ok(Adt165::C0)
                    }
                    Some(v10) => {
                        Ok(f845(f846(v9, v10, List::<(Adt9, Adt9)>::nil()), v2))
                    }
                }
            }
            Adt9::C5(v11) => {
                let m2670 = f234(v1);
                match m2670 {
                    None => {
                        Ok(Adt165::C0)
                    }
                    Some(v12) => {
                        let a2672 = f615(v11.clone(), 0u64)?;
                        let a2673 = f615(match v12.clone() {
                            Adt8::C0(h2671) => {
                                h2671
                            }
                        }, 0u64)?;
                        if nat_eq(a2672, a2673) {
                            Ok(f845(f847(v11, match v12 {
                                Adt8::C0(h2674) => {
                                    h2674
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

pub fn f782(v0: Adt161, v1: Adt161) -> Adt161 {
    let m2675 = v0;
    match m2675 {
        Adt161::C0 => {
            f848(v1)
        }
        Adt161::C1 => {
            Adt161::C1
        }
        Adt161::C2 => {
            v1
        }
    }
}

pub fn f783(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt161> {
    let m2676 = f700(f849(v0, v1)?, f849(v2, v3)?);
    match m2676 {
        Adt161::C0 => {
            Ok(Adt161::C0)
        }
        Adt161::C1 => {
            f850(v0, f851(v0)?, v2, f851(v2)?)
        }
        Adt161::C2 => {
            Ok(Adt161::C2)
        }
    }
}

pub fn f784(v0: i64) -> R<i64> {
    let m2679 = {
        let a2677 = v0;
        let a2678 = 0i64;
        compare(a2677, a2678)
    };
    if match m2679 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a2680 = v0;
        int_neg(a2680)
    } else {
        Ok(v0)
    }
}

pub fn f785(v0: u64, v1: u64) -> u64 {
    let a2681 = v0;
    let a2682 = v1;
    let a2683 = 0u64;
    nat_rem(a2681, a2682, a2683)
}

pub fn f786(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2686 = {
            let a2684 = v0;
            let a2685 = 4u64;
            nat_mul(a2684, a2685)?
        };
        let a2687 = f788(v1, 16u64);
        nat_add(a2686, a2687)?
    }, List::<u64>::nil()))
}

pub fn f787(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2690 = {
            let a2688 = v0;
            let a2689 = 4u64;
            nat_mul(a2688, a2689)?
        };
        let a2691 = f788(v1, 16u64);
        nat_add(a2690, a2691)?
    }, List::cons({
        let a2694 = {
            let a2692 = f785(v1, 16u64);
            let a2693 = 16u64;
            nat_mul(a2692, a2693)?
        };
        let a2695 = f788(v2, 4u64);
        nat_add(a2694, a2695)?
    }, List::<u64>::nil())))
}

pub fn f788(v0: u64, v1: u64) -> u64 {
    let a2696 = v0;
    let a2697 = v1;
    let a2698 = 0u64;
    nat_quot(a2696, a2697, a2698)
}

pub fn f789(p2710: u64, p2711: i64, p2712: List<u64>) -> R<List<u64>> {
    let m2709 = (p2710, (p2711, p2712));
    {
        let mut m2709 = m2709;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2709;
                let m2699 = v0;
                if m2699 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2699 - 1;
                    let v4: i64 = {
                        let a2700 = v1;
                        let a2701 = 10i64;
                        let a2702 = 0i64;
                        int_quot(a2700, a2701, a2702)?
                    };
                    let v5: i64 = {
                        let a2703 = v1;
                        let a2704 = 10i64;
                        let a2705 = 0i64;
                        int_rem(a2703, a2704, a2705)
                    };
                    let m2708 = {
                        let a2706 = v4;
                        let a2707 = 1i64;
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f852(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f852(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2709 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f790(v0: u64) -> R<u64> {
    let a2713 = v0;
    let a2714 = 10u64;
    if nat_lt(a2713, a2714) {
        let a2715 = 48u64;
        let a2716 = v0;
        nat_add(a2715, a2716)
    } else {
        let a2717 = 87u64;
        let a2718 = v0;
        nat_add(a2717, a2718)
    }
}

pub fn f791(v0: Str, v1: Option<Adt11>) -> List<(Str, Adt9)> {
    let m2719 = v1;
    match m2719 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f853(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f792(v0: Option<Adt12>) -> List<(Str, Adt9)> {
    let m2720 = v0;
    match m2720 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt9::C5({
                let a2723 = f717(Str::lit("subscribe"), match v1.clone() {
                    Adt12::C0(h2721, _) => {
                        h2721
                    }
                });
                let a2724 = f717(Str::lit("listChanged"), match v1 {
                    Adt12::C0(_, h2722) => {
                        h2722
                    }
                });
                append_list(a2723, a2724)
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f793(v0: Adt14) -> Str {
    let m2725 = v0;
    match m2725 {
        Adt14::C0 => {
            Str::lit("public")
        }
        Adt14::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f794(p2728: List<Adt39>, p2729: List<Adt9>) -> List<Adt9> {
    let m2727 = (p2728, p2729);
    {
        let mut m2727 = m2727;
        loop {
            let step = {
                let (v0, v1) = m2727;
                let m2726 = v0;
                match m2726.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt39>, List<Adt9>), List<Adt9>>((v3, List::cons(f854(v2), v1)))
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

pub fn f795(p2733: List<Adt42>, p2734: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m2732 = (p2733, p2734);
    {
        let mut m2732 = m2732;
        loop {
            let step = {
                let (v0, v1) = m2732;
                let m2730 = v0;
                match m2730.uncons() {
                    None => {
                        Err::<(List<Adt42>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2731 = f855(v2)?;
                        match m2731 {
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
                Ok(next) => { m2732 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f796(p2737: List<Adt48>, p2738: List<Adt9>) -> R<List<Adt9>> {
    let m2736 = (p2737, p2738);
    {
        let mut m2736 = m2736;
        loop {
            let step = {
                let (v0, v1) = m2736;
                let m2735 = v0;
                match m2735.uncons() {
                    None => {
                        Err::<(List<Adt48>, List<Adt9>), R<List<Adt9>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt48>, List<Adt9>), R<List<Adt9>>>((v3, List::cons(f856(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2736 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f797(p2741: List<Adt50>, p2742: List<Adt9>) -> List<Adt9> {
    let m2740 = (p2741, p2742);
    {
        let mut m2740 = m2740;
        loop {
            let step = {
                let (v0, v1) = m2740;
                let m2739 = v0;
                match m2739.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt50>, List<Adt9>), List<Adt9>>((v3, List::cons(f857(v2), v1)))
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

pub fn f798(p2745: List<Adt58>, p2746: List<Adt9>) -> List<Adt9> {
    let m2744 = (p2745, p2746);
    {
        let mut m2744 = m2744;
        loop {
            let step = {
                let (v0, v1) = m2744;
                let m2743 = v0;
                match m2743.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt9>), List<Adt9>>((v3, List::cons(f858(v2), v1)))
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

pub fn f799(p2749: List<Adt60>, p2750: List<Adt9>) -> List<Adt9> {
    let m2748 = (p2749, p2750);
    {
        let mut m2748 = m2748;
        loop {
            let step = {
                let (v0, v1) = m2748;
                let m2747 = v0;
                match m2747.uncons() {
                    None => {
                        Err::<(List<Adt60>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt60>, List<Adt9>), List<Adt9>>((v3, List::cons(f859(v2), v1)))
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

pub fn f800(p2754: List<Adt63>, p2755: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m2753 = (p2754, p2755);
    {
        let mut m2753 = m2753;
        loop {
            let step = {
                let (v0, v1) = m2753;
                let m2751 = v0;
                match m2751.uncons() {
                    None => {
                        Err::<(List<Adt63>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2752 = f860(v2)?;
                        match m2752 {
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
                Ok(next) => { m2753 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f801(v0: Adt68) -> R<Option<Adt9>> {
    let m2756 = v0;
    match m2756 {
        Adt68::C0(v1) => {
            Ok(Some(f861(Str::lit("elicitation/create"), Some(f862(v1)))))
        }
        Adt68::C1(v2) => {
            let m2757 = f863(v2)?;
            match m2757 {
                None => {
                    Ok(None::<Adt9>)
                }
                Some(v3) => {
                    Ok(Some(f861(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt68::C2(v4) => {
            Ok(Some(f861(Str::lit("roots/list"), {
                let m2758 = v4;
                match m2758 {
                    None => {
                        None::<Adt9>
                    }
                    Some(v5) => {
                        Some(Adt9::C5(match v5 {
                            Adt8::C0(h2759) => {
                                h2759
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f802(v0: List<(Str, Adt68)>) -> Option<Adt9> {
    let m2760 = f864(v0, List::<(Str, Adt9)>::nil());
    match m2760 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            Some(Adt9::C5(f576(v1, List::<(Str, Adt9)>::nil())))
        }
    }
}

pub fn f803(v0: Str, v1: Option<Str>) -> List<(Str, Adt9)> {
    let m2761 = v1;
    match m2761 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C3(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f804(v0: Option<i64>) -> Adt9 {
    let m2762 = v0;
    match m2762 {
        None => {
            Adt9::C0
        }
        Some(v1) => {
            f865(v1)
        }
    }
}

pub fn f805(v0: Str, v1: Option<i64>) -> List<(Str, Adt9)> {
    let m2763 = v1;
    match m2763 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f865(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f806(v0: Option<List<Adt5>>) -> List<(Str, Adt9)> {
    let m2764 = v0;
    match m2764 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt9::C4(f866(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f807(v0: Adt9) -> Option<Adt88> {
    let m2765 = f234(v0);
    match m2765 {
        None => {
            None::<Adt88>
        }
        Some(v1) => {
            let m2766 = f867(Str::lit("task"), v1.clone());
            match m2766 {
                None => {
                    None::<Adt88>
                }
                Some(v2) => {
                    let m2767 = f868(v2);
                    match m2767 {
                        None => {
                            None::<Adt88>
                        }
                        Some(v3) => {
                            let m2768 = f869(Str::lit("createdMs"), v1.clone());
                            match m2768 {
                                None => {
                                    None::<Adt88>
                                }
                                Some(v4) => {
                                    let m2769 = f867(Str::lit("plan"), v1);
                                    match m2769 {
                                        None => {
                                            None::<Adt88>
                                        }
                                        Some(v5) => {
                                            let m2770 = f870(v5);
                                            match m2770 {
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

pub fn f808(p2774: List<Adt88>, p2775: List<Adt9>) -> Option<List<Adt9>> {
    let m2773 = (p2774, p2775);
    {
        let mut m2773 = m2773;
        loop {
            let step = {
                let (v0, v1) = m2773;
                let m2771 = v0;
                match m2771.uncons() {
                    None => {
                        Err::<(List<Adt88>, List<Adt9>), Option<List<Adt9>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2772 = f871(v2);
                        match m2772 {
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
                Ok(next) => { m2773 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f809(v0: Adt88, v1: i64) -> R<bool> {
    let m2779 = {
        let m2778 = f731(match v0.clone() {
            Adt88::C0(h2777, _, _) => {
                h2777
            }
        });
        match m2778 {
            Adt67::C0(_, _, _, _, h2776, _) => {
                h2776
            }
        }
    };
    match m2779 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a2786 = {
                let m2785 = {
                    let a2783 = {
                        let a2781 = v1;
                        let a2782 = match v0 {
                            Adt88::C0(_, h2780, _) => {
                                h2780
                            }
                        };
                        int_sub(a2781, a2782)?
                    };
                    let a2784 = v2;
                    compare(a2783, a2784)
                };
                match m2785 {
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
            Ok(bool_not(a2786))
        }
    }
}

pub fn f810() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f811() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f812() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f813() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f814() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f815(p2791: List<List<u64>>, p2792: List<u64>) -> List<u64> {
    let m2790 = (p2791, p2792);
    {
        let mut m2790 = m2790;
        loop {
            let step = {
                let (v0, v1) = m2790;
                let m2787 = v0;
                match m2787.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2788 = f872(v2);
                            let a2789 = v1;
                            append_list(a2788, a2789)
                        }))
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

pub fn f816(v0: Str) -> R<Str> {
    let m2795 = {
        let a2794 = f358(f255(), f432(f873(f25({
            let a2793 = v0;
            utf8_encode(a2793)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2794)
    };
    match m2795 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f817(v0: List<u64>) -> bool {
    let m2796 = v0;
    match m2796.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2797 = v1;
            let a2798 = 48u64;
            if nat_eq(a2797, a2798) {
                f874(v2)
            } else {
                false
            }
        }
    }
}

pub fn f818(v0: Adt3, v1: Fn3, v2: u64, v3: Adt18) -> R<Result<Adt142, Adt145>> {
    if f875(f876(v3.clone())) {
        Ok(Ok::<Adt142, Adt145>(f877(v0.clone(), Adt36::C0(f878(v0)))))
    } else {
        Ok(f879(v0.clone(), v3.clone(), f880(match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h2799) => {
                h2799
            }
        }, match v0 {
            Adt3::C0(_, _, _, _, _, h2800, _) => {
                h2800
            }
        }, v2, v1, v3)?))
    }
}

pub fn f819(v0: Adt20, _: Adt8) -> Result<Adt18, Adt146> {
    Ok::<Adt18, Adt146>(Adt18::C0(Adt19::C0(v0)))
}

pub fn f820(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2801 = f881(Str::lit("cursor"), v1);
    match m2801 {
        Ok(v2) => {
            Ok::<Adt18, Adt146>(Adt18::C1(Adt24::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt146>(v3)
        }
    }
}

pub fn f821(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2802 = f882(Str::lit("name"), v1.clone());
    match m2802 {
        Ok(v2) => {
            let m2803 = f883(Str::lit("arguments"), v1.clone());
            match m2803 {
                Ok(v3) => {
                    let m2804 = f883(Str::lit("inputResponses"), v1.clone());
                    match m2804 {
                        Ok(v4) => {
                            let m2805 = f881(Str::lit("requestState"), v1);
                            match m2805 {
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

pub fn f822(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2806 = f881(Str::lit("cursor"), v1);
    match m2806 {
        Ok(v2) => {
            Ok::<Adt18, Adt146>(Adt18::C3(Adt24::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt146>(v3)
        }
    }
}

pub fn f823(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2807 = f882(Str::lit("uri"), v1.clone());
    match m2807 {
        Ok(v2) => {
            let m2808 = f883(Str::lit("inputResponses"), v1.clone());
            match m2808 {
                Ok(v3) => {
                    let m2809 = f881(Str::lit("requestState"), v1);
                    match m2809 {
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

pub fn f824(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2810 = f881(Str::lit("cursor"), v1);
    match m2810 {
        Ok(v2) => {
            Ok::<Adt18, Adt146>(Adt18::C5(Adt24::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt146>(v3)
        }
    }
}

pub fn f825(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2811 = f881(Str::lit("cursor"), v1);
    match m2811 {
        Ok(v2) => {
            Ok::<Adt18, Adt146>(Adt18::C6(Adt24::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt146>(v3)
        }
    }
}

pub fn f826(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2812 = f882(Str::lit("name"), v1.clone());
    match m2812 {
        Ok(v2) => {
            let m2813 = f884(Str::lit("arguments"), v1.clone());
            match m2813 {
                Ok(v3) => {
                    let m2814 = f883(Str::lit("inputResponses"), v1.clone());
                    match m2814 {
                        Ok(v4) => {
                            let m2815 = f881(Str::lit("requestState"), v1);
                            match m2815 {
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

pub fn f827(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2816 = f885(Str::lit("ref"), v1.clone());
    match m2816 {
        Ok(v2) => {
            let m2817 = f886(v2);
            match m2817 {
                Ok(v3) => {
                    let m2818 = f885(Str::lit("argument"), v1.clone());
                    match m2818 {
                        Ok(v4) => {
                            let m2819 = f887(v4);
                            match m2819 {
                                Ok(v5) => {
                                    let m2820 = f888(v1);
                                    match m2820 {
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

pub fn f828(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2821 = f882(Str::lit("taskId"), v1);
    match m2821 {
        Ok(v2) => {
            Ok::<Adt18, Adt146>(Adt18::C9(Adt33::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt146>(v3)
        }
    }
}

pub fn f829(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2822 = f882(Str::lit("taskId"), v1.clone());
    match m2822 {
        Ok(v2) => {
            let m2823 = f885(Str::lit("inputResponses"), v1);
            match m2823 {
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

pub fn f830(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2824 = f882(Str::lit("taskId"), v1);
    match m2824 {
        Ok(v2) => {
            Ok::<Adt18, Adt146>(Adt18::C11(Adt35::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt146>(v3)
        }
    }
}

pub fn f831(v0: Adt123, v1: Adt20, v2: Option<Adt145>) -> Adt157 {
    let m2825 = v2;
    match m2825 {
        None => {
            Adt157::C0(Adt158::C0(match v0.clone() {
                Adt123::C0(h2826, _, _) => {
                    h2826
                }
            }, match v0.clone() {
                Adt123::C0(_, h2827, _) => {
                    h2827
                }
            }, v1, f763(match v0 {
                Adt123::C0(_, _, h2828) => {
                    h2828
                }
            })))
        }
        Some(v3) => {
            Adt157::C1(v3)
        }
    }
}

pub fn f832(v0: Str, v1: List<Str>) -> Option<Adt145> {
    f889(f890(v0.clone(), v1.clone()), v0, v1)
}

pub fn f833(v0: Adt128, v1: Adt159, v2: Str, v3: Str, v4: Adt8) -> R<Option<Adt129>> {
    let m2830 = match v0.clone() {
        Adt128::C0(h2829, _, _, _, _) => {
            h2829
        }
    };
    match m2830 {
        None => {
            Ok(Some(Adt129::C0))
        }
        Some(v5) => {
            let a2831 = v5;
            let a2832 = v3;
            if equal(a2831, a2832) {
                let m2834 = match v0.clone() {
                    Adt128::C0(_, h2833, _, _, _) => {
                        h2833
                    }
                };
                match m2834 {
                    None => {
                        Ok(Some(Adt129::C1))
                    }
                    Some(v6) => {
                        let a2835 = v6;
                        let a2836 = v2;
                        if equal(a2835, a2836) {
                            f891(f892(match v0.clone() {
                                Adt128::C0(_, _, h2837, _, _) => {
                                    h2837
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt128::C0(_, _, _, _, h2838) => {
                                    h2838
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

pub fn f834(v0: Adt8) -> Adt166 {
    f893(f233(f836(), v0.clone()), f233(f837(), v0))
}

pub fn f835(v0: Adt8, v1: Adt115, v2: Adt115) -> Result<Adt20, Adt146> {
    let m2839 = v1;
    match m2839 {
        Adt115::C0 => {
            Err::<Adt20, Adt146>(Adt146::C2)
        }
        Adt115::C1(v3) => {
            let m2840 = v2;
            match m2840 {
                Adt115::C0 => {
                    Err::<Adt20, Adt146>(Adt146::C3)
                }
                Adt115::C1(v4) => {
                    f894(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f836() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f837() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f838(p2844: List<Adt9>) -> bool {
    let m2843 = p2844;
    {
        let mut m2843 = m2843;
        loop {
            let step = {
                let v0 = m2843;
                let m2841 = v0;
                match m2841.uncons() {
                    None => {
                        Err::<List<Adt9>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2842 = f283(v1);
                        match m2842 {
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
                Ok(next) => { m2843 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f839(p2848: List<Adt9>) -> bool {
    let m2847 = p2848;
    {
        let mut m2847 = m2847;
        loop {
            let step = {
                let v0 = m2847;
                let m2845 = v0;
                match m2845.uncons() {
                    None => {
                        Err::<List<Adt9>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2846 = f283(v1);
                        match m2846 {
                            None => {
                                Err::<List<Adt9>, bool>(false)
                            }
                            Some(v3) => {
                                if f604(v3) {
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
                Ok(next) => { m2847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f840(v0: Str) -> Option<Adt16> {
    let a2849 = v0.clone();
    let a2850 = Str::lit("string");
    if equal(a2849, a2850) {
        Some(Adt16::C0)
    } else {
        let a2851 = v0.clone();
        let a2852 = Str::lit("integer");
        if equal(a2851, a2852) {
            Some(Adt16::C1)
        } else {
            let a2853 = v0;
            let a2854 = Str::lit("boolean");
            if equal(a2853, a2854) {
                Some(Adt16::C2)
            } else {
                None::<Adt16>
            }
        }
    }
}

pub fn f841(p2863: List<u64>, p2864: List<u64>) -> R<List<u64>> {
    let m2862 = (p2863, p2864);
    {
        let mut m2862 = m2862;
        loop {
            let step = {
                let (v0, v1) = m2862;
                let m2855 = v0;
                match m2855.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2856 = 65u64;
                        let a2857 = v2;
                        if if nat_le(a2856, a2857) {
                            let a2858 = v2;
                            let a2859 = 90u64;
                            nat_le(a2858, a2859)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2860 = v2;
                                let a2861 = 32u64;
                                nat_add(a2860, a2861)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
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

pub fn f842(p2871: List<u64>, p2872: List<u64>) -> bool {
    let m2870 = (p2871, p2872);
    {
        let mut m2870 = m2870;
        loop {
            let step = {
                let (v0, v1) = m2870;
                let m2865 = v0;
                match m2865.uncons() {
                    None => {
                        let m2866 = v1;
                        match m2866.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2867 = v1;
                        match m2867.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2868 = v4;
                                let a2869 = v6;
                                if nat_eq(a2868, a2869) {
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
                Ok(next) => { m2870 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f843(v0: Adt9) -> u64 {
    let m2873 = v0;
    match m2873 {
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

pub fn f844(v0: Adt161) -> bool {
    let m2874 = v0;
    match m2874 {
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

pub fn f845(v0: Option<List<(Adt9, Adt9)>>, v1: List<(Adt9, Adt9)>) -> Adt165 {
    let m2875 = v0;
    match m2875 {
        None => {
            Adt165::C0
        }
        Some(v2) => {
            Adt165::C1({
                let a2876 = v2;
                let a2877 = v1;
                append_list(a2876, a2877)
            })
        }
    }
}

pub fn f846(p2882: List<Adt9>, p2883: List<Adt9>, p2884: List<(Adt9, Adt9)>) -> Option<List<(Adt9, Adt9)>> {
    let m2881 = (p2882, (p2883, p2884));
    {
        let mut m2881 = m2881;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2881;
                let m2878 = v0;
                match m2878.uncons() {
                    None => {
                        let m2879 = v1;
                        match m2879.uncons() {
                            None => {
                                Err::<(List<Adt9>, (List<Adt9>, List<(Adt9, Adt9)>)), Option<List<(Adt9, Adt9)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt9>, (List<Adt9>, List<(Adt9, Adt9)>)), Option<List<(Adt9, Adt9)>>>(None::<List<(Adt9, Adt9)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2880 = v1;
                        match m2880.uncons() {
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
                Ok(next) => { m2881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f847(p2890: List<(Str, Adt9)>, v1: List<(Str, Adt9)>, p2891: List<(Adt9, Adt9)>) -> Option<List<(Adt9, Adt9)>> {
    let m2889 = (p2890, p2891);
    {
        let mut m2889 = m2889;
        loop {
            let step = {
                let (v0, v2) = m2889;
                let m2885 = v0;
                match m2885.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Adt9, Adt9)>), Option<List<(Adt9, Adt9)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2887 = f305({
                            let (h2886, _) = v3;
                            h2886
                        }, v1.clone());
                        match m2887 {
                            Adt115::C0 => {
                                Err::<(List<(Str, Adt9)>, List<(Adt9, Adt9)>), Option<List<(Adt9, Adt9)>>>(None::<List<(Adt9, Adt9)>>)
                            }
                            Adt115::C1(v5) => {
                                Ok::<(List<(Str, Adt9)>, List<(Adt9, Adt9)>), Option<List<(Adt9, Adt9)>>>((v4, List::cons(({
                                    let (_, h2888) = v3;
                                    h2888
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2889 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f848(v0: Adt161) -> Adt161 {
    let m2892 = v0;
    match m2892 {
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

pub fn f849(v0: i64, v1: i64) -> R<i64> {
    f355(f285(f851(v0)?, 0i64)?, v1)
}

pub fn f850(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt161> {
    let a2893 = v1;
    let a2894 = v3;
    if nat_lt(a2893, a2894) {
        Ok(f700(f895({
            let a2895 = v3;
            let a2896 = v1;
            nat_sub(a2895, a2896)
        }, v0)?, v2))
    } else {
        let a2897 = v3;
        let a2898 = v1;
        if nat_lt(a2897, a2898) {
            Ok(f700(v0, f895({
                let a2899 = v1;
                let a2900 = v3;
                nat_sub(a2899, a2900)
            }, v2)?))
        } else {
            Ok(f700(v0, v2))
        }
    }
}

pub fn f851(v0: i64) -> R<u64> {
    f896(20u64, v0, 0u64)
}

pub fn f852(v0: i64) -> u64 {
    let m2903 = {
        let a2901 = v0;
        let a2902 = 1i64;
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
        48u64
    } else {
        let m2906 = {
            let a2904 = v0;
            let a2905 = 2i64;
            compare(a2904, a2905)
        };
        if match m2906 {
            Ordering::Less => {
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
            let m2909 = {
                let a2907 = v0;
                let a2908 = 3i64;
                compare(a2907, a2908)
            };
            if match m2909 {
                Ordering::Less => {
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
                let m2912 = {
                    let a2910 = v0;
                    let a2911 = 4i64;
                    compare(a2910, a2911)
                };
                if match m2912 {
                    Ordering::Less => {
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
                    let m2915 = {
                        let a2913 = v0;
                        let a2914 = 5i64;
                        compare(a2913, a2914)
                    };
                    if match m2915 {
                        Ordering::Less => {
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
                        let m2918 = {
                            let a2916 = v0;
                            let a2917 = 6i64;
                            compare(a2916, a2917)
                        };
                        if match m2918 {
                            Ordering::Less => {
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
                            let m2921 = {
                                let a2919 = v0;
                                let a2920 = 7i64;
                                compare(a2919, a2920)
                            };
                            if match m2921 {
                                Ordering::Less => {
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
                                let m2924 = {
                                    let a2922 = v0;
                                    let a2923 = 8i64;
                                    compare(a2922, a2923)
                                };
                                if match m2924 {
                                    Ordering::Less => {
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
                                    let m2927 = {
                                        let a2925 = v0;
                                        let a2926 = 9i64;
                                        compare(a2925, a2926)
                                    };
                                    if match m2927 {
                                        Ordering::Less => {
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

pub fn f853(v0: Adt11) -> Adt9 {
    Adt9::C5(f717(Str::lit("listChanged"), match v0 {
        Adt11::C0(h2928) => {
            h2928
        }
    }))
}

pub fn f854(v0: Adt39) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
        Adt39::C0(h2929, _, _, _, _, _, _, _) => {
            h2929
        }
    })), {
        let a2947 = f712(Str::lit("title"), match v0.clone() {
            Adt39::C0(_, h2930, _, _, _, _, _, _) => {
                h2930
            }
        });
        let a2948 = {
            let a2945 = f806(match v0.clone() {
                Adt39::C0(_, _, h2931, _, _, _, _, _) => {
                    h2931
                }
            });
            let a2946 = {
                let a2943 = f712(Str::lit("description"), match v0.clone() {
                    Adt39::C0(_, _, _, h2932, _, _, _, _) => {
                        h2932
                    }
                });
                let a2944 = List::cons((Str::lit("inputSchema"), Adt9::C5({
                    let m2935 = match v0.clone() {
                        Adt39::C0(_, _, _, _, h2934, _, _, _) => {
                            h2934
                        }
                    };
                    match m2935 {
                        Adt8::C0(h2933) => {
                            h2933
                        }
                    }
                })), {
                    let a2941 = f753(Str::lit("outputSchema"), match v0.clone() {
                        Adt39::C0(_, _, _, _, _, h2936, _, _) => {
                            h2936
                        }
                    });
                    let a2942 = {
                        let a2939 = f897(match v0.clone() {
                            Adt39::C0(_, _, _, _, _, _, h2937, _) => {
                                h2937
                            }
                        });
                        let a2940 = f898(match v0 {
                            Adt39::C0(_, _, _, _, _, _, _, h2938) => {
                                h2938
                            }
                        });
                        append_list(a2939, a2940)
                    };
                    append_list(a2941, a2942)
                });
                append_list(a2943, a2944)
            };
            append_list(a2945, a2946)
        };
        append_list(a2947, a2948)
    }))
}

pub fn f855(v0: Adt42) -> R<Option<Adt9>> {
    let m2949 = v0;
    match m2949 {
        Adt42::C0(v1) => {
            Ok(Some(f899(v1)))
        }
        Adt42::C1(v2) => {
            Ok(Some(f900(Str::lit("image"), match v2.clone() {
                Adt46::C0(h2950, _, _, _) => {
                    h2950
                }
            }, match v2.clone() {
                Adt46::C0(_, h2951, _, _) => {
                    h2951
                }
            }, match v2.clone() {
                Adt46::C0(_, _, h2952, _) => {
                    h2952
                }
            }, match v2 {
                Adt46::C0(_, _, _, h2953) => {
                    h2953
                }
            })))
        }
        Adt42::C2(v3) => {
            Ok(Some(f900(Str::lit("audio"), match v3.clone() {
                Adt47::C0(h2954, _, _, _) => {
                    h2954
                }
            }, match v3.clone() {
                Adt47::C0(_, h2955, _, _) => {
                    h2955
                }
            }, match v3.clone() {
                Adt47::C0(_, _, h2956, _) => {
                    h2956
                }
            }, match v3 {
                Adt47::C0(_, _, _, h2957) => {
                    h2957
                }
            })))
        }
        Adt42::C3(v4) => {
            Ok(Some(f901(v4)?))
        }
        Adt42::C4(v5) => {
            Ok(Some(f902(v5)))
        }
    }
}

pub fn f856(v0: Adt48) -> R<Adt9> {
    Ok(Adt9::C5({
        let a2981 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt48::C0(h2958, _, _, _, _, _, _, _, _) => {
                h2958
            }
        })), List::<(Str, Adt9)>::nil());
        let a2982 = {
            let a2979 = f712(Str::lit("title"), match v0.clone() {
                Adt48::C0(_, h2959, _, _, _, _, _, _, _) => {
                    h2959
                }
            });
            let a2980 = {
                let a2977 = f806(match v0.clone() {
                    Adt48::C0(_, _, h2960, _, _, _, _, _, _) => {
                        h2960
                    }
                });
                let a2978 = {
                    let a2975 = List::cons((Str::lit("uri"), Adt9::C3(match v0.clone() {
                        Adt48::C0(_, _, _, h2961, _, _, _, _, _) => {
                            h2961
                        }
                    })), List::<(Str, Adt9)>::nil());
                    let a2976 = {
                        let a2973 = f712(Str::lit("description"), match v0.clone() {
                            Adt48::C0(_, _, _, _, h2962, _, _, _, _) => {
                                h2962
                            }
                        });
                        let a2974 = {
                            let a2971 = f712(Str::lit("mimeType"), match v0.clone() {
                                Adt48::C0(_, _, _, _, _, h2963, _, _, _) => {
                                    h2963
                                }
                            });
                            let a2972 = {
                                let a2969 = f903(match v0.clone() {
                                    Adt48::C0(_, _, _, _, _, _, h2964, _, _) => {
                                        h2964
                                    }
                                });
                                let a2970 = {
                                    let a2967 = f904(match v0.clone() {
                                        Adt48::C0(_, _, _, _, _, _, _, h2965, _) => {
                                            h2965
                                        }
                                    })?;
                                    let a2968 = f898(match v0 {
                                        Adt48::C0(_, _, _, _, _, _, _, _, h2966) => {
                                            h2966
                                        }
                                    });
                                    append_list(a2967, a2968)
                                };
                                append_list(a2969, a2970)
                            };
                            append_list(a2971, a2972)
                        };
                        append_list(a2973, a2974)
                    };
                    append_list(a2975, a2976)
                };
                append_list(a2977, a2978)
            };
            append_list(a2979, a2980)
        };
        append_list(a2981, a2982)
    }))
}

pub fn f857(v0: Adt50) -> Adt9 {
    let m2983 = v0;
    match m2983 {
        Adt50::C0(v1) => {
            Adt9::C5(List::cons((Str::lit("uri"), Adt9::C3(match v1.clone() {
                Adt51::C0(h2984, _, _, _) => {
                    h2984
                }
            })), {
                let a2988 = f712(Str::lit("mimeType"), match v1.clone() {
                    Adt51::C0(_, h2985, _, _) => {
                        h2985
                    }
                });
                let a2989 = List::cons((Str::lit("text"), Adt9::C3(match v1.clone() {
                    Adt51::C0(_, _, h2986, _) => {
                        h2986
                    }
                })), f898(match v1 {
                    Adt51::C0(_, _, _, h2987) => {
                        h2987
                    }
                }));
                append_list(a2988, a2989)
            }))
        }
        Adt50::C1(v2) => {
            Adt9::C5(List::cons((Str::lit("uri"), Adt9::C3(match v2.clone() {
                Adt52::C0(h2990, _, _, _) => {
                    h2990
                }
            })), {
                let a2994 = f712(Str::lit("mimeType"), match v2.clone() {
                    Adt52::C0(_, h2991, _, _) => {
                        h2991
                    }
                });
                let a2995 = List::cons((Str::lit("blob"), Adt9::C3(match v2.clone() {
                    Adt52::C0(_, _, h2992, _) => {
                        h2992
                    }
                })), f898(match v2 {
                    Adt52::C0(_, _, _, h2993) => {
                        h2993
                    }
                }));
                append_list(a2994, a2995)
            }))
        }
    }
}

pub fn f858(v0: Adt58) -> Adt9 {
    Adt9::C5({
        let a3016 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt58::C0(h2996, _, _, _, _, _, _, _) => {
                h2996
            }
        })), List::<(Str, Adt9)>::nil());
        let a3017 = {
            let a3014 = f712(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h2997, _, _, _, _, _, _) => {
                    h2997
                }
            });
            let a3015 = {
                let a3012 = f806(match v0.clone() {
                    Adt58::C0(_, _, h2998, _, _, _, _, _) => {
                        h2998
                    }
                });
                let a3013 = {
                    let a3010 = List::cons((Str::lit("uriTemplate"), Adt9::C3(match v0.clone() {
                        Adt58::C0(_, _, _, h2999, _, _, _, _) => {
                            h2999
                        }
                    })), List::<(Str, Adt9)>::nil());
                    let a3011 = {
                        let a3008 = f712(Str::lit("description"), match v0.clone() {
                            Adt58::C0(_, _, _, _, h3000, _, _, _) => {
                                h3000
                            }
                        });
                        let a3009 = {
                            let a3006 = f712(Str::lit("mimeType"), match v0.clone() {
                                Adt58::C0(_, _, _, _, _, h3001, _, _) => {
                                    h3001
                                }
                            });
                            let a3007 = {
                                let a3004 = f903(match v0.clone() {
                                    Adt58::C0(_, _, _, _, _, _, h3002, _) => {
                                        h3002
                                    }
                                });
                                let a3005 = f898(match v0 {
                                    Adt58::C0(_, _, _, _, _, _, _, h3003) => {
                                        h3003
                                    }
                                });
                                append_list(a3004, a3005)
                            };
                            append_list(a3006, a3007)
                        };
                        append_list(a3008, a3009)
                    };
                    append_list(a3010, a3011)
                };
                append_list(a3012, a3013)
            };
            append_list(a3014, a3015)
        };
        append_list(a3016, a3017)
    })
}

pub fn f859(v0: Adt60) -> Adt9 {
    Adt9::C5({
        let a3033 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt60::C0(h3018, _, _, _, _, _) => {
                h3018
            }
        })), List::<(Str, Adt9)>::nil());
        let a3034 = {
            let a3031 = f712(Str::lit("title"), match v0.clone() {
                Adt60::C0(_, h3019, _, _, _, _) => {
                    h3019
                }
            });
            let a3032 = {
                let a3029 = f806(match v0.clone() {
                    Adt60::C0(_, _, h3020, _, _, _) => {
                        h3020
                    }
                });
                let a3030 = {
                    let a3027 = f712(Str::lit("description"), match v0.clone() {
                        Adt60::C0(_, _, _, h3021, _, _) => {
                            h3021
                        }
                    });
                    let a3028 = {
                        let a3025 = {
                            let m3023 = match v0.clone() {
                                Adt60::C0(_, _, _, _, h3022, _) => {
                                    h3022
                                }
                            };
                            match m3023 {
                                None => {
                                    List::<(Str, Adt9)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt9::C4(f905(v1))), List::<(Str, Adt9)>::nil())
                                }
                            }
                        };
                        let a3026 = f898(match v0 {
                            Adt60::C0(_, _, _, _, _, h3024) => {
                                h3024
                            }
                        });
                        append_list(a3025, a3026)
                    };
                    append_list(a3027, a3028)
                };
                append_list(a3029, a3030)
            };
            append_list(a3031, a3032)
        };
        append_list(a3033, a3034)
    })
}

pub fn f860(v0: Adt63) -> R<Option<Adt9>> {
    let m3036 = f855(match v0.clone() {
        Adt63::C0(_, h3035) => {
            h3035
        }
    })?;
    match m3036 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("role"), Adt9::C3(f906(match v0 {
                Adt63::C0(h3037, _) => {
                    h3037
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt9)>::nil())))))
        }
    }
}

pub fn f861(v0: Str, v1: Option<Adt9>) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("method"), Adt9::C3(v0)), f716(Str::lit("params"), v1)))
}

pub fn f862(v0: Adt69) -> Adt9 {
    let m3038 = v0;
    match m3038 {
        Adt69::C0(v1) => {
            f907(v1)
        }
        Adt69::C1(v2) => {
            Adt9::C5(List::cons((Str::lit("mode"), Adt9::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt9::C3(match v2.clone() {
                Adt72::C0(h3039, _) => {
                    h3039
                }
            })), List::cons((Str::lit("url"), Adt9::C3(match v2 {
                Adt72::C0(_, h3040) => {
                    h3040
                }
            })), List::<(Str, Adt9)>::nil()))))
        }
    }
}

pub fn f863(v0: Adt73) -> R<Option<Adt9>> {
    let m3042 = f908(match v0.clone() {
        Adt73::C0(h3041, _, _, _, _, _, _, _, _, _) => {
            h3041
        }
    })?;
    match m3042 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("messages"), Adt9::C4(v1)), {
                let a3044 = f909(v0.clone());
                let a3045 = List::cons((Str::lit("maxTokens"), f119(match v0.clone() {
                    Adt73::C0(_, _, _, _, _, h3043, _, _, _, _) => {
                        h3043
                    }
                })?), f910(v0));
                append_list(a3044, a3045)
            }))))
        }
    }
}

pub fn f864(p3051: List<(Str, Adt68)>, p3052: List<(Str, Adt9)>) -> Option<List<(Str, Adt9)>> {
    let m3050 = (p3051, p3052);
    {
        let mut m3050 = m3050;
        loop {
            let step = {
                let (v0, v1) = m3050;
                let m3046 = v0;
                match m3046.uncons() {
                    None => {
                        Err::<(List<(Str, Adt68)>, List<(Str, Adt9)>), Option<List<(Str, Adt9)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m3048 = f911({
                            let (_, h3047) = v2;
                            h3047
                        });
                        match m3048 {
                            None => {
                                Err::<(List<(Str, Adt68)>, List<(Str, Adt9)>), Option<List<(Str, Adt9)>>>(None::<List<(Str, Adt9)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt68)>, List<(Str, Adt9)>), Option<List<(Str, Adt9)>>>((v3, List::cons(({
                                    let (h3049, _) = v2;
                                    h3049
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3050 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f865(v0: i64) -> Adt9 {
    Adt9::C2(Adt10::C0(v0, 0i64))
}

pub fn f866(v0: List<Adt5>) -> List<Adt9> {
    f574(f912(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f867(v0: Str, v1: Adt8) -> Option<Adt8> {
    let m3053 = f233(v0, v1);
    match m3053 {
        Adt115::C0 => {
            None::<Adt8>
        }
        Adt115::C1(v2) => {
            f234(v2)
        }
    }
}

pub fn f868(v0: Adt8) -> Option<Adt66> {
    let m3054 = f913(v0.clone());
    match m3054 {
        None => {
            None::<Adt66>
        }
        Some(v1) => {
            let m3055 = f914(Str::lit("status"), v0.clone());
            match m3055 {
                None => {
                    None::<Adt66>
                }
                Some(v2) => {
                    let a3056 = v2.clone();
                    let a3057 = Str::lit("working");
                    if equal(a3056, a3057) {
                        Some(Adt66::C0(v1))
                    } else {
                        let a3058 = v2.clone();
                        let a3059 = Str::lit("cancelled");
                        if equal(a3058, a3059) {
                            Some(Adt66::C4(v1))
                        } else {
                            let a3060 = v2.clone();
                            let a3061 = Str::lit("input_required");
                            if equal(a3060, a3061) {
                                let m3062 = f915(Str::lit("inputRequests"), v0);
                                match m3062 {
                                    None => {
                                        None::<Adt66>
                                    }
                                    Some(v3) => {
                                        Some(Adt66::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a3063 = v2.clone();
                                let a3064 = Str::lit("completed");
                                if equal(a3063, a3064) {
                                    let m3065 = f867(Str::lit("result"), v0);
                                    match m3065 {
                                        None => {
                                            None::<Adt66>
                                        }
                                        Some(v4) => {
                                            Some(Adt66::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a3066 = v2;
                                    let a3067 = Str::lit("failed");
                                    if equal(a3066, a3067) {
                                        let m3068 = f867(Str::lit("error"), v0);
                                        match m3068 {
                                            None => {
                                                None::<Adt66>
                                            }
                                            Some(v5) => {
                                                let m3069 = f916(v5);
                                                match m3069 {
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

pub fn f869(v0: Str, v1: Adt8) -> Option<i64> {
    let m3070 = f233(v0, v1);
    match m3070 {
        Adt115::C0 => {
            None::<i64>
        }
        Adt115::C1(v2) => {
            f699(v2)
        }
    }
}

pub fn f870(v0: Adt8) -> Option<Adt89> {
    let m3071 = f869(Str::lit("parkAfterMs"), v0.clone());
    match m3071 {
        None => {
            None::<Adt89>
        }
        Some(v1) => {
            let m3072 = f915(Str::lit("parked"), v0.clone());
            match m3072 {
                None => {
                    None::<Adt89>
                }
                Some(v2) => {
                    let m3073 = f869(Str::lit("finishAfterMs"), v0.clone());
                    match m3073 {
                        None => {
                            None::<Adt89>
                        }
                        Some(v3) => {
                            let m3074 = f867(Str::lit("outcome"), v0);
                            match m3074 {
                                None => {
                                    None::<Adt89>
                                }
                                Some(v4) => {
                                    let m3075 = f917(v4);
                                    match m3075 {
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

pub fn f871(v0: Adt88) -> Option<Adt9> {
    let m3077 = f644(match v0.clone() {
        Adt88::C0(h3076, _, _) => {
            h3076
        }
    });
    match m3077 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            let m3079 = f918(match v0.clone() {
                Adt88::C0(_, _, h3078) => {
                    h3078
                }
            });
            match m3079 {
                None => {
                    None::<Adt9>
                }
                Some(v2) => {
                    Some(Adt9::C5(List::cons((Str::lit("task"), Adt9::C5(v1)), List::cons((Str::lit("createdMs"), f865(match v0 {
                        Adt88::C0(_, h3080, _) => {
                            h3080
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt9)>::nil())))))
                }
            }
        }
    }
}

pub fn f872(v0: List<u64>) -> List<u64> {
    let a3083 = f919();
    let a3084 = {
        let a3081 = v0;
        let a3082 = f920();
        append_list(a3081, a3082)
    };
    append_list(a3083, a3084)
}

pub fn f873(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f921(v0, 0u64, v1)
}

pub fn f874(p3091: List<u64>) -> bool {
    let m3090 = p3091;
    {
        let mut m3090 = m3090;
        loop {
            let step = {
                let v0 = m3090;
                let m3085 = v0;
                match m3085.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a3086 = v1;
                        let a3087 = 48u64;
                        if if nat_eq(a3086, a3087) {
                            true
                        } else {
                            let a3088 = v1;
                            let a3089 = 46u64;
                            nat_eq(a3088, a3089)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3090 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f875(v0: Adt153) -> bool {
    let m3092 = v0;
    match m3092 {
        Adt153::C0 => {
            true
        }
        Adt153::C1 => {
            false
        }
        Adt153::C2 => {
            false
        }
        Adt153::C3 => {
            false
        }
        Adt153::C4 => {
            false
        }
        Adt153::C5 => {
            false
        }
        Adt153::C6 => {
            false
        }
        Adt153::C7 => {
            false
        }
        Adt153::C8 => {
            false
        }
        Adt153::C9 => {
            false
        }
        Adt153::C10 => {
            false
        }
        Adt153::C11 => {
            false
        }
    }
}

pub fn f876(v0: Adt18) -> Adt153 {
    let m3093 = v0;
    match m3093 {
        Adt18::C0(_) => {
            Adt153::C0
        }
        Adt18::C1(_) => {
            Adt153::C1
        }
        Adt18::C2(_) => {
            Adt153::C2
        }
        Adt18::C3(_) => {
            Adt153::C3
        }
        Adt18::C4(_) => {
            Adt153::C4
        }
        Adt18::C5(_) => {
            Adt153::C5
        }
        Adt18::C6(_) => {
            Adt153::C6
        }
        Adt18::C7(_) => {
            Adt153::C7
        }
        Adt18::C8(_) => {
            Adt153::C8
        }
        Adt18::C9(_) => {
            Adt153::C9
        }
        Adt18::C10(_) => {
            Adt153::C10
        }
        Adt18::C11(_) => {
            Adt153::C11
        }
    }
}

pub fn f877(v0: Adt3, v1: Adt36) -> Adt142 {
    Adt142::C0(v1, Adt143::C0(Some(match v0 {
        Adt3::C0(h3094, _, _, _, _, _, _) => {
            h3094
        }
    }), None::<Adt8>), List::<Adt144>::nil())
}

pub fn f878(v0: Adt3) -> Adt37 {
    Adt37::C0(match v0.clone() {
        Adt3::C0(_, _, _, h3095, _, _, _) => {
            h3095
        }
    }, f443(), match v0.clone() {
        Adt3::C0(_, _, h3096, _, _, _, _) => {
            h3096
        }
    }, match v0 {
        Adt3::C0(_, h3097, _, _, _, _, _) => {
            h3097
        }
    })
}

pub fn f879(v0: Adt3, v1: Adt18, v2: Result<Adt36, Adt93>) -> Result<Adt142, Adt145> {
    let m3098 = v2;
    match m3098 {
        Ok(v4) => {
            if f922(f876(v1.clone()), v4.clone()) {
                f923(v0, v1, v4)
            } else {
                Err::<Adt142, Adt145>(Adt145::C4(f924()))
            }
        }
        Err(v3) => {
            Err::<Adt142, Adt145>(Adt145::C4(v3))
        }
    }
}

pub fn f880(v0: Bytes, v1: u64, v2: u64, v3: Fn3, v4: Adt18) -> R<Result<Adt36, Adt93>> {
    f925(v0.clone(), v1, v2, v3, v4.clone(), f926(v0, v4)?)
}

pub fn f881(v0: Str, v1: Adt8) -> Result<Option<Str>, Adt146> {
    let m3099 = f233(v0.clone(), v1);
    match m3099 {
        Adt115::C0 => {
            Ok::<Option<Str>, Adt146>(None::<Str>)
        }
        Adt115::C1(v2) => {
            let m3100 = f283(v2);
            match m3100 {
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

pub fn f882(v0: Str, v1: Adt8) -> Result<Str, Adt146> {
    let m3101 = f233(v0.clone(), v1);
    match m3101 {
        Adt115::C0 => {
            Err::<Str, Adt146>(Adt146::C7(v0))
        }
        Adt115::C1(v2) => {
            let m3102 = f283(v2);
            match m3102 {
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

pub fn f883(v0: Str, v1: Adt8) -> Result<Option<Adt8>, Adt146> {
    let m3103 = f233(v0.clone(), v1);
    match m3103 {
        Adt115::C0 => {
            Ok::<Option<Adt8>, Adt146>(None::<Adt8>)
        }
        Adt115::C1(v2) => {
            let m3104 = f234(v2);
            match m3104 {
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

pub fn f884(v0: Str, v1: Adt8) -> Result<Option<List<(Str, Str)>>, Adt146> {
    let m3105 = f233(v0.clone(), v1);
    match m3105 {
        Adt115::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt146>(None::<List<(Str, Str)>>)
        }
        Adt115::C1(v2) => {
            let m3106 = f234(v2);
            match m3106 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt146>(Adt146::C8(v0))
                }
                Some(v3) => {
                    let m3108 = f927(v0, match v3 {
                        Adt8::C0(h3107) => {
                            h3107
                        }
                    });
                    match m3108 {
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

pub fn f885(v0: Str, v1: Adt8) -> Result<Adt8, Adt146> {
    let m3109 = f233(v0.clone(), v1);
    match m3109 {
        Adt115::C0 => {
            Err::<Adt8, Adt146>(Adt146::C7(v0))
        }
        Adt115::C1(v2) => {
            let m3110 = f234(v2);
            match m3110 {
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

pub fn f886(v0: Adt8) -> Result<Adt29, Adt146> {
    let m3111 = f882(Str::lit("type"), v0.clone());
    match m3111 {
        Ok(v1) => {
            let a3112 = v1.clone();
            let a3113 = Str::lit("ref/prompt");
            if equal(a3112, a3113) {
                let m3114 = f882(Str::lit("name"), v0.clone());
                match m3114 {
                    Ok(v2) => {
                        let m3115 = f881(Str::lit("title"), v0);
                        match m3115 {
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
                let a3116 = v1;
                let a3117 = Str::lit("ref/resource");
                if equal(a3116, a3117) {
                    let m3118 = f882(Str::lit("uri"), v0);
                    match m3118 {
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

pub fn f887(v0: Adt8) -> Result<Adt31, Adt146> {
    let m3119 = f882(Str::lit("name"), v0.clone());
    match m3119 {
        Ok(v1) => {
            let m3120 = f882(Str::lit("value"), v0);
            match m3120 {
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

pub fn f888(v0: Adt8) -> Result<Option<Adt32>, Adt146> {
    let m3121 = f233(Str::lit("context"), v0);
    match m3121 {
        Adt115::C0 => {
            Ok::<Option<Adt32>, Adt146>(None::<Adt32>)
        }
        Adt115::C1(v1) => {
            let m3122 = f234(v1);
            match m3122 {
                None => {
                    Err::<Option<Adt32>, Adt146>(Adt146::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m3123 = f884(Str::lit("arguments"), v2);
                    match m3123 {
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

pub fn f889(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt145> {
    if v0 {
        None::<Adt145>
    } else {
        Some(Adt145::C1(v1, v2))
    }
}

pub fn f890(v0: Str, p3128: List<Str>) -> bool {
    let m3127 = p3128;
    {
        let mut m3127 = m3127;
        loop {
            let step = {
                let v1 = m3127;
                let m3124 = v1;
                match m3124.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3125 = v2;
                        let a3126 = v0.clone();
                        if equal(a3125, a3126) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f891(v0: Option<Adt129>, v1: List<Adt131>, v2: Adt8) -> R<Option<Adt129>> {
    let m3129 = v0;
    match m3129 {
        None => {
            f928(v1, f929(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f892(v0: Option<Str>, v1: Adt159, v2: Adt8) -> Option<Adt129> {
    let m3130 = v1.clone();
    match m3130 {
        Adt159::C0 => {
            None::<Adt129>
        }
        Adt159::C1 => {
            f930(v0, f931(v1, v2))
        }
        Adt159::C2 => {
            f930(v0, f931(v1, v2))
        }
        Adt159::C3 => {
            f930(v0, f931(v1, v2))
        }
    }
}

pub fn f893(v0: Adt115, v1: Adt115) -> Adt166 {
    let m3131 = v0;
    match m3131 {
        Adt115::C0 => {
            Adt166::C1
        }
        Adt115::C1(_) => {
            let m3132 = v1;
            match m3132 {
                Adt115::C0 => {
                    Adt166::C2
                }
                Adt115::C1(_) => {
                    Adt166::C0
                }
            }
        }
    }
}

pub fn f894(v0: Adt8, v1: Adt9, v2: Adt9) -> Result<Adt20, Adt146> {
    let m3133 = f283(v1);
    match m3133 {
        None => {
            Err::<Adt20, Adt146>(Adt146::C5)
        }
        Some(v3) => {
            let m3134 = f932(v2);
            match m3134 {
                None => {
                    Err::<Adt20, Adt146>(Adt146::C6)
                }
                Some(v4) => {
                    let m3135 = f883(f933(), v0.clone());
                    match m3135 {
                        Ok(v6) => {
                            let m3136 = f934(v0.clone());
                            match m3136 {
                                Ok(v8) => {
                                    let m3137 = f935(v0.clone());
                                    match m3137 {
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

pub fn f895(p3142: u64, p3143: i64) -> R<i64> {
    let m3141 = (p3142, p3143);
    {
        let mut m3141 = m3141;
        loop {
            let step = {
                let (v0, v1) = m3141;
                let m3138 = v0;
                if m3138 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m3138 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a3139 = v1;
                        let a3140 = 10i64;
                        int_mul(a3139, a3140)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f896(p3154: u64, p3155: i64, p3156: u64) -> R<u64> {
    let m3153 = (p3154, (p3155, p3156));
    {
        let mut m3153 = m3153;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3153;
                let m3144 = v0;
                if m3144 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m3144 - 1;
                    let v4: u64 = {
                        let a3145 = v2;
                        let a3146 = 1u64;
                        nat_add(a3145, a3146)?
                    };
                    let m3149 = {
                        let a3147 = v1;
                        let a3148 = 10i64;
                        compare(a3147, a3148)
                    };
                    if match m3149 {
                        Ordering::Less => {
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
                            let a3150 = v1;
                            let a3151 = 10i64;
                            let a3152 = 0i64;
                            int_quot(a3150, a3151, a3152)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m3153 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f897(v0: Option<Adt40>) -> List<(Str, Adt9)> {
    let m3157 = v0;
    match m3157 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt9::C5({
                let a3169 = f712(Str::lit("title"), match v1.clone() {
                    Adt40::C0(h3158, _, _, _, _) => {
                        h3158
                    }
                });
                let a3170 = {
                    let a3167 = f717(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt40::C0(_, h3159, _, _, _) => {
                            h3159
                        }
                    });
                    let a3168 = {
                        let a3165 = f717(Str::lit("destructiveHint"), match v1.clone() {
                            Adt40::C0(_, _, h3160, _, _) => {
                                h3160
                            }
                        });
                        let a3166 = {
                            let a3163 = f717(Str::lit("idempotentHint"), match v1.clone() {
                                Adt40::C0(_, _, _, h3161, _) => {
                                    h3161
                                }
                            });
                            let a3164 = f717(Str::lit("openWorldHint"), match v1 {
                                Adt40::C0(_, _, _, _, h3162) => {
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
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f898(v0: Option<Adt8>) -> List<(Str, Adt9)> {
    f753(Str::lit("_meta"), v0)
}

pub fn f899(v0: Adt43) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt9::C3(match v0.clone() {
        Adt43::C0(h3171, _, _) => {
            h3171
        }
    })), {
        let a3174 = f903(match v0.clone() {
            Adt43::C0(_, h3172, _) => {
                h3172
            }
        });
        let a3175 = f898(match v0 {
            Adt43::C0(_, _, h3173) => {
                h3173
            }
        });
        append_list(a3174, a3175)
    })))
}

pub fn f900(v0: Str, v1: Str, v2: Str, v3: Option<Adt44>, v4: Option<Adt8>) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v0)), List::cons((Str::lit("data"), Adt9::C3(v1)), List::cons((Str::lit("mimeType"), Adt9::C3(v2)), {
        let a3176 = f903(v3);
        let a3177 = f898(v4);
        append_list(a3176, a3177)
    }))))
}

pub fn f901(v0: Adt48) -> R<Adt9> {
    Ok(Adt9::C5({
        let a3201 = List::cons((Str::lit("type"), Adt9::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt48::C0(h3178, _, _, _, _, _, _, _, _) => {
                h3178
            }
        })), List::<(Str, Adt9)>::nil()));
        let a3202 = {
            let a3199 = f712(Str::lit("title"), match v0.clone() {
                Adt48::C0(_, h3179, _, _, _, _, _, _, _) => {
                    h3179
                }
            });
            let a3200 = {
                let a3197 = f806(match v0.clone() {
                    Adt48::C0(_, _, h3180, _, _, _, _, _, _) => {
                        h3180
                    }
                });
                let a3198 = {
                    let a3195 = List::cons((Str::lit("uri"), Adt9::C3(match v0.clone() {
                        Adt48::C0(_, _, _, h3181, _, _, _, _, _) => {
                            h3181
                        }
                    })), List::<(Str, Adt9)>::nil());
                    let a3196 = {
                        let a3193 = f712(Str::lit("description"), match v0.clone() {
                            Adt48::C0(_, _, _, _, h3182, _, _, _, _) => {
                                h3182
                            }
                        });
                        let a3194 = {
                            let a3191 = f712(Str::lit("mimeType"), match v0.clone() {
                                Adt48::C0(_, _, _, _, _, h3183, _, _, _) => {
                                    h3183
                                }
                            });
                            let a3192 = {
                                let a3189 = f903(match v0.clone() {
                                    Adt48::C0(_, _, _, _, _, _, h3184, _, _) => {
                                        h3184
                                    }
                                });
                                let a3190 = {
                                    let a3187 = f904(match v0.clone() {
                                        Adt48::C0(_, _, _, _, _, _, _, h3185, _) => {
                                            h3185
                                        }
                                    })?;
                                    let a3188 = f898(match v0 {
                                        Adt48::C0(_, _, _, _, _, _, _, _, h3186) => {
                                            h3186
                                        }
                                    });
                                    append_list(a3187, a3188)
                                };
                                append_list(a3189, a3190)
                            };
                            append_list(a3191, a3192)
                        };
                        append_list(a3193, a3194)
                    };
                    append_list(a3195, a3196)
                };
                append_list(a3197, a3198)
            };
            append_list(a3199, a3200)
        };
        append_list(a3201, a3202)
    }))
}

pub fn f902(v0: Adt49) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f857(match v0.clone() {
        Adt49::C0(h3203, _, _) => {
            h3203
        }
    })), {
        let a3206 = f903(match v0.clone() {
            Adt49::C0(_, h3204, _) => {
                h3204
            }
        });
        let a3207 = f898(match v0 {
            Adt49::C0(_, _, h3205) => {
                h3205
            }
        });
        append_list(a3206, a3207)
    })))
}

pub fn f903(v0: Option<Adt44>) -> List<(Str, Adt9)> {
    let m3208 = v0;
    match m3208 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt9::C5({
                let a3214 = f936(match v1.clone() {
                    Adt44::C0(h3209, _, _) => {
                        h3209
                    }
                });
                let a3215 = {
                    let a3212 = f937(match v1.clone() {
                        Adt44::C0(_, h3210, _) => {
                            h3210
                        }
                    });
                    let a3213 = f712(Str::lit("lastModified"), match v1 {
                        Adt44::C0(_, _, h3211) => {
                            h3211
                        }
                    });
                    append_list(a3212, a3213)
                };
                append_list(a3214, a3215)
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f904(v0: Option<u64>) -> R<List<(Str, Adt9)>> {
    let m3216 = v0;
    match m3216 {
        None => {
            Ok(List::<(Str, Adt9)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f119(v1)?), List::<(Str, Adt9)>::nil()))
        }
    }
}

pub fn f905(v0: List<Adt61>) -> List<Adt9> {
    f574(f938(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f906(v0: Adt45) -> Str {
    let m3217 = v0;
    match m3217 {
        Adt45::C0 => {
            Str::lit("user")
        }
        Adt45::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f907(v0: Adt70) -> Adt9 {
    Adt9::C5({
        let a3224 = {
            let m3219 = match v0.clone() {
                Adt70::C0(h3218, _, _) => {
                    h3218
                }
            };
            match m3219 {
                None => {
                    List::<(Str, Adt9)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt9::C3(Str::lit("form"))), List::<(Str, Adt9)>::nil())
                }
            }
        };
        let a3225 = List::cons((Str::lit("message"), Adt9::C3(match v0.clone() {
            Adt70::C0(_, h3220, _) => {
                h3220
            }
        })), List::cons((Str::lit("requestedSchema"), Adt9::C5({
            let m3223 = match v0 {
                Adt70::C0(_, _, h3222) => {
                    h3222
                }
            };
            match m3223 {
                Adt8::C0(h3221) => {
                    h3221
                }
            }
        })), List::<(Str, Adt9)>::nil()));
        append_list(a3224, a3225)
    })
}

pub fn f908(v0: List<Adt74>) -> R<Option<List<Adt9>>> {
    let m3226 = f939(v0, List::<Adt9>::nil())?;
    match m3226 {
        None => {
            Ok(None::<List<Adt9>>)
        }
        Some(v1) => {
            Ok(Some(f574(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f909(v0: Adt73) -> List<(Str, Adt9)> {
    let a3237 = {
        let m3228 = match v0.clone() {
            Adt73::C0(_, h3227, _, _, _, _, _, _, _, _) => {
                h3227
            }
        };
        match m3228 {
            None => {
                List::<(Str, Adt9)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f940(v1)), List::<(Str, Adt9)>::nil())
            }
        }
    };
    let a3238 = {
        let a3235 = f712(Str::lit("systemPrompt"), match v0.clone() {
            Adt73::C0(_, _, h3229, _, _, _, _, _, _, _) => {
                h3229
            }
        });
        let a3236 = {
            let a3233 = {
                let m3231 = match v0.clone() {
                    Adt73::C0(_, _, _, h3230, _, _, _, _, _, _) => {
                        h3230
                    }
                };
                match m3231 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt9::C3(f941(v2))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            let a3234 = f942(Str::lit("temperature"), match v0 {
                Adt73::C0(_, _, _, _, h3232, _, _, _, _, _) => {
                    h3232
                }
            });
            append_list(a3233, a3234)
        };
        append_list(a3235, a3236)
    };
    append_list(a3237, a3238)
}

pub fn f910(v0: Adt73) -> List<(Str, Adt9)> {
    let a3250 = {
        let m3240 = match v0.clone() {
            Adt73::C0(_, _, _, _, _, _, h3239, _, _, _) => {
                h3239
            }
        };
        match m3240 {
            None => {
                List::<(Str, Adt9)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f710(v1)), List::<(Str, Adt9)>::nil())
            }
        }
    };
    let a3251 = {
        let a3248 = f753(Str::lit("metadata"), match v0.clone() {
            Adt73::C0(_, _, _, _, _, _, _, h3241, _, _) => {
                h3241
            }
        });
        let a3249 = {
            let a3246 = {
                let m3243 = match v0.clone() {
                    Adt73::C0(_, _, _, _, _, _, _, _, h3242, _) => {
                        h3242
                    }
                };
                match m3243 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt9::C4(f714(v2))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            let a3247 = {
                let m3245 = match v0 {
                    Adt73::C0(_, _, _, _, _, _, _, _, _, h3244) => {
                        h3244
                    }
                };
                match m3245 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt9::C5(f943(v3))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            append_list(a3246, a3247)
        };
        append_list(a3248, a3249)
    };
    append_list(a3250, a3251)
}

pub fn f911(v0: Adt68) -> Option<Adt9> {
    let m3252 = v0;
    match m3252 {
        Adt68::C0(v1) => {
            Some(Adt9::C5(List::cons((Str::lit("method"), Adt9::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt9::C5(f944(v1))), List::<(Str, Adt9)>::nil()))))
        }
        Adt68::C1(_) => {
            None::<Adt9>
        }
        Adt68::C2(v3) => {
            Some(Adt9::C5(List::cons((Str::lit("method"), Adt9::C3(Str::lit("roots/list"))), {
                let m3253 = v3;
                match m3253 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt9::C5(match v4 {
                            Adt8::C0(h3254) => {
                                h3254
                            }
                        })), List::<(Str, Adt9)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f912(p3257: List<Adt5>, p3258: List<Adt9>) -> List<Adt9> {
    let m3256 = (p3257, p3258);
    {
        let mut m3256 = m3256;
        loop {
            let step = {
                let (v0, v1) = m3256;
                let m3255 = v0;
                match m3255.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt5>, List<Adt9>), List<Adt9>>((v3, List::cons(f945(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3256 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f913(v0: Adt8) -> Option<Adt67> {
    let m3259 = f914(Str::lit("taskId"), v0.clone());
    match m3259 {
        None => {
            None::<Adt67>
        }
        Some(v1) => {
            let m3260 = f914(Str::lit("createdAt"), v0.clone());
            match m3260 {
                None => {
                    None::<Adt67>
                }
                Some(v2) => {
                    let m3261 = f914(Str::lit("lastUpdatedAt"), v0.clone());
                    match m3261 {
                        None => {
                            None::<Adt67>
                        }
                        Some(v3) => {
                            Some(Adt67::C0(v1, f914(Str::lit("statusMessage"), v0.clone()), v2, v3, f869(Str::lit("ttlMs"), v0.clone()), f869(Str::lit("pollIntervalMs"), v0)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f914(v0: Str, v1: Adt8) -> Option<Str> {
    let m3262 = f233(v0, v1);
    match m3262 {
        Adt115::C0 => {
            None::<Str>
        }
        Adt115::C1(v2) => {
            f283(v2)
        }
    }
}

pub fn f915(v0: Str, v1: Adt8) -> Option<List<(Str, Adt68)>> {
    let m3263 = f867(v0, v1);
    match m3263 {
        None => {
            None::<List<(Str, Adt68)>>
        }
        Some(v2) => {
            f946(match v2 {
                Adt8::C0(h3264) => {
                    h3264
                }
            }, List::<(Str, Adt68)>::nil())
        }
    }
}

pub fn f916(v0: Adt8) -> Option<Adt84> {
    let m3265 = f869(Str::lit("code"), v0.clone());
    match m3265 {
        None => {
            None::<Adt84>
        }
        Some(v1) => {
            let m3266 = f914(Str::lit("message"), v0.clone());
            match m3266 {
                None => {
                    None::<Adt84>
                }
                Some(v2) => {
                    Some(Adt84::C0(v1, v2, {
                        let m3267 = f233(Str::lit("data"), v0);
                        match m3267 {
                            Adt115::C0 => {
                                None::<Adt9>
                            }
                            Adt115::C1(v3) => {
                                Some(v3)
                            }
                        }
                    }))
                }
            }
        }
    }
}

pub fn f917(v0: Adt8) -> Option<Adt90> {
    let m3268 = f867(Str::lit("completes"), v0.clone());
    match m3268 {
        None => {
            let m3269 = f867(Str::lit("fails"), v0);
            match m3269 {
                None => {
                    None::<Adt90>
                }
                Some(v2) => {
                    let m3270 = f916(v2);
                    match m3270 {
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

pub fn f918(v0: Adt89) -> Option<Adt9> {
    let m3272 = f802(match v0.clone() {
        Adt89::C0(_, h3271, _, _) => {
            h3271
        }
    });
    match m3272 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            Some(Adt9::C5(List::cons((Str::lit("parkAfterMs"), f865(match v0.clone() {
                Adt89::C0(h3273, _, _, _) => {
                    h3273
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f865(match v0.clone() {
                Adt89::C0(_, _, h3274, _) => {
                    h3274
                }
            })), List::cons((Str::lit("outcome"), f947(match v0 {
                Adt89::C0(_, _, _, h3275) => {
                    h3275
                }
            })), List::<(Str, Adt9)>::nil()))))))
        }
    }
}

pub fn f919() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f920() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f921(p3280: List<u64>, p3281: u64, p3282: List<u64>) -> List<u64> {
    let m3279 = (p3280, (p3281, p3282));
    {
        let mut m3279 = m3279;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3279;
                let m3276 = v0;
                match m3276.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f948(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt167 = f949(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt167::C0(h3277, _) => {
                                h3277
                            }
                        }, match v5 {
                            Adt167::C0(_, h3278) => {
                                h3278
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m3279 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f922(v0: Adt153, v1: Adt36) -> bool {
    let m3283 = v1;
    match m3283 {
        Adt36::C0(_) => {
            f950(v0, Adt153::C0)
        }
        Adt36::C1(_) => {
            f950(v0, Adt153::C1)
        }
        Adt36::C2(_) => {
            f950(v0, Adt153::C2)
        }
        Adt36::C3(_) => {
            f950(v0, Adt153::C2)
        }
        Adt36::C4(_) => {
            f950(v0, Adt153::C3)
        }
        Adt36::C5(_) => {
            f950(v0, Adt153::C4)
        }
        Adt36::C6(_) => {
            f950(v0, Adt153::C5)
        }
        Adt36::C7(_) => {
            f950(v0, Adt153::C6)
        }
        Adt36::C8(_) => {
            f950(v0, Adt153::C7)
        }
        Adt36::C9(_) => {
            f950(v0, Adt153::C8)
        }
        Adt36::C10(_) => {
            f950(v0, Adt153::C9)
        }
        Adt36::C11 => {
            if f950(v0.clone(), Adt153::C10) {
                true
            } else {
                f950(v0, Adt153::C11)
            }
        }
        Adt36::C12(_) => {
            f951(v0)
        }
        Adt36::C13(_) => {
            f952(v0)
        }
        Adt36::C14(v15) => {
            let m3285 = match v15 {
                Adt91::C0(_, h3284, _) => {
                    h3284
                }
            };
            match m3285 {
                Adt92::C0 => {
                    f950(v0, Adt153::C9)
                }
                Adt92::C1 => {
                    if f950(v0.clone(), Adt153::C10) {
                        true
                    } else {
                        f950(v0, Adt153::C11)
                    }
                }
            }
        }
    }
}

pub fn f923(v0: Adt3, v1: Adt18, v2: Adt36) -> Result<Adt142, Adt145> {
    let m3286 = v2.clone();
    match m3286 {
        Adt36::C0(_) => {
            Ok::<Adt142, Adt145>(f877(v0, v2))
        }
        Adt36::C1(_) => {
            Ok::<Adt142, Adt145>(f877(v0, v2))
        }
        Adt36::C2(_) => {
            Ok::<Adt142, Adt145>(f877(v0, v2))
        }
        Adt36::C3(v9) => {
            Ok::<Adt142, Adt145>(f957(v0, f954(v1), v9))
        }
        Adt36::C4(_) => {
            Ok::<Adt142, Adt145>(f877(v0, v2))
        }
        Adt36::C5(_) => {
            Ok::<Adt142, Adt145>(f877(v0, v2))
        }
        Adt36::C6(_) => {
            Ok::<Adt142, Adt145>(f877(v0, v2))
        }
        Adt36::C7(_) => {
            Ok::<Adt142, Adt145>(f877(v0, v2))
        }
        Adt36::C8(_) => {
            Ok::<Adt142, Adt145>(f877(v0, v2))
        }
        Adt36::C9(_) => {
            Ok::<Adt142, Adt145>(f877(v0, v2))
        }
        Adt36::C10(_) => {
            Ok::<Adt142, Adt145>(f877(v0, v2))
        }
        Adt36::C11 => {
            Ok::<Adt142, Adt145>(f877(v0, v2))
        }
        Adt36::C12(v3) => {
            f953(v0, f954(v1), v3)
        }
        Adt36::C13(v4) => {
            f955(v0, f954(v1), v4)
        }
        Adt36::C14(v5) => {
            f956(v0, f954(v1), v5)
        }
    }
}

pub fn f924() -> Adt93 {
    Adt93::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f925(v0: Bytes, v1: u64, v2: u64, v3: Fn3, v4: Adt18, v5: Result<Adt18, Adt93>) -> R<Result<Adt36, Adt93>> {
    let m3287 = v5;
    match m3287 {
        Ok(v7) => {
            f958(v0, v4, f959(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt36, Adt93>(v6))
        }
    }
}

pub fn f926(v0: Bytes, v1: Adt18) -> R<Result<Adt18, Adt93>> {
    let m3288 = v1.clone();
    match m3288 {
        Adt18::C0(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C1(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C2(v2) => {
            f960(v0, v2)
        }
        Adt18::C3(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C4(v3) => {
            f961(v0, v3)
        }
        Adt18::C5(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C6(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C7(v4) => {
            f962(v0, v4)
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

pub fn f927(v0: Str, v1: List<(Str, Adt9)>) -> Result<List<(Str, Str)>, Adt146> {
    let m3289 = f963(v0, v1, List::<(Str, Str)>::nil());
    match m3289 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt146>(f964(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt146>(v3)
        }
    }
}

pub fn f928(p3293: List<Adt131>, v1: Adt8) -> R<Option<Adt129>> {
    let m3292 = p3293;
    {
        let mut m3292 = m3292;
        loop {
            let step = {
                let v0 = m3292;
                let m3290 = v0;
                match m3290.uncons() {
                    None => {
                        Err::<List<Adt131>, R<Option<Adt129>>>(Ok(None::<Adt129>))
                    }
                    Some((v2, v3)) => {
                        let m3291 = f965(v2, v1.clone())?;
                        match m3291 {
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
                Ok(next) => { m3292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f929(v0: Adt8) -> Adt8 {
    let m3294 = f233(Str::lit("arguments"), v0);
    match m3294 {
        Adt115::C0 => {
            f62()
        }
        Adt115::C1(v1) => {
            let m3295 = f234(v1);
            match m3295 {
                None => {
                    f62()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f930(v0: Option<Str>, v1: Option<Str>) -> Option<Adt129> {
    let m3296 = v0;
    match m3296 {
        None => {
            Some(Adt129::C2)
        }
        Some(v2) => {
            let m3297 = v1;
            match m3297 {
                None => {
                    None::<Adt129>
                }
                Some(v3) => {
                    let a3298 = v2;
                    let a3299 = v3;
                    if equal(a3298, a3299) {
                        None::<Adt129>
                    } else {
                        Some(Adt129::C5)
                    }
                }
            }
        }
    }
}

pub fn f931(v0: Adt159, v1: Adt8) -> Option<Str> {
    let m3300 = v0;
    match m3300 {
        Adt159::C0 => {
            None::<Str>
        }
        Adt159::C1 => {
            let m3301 = f233(Str::lit("name"), v1);
            match m3301 {
                Adt115::C0 => {
                    None::<Str>
                }
                Adt115::C1(v2) => {
                    f283(v2)
                }
            }
        }
        Adt159::C2 => {
            let m3302 = f233(Str::lit("uri"), v1);
            match m3302 {
                Adt115::C0 => {
                    None::<Str>
                }
                Adt115::C1(v3) => {
                    f283(v3)
                }
            }
        }
        Adt159::C3 => {
            let m3303 = f233(Str::lit("taskId"), v1);
            match m3303 {
                Adt115::C0 => {
                    None::<Str>
                }
                Adt115::C1(v4) => {
                    f283(v4)
                }
            }
        }
    }
}

pub fn f932(v0: Adt9) -> Option<Adt21> {
    let m3304 = f234(v0);
    match m3304 {
        None => {
            None::<Adt21>
        }
        Some(v1) => {
            let m3305 = f966(Str::lit("experimental"), v1.clone());
            match m3305 {
                None => {
                    None::<Adt21>
                }
                Some(v2) => {
                    let m3306 = f966(Str::lit("roots"), v1.clone());
                    match m3306 {
                        None => {
                            None::<Adt21>
                        }
                        Some(v3) => {
                            let m3307 = f966(Str::lit("sampling"), v1.clone());
                            match m3307 {
                                None => {
                                    None::<Adt21>
                                }
                                Some(v4) => {
                                    let m3308 = f966(Str::lit("elicitation"), v1.clone());
                                    match m3308 {
                                        None => {
                                            None::<Adt21>
                                        }
                                        Some(v5) => {
                                            let m3309 = f966(Str::lit("extensions"), v1);
                                            match m3309 {
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

pub fn f933() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f934(v0: Adt8) -> Result<Option<Adt22>, Adt146> {
    let m3310 = f881(f967(), v0);
    match m3310 {
        Ok(v2) => {
            let m3311 = v2;
            match m3311 {
                None => {
                    Ok::<Option<Adt22>, Adt146>(None::<Adt22>)
                }
                Some(v3) => {
                    let m3312 = f968(v3);
                    match m3312 {
                        Adt168::C0 => {
                            Err::<Option<Adt22>, Adt146>(Adt146::C8(f967()))
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

pub fn f935(v0: Adt8) -> Result<Option<Adt23>, Adt146> {
    let m3313 = f233(f969(), v0);
    match m3313 {
        Adt115::C0 => {
            Ok::<Option<Adt23>, Adt146>(None::<Adt23>)
        }
        Adt115::C1(v1) => {
            let m3314 = f283(v1.clone());
            match m3314 {
                None => {
                    let m3315 = f699(v1);
                    match m3315 {
                        None => {
                            Err::<Option<Adt23>, Adt146>(Adt146::C8(f969()))
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

pub fn f936(v0: Option<List<Adt45>>) -> List<(Str, Adt9)> {
    let m3316 = v0;
    match m3316 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt9::C4(f970(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f937(v0: Option<Adt10>) -> List<(Str, Adt9)> {
    let m3317 = v0;
    match m3317 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt9::C2(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f938(p3320: List<Adt61>, p3321: List<Adt9>) -> List<Adt9> {
    let m3319 = (p3320, p3321);
    {
        let mut m3319 = m3319;
        loop {
            let step = {
                let (v0, v1) = m3319;
                let m3318 = v0;
                match m3318.uncons() {
                    None => {
                        Err::<(List<Adt61>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt61>, List<Adt9>), List<Adt9>>((v3, List::cons(f971(v2), v1)))
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

pub fn f939(p3325: List<Adt74>, p3326: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m3324 = (p3325, p3326);
    {
        let mut m3324 = m3324;
        loop {
            let step = {
                let (v0, v1) = m3324;
                let m3322 = v0;
                match m3322.uncons() {
                    None => {
                        Err::<(List<Adt74>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3323 = f972(v2)?;
                        match m3323 {
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
                Ok(next) => { m3324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f940(v0: Adt79) -> Adt9 {
    Adt9::C5({
        let a3336 = {
            let m3328 = match v0.clone() {
                Adt79::C0(h3327, _, _, _) => {
                    h3327
                }
            };
            match m3328 {
                None => {
                    List::<(Str, Adt9)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt9::C4(f574(f973(v1, List::<Adt9>::nil()), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())
                }
            }
        };
        let a3337 = {
            let a3334 = f942(Str::lit("costPriority"), match v0.clone() {
                Adt79::C0(_, h3329, _, _) => {
                    h3329
                }
            });
            let a3335 = {
                let a3332 = f942(Str::lit("speedPriority"), match v0.clone() {
                    Adt79::C0(_, _, h3330, _) => {
                        h3330
                    }
                });
                let a3333 = f942(Str::lit("intelligencePriority"), match v0 {
                    Adt79::C0(_, _, _, h3331) => {
                        h3331
                    }
                });
                append_list(a3332, a3333)
            };
            append_list(a3334, a3335)
        };
        append_list(a3336, a3337)
    })
}

pub fn f941(v0: Adt81) -> Str {
    let m3338 = v0;
    match m3338 {
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

pub fn f942(v0: Str, v1: Option<Adt10>) -> List<(Str, Adt9)> {
    let m3339 = v1;
    match m3339 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C2(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f943(v0: Adt82) -> List<(Str, Adt9)> {
    let m3341 = match v0 {
        Adt82::C0(h3340) => {
            h3340
        }
    };
    match m3341 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt9::C3(f974(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f944(v0: Adt69) -> List<(Str, Adt9)> {
    let m3342 = v0;
    match m3342 {
        Adt69::C0(v1) => {
            let a3349 = {
                let m3344 = match v1.clone() {
                    Adt70::C0(h3343, _, _) => {
                        h3343
                    }
                };
                match m3344 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt9::C3(Str::lit("form"))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            let a3350 = List::cons((Str::lit("message"), Adt9::C3(match v1.clone() {
                Adt70::C0(_, h3345, _) => {
                    h3345
                }
            })), List::cons((Str::lit("requestedSchema"), Adt9::C5({
                let m3348 = match v1 {
                    Adt70::C0(_, _, h3347) => {
                        h3347
                    }
                };
                match m3348 {
                    Adt8::C0(h3346) => {
                        h3346
                    }
                }
            })), List::<(Str, Adt9)>::nil()));
            append_list(a3349, a3350)
        }
        Adt69::C1(v3) => {
            List::cons((Str::lit("mode"), Adt9::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt9::C3(match v3.clone() {
                Adt72::C0(h3351, _) => {
                    h3351
                }
            })), List::cons((Str::lit("url"), Adt9::C3(match v3 {
                Adt72::C0(_, h3352) => {
                    h3352
                }
            })), List::<(Str, Adt9)>::nil())))
        }
    }
}

pub fn f945(v0: Adt5) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("src"), Adt9::C3(match v0.clone() {
        Adt5::C0(h3353, _, _, _) => {
            h3353
        }
    })), {
        let a3359 = f712(Str::lit("mimeType"), match v0.clone() {
            Adt5::C0(_, h3354, _, _) => {
                h3354
            }
        });
        let a3360 = {
            let a3357 = f975(match v0.clone() {
                Adt5::C0(_, _, h3355, _) => {
                    h3355
                }
            });
            let a3358 = f976(match v0 {
                Adt5::C0(_, _, _, h3356) => {
                    h3356
                }
            });
            append_list(a3357, a3358)
        };
        append_list(a3359, a3360)
    }))
}

pub fn f946(p3366: List<(Str, Adt9)>, p3367: List<(Str, Adt68)>) -> Option<List<(Str, Adt68)>> {
    let m3365 = (p3366, p3367);
    {
        let mut m3365 = m3365;
        loop {
            let step = {
                let (v0, v1) = m3365;
                let m3361 = v0;
                match m3361.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Str, Adt68)>), Option<List<(Str, Adt68)>>>(Some(f977(v1, List::<(Str, Adt68)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m3363 = f978({
                            let (_, h3362) = v2;
                            h3362
                        });
                        match m3363 {
                            None => {
                                Err::<(List<(Str, Adt9)>, List<(Str, Adt68)>), Option<List<(Str, Adt68)>>>(None::<List<(Str, Adt68)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt9)>, List<(Str, Adt68)>), Option<List<(Str, Adt68)>>>((v3, List::cons(({
                                    let (h3364, _) = v2;
                                    h3364
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3365 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f947(v0: Adt90) -> Adt9 {
    let m3368 = v0;
    match m3368 {
        Adt90::C0(v1) => {
            Adt9::C5(List::cons((Str::lit("completes"), Adt9::C5(match v1 {
                Adt8::C0(h3369) => {
                    h3369
                }
            })), List::<(Str, Adt9)>::nil()))
        }
        Adt90::C1(v2) => {
            Adt9::C5(List::cons((Str::lit("fails"), f375(v2)), List::<(Str, Adt9)>::nil()))
        }
    }
}

pub fn f948(v0: u64, v1: List<u64>) -> List<u64> {
    let a3370 = v0;
    let a3371 = 0u64;
    if nat_eq(a3370, a3371) {
        v1
    } else {
        let a3372 = v0;
        let a3373 = 1u64;
        if nat_eq(a3372, a3373) {
            List::cons(194u64, v1)
        } else {
            let a3374 = v0;
            let a3375 = 2u64;
            if nat_eq(a3374, a3375) {
                List::cons(226u64, v1)
            } else {
                let a3376 = v0;
                let a3377 = 3u64;
                if nat_eq(a3376, a3377) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f949(v0: u64, v1: u64, v2: List<u64>) -> Adt167 {
    let a3378 = v0;
    let a3379 = 0u64;
    if nat_eq(a3378, a3379) {
        f979(v1, v2)
    } else {
        let a3380 = v0;
        let a3381 = 1u64;
        if nat_eq(a3380, a3381) {
            let a3382 = 128u64;
            let a3383 = v1;
            if if nat_le(a3382, a3383) {
                let a3384 = v1;
                let a3385 = 159u64;
                nat_le(a3384, a3385)
            } else {
                false
            } {
                Adt167::C0(0u64, f980(v2))
            } else {
                f979(v1, List::cons(194u64, v2))
            }
        } else {
            let a3386 = v0;
            let a3387 = 2u64;
            if nat_eq(a3386, a3387) {
                let a3388 = v1;
                let a3389 = 128u64;
                if nat_eq(a3388, a3389) {
                    Adt167::C0(3u64, v2)
                } else {
                    let a3390 = v1;
                    let a3391 = 129u64;
                    if nat_eq(a3390, a3391) {
                        Adt167::C0(4u64, v2)
                    } else {
                        f979(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a3392 = v0;
                let a3393 = 3u64;
                if nat_eq(a3392, a3393) {
                    let a3394 = 170u64;
                    let a3395 = v1;
                    if if nat_le(a3394, a3395) {
                        let a3396 = v1;
                        let a3397 = 174u64;
                        nat_le(a3396, a3397)
                    } else {
                        false
                    } {
                        Adt167::C0(0u64, f980(v2))
                    } else {
                        f979(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a3398 = 166u64;
                    let a3399 = v1;
                    if if nat_le(a3398, a3399) {
                        let a3400 = v1;
                        let a3401 = 169u64;
                        nat_le(a3400, a3401)
                    } else {
                        false
                    } {
                        Adt167::C0(0u64, f980(v2))
                    } else {
                        f979(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f950(v0: Adt153, v1: Adt153) -> bool {
    let a3402 = f757(v0);
    let a3403 = f757(v1);
    equal(a3402, a3403)
}

pub fn f951(v0: Adt153) -> bool {
    let m3404 = v0;
    match m3404 {
        Adt153::C0 => {
            false
        }
        Adt153::C1 => {
            false
        }
        Adt153::C2 => {
            true
        }
        Adt153::C3 => {
            false
        }
        Adt153::C4 => {
            true
        }
        Adt153::C5 => {
            false
        }
        Adt153::C6 => {
            false
        }
        Adt153::C7 => {
            true
        }
        Adt153::C8 => {
            false
        }
        Adt153::C9 => {
            false
        }
        Adt153::C10 => {
            false
        }
        Adt153::C11 => {
            false
        }
    }
}

pub fn f952(v0: Adt153) -> bool {
    let m3405 = v0;
    match m3405 {
        Adt153::C0 => {
            false
        }
        Adt153::C1 => {
            false
        }
        Adt153::C2 => {
            true
        }
        Adt153::C3 => {
            false
        }
        Adt153::C4 => {
            false
        }
        Adt153::C5 => {
            false
        }
        Adt153::C6 => {
            false
        }
        Adt153::C7 => {
            false
        }
        Adt153::C8 => {
            false
        }
        Adt153::C9 => {
            false
        }
        Adt153::C10 => {
            false
        }
        Adt153::C11 => {
            false
        }
    }
}

pub fn f953(v0: Adt3, v1: Adt20, v2: Adt85) -> Result<Adt142, Adt145> {
    f981(v0, v2.clone(), f982(match v1 {
        Adt20::C0(_, h3406, _, _, _, _) => {
            h3406
        }
    }, f725(v2)))
}

pub fn f954(v0: Adt18) -> Adt20 {
    let m3407 = v0;
    match m3407 {
        Adt18::C0(v1) => {
            match v1 {
                Adt19::C0(h3408) => {
                    h3408
                }
            }
        }
        Adt18::C1(v2) => {
            match v2 {
                Adt24::C0(h3409, _) => {
                    h3409
                }
            }
        }
        Adt18::C2(v3) => {
            match v3 {
                Adt25::C0(h3410, _, _, _, _) => {
                    h3410
                }
            }
        }
        Adt18::C3(v4) => {
            match v4 {
                Adt24::C0(h3411, _) => {
                    h3411
                }
            }
        }
        Adt18::C4(v5) => {
            match v5 {
                Adt26::C0(h3412, _, _, _) => {
                    h3412
                }
            }
        }
        Adt18::C5(v6) => {
            match v6 {
                Adt24::C0(h3413, _) => {
                    h3413
                }
            }
        }
        Adt18::C6(v7) => {
            match v7 {
                Adt24::C0(h3414, _) => {
                    h3414
                }
            }
        }
        Adt18::C7(v8) => {
            match v8 {
                Adt27::C0(h3415, _, _, _, _) => {
                    h3415
                }
            }
        }
        Adt18::C8(v9) => {
            match v9 {
                Adt28::C0(h3416, _, _, _) => {
                    h3416
                }
            }
        }
        Adt18::C9(v10) => {
            match v10 {
                Adt33::C0(h3417, _) => {
                    h3417
                }
            }
        }
        Adt18::C10(v11) => {
            match v11 {
                Adt34::C0(h3418, _, _) => {
                    h3418
                }
            }
        }
        Adt18::C11(v12) => {
            match v12 {
                Adt35::C0(h3419, _) => {
                    h3419
                }
            }
        }
    }
}

pub fn f955(v0: Adt3, v1: Adt20, v2: Adt86) -> Result<Adt142, Adt145> {
    f983(v0, v2, f984(match v1 {
        Adt20::C0(_, h3420, _, _, _, _) => {
            h3420
        }
    }))
}

pub fn f956(v0: Adt3, v1: Adt20, v2: Adt91) -> Result<Adt142, Adt145> {
    f985(v0, v2, f984(match v1 {
        Adt20::C0(_, h3421, _, _, _, _) => {
            h3421
        }
    }))
}

pub fn f957(v0: Adt3, v1: Adt20, v2: Adt53) -> Adt142 {
    Adt142::C0(Adt36::C2(match v2.clone() {
        Adt53::C0(h3422, _) => {
            h3422
        }
    }), {
        let m3424 = f877(v0, Adt36::C11);
        match m3424 {
            Adt142::C0(_, h3423, _) => {
                h3423
            }
        }
    }, f986(v1, match v2 {
        Adt53::C0(_, h3425) => {
            h3425
        }
    }))
}

pub fn f958(v0: Bytes, v1: Adt18, v2: Result<Adt36, Adt93>) -> R<Result<Adt36, Adt93>> {
    let m3426 = v2;
    match m3426 {
        Ok(v4) => {
            f987(v0, f876(v1.clone()), f988(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt36, Adt93>(v3))
        }
    }
}

pub fn f959(v0: u64, v1: u64, v2: Fn3, v3: Adt18) -> R<Result<Adt36, Adt93>> {
    let m3427 = v3.clone();
    match m3427 {
        Adt18::C0(_) => {
            let c3429: Fn3 = v2;
            c3429.apply(v3)
        }
        Adt18::C1(_) => {
            let c3430: Fn3 = v2;
            c3430.apply(v3)
        }
        Adt18::C2(_) => {
            if f989(v0, v1) {
                let c3428: Fn3 = v2;
                c3428.apply(v3)
            } else {
                Ok(Err::<Adt36, Adt93>(Adt93::C7(v0)))
            }
        }
        Adt18::C3(_) => {
            let c3431: Fn3 = v2;
            c3431.apply(v3)
        }
        Adt18::C4(_) => {
            let c3432: Fn3 = v2;
            c3432.apply(v3)
        }
        Adt18::C5(_) => {
            let c3433: Fn3 = v2;
            c3433.apply(v3)
        }
        Adt18::C6(_) => {
            let c3434: Fn3 = v2;
            c3434.apply(v3)
        }
        Adt18::C7(_) => {
            let c3435: Fn3 = v2;
            c3435.apply(v3)
        }
        Adt18::C8(_) => {
            let c3436: Fn3 = v2;
            c3436.apply(v3)
        }
        Adt18::C9(_) => {
            let c3437: Fn3 = v2;
            c3437.apply(v3)
        }
        Adt18::C10(_) => {
            let c3438: Fn3 = v2;
            c3438.apply(v3)
        }
        Adt18::C11(_) => {
            let c3439: Fn3 = v2;
            c3439.apply(v3)
        }
    }
}

pub fn f960(v0: Bytes, v1: Adt25) -> R<Result<Adt18, Adt93>> {
    let m3443 = f990(v0, Str::lit("tools/call"), match v1.clone() {
        Adt25::C0(_, h3440, _, _, _) => {
            h3440
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, _, h3441) => {
            h3441
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, h3442, _) => {
            h3442
        }
    })?;
    match m3443 {
        Ok(v3) => {
            Ok(Ok::<Adt18, Adt93>(Adt18::C2(Adt25::C0(match v1.clone() {
                Adt25::C0(h3444, _, _, _, _) => {
                    h3444
                }
            }, match v1.clone() {
                Adt25::C0(_, h3445, _, _, _) => {
                    h3445
                }
            }, match v1.clone() {
                Adt25::C0(_, _, h3446, _, _) => {
                    h3446
                }
            }, match v1 {
                Adt25::C0(_, _, _, h3447, _) => {
                    h3447
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt18, Adt93>(v2))
        }
    }
}

pub fn f961(v0: Bytes, v1: Adt26) -> R<Result<Adt18, Adt93>> {
    let m3451 = f990(v0, Str::lit("resources/read"), match v1.clone() {
        Adt26::C0(_, h3448, _, _) => {
            h3448
        }
    }, match v1.clone() {
        Adt26::C0(_, _, _, h3449) => {
            h3449
        }
    }, match v1.clone() {
        Adt26::C0(_, _, h3450, _) => {
            h3450
        }
    })?;
    match m3451 {
        Ok(v3) => {
            Ok(Ok::<Adt18, Adt93>(Adt18::C4(Adt26::C0(match v1.clone() {
                Adt26::C0(h3452, _, _, _) => {
                    h3452
                }
            }, match v1.clone() {
                Adt26::C0(_, h3453, _, _) => {
                    h3453
                }
            }, match v1 {
                Adt26::C0(_, _, h3454, _) => {
                    h3454
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt18, Adt93>(v2))
        }
    }
}

pub fn f962(v0: Bytes, v1: Adt27) -> R<Result<Adt18, Adt93>> {
    let m3458 = f990(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt27::C0(_, h3455, _, _, _) => {
            h3455
        }
    }, match v1.clone() {
        Adt27::C0(_, _, _, _, h3456) => {
            h3456
        }
    }, match v1.clone() {
        Adt27::C0(_, _, _, h3457, _) => {
            h3457
        }
    })?;
    match m3458 {
        Ok(v3) => {
            Ok(Ok::<Adt18, Adt93>(Adt18::C7(Adt27::C0(match v1.clone() {
                Adt27::C0(h3459, _, _, _, _) => {
                    h3459
                }
            }, match v1.clone() {
                Adt27::C0(_, h3460, _, _, _) => {
                    h3460
                }
            }, match v1.clone() {
                Adt27::C0(_, _, h3461, _, _) => {
                    h3461
                }
            }, match v1 {
                Adt27::C0(_, _, _, h3462, _) => {
                    h3462
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt18, Adt93>(v2))
        }
    }
}

pub fn f963(v0: Str, p3468: List<(Str, Adt9)>, p3469: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt146> {
    let m3467 = (p3468, p3469);
    {
        let mut m3467 = m3467;
        loop {
            let step = {
                let (v1, v2) = m3467;
                let m3463 = v1;
                match m3463.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt146>>(Ok::<List<(Str, Str)>, Adt146>(v2))
                    }
                    Some((v3, v4)) => {
                        let m3465 = f283({
                            let (_, h3464) = v3;
                            h3464
                        });
                        match m3465 {
                            None => {
                                Err::<(List<(Str, Adt9)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt146>>(Err::<List<(Str, Str)>, Adt146>(Adt146::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt9)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt146>>((v4, List::cons(({
                                    let (h3466, _) = v3;
                                    h3466
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f964(p3472: List<(Str, Str)>, p3473: List<(Str, Str)>) -> List<(Str, Str)> {
    let m3471 = (p3472, p3473);
    {
        let mut m3471 = m3471;
        loop {
            let step = {
                let (v0, v1) = m3471;
                let m3470 = v0;
                match m3470.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3471 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f965(v0: Adt131, v1: Adt8) -> R<Option<Adt129>> {
    let m3475 = f991(match v0.clone() {
        Adt131::C0(_, h3474, _, _) => {
            h3474
        }
    }, v1);
    match m3475 {
        Adt115::C0 => {
            Ok(f992(v0))
        }
        Adt115::C1(v2) => {
            if f993(v2.clone()) {
                Ok(f992(v0))
            } else {
                f994(v0, v2)
            }
        }
    }
}

pub fn f966(v0: Str, v1: Adt8) -> Option<Option<Adt8>> {
    let m3476 = f233(v0, v1);
    match m3476 {
        Adt115::C0 => {
            Some(None::<Adt8>)
        }
        Adt115::C1(v2) => {
            let m3477 = f234(v2);
            match m3477 {
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

pub fn f967() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f968(v0: Str) -> Adt168 {
    let a3478 = v0.clone();
    let a3479 = Str::lit("debug");
    if equal(a3478, a3479) {
        Adt168::C1(Adt22::C0)
    } else {
        let a3480 = v0.clone();
        let a3481 = Str::lit("info");
        if equal(a3480, a3481) {
            Adt168::C1(Adt22::C1)
        } else {
            let a3482 = v0.clone();
            let a3483 = Str::lit("notice");
            if equal(a3482, a3483) {
                Adt168::C1(Adt22::C2)
            } else {
                let a3484 = v0.clone();
                let a3485 = Str::lit("warning");
                if equal(a3484, a3485) {
                    Adt168::C1(Adt22::C3)
                } else {
                    let a3486 = v0.clone();
                    let a3487 = Str::lit("error");
                    if equal(a3486, a3487) {
                        Adt168::C1(Adt22::C4)
                    } else {
                        let a3488 = v0.clone();
                        let a3489 = Str::lit("critical");
                        if equal(a3488, a3489) {
                            Adt168::C1(Adt22::C5)
                        } else {
                            let a3490 = v0.clone();
                            let a3491 = Str::lit("alert");
                            if equal(a3490, a3491) {
                                Adt168::C1(Adt22::C6)
                            } else {
                                let a3492 = v0;
                                let a3493 = Str::lit("emergency");
                                if equal(a3492, a3493) {
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

pub fn f969() -> Str {
    Str::lit("progressToken")
}

pub fn f970(v0: List<Adt45>) -> List<Adt9> {
    f574(f995(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f971(v0: Adt61) -> Adt9 {
    Adt9::C5({
        let a3502 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt61::C0(h3494, _, _, _) => {
                h3494
            }
        })), List::<(Str, Adt9)>::nil());
        let a3503 = {
            let a3500 = f712(Str::lit("title"), match v0.clone() {
                Adt61::C0(_, h3495, _, _) => {
                    h3495
                }
            });
            let a3501 = {
                let a3498 = f712(Str::lit("description"), match v0.clone() {
                    Adt61::C0(_, _, h3496, _) => {
                        h3496
                    }
                });
                let a3499 = f717(Str::lit("required"), match v0 {
                    Adt61::C0(_, _, _, h3497) => {
                        h3497
                    }
                });
                append_list(a3498, a3499)
            };
            append_list(a3500, a3501)
        };
        append_list(a3502, a3503)
    })
}

pub fn f972(v0: Adt74) -> R<Option<Adt9>> {
    let m3505 = f996(match v0.clone() {
        Adt74::C0(_, h3504, _) => {
            h3504
        }
    })?;
    match m3505 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("role"), Adt9::C3(f906(match v0.clone() {
                Adt74::C0(h3506, _, _) => {
                    h3506
                }
            }))), List::cons((Str::lit("content"), v1), f898(match v0 {
                Adt74::C0(_, _, h3507) => {
                    h3507
                }
            }))))))
        }
    }
}

pub fn f973(p3511: List<Adt80>, p3512: List<Adt9>) -> List<Adt9> {
    let m3510 = (p3511, p3512);
    {
        let mut m3510 = m3510;
        loop {
            let step = {
                let (v0, v1) = m3510;
                let m3508 = v0;
                match m3508.uncons() {
                    None => {
                        Err::<(List<Adt80>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt80>, List<Adt9>), List<Adt9>>((v3, List::cons(Adt9::C5(f712(Str::lit("name"), match v2 {
                            Adt80::C0(h3509) => {
                                h3509
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3510 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f974(v0: Adt83) -> Str {
    let m3513 = v0;
    match m3513 {
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

pub fn f975(v0: Option<List<Str>>) -> List<(Str, Adt9)> {
    let m3514 = v0;
    match m3514 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt9::C4(f666(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f976(v0: Option<Adt6>) -> List<(Str, Adt9)> {
    let m3515 = v0;
    match m3515 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt9::C3(f997(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f977(p3518: List<(Str, Adt68)>, p3519: List<(Str, Adt68)>) -> List<(Str, Adt68)> {
    let m3517 = (p3518, p3519);
    {
        let mut m3517 = m3517;
        loop {
            let step = {
                let (v0, v1) = m3517;
                let m3516 = v0;
                match m3516.uncons() {
                    None => {
                        Err::<(List<(Str, Adt68)>, List<(Str, Adt68)>), List<(Str, Adt68)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt68)>, List<(Str, Adt68)>), List<(Str, Adt68)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3517 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f978(v0: Adt9) -> Option<Adt68> {
    let m3520 = f234(v0);
    match m3520 {
        None => {
            None::<Adt68>
        }
        Some(v1) => {
            let m3521 = f914(Str::lit("method"), v1.clone());
            match m3521 {
                None => {
                    None::<Adt68>
                }
                Some(v2) => {
                    let a3522 = v2.clone();
                    let a3523 = Str::lit("elicitation/create");
                    if equal(a3522, a3523) {
                        let m3524 = f867(Str::lit("params"), v1);
                        match m3524 {
                            None => {
                                None::<Adt68>
                            }
                            Some(v3) => {
                                f998(v3)
                            }
                        }
                    } else {
                        let a3525 = v2;
                        let a3526 = Str::lit("roots/list");
                        if equal(a3525, a3526) {
                            f999(v1)
                        } else {
                            None::<Adt68>
                        }
                    }
                }
            }
        }
    }
}

pub fn f979(v0: u64, v1: List<u64>) -> Adt167 {
    let a3527 = v0;
    let a3528 = 32u64;
    if nat_lt(a3527, a3528) {
        if f1000(v0) {
            Adt167::C0(0u64, List::cons(v0, v1))
        } else {
            Adt167::C0(0u64, f980(v1))
        }
    } else {
        let a3529 = v0;
        let a3530 = 127u64;
        if nat_eq(a3529, a3530) {
            Adt167::C0(0u64, f980(v1))
        } else {
            let a3531 = v0;
            let a3532 = 194u64;
            if nat_eq(a3531, a3532) {
                Adt167::C0(1u64, v1)
            } else {
                let a3533 = v0;
                let a3534 = 226u64;
                if nat_eq(a3533, a3534) {
                    Adt167::C0(2u64, v1)
                } else {
                    Adt167::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f980(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f981(v0: Adt3, v1: Adt85, v2: bool) -> Result<Adt142, Adt145> {
    if v2 {
        Ok::<Adt142, Adt145>(f877(v0, Adt36::C12(v1)))
    } else {
        Err::<Adt142, Adt145>(Adt145::C4(Adt93::C8(f1001(f725(v1)))))
    }
}

pub fn f982(v0: Adt21, v1: List<(Str, Adt68)>) -> bool {
    let a3535 = f1002(v1.clone());
    if if if bool_not(a3535) {
        true
    } else {
        f294(match v0.clone() {
            Adt21::C0(_, _, _, h3536, _) => {
                h3536
            }
        })
    } {
        let a3537 = f1003(v1.clone());
        if bool_not(a3537) {
            true
        } else {
            f294(match v0.clone() {
                Adt21::C0(_, _, h3538, _, _) => {
                    h3538
                }
            })
        }
    } else {
        false
    } {
        let a3539 = f1004(v1);
        if bool_not(a3539) {
            true
        } else {
            f294(match v0 {
                Adt21::C0(_, h3540, _, _, _) => {
                    h3540
                }
            })
        }
    } else {
        false
    }
}

pub fn f983(v0: Adt3, v1: Adt86, v2: bool) -> Result<Adt142, Adt145> {
    if v2 {
        Ok::<Adt142, Adt145>(f877(v0, Adt36::C13(v1)))
    } else {
        Err::<Adt142, Adt145>(Adt145::C4(Adt93::C8(f1005())))
    }
}

pub fn f984(v0: Adt21) -> bool {
    f760(match v0 {
        Adt21::C0(_, _, _, _, h3541) => {
            h3541
        }
    }, f63())
}

pub fn f985(v0: Adt3, v1: Adt91, v2: bool) -> Result<Adt142, Adt145> {
    if v2 {
        Ok::<Adt142, Adt145>(f877(v0, Adt36::C14(v1)))
    } else {
        Err::<Adt142, Adt145>(Adt145::C4(Adt93::C8(f1005())))
    }
}

pub fn f986(v0: Adt20, v1: List<Adt54>) -> List<Adt144> {
    let m3543 = match v0 {
        Adt20::C0(_, _, _, _, h3542, _) => {
            h3542
        }
    };
    match m3543 {
        None => {
            List::<Adt144>::nil()
        }
        Some(v2) => {
            f1006(f1007(v2, v1, List::<Adt144>::nil()), List::<Adt144>::nil())
        }
    }
}

pub fn f987(v0: Bytes, v1: Adt153, v2: Str, v3: Adt36) -> R<Result<Adt36, Adt93>> {
    let m3544 = v3.clone();
    match m3544 {
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
            f1008(v0, v1, v2, v4)
        }
        Adt36::C13(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C14(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
    }
}

pub fn f988(v0: Adt18) -> Str {
    let m3545 = v0;
    match m3545 {
        Adt18::C0(_) => {
            Str::lit("")
        }
        Adt18::C1(_) => {
            Str::lit("")
        }
        Adt18::C2(v1) => {
            match v1 {
                Adt25::C0(_, h3546, _, _, _) => {
                    h3546
                }
            }
        }
        Adt18::C3(_) => {
            Str::lit("")
        }
        Adt18::C4(v2) => {
            match v2 {
                Adt26::C0(_, h3547, _, _) => {
                    h3547
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
                Adt27::C0(_, h3548, _, _, _) => {
                    h3548
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

pub fn f989(v0: u64, v1: u64) -> bool {
    let a3549 = v1;
    let a3550 = v0;
    nat_lt(a3549, a3550)
}

pub fn f990(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt8>) -> R<Result<Option<Str>, Adt93>> {
    Ok(f1009(f1010(v0, v1, v2, v3)?, f1011(v4)))
}

pub fn f991(p3556: List<Str>, p3557: Adt8) -> Adt115 {
    let m3555 = (p3556, p3557);
    {
        let mut m3555 = m3555;
        loop {
            let step = {
                let (v0, v1) = m3555;
                let m3551 = v0;
                match m3551.uncons() {
                    None => {
                        Err::<(List<Str>, Adt8), Adt115>(Adt115::C0)
                    }
                    Some((v2, v3)) => {
                        let m3552 = f233(v2, v1);
                        match m3552 {
                            Adt115::C0 => {
                                Err::<(List<Str>, Adt8), Adt115>(Adt115::C0)
                            }
                            Adt115::C1(v4) => {
                                let m3553 = v3.clone();
                                match m3553.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt8), Adt115>(Adt115::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3554 = f234(v4);
                                        match m3554 {
                                            None => {
                                                Err::<(List<Str>, Adt8), Adt115>(Adt115::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt8), Adt115>((v3, v7))
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
                Ok(next) => { m3555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f992(v0: Adt131) -> Option<Adt129> {
    let m3559 = match v0.clone() {
        Adt131::C0(_, _, _, h3558) => {
            h3558
        }
    };
    match m3559 {
        Adt132::C0 => {
            None::<Adt129>
        }
        Adt132::C1 => {
            Some(Adt129::C10(match v0 {
                Adt131::C0(h3560, _, _, _) => {
                    h3560
                }
            }))
        }
        Adt132::C2(_) => {
            None::<Adt129>
        }
    }
}

pub fn f993(v0: Adt9) -> bool {
    let m3561 = v0;
    match m3561 {
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

pub fn f994(v0: Adt131, v1: Adt9) -> R<Option<Adt129>> {
    let m3563 = match v0.clone() {
        Adt131::C0(_, _, _, h3562) => {
            h3562
        }
    };
    match m3563 {
        Adt132::C0 => {
            Ok(Some(Adt129::C8(match v0 {
                Adt131::C0(h3564, _, _, _) => {
                    h3564
                }
            })))
        }
        Adt132::C1 => {
            Ok(Some(Adt129::C10(match v0 {
                Adt131::C0(h3565, _, _, _) => {
                    h3565
                }
            })))
        }
        Adt132::C2(v2) => {
            if f1012(match v0.clone() {
                Adt131::C0(_, _, h3566, _) => {
                    h3566
                }
            }, v2, v1)? {
                Ok(None::<Adt129>)
            } else {
                Ok(Some(Adt129::C9(match v0 {
                    Adt131::C0(h3567, _, _, _) => {
                        h3567
                    }
                })))
            }
        }
    }
}

pub fn f995(p3570: List<Adt45>, p3571: List<Adt9>) -> List<Adt9> {
    let m3569 = (p3570, p3571);
    {
        let mut m3569 = m3569;
        loop {
            let step = {
                let (v0, v1) = m3569;
                let m3568 = v0;
                match m3568.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt9>), List<Adt9>>((v3, List::cons(Adt9::C3(f906(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3569 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f996(v0: Adt75) -> R<Option<Adt9>> {
    let m3572 = v0;
    match m3572 {
        Adt75::C0(v1) => {
            f1013(v1)
        }
        Adt75::C1(v2) => {
            let m3573 = f1014(v2, List::<Adt9>::nil())?;
            match m3573 {
                None => {
                    Ok(None::<Adt9>)
                }
                Some(v3) => {
                    Ok(Some(Adt9::C4(f574(v3, List::<Adt9>::nil()))))
                }
            }
        }
    }
}

pub fn f997(v0: Adt6) -> Str {
    let m3574 = v0;
    match m3574 {
        Adt6::C0 => {
            Str::lit("light")
        }
        Adt6::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f998(v0: Adt8) -> Option<Adt68> {
    let m3575 = f914(Str::lit("message"), v0.clone());
    match m3575 {
        None => {
            None::<Adt68>
        }
        Some(v1) => {
            let m3576 = f233(Str::lit("mode"), v0.clone());
            match m3576 {
                Adt115::C0 => {
                    let m3577 = f867(Str::lit("requestedSchema"), v0);
                    match m3577 {
                        None => {
                            None::<Adt68>
                        }
                        Some(v2) => {
                            Some(Adt68::C0(Adt69::C0(Adt70::C0(None::<Adt71>, v1, v2))))
                        }
                    }
                }
                Adt115::C1(v3) => {
                    let m3578 = f283(v3);
                    match m3578 {
                        None => {
                            None::<Adt68>
                        }
                        Some(v4) => {
                            let a3579 = v4.clone();
                            let a3580 = Str::lit("url");
                            if equal(a3579, a3580) {
                                let m3581 = f914(Str::lit("url"), v0);
                                match m3581 {
                                    None => {
                                        None::<Adt68>
                                    }
                                    Some(v5) => {
                                        Some(Adt68::C0(Adt69::C1(Adt72::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a3582 = v4;
                                let a3583 = Str::lit("form");
                                if equal(a3582, a3583) {
                                    let m3584 = f867(Str::lit("requestedSchema"), v0);
                                    match m3584 {
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

pub fn f999(v0: Adt8) -> Option<Adt68> {
    let m3585 = f233(Str::lit("params"), v0);
    match m3585 {
        Adt115::C0 => {
            Some(Adt68::C2(None::<Adt8>))
        }
        Adt115::C1(v1) => {
            let m3586 = f234(v1);
            match m3586 {
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

pub fn f1000(v0: u64) -> bool {
    let a3587 = v0;
    let a3588 = 9u64;
    if nat_eq(a3587, a3588) {
        true
    } else {
        let a3589 = v0;
        let a3590 = 10u64;
        if nat_eq(a3589, a3590) {
            true
        } else {
            let a3591 = v0;
            let a3592 = 13u64;
            nat_eq(a3591, a3592)
        }
    }
}

pub fn f1001(v0: List<(Str, Adt68)>) -> Adt21 {
    Adt21::C0(None::<Adt8>, if f1004(v0.clone()) {
        Some(f62())
    } else {
        None::<Adt8>
    }, if f1003(v0.clone()) {
        Some(f62())
    } else {
        None::<Adt8>
    }, if f1002(v0) {
        Some(f62())
    } else {
        None::<Adt8>
    }, None::<Adt8>)
}

pub fn f1002(p3597: List<(Str, Adt68)>) -> bool {
    let m3596 = p3597;
    {
        let mut m3596 = m3596;
        loop {
            let step = {
                let v0 = m3596;
                let m3593 = v0;
                match m3593.uncons() {
                    None => {
                        Err::<List<(Str, Adt68)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3595 = {
                            let (_, h3594) = v1;
                            h3594
                        };
                        match m3595 {
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
                Ok(next) => { m3596 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1003(p3602: List<(Str, Adt68)>) -> bool {
    let m3601 = p3602;
    {
        let mut m3601 = m3601;
        loop {
            let step = {
                let v0 = m3601;
                let m3598 = v0;
                match m3598.uncons() {
                    None => {
                        Err::<List<(Str, Adt68)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3600 = {
                            let (_, h3599) = v1;
                            h3599
                        };
                        match m3600 {
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
                Ok(next) => { m3601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1004(p3607: List<(Str, Adt68)>) -> bool {
    let m3606 = p3607;
    {
        let mut m3606 = m3606;
        loop {
            let step = {
                let v0 = m3606;
                let m3603 = v0;
                match m3603.uncons() {
                    None => {
                        Err::<List<(Str, Adt68)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3605 = {
                            let (_, h3604) = v1;
                            h3604
                        };
                        match m3605 {
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
                Ok(next) => { m3606 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1005() -> Adt21 {
    Adt21::C0(None::<Adt8>, None::<Adt8>, None::<Adt8>, None::<Adt8>, Some(Adt8::C0(List::cons((f63(), Adt9::C5(List::<(Str, Adt9)>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f1006(p3610: List<Adt144>, p3611: List<Adt144>) -> List<Adt144> {
    let m3609 = (p3610, p3611);
    {
        let mut m3609 = m3609;
        loop {
            let step = {
                let (v0, v1) = m3609;
                let m3608 = v0;
                match m3608.uncons() {
                    None => {
                        Err::<(List<Adt144>, List<Adt144>), List<Adt144>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt144>, List<Adt144>), List<Adt144>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3609 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1007(v0: Adt23, p3614: List<Adt54>, p3615: List<Adt144>) -> List<Adt144> {
    let m3613 = (p3614, p3615);
    {
        let mut m3613 = m3613;
        loop {
            let step = {
                let (v1, v2) = m3613;
                let m3612 = v1;
                match m3612.uncons() {
                    None => {
                        Err::<(List<Adt54>, List<Adt144>), List<Adt144>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt54>, List<Adt144>), List<Adt144>>((v4, List::cons(Adt144::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m3613 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1008(v0: Bytes, v1: Adt153, v2: Str, v3: Adt85) -> R<Result<Adt36, Adt93>> {
    if f951(v1.clone()) {
        if f1015(v3.clone()) {
            let m3616 = f1016(v0, f757(v1), v2, v3)?;
            match m3616 {
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
        Ok(Err::<Adt36, Adt93>(Adt93::C12(f757(v1))))
    }
}

pub fn f1009(v0: Result<Option<Str>, Adt93>, v1: bool) -> Result<Option<Str>, Adt93> {
    let m3617 = v0;
    match m3617 {
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

pub fn f1010(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt93>> {
    let m3618 = v3;
    match m3618 {
        None => {
            Ok(Ok::<Option<Str>, Adt93>(None::<Str>))
        }
        Some(v4) => {
            Ok(f1017(f1018(v0, v1, v2, v4)?))
        }
    }
}

pub fn f1011(v0: Option<Adt8>) -> bool {
    let m3619 = v0;
    match m3619 {
        None => {
            true
        }
        Some(v1) => {
            f1019(match v1 {
                Adt8::C0(h3620) => {
                    h3620
                }
            })
        }
    }
}

pub fn f1012(v0: Adt16, v1: Str, v2: Adt9) -> R<bool> {
    let m3621 = v0;
    match m3621 {
        Adt16::C0 => {
            let m3622 = f283(v2);
            match m3622 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3623 = v1;
                    let a3624 = v3;
                    Ok(equal(a3623, a3624))
                }
            }
        }
        Adt16::C1 => {
            f1020(v1, v2)
        }
        Adt16::C2 => {
            let m3625 = f356(v2);
            match m3625 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3626 = v1;
                        let a3627 = Str::lit("true");
                        Ok(equal(a3626, a3627))
                    } else {
                        let a3628 = v1;
                        let a3629 = Str::lit("false");
                        Ok(equal(a3628, a3629))
                    }
                }
            }
        }
    }
}

pub fn f1013(v0: Adt76) -> R<Option<Adt9>> {
    let m3630 = v0;
    match m3630 {
        Adt76::C0(v1) => {
            Ok(Some(f899(v1)))
        }
        Adt76::C1(v2) => {
            Ok(Some(f900(Str::lit("image"), match v2.clone() {
                Adt46::C0(h3631, _, _, _) => {
                    h3631
                }
            }, match v2.clone() {
                Adt46::C0(_, h3632, _, _) => {
                    h3632
                }
            }, match v2.clone() {
                Adt46::C0(_, _, h3633, _) => {
                    h3633
                }
            }, match v2 {
                Adt46::C0(_, _, _, h3634) => {
                    h3634
                }
            })))
        }
        Adt76::C2(v3) => {
            Ok(Some(f900(Str::lit("audio"), match v3.clone() {
                Adt47::C0(h3635, _, _, _) => {
                    h3635
                }
            }, match v3.clone() {
                Adt47::C0(_, h3636, _, _) => {
                    h3636
                }
            }, match v3.clone() {
                Adt47::C0(_, _, h3637, _) => {
                    h3637
                }
            }, match v3 {
                Adt47::C0(_, _, _, h3638) => {
                    h3638
                }
            })))
        }
        Adt76::C3(v4) => {
            Ok(Some(f1021(v4)))
        }
        Adt76::C4(v5) => {
            f1022(v5)
        }
    }
}

pub fn f1014(p3642: List<Adt76>, p3643: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m3641 = (p3642, p3643);
    {
        let mut m3641 = m3641;
        loop {
            let step = {
                let (v0, v1) = m3641;
                let m3639 = v0;
                match m3639.uncons() {
                    None => {
                        Err::<(List<Adt76>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3640 = f1013(v2)?;
                        match m3640 {
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
                Ok(next) => { m3641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1015(v0: Adt85) -> bool {
    let m3644 = v0;
    match m3644 {
        Adt85::C0(v1) => {
            let m3645 = v1;
            match m3645.uncons() {
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

pub fn f1016(v0: Bytes, v1: Str, v2: Str, v3: Adt85) -> R<Result<Adt85, Adt93>> {
    let m3646 = v3.clone();
    match m3646 {
        Adt85::C0(_) => {
            Ok(Ok::<Adt85, Adt93>(v3))
        }
        Adt85::C1(v5) => {
            Ok(f1023(f1024(v0, v1, v2, v5)?, List::<(Str, Adt68)>::nil()))
        }
        Adt85::C2(v6, v7) => {
            Ok(f1023(f1024(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f1017(v0: Option<Str>) -> Result<Option<Str>, Adt93> {
    let m3647 = v0;
    match m3647 {
        None => {
            Err::<Option<Str>, Adt93>(Adt93::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt93>(Some(v1))
        }
    }
}

pub fn f1018(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3651 = f1025(v0, {
        let a3648 = v1;
        utf8_encode(a3648)
    }, {
        let a3649 = v2;
        utf8_encode(a3649)
    }, {
        let a3650 = v3;
        utf8_encode(a3650)
    })?;
    match m3651 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3652 = v4;
            Ok(utf8_decode(a3652))
        }
    }
}

pub fn f1019(p3657: List<(Str, Adt9)>) -> bool {
    let m3656 = p3657;
    {
        let mut m3656 = m3656;
        loop {
            let step = {
                let v0 = m3656;
                let m3653 = v0;
                match m3653.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3655 = f234({
                            let (_, h3654) = v1;
                            h3654
                        });
                        match m3655 {
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
                Ok(next) => { m3656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1020(v0: Str, v1: Adt9) -> R<bool> {
    let m3658 = f609(v1);
    match m3658 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3659 = f499(v2)?;
            match m3659 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3661 = f1026(f25({
                        let a3660 = v0;
                        utf8_encode(a3660)
                    })?)?;
                    match m3661 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f681({
                                let a3662 = v3;
                                let a3663 = v4;
                                int_sub(a3662, a3663)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f1021(v0: Adt77) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt9::C3(match v0.clone() {
        Adt77::C0(h3664, _, _, _) => {
            h3664
        }
    })), List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
        Adt77::C0(_, h3665, _, _) => {
            h3665
        }
    })), List::cons((Str::lit("input"), Adt9::C5({
        let m3668 = match v0.clone() {
            Adt77::C0(_, _, h3667, _) => {
                h3667
            }
        };
        match m3668 {
            Adt8::C0(h3666) => {
                h3666
            }
        }
    })), f898(match v0 {
        Adt77::C0(_, _, _, h3669) => {
            h3669
        }
    }))))))
}

pub fn f1022(v0: Adt78) -> R<Option<Adt9>> {
    let m3671 = f715(match v0.clone() {
        Adt78::C0(_, h3670, _, _, _) => {
            h3670
        }
    })?;
    match m3671 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt9::C3(match v0.clone() {
                Adt78::C0(h3672, _, _, _, _) => {
                    h3672
                }
            })), List::cons((Str::lit("content"), Adt9::C4(v1)), {
                let a3678 = f716(Str::lit("structuredContent"), match v0.clone() {
                    Adt78::C0(_, _, h3673, _, _) => {
                        h3673
                    }
                });
                let a3679 = {
                    let a3676 = f717(Str::lit("isError"), match v0.clone() {
                        Adt78::C0(_, _, _, h3674, _) => {
                            h3674
                        }
                    });
                    let a3677 = f898(match v0 {
                        Adt78::C0(_, _, _, _, h3675) => {
                            h3675
                        }
                    });
                    append_list(a3676, a3677)
                };
                append_list(a3678, a3679)
            }))))))
        }
    }
}

pub fn f1023(v0: Option<Str>, v1: List<(Str, Adt68)>) -> Result<Adt85, Adt93> {
    let m3680 = v0;
    match m3680 {
        None => {
            Err::<Adt85, Adt93>(Adt93::C14)
        }
        Some(v2) => {
            let m3681 = v1.clone();
            match m3681.uncons() {
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

pub fn f1024(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3685 = f1027(v0, {
        let a3682 = v1;
        utf8_encode(a3682)
    }, {
        let a3683 = v2;
        utf8_encode(a3683)
    }, {
        let a3684 = v3;
        utf8_encode(a3684)
    })?;
    match m3685 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3686 = v4;
            Ok(utf8_decode(a3686))
        }
    }
}

pub fn f1025(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3688 = f1028();
    let a3689 = {
        let a3687 = v0.clone();
        length_bytes(a3687)
    };
    if nat_le(a3688, a3689) {
        let a3691 = f1029(v3.clone())?;
        let a3692 = {
            let a3690 = v3.clone();
            length_bytes(a3690)
        };
        if nat_eq(a3691, a3692) {
            Ok(None::<Bytes>)
        } else {
            f1030(v0, v1, v2, v3.clone(), f1029(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1026(v0: List<u64>) -> R<Option<i64>> {
    let m3693 = v0.clone();
    match m3693.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3694 = v1;
            let a3695 = 45u64;
            if nat_eq(a3694, a3695) {
                let m3696 = v2.clone();
                match m3696.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3697 = f1031(v2, 0i64)?;
                        match m3697 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3698 = v5;
                                    int_neg(a3698)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f1031(v0, 0i64)
            }
        }
    }
}

pub fn f1027(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3700 = f1028();
    let a3701 = {
        let a3699 = v0.clone();
        length_bytes(a3699)
    };
    if nat_le(a3700, a3701) {
        Ok(Some({
            let a3704 = f1032(v3.clone())?;
            let a3705 = {
                let a3702 = Bytes::lit(&[0x2e]);
                let a3703 = f1032(f1033(v0, v1, v2, v3)?)?;
                append_bytes(a3702, a3703)
            };
            append_bytes(a3704, a3705)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1028() -> u64 {
    32u64
}

pub fn f1029(v0: Bytes) -> R<u64> {
    let (_, h3706) = f1034(Fn9::F1035(v0.clone()), {
        let a3707 = v0.clone();
        length_bytes(a3707)
    }, (0u64, {
        let a3708 = v0;
        length_bytes(a3708)
    }))?;
    Ok(h3706)
}

pub fn f1030(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3712 = {
        let a3709 = v3.clone();
        let a3710 = 0u64;
        let a3711 = v4;
        slice_bytes(a3709, a3710, a3711)
    };
    match m3712 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3723 = {
                let a3720 = v3.clone();
                let a3721 = {
                    let a3713 = v4;
                    let a3714 = 1u64;
                    nat_add(a3713, a3714)?
                };
                let a3722 = {
                    let a3718 = {
                        let a3715 = v3;
                        length_bytes(a3715)
                    };
                    let a3719 = {
                        let a3716 = v4;
                        let a3717 = 1u64;
                        nat_add(a3716, a3717)?
                    };
                    nat_sub(a3718, a3719)
                };
                slice_bytes(a3720, a3721, a3722)
            };
            match m3723 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f1036(v0, v1, v2, f1037(v5)?, f1037(v6)?)
                }
            }
        }
    }
}

pub fn f1031(p3729: List<u64>, p3730: i64) -> R<Option<i64>> {
    let m3728 = (p3729, p3730);
    {
        let mut m3728 = m3728;
        loop {
            let step = {
                let (v0, v1) = m3728;
                let m3724 = v0;
                match m3724.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3725 = f1038(v2);
                        match m3725 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f355({
                                    let a3726 = v1;
                                    let a3727 = 10i64;
                                    int_mul(a3726, a3727)?
                                }, f573(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3728 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1032(v0: Bytes) -> R<Bytes> {
    let (_, h3731) = f1039(Fn10::F1040(v0.clone()), {
        let a3732 = v0;
        length_bytes(a3732)
    }, (0u64, f255()))?;
    Ok(h3731)
}

pub fn f1033(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f1041(v0, {
        let a3733 = f1042(v1, v2);
        let a3734 = v3;
        append_bytes(a3733, a3734)
    })
}

pub fn f1034(v0: Fn9, p3738: u64, p3739: (u64, u64)) -> R<(u64, u64)> {
    let m3737 = (p3738, p3739);
    {
        let mut m3737 = m3737;
        loop {
            let step = {
                let (v1, v2) = m3737;
                let m3735 = v1;
                if m3735 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3735 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3736: Fn9 = v0.clone();
                        c3736.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3737 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1035(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3741 = {
            let (h3740, _) = v1;
            h3740
        };
        let a3742 = 1u64;
        nat_add(a3741, a3742)?
    }, {
        let a3745 = {
            let (_, h3743) = v1;
            h3743
        };
        let a3746 = {
            let a3744 = v0.clone();
            length_bytes(a3744)
        };
        if nat_eq(a3745, a3746) {
            let a3748 = f58(v0, {
                let (h3747, _) = v1;
                h3747
            })?;
            let a3749 = 46u64;
            if nat_eq(a3748, a3749) {
                let (h3750, _) = v1;
                h3750
            } else {
                let (_, h3751) = v1;
                h3751
            }
        } else {
            let (_, h3752) = v1;
            h3752
        }
    }))
}

pub fn f1036(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3753 = v3;
    match m3753 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3754 = v4;
            match m3754 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f1043(f1033(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f1037(v0: Bytes) -> R<Option<Bytes>> {
    let a3759 = {
        let a3756 = {
            let a3755 = v0.clone();
            length_bytes(a3755)
        };
        let a3757 = 2u64;
        let a3758 = 0u64;
        nat_rem(a3756, a3757, a3758)
    };
    let a3760 = 0u64;
    if nat_eq(a3759, a3760) {
        let v1: (u64, (Bytes, u64)) = f1044(Fn11::F1045(v0.clone()), {
            let a3762 = {
                let a3761 = v0;
                length_bytes(a3761)
            };
            let a3763 = 2u64;
            let a3764 = 0u64;
            nat_quot(a3762, a3763, a3764)
        }, (0u64, (f255(), 0u64)))?;
        let a3767 = {
            let (_, h3765) = {
                let (_, h3766) = v1.clone();
                h3766
            };
            h3765
        };
        let a3768 = 0u64;
        if nat_eq(a3767, a3768) {
            Ok(Some({
                let (h3769, _) = {
                    let (_, h3770) = v1;
                    h3770
                };
                h3769
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1038(v0: u64) -> Option<u64> {
    let a3771 = v0;
    let a3772 = 48u64;
    if if nat_lt(a3771, a3772) {
        true
    } else {
        let a3773 = 57u64;
        let a3774 = v0;
        nat_lt(a3773, a3774)
    } {
        None::<u64>
    } else {
        Some({
            let a3775 = v0;
            let a3776 = 48u64;
            nat_sub(a3775, a3776)
        })
    }
}

pub fn f1039(v0: Fn10, p3780: u64, p3781: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3779 = (p3780, p3781);
    {
        let mut m3779 = m3779;
        loop {
            let step = {
                let (v1, v2) = m3779;
                let m3777 = v1;
                if m3777 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3777 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3778: Fn10 = v0.clone();
                        c3778.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1040(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3783 = {
            let (h3782, _) = v1.clone();
            h3782
        };
        let a3784 = 1u64;
        nat_add(a3783, a3784)?
    }, {
        let a3787 = {
            let (_, h3785) = v1;
            h3785
        };
        let a3788 = f1046(f58(v0, {
            let (h3786, _) = v1;
            h3786
        })?)?;
        append_bytes(a3787, a3788)
    }))
}

pub fn f1041(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f1047(f1048(v0, v1)?)
}

pub fn f1042(v0: Bytes, v1: Bytes) -> Bytes {
    let a3795 = f1049({
        let a3789 = v0.clone();
        length_bytes(a3789)
    });
    let a3796 = {
        let a3793 = v0;
        let a3794 = {
            let a3791 = f1049({
                let a3790 = v1.clone();
                length_bytes(a3790)
            });
            let a3792 = v1;
            append_bytes(a3791, a3792)
        };
        append_bytes(a3793, a3794)
    };
    append_bytes(a3795, a3796)
}

pub fn f1043(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3799 = {
        let a3797 = v0.clone();
        length_bytes(a3797)
    };
    let a3800 = {
        let a3798 = v1.clone();
        length_bytes(a3798)
    };
    if nat_eq(a3799, a3800) {
        let a3801 = f1050(v0, v1)?;
        let a3802 = 0u64;
        Ok(nat_eq(a3801, a3802))
    } else {
        Ok(false)
    }
}

pub fn f1044(v0: Fn11, p3806: u64, p3807: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3805 = (p3806, p3807);
    {
        let mut m3805 = m3805;
        loop {
            let step = {
                let (v1, v2) = m3805;
                let m3803 = v1;
                if m3803 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3803 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3804: Fn11 = v0.clone();
                        c3804.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3805 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1045(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f1051(v0, v1)
}

pub fn f1046(v0: u64) -> R<Bytes> {
    let a3814 = f1052({
        let a3808 = v0;
        let a3809 = 16u64;
        let a3810 = 0u64;
        nat_quot(a3808, a3809, a3810)
    })?;
    let a3815 = f1052({
        let a3811 = v0;
        let a3812 = 16u64;
        let a3813 = 0u64;
        nat_rem(a3811, a3812, a3813)
    })?;
    Ok(append_bytes(a3814, a3815))
}

pub fn f1047(v0: Adt169) -> R<Bytes> {
    let a3836 = f1053(match v0.clone() {
        Adt169::C0(h3816, _, _, _, _, _, _, _) => {
            h3816
        }
    })?;
    let a3837 = {
        let a3834 = f1053(match v0.clone() {
            Adt169::C0(_, h3817, _, _, _, _, _, _) => {
                h3817
            }
        })?;
        let a3835 = {
            let a3832 = f1053(match v0.clone() {
                Adt169::C0(_, _, h3818, _, _, _, _, _) => {
                    h3818
                }
            })?;
            let a3833 = {
                let a3830 = f1053(match v0.clone() {
                    Adt169::C0(_, _, _, h3819, _, _, _, _) => {
                        h3819
                    }
                })?;
                let a3831 = {
                    let a3828 = f1053(match v0.clone() {
                        Adt169::C0(_, _, _, _, h3820, _, _, _) => {
                            h3820
                        }
                    })?;
                    let a3829 = {
                        let a3826 = f1053(match v0.clone() {
                            Adt169::C0(_, _, _, _, _, h3821, _, _) => {
                                h3821
                            }
                        })?;
                        let a3827 = {
                            let a3824 = f1053(match v0.clone() {
                                Adt169::C0(_, _, _, _, _, _, h3822, _) => {
                                    h3822
                                }
                            })?;
                            let a3825 = f1053(match v0 {
                                Adt169::C0(_, _, _, _, _, _, _, h3823) => {
                                    h3823
                                }
                            })?;
                            append_bytes(a3824, a3825)
                        };
                        append_bytes(a3826, a3827)
                    };
                    append_bytes(a3828, a3829)
                };
                append_bytes(a3830, a3831)
            };
            append_bytes(a3832, a3833)
        };
        append_bytes(a3834, a3835)
    };
    Ok(append_bytes(a3836, a3837))
}

pub fn f1048(v0: Bytes, v1: Bytes) -> R<Adt169> {
    let v2: Adt170 = f1054(v0)?;
    let v3: Adt169 = f1055(f1056(f1057(), f1058(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f1056(f1056(f1057(), f1058(v2, 1549556828u32))?, f1059(v3))
}

pub fn f1049(v0: u64) -> Bytes {
    let a3863 = f468({
        let a3841 = {
            let a3838 = v0;
            let a3839 = 16777216u64;
            let a3840 = 0u64;
            nat_quot(a3838, a3839, a3840)
        };
        let a3842 = 256u64;
        let a3843 = 0u64;
        nat_rem(a3841, a3842, a3843)
    });
    let a3864 = {
        let a3861 = f468({
            let a3847 = {
                let a3844 = v0;
                let a3845 = 65536u64;
                let a3846 = 0u64;
                nat_quot(a3844, a3845, a3846)
            };
            let a3848 = 256u64;
            let a3849 = 0u64;
            nat_rem(a3847, a3848, a3849)
        });
        let a3862 = {
            let a3859 = f468({
                let a3853 = {
                    let a3850 = v0;
                    let a3851 = 256u64;
                    let a3852 = 0u64;
                    nat_quot(a3850, a3851, a3852)
                };
                let a3854 = 256u64;
                let a3855 = 0u64;
                nat_rem(a3853, a3854, a3855)
            });
            let a3860 = f468({
                let a3856 = v0;
                let a3857 = 256u64;
                let a3858 = 0u64;
                nat_rem(a3856, a3857, a3858)
            });
            append_bytes(a3859, a3860)
        };
        append_bytes(a3861, a3862)
    };
    append_bytes(a3863, a3864)
}

pub fn f1050(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3865) = f1034(Fn9::F1060(v0.clone(), v1), {
        let a3866 = v0;
        length_bytes(a3866)
    }, (0u64, 0u64))?;
    Ok(h3865)
}

pub fn f1051(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f1061(f58(v0.clone(), {
        let (h3867, _) = v1.clone();
        h3867
    })?);
    let v3: u64 = f1061(f58(v0, {
        let a3869 = {
            let (h3868, _) = v1.clone();
            h3868
        };
        let a3870 = 1u64;
        nat_add(a3869, a3870)?
    })?);
    let a3871 = v2;
    let a3872 = 16u64;
    if if nat_lt(a3871, a3872) {
        let a3873 = v3;
        let a3874 = 16u64;
        nat_lt(a3873, a3874)
    } else {
        false
    } {
        Ok(({
            let a3876 = {
                let (h3875, _) = v1.clone();
                h3875
            };
            let a3877 = 2u64;
            nat_add(a3876, a3877)?
        }, ({
            let a3884 = {
                let (h3878, _) = {
                    let (_, h3879) = v1.clone();
                    h3879
                };
                h3878
            };
            let a3885 = f468({
                let a3882 = {
                    let a3880 = v2;
                    let a3881 = 16u64;
                    nat_mul(a3880, a3881)?
                };
                let a3883 = v3;
                nat_add(a3882, a3883)?
            });
            append_bytes(a3884, a3885)
        }, {
            let (_, h3886) = {
                let (_, h3887) = v1;
                h3887
            };
            h3886
        })))
    } else {
        Ok(({
            let a3889 = {
                let (h3888, _) = v1.clone();
                h3888
            };
            let a3890 = 2u64;
            nat_add(a3889, a3890)?
        }, ({
            let (h3891, _) = {
                let (_, h3892) = v1.clone();
                h3892
            };
            h3891
        }, {
            let a3895 = {
                let (_, h3893) = {
                    let (_, h3894) = v1;
                    h3894
                };
                h3893
            };
            let a3896 = 1u64;
            nat_add(a3895, a3896)?
        })))
    }
}

pub fn f1052(v0: u64) -> R<Bytes> {
    Ok(f468({
        let a3897 = v0;
        let a3898 = 10u64;
        if nat_lt(a3897, a3898) {
            let a3899 = 48u64;
            let a3900 = v0;
            nat_add(a3899, a3900)?
        } else {
            let a3901 = 87u64;
            let a3902 = v0;
            nat_add(a3901, a3902)?
        }
    }))
}

pub fn f1053(v0: u32) -> R<Bytes> {
    let a3907 = f1062(v0, 24u32)?;
    let a3908 = {
        let a3905 = f1062(v0, 16u32)?;
        let a3906 = {
            let a3903 = f1062(v0, 8u32)?;
            let a3904 = f1062(v0, 0u32)?;
            append_bytes(a3903, a3904)
        };
        append_bytes(a3905, a3906)
    };
    Ok(append_bytes(a3907, a3908))
}

pub fn f1054(v0: Bytes) -> R<Adt170> {
    let a3910 = f1063();
    let a3911 = {
        let a3909 = v0.clone();
        length_bytes(a3909)
    };
    if nat_lt(a3910, a3911) {
        Ok(f1064(f1065(v0)?))
    } else {
        f1066(v0)
    }
}

pub fn f1055(v0: Adt169, v1: u64, v2: Bytes) -> R<Adt169> {
    let v3: u64 = {
        let a3912 = v2.clone();
        length_bytes(a3912)
    };
    let v4: u64 = {
        let a3918 = {
            let a3915 = {
                let a3913 = v3;
                let a3914 = 72u64;
                nat_add(a3913, a3914)?
            };
            let a3916 = 64u64;
            let a3917 = 0u64;
            nat_quot(a3915, a3916, a3917)
        };
        let a3919 = 64u64;
        nat_mul(a3918, a3919)?
    };
    let (_, h3920) = f1067(Fn12::F1068(v2, v1, v3, v4), {
        let a3921 = v4;
        let a3922 = 64u64;
        let a3923 = 0u64;
        nat_quot(a3921, a3922, a3923)
    }, (0u64, v0))?;
    Ok(h3920)
}

pub fn f1056(v0: Adt169, v1: Adt170) -> R<Adt169> {
    let v2: Adt171 = f1069(Fn13::F1070, 64u64, Adt171::C0(match v0.clone() {
        Adt169::C0(h3924, _, _, _, _, _, _, _) => {
            h3924
        }
    }, match v0.clone() {
        Adt169::C0(_, h3925, _, _, _, _, _, _) => {
            h3925
        }
    }, match v0.clone() {
        Adt169::C0(_, _, h3926, _, _, _, _, _) => {
            h3926
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, h3927, _, _, _, _) => {
            h3927
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, h3928, _, _, _) => {
            h3928
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, h3929, _, _) => {
            h3929
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, h3930, _) => {
            h3930
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, _, h3931) => {
            h3931
        }
    }, v1, 0u64))?;
    Ok(Adt169::C0(f1071(match v0.clone() {
        Adt169::C0(h3932, _, _, _, _, _, _, _) => {
            h3932
        }
    }, match v2.clone() {
        Adt171::C0(h3933, _, _, _, _, _, _, _, _, _) => {
            h3933
        }
    }), f1071(match v0.clone() {
        Adt169::C0(_, h3934, _, _, _, _, _, _) => {
            h3934
        }
    }, match v2.clone() {
        Adt171::C0(_, h3935, _, _, _, _, _, _, _, _) => {
            h3935
        }
    }), f1071(match v0.clone() {
        Adt169::C0(_, _, h3936, _, _, _, _, _) => {
            h3936
        }
    }, match v2.clone() {
        Adt171::C0(_, _, h3937, _, _, _, _, _, _, _) => {
            h3937
        }
    }), f1071(match v0.clone() {
        Adt169::C0(_, _, _, h3938, _, _, _, _) => {
            h3938
        }
    }, match v2.clone() {
        Adt171::C0(_, _, _, h3939, _, _, _, _, _, _) => {
            h3939
        }
    }), f1071(match v0.clone() {
        Adt169::C0(_, _, _, _, h3940, _, _, _) => {
            h3940
        }
    }, match v2.clone() {
        Adt171::C0(_, _, _, _, h3941, _, _, _, _, _) => {
            h3941
        }
    }), f1071(match v0.clone() {
        Adt169::C0(_, _, _, _, _, h3942, _, _) => {
            h3942
        }
    }, match v2.clone() {
        Adt171::C0(_, _, _, _, _, h3943, _, _, _, _) => {
            h3943
        }
    }), f1071(match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, h3944, _) => {
            h3944
        }
    }, match v2.clone() {
        Adt171::C0(_, _, _, _, _, _, h3945, _, _, _) => {
            h3945
        }
    }), f1071(match v0 {
        Adt169::C0(_, _, _, _, _, _, _, h3946) => {
            h3946
        }
    }, match v2 {
        Adt171::C0(_, _, _, _, _, _, _, h3947, _, _) => {
            h3947
        }
    })))
}

pub fn f1057() -> Adt169 {
    Adt169::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f1058(v0: Adt170, v1: u32) -> Adt170 {
    Adt170::C0({
        let a3949 = match v0.clone() {
            Adt170::C0(h3948, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3948
            }
        };
        let a3950 = v1;
        fixed_u32::bit_xor(a3949, a3950)
    }, {
        let a3952 = match v0.clone() {
            Adt170::C0(_, h3951, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3951
            }
        };
        let a3953 = v1;
        fixed_u32::bit_xor(a3952, a3953)
    }, {
        let a3955 = match v0.clone() {
            Adt170::C0(_, _, h3954, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3954
            }
        };
        let a3956 = v1;
        fixed_u32::bit_xor(a3955, a3956)
    }, {
        let a3958 = match v0.clone() {
            Adt170::C0(_, _, _, h3957, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3957
            }
        };
        let a3959 = v1;
        fixed_u32::bit_xor(a3958, a3959)
    }, {
        let a3961 = match v0.clone() {
            Adt170::C0(_, _, _, _, h3960, _, _, _, _, _, _, _, _, _, _, _) => {
                h3960
            }
        };
        let a3962 = v1;
        fixed_u32::bit_xor(a3961, a3962)
    }, {
        let a3964 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, h3963, _, _, _, _, _, _, _, _, _, _) => {
                h3963
            }
        };
        let a3965 = v1;
        fixed_u32::bit_xor(a3964, a3965)
    }, {
        let a3967 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, h3966, _, _, _, _, _, _, _, _, _) => {
                h3966
            }
        };
        let a3968 = v1;
        fixed_u32::bit_xor(a3967, a3968)
    }, {
        let a3970 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, h3969, _, _, _, _, _, _, _, _) => {
                h3969
            }
        };
        let a3971 = v1;
        fixed_u32::bit_xor(a3970, a3971)
    }, {
        let a3973 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, h3972, _, _, _, _, _, _, _) => {
                h3972
            }
        };
        let a3974 = v1;
        fixed_u32::bit_xor(a3973, a3974)
    }, {
        let a3976 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, h3975, _, _, _, _, _, _) => {
                h3975
            }
        };
        let a3977 = v1;
        fixed_u32::bit_xor(a3976, a3977)
    }, {
        let a3979 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, h3978, _, _, _, _, _) => {
                h3978
            }
        };
        let a3980 = v1;
        fixed_u32::bit_xor(a3979, a3980)
    }, {
        let a3982 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, h3981, _, _, _, _) => {
                h3981
            }
        };
        let a3983 = v1;
        fixed_u32::bit_xor(a3982, a3983)
    }, {
        let a3985 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3984, _, _, _) => {
                h3984
            }
        };
        let a3986 = v1;
        fixed_u32::bit_xor(a3985, a3986)
    }, {
        let a3988 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3987, _, _) => {
                h3987
            }
        };
        let a3989 = v1;
        fixed_u32::bit_xor(a3988, a3989)
    }, {
        let a3991 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3990, _) => {
                h3990
            }
        };
        let a3992 = v1;
        fixed_u32::bit_xor(a3991, a3992)
    }, {
        let a3994 = match v0 {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3993) => {
                h3993
            }
        };
        let a3995 = v1;
        fixed_u32::bit_xor(a3994, a3995)
    })
}

pub fn f1059(v0: Adt169) -> Adt170 {
    Adt170::C0(match v0.clone() {
        Adt169::C0(h3996, _, _, _, _, _, _, _) => {
            h3996
        }
    }, match v0.clone() {
        Adt169::C0(_, h3997, _, _, _, _, _, _) => {
            h3997
        }
    }, match v0.clone() {
        Adt169::C0(_, _, h3998, _, _, _, _, _) => {
            h3998
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, h3999, _, _, _, _) => {
            h3999
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, h4000, _, _, _) => {
            h4000
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, h4001, _, _) => {
            h4001
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, h4002, _) => {
            h4002
        }
    }, match v0 {
        Adt169::C0(_, _, _, _, _, _, _, h4003) => {
            h4003
        }
    }, 2147483648u32, f1072(), f1072(), f1072(), f1072(), f1072(), f1072(), 768u32)
}

pub fn f1060(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a4005 = {
            let (h4004, _) = v2;
            h4004
        };
        let a4006 = 1u64;
        nat_add(a4005, a4006)?
    }, {
        let a4058 = {
            let (_, h4007) = v2;
            h4007
        };
        let a4059 = {
            let v3: u8 = {
                let a4010 = f1073(v0, {
                    let (h4008, _) = v2;
                    h4008
                });
                let a4011 = f1073(v1, {
                    let (h4009, _) = v2;
                    h4009
                });
                fixed_u8::bit_xor(a4010, a4011)
            };
            let a4056 = {
                let a4032 = {
                    let a4020 = {
                        let a4014 = {
                            let a4012 = v3;
                            let a4013 = 128u8;
                            fixed_u8::bit_and(a4012, a4013)
                        };
                        let a4015 = 128u8;
                        if equal(a4014, a4015) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a4021 = {
                        let a4018 = {
                            let a4016 = v3;
                            let a4017 = 64u8;
                            fixed_u8::bit_and(a4016, a4017)
                        };
                        let a4019 = 64u8;
                        if equal(a4018, a4019) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4020, a4021)?
                };
                let a4033 = {
                    let a4030 = {
                        let a4024 = {
                            let a4022 = v3;
                            let a4023 = 32u8;
                            fixed_u8::bit_and(a4022, a4023)
                        };
                        let a4025 = 32u8;
                        if equal(a4024, a4025) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a4031 = {
                        let a4028 = {
                            let a4026 = v3;
                            let a4027 = 16u8;
                            fixed_u8::bit_and(a4026, a4027)
                        };
                        let a4029 = 16u8;
                        if equal(a4028, a4029) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4030, a4031)?
                };
                nat_add(a4032, a4033)?
            };
            let a4057 = {
                let a4054 = {
                    let a4042 = {
                        let a4036 = {
                            let a4034 = v3;
                            let a4035 = 8u8;
                            fixed_u8::bit_and(a4034, a4035)
                        };
                        let a4037 = 8u8;
                        if equal(a4036, a4037) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a4043 = {
                        let a4040 = {
                            let a4038 = v3;
                            let a4039 = 4u8;
                            fixed_u8::bit_and(a4038, a4039)
                        };
                        let a4041 = 4u8;
                        if equal(a4040, a4041) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4042, a4043)?
                };
                let a4055 = {
                    let a4052 = {
                        let a4046 = {
                            let a4044 = v3;
                            let a4045 = 2u8;
                            fixed_u8::bit_and(a4044, a4045)
                        };
                        let a4047 = 2u8;
                        if equal(a4046, a4047) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a4053 = {
                        let a4050 = {
                            let a4048 = v3;
                            let a4049 = 1u8;
                            fixed_u8::bit_and(a4048, a4049)
                        };
                        let a4051 = 1u8;
                        if equal(a4050, a4051) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4052, a4053)?
                };
                nat_add(a4054, a4055)?
            };
            nat_add(a4056, a4057)?
        };
        nat_add(a4058, a4059)?
    }))
}

pub fn f1061(v0: u64) -> u64 {
    let a4060 = v0;
    let a4061 = 48u64;
    if nat_lt(a4060, a4061) {
        16u64
    } else {
        let a4062 = v0;
        let a4063 = 58u64;
        if nat_lt(a4062, a4063) {
            let a4064 = v0;
            let a4065 = 48u64;
            nat_sub(a4064, a4065)
        } else {
            let a4066 = v0;
            let a4067 = 97u64;
            if nat_lt(a4066, a4067) {
                16u64
            } else {
                let a4068 = v0;
                let a4069 = 103u64;
                if nat_lt(a4068, a4069) {
                    let a4070 = v0;
                    let a4071 = 87u64;
                    nat_sub(a4070, a4071)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f1062(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f468({
        let v2: u32 = {
            let a4072 = f1074(v0, v1);
            let a4073 = 255u32;
            fixed_u32::bit_and(a4072, a4073)
        };
        let a4262 = {
            let a4166 = {
                let a4118 = {
                    let a4094 = {
                        let a4082 = {
                            let a4076 = {
                                let a4074 = v2;
                                let a4075 = 2147483648u32;
                                fixed_u32::bit_and(a4074, a4075)
                            };
                            let a4077 = 2147483648u32;
                            if equal(a4076, a4077) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a4083 = {
                            let a4080 = {
                                let a4078 = v2;
                                let a4079 = 1073741824u32;
                                fixed_u32::bit_and(a4078, a4079)
                            };
                            let a4081 = 1073741824u32;
                            if equal(a4080, a4081) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4082, a4083)?
                    };
                    let a4095 = {
                        let a4092 = {
                            let a4086 = {
                                let a4084 = v2;
                                let a4085 = 536870912u32;
                                fixed_u32::bit_and(a4084, a4085)
                            };
                            let a4087 = 536870912u32;
                            if equal(a4086, a4087) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a4093 = {
                            let a4090 = {
                                let a4088 = v2;
                                let a4089 = 268435456u32;
                                fixed_u32::bit_and(a4088, a4089)
                            };
                            let a4091 = 268435456u32;
                            if equal(a4090, a4091) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4092, a4093)?
                    };
                    nat_add(a4094, a4095)?
                };
                let a4119 = {
                    let a4116 = {
                        let a4104 = {
                            let a4098 = {
                                let a4096 = v2;
                                let a4097 = 134217728u32;
                                fixed_u32::bit_and(a4096, a4097)
                            };
                            let a4099 = 134217728u32;
                            if equal(a4098, a4099) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a4105 = {
                            let a4102 = {
                                let a4100 = v2;
                                let a4101 = 67108864u32;
                                fixed_u32::bit_and(a4100, a4101)
                            };
                            let a4103 = 67108864u32;
                            if equal(a4102, a4103) {
                                67108864u64
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
                                let a4107 = 33554432u32;
                                fixed_u32::bit_and(a4106, a4107)
                            };
                            let a4109 = 33554432u32;
                            if equal(a4108, a4109) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a4115 = {
                            let a4112 = {
                                let a4110 = v2;
                                let a4111 = 16777216u32;
                                fixed_u32::bit_and(a4110, a4111)
                            };
                            let a4113 = 16777216u32;
                            if equal(a4112, a4113) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4114, a4115)?
                    };
                    nat_add(a4116, a4117)?
                };
                nat_add(a4118, a4119)?
            };
            let a4167 = {
                let a4164 = {
                    let a4140 = {
                        let a4128 = {
                            let a4122 = {
                                let a4120 = v2;
                                let a4121 = 8388608u32;
                                fixed_u32::bit_and(a4120, a4121)
                            };
                            let a4123 = 8388608u32;
                            if equal(a4122, a4123) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a4129 = {
                            let a4126 = {
                                let a4124 = v2;
                                let a4125 = 4194304u32;
                                fixed_u32::bit_and(a4124, a4125)
                            };
                            let a4127 = 4194304u32;
                            if equal(a4126, a4127) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4128, a4129)?
                    };
                    let a4141 = {
                        let a4138 = {
                            let a4132 = {
                                let a4130 = v2;
                                let a4131 = 2097152u32;
                                fixed_u32::bit_and(a4130, a4131)
                            };
                            let a4133 = 2097152u32;
                            if equal(a4132, a4133) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a4139 = {
                            let a4136 = {
                                let a4134 = v2;
                                let a4135 = 1048576u32;
                                fixed_u32::bit_and(a4134, a4135)
                            };
                            let a4137 = 1048576u32;
                            if equal(a4136, a4137) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4138, a4139)?
                    };
                    nat_add(a4140, a4141)?
                };
                let a4165 = {
                    let a4162 = {
                        let a4150 = {
                            let a4144 = {
                                let a4142 = v2;
                                let a4143 = 524288u32;
                                fixed_u32::bit_and(a4142, a4143)
                            };
                            let a4145 = 524288u32;
                            if equal(a4144, a4145) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a4151 = {
                            let a4148 = {
                                let a4146 = v2;
                                let a4147 = 262144u32;
                                fixed_u32::bit_and(a4146, a4147)
                            };
                            let a4149 = 262144u32;
                            if equal(a4148, a4149) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4150, a4151)?
                    };
                    let a4163 = {
                        let a4160 = {
                            let a4154 = {
                                let a4152 = v2;
                                let a4153 = 131072u32;
                                fixed_u32::bit_and(a4152, a4153)
                            };
                            let a4155 = 131072u32;
                            if equal(a4154, a4155) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a4161 = {
                            let a4158 = {
                                let a4156 = v2;
                                let a4157 = 65536u32;
                                fixed_u32::bit_and(a4156, a4157)
                            };
                            let a4159 = 65536u32;
                            if equal(a4158, a4159) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4160, a4161)?
                    };
                    nat_add(a4162, a4163)?
                };
                nat_add(a4164, a4165)?
            };
            nat_add(a4166, a4167)?
        };
        let a4263 = {
            let a4260 = {
                let a4212 = {
                    let a4188 = {
                        let a4176 = {
                            let a4170 = {
                                let a4168 = v2;
                                let a4169 = 32768u32;
                                fixed_u32::bit_and(a4168, a4169)
                            };
                            let a4171 = 32768u32;
                            if equal(a4170, a4171) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a4177 = {
                            let a4174 = {
                                let a4172 = v2;
                                let a4173 = 16384u32;
                                fixed_u32::bit_and(a4172, a4173)
                            };
                            let a4175 = 16384u32;
                            if equal(a4174, a4175) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4176, a4177)?
                    };
                    let a4189 = {
                        let a4186 = {
                            let a4180 = {
                                let a4178 = v2;
                                let a4179 = 8192u32;
                                fixed_u32::bit_and(a4178, a4179)
                            };
                            let a4181 = 8192u32;
                            if equal(a4180, a4181) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a4187 = {
                            let a4184 = {
                                let a4182 = v2;
                                let a4183 = 4096u32;
                                fixed_u32::bit_and(a4182, a4183)
                            };
                            let a4185 = 4096u32;
                            if equal(a4184, a4185) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4186, a4187)?
                    };
                    nat_add(a4188, a4189)?
                };
                let a4213 = {
                    let a4210 = {
                        let a4198 = {
                            let a4192 = {
                                let a4190 = v2;
                                let a4191 = 2048u32;
                                fixed_u32::bit_and(a4190, a4191)
                            };
                            let a4193 = 2048u32;
                            if equal(a4192, a4193) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a4199 = {
                            let a4196 = {
                                let a4194 = v2;
                                let a4195 = 1024u32;
                                fixed_u32::bit_and(a4194, a4195)
                            };
                            let a4197 = 1024u32;
                            if equal(a4196, a4197) {
                                1024u64
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
                                let a4201 = 512u32;
                                fixed_u32::bit_and(a4200, a4201)
                            };
                            let a4203 = 512u32;
                            if equal(a4202, a4203) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a4209 = {
                            let a4206 = {
                                let a4204 = v2;
                                let a4205 = 256u32;
                                fixed_u32::bit_and(a4204, a4205)
                            };
                            let a4207 = 256u32;
                            if equal(a4206, a4207) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4208, a4209)?
                    };
                    nat_add(a4210, a4211)?
                };
                nat_add(a4212, a4213)?
            };
            let a4261 = {
                let a4258 = {
                    let a4234 = {
                        let a4222 = {
                            let a4216 = {
                                let a4214 = v2;
                                let a4215 = 128u32;
                                fixed_u32::bit_and(a4214, a4215)
                            };
                            let a4217 = 128u32;
                            if equal(a4216, a4217) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a4223 = {
                            let a4220 = {
                                let a4218 = v2;
                                let a4219 = 64u32;
                                fixed_u32::bit_and(a4218, a4219)
                            };
                            let a4221 = 64u32;
                            if equal(a4220, a4221) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4222, a4223)?
                    };
                    let a4235 = {
                        let a4232 = {
                            let a4226 = {
                                let a4224 = v2;
                                let a4225 = 32u32;
                                fixed_u32::bit_and(a4224, a4225)
                            };
                            let a4227 = 32u32;
                            if equal(a4226, a4227) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a4233 = {
                            let a4230 = {
                                let a4228 = v2;
                                let a4229 = 16u32;
                                fixed_u32::bit_and(a4228, a4229)
                            };
                            let a4231 = 16u32;
                            if equal(a4230, a4231) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4232, a4233)?
                    };
                    nat_add(a4234, a4235)?
                };
                let a4259 = {
                    let a4256 = {
                        let a4244 = {
                            let a4238 = {
                                let a4236 = v2;
                                let a4237 = 8u32;
                                fixed_u32::bit_and(a4236, a4237)
                            };
                            let a4239 = 8u32;
                            if equal(a4238, a4239) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a4245 = {
                            let a4242 = {
                                let a4240 = v2;
                                let a4241 = 4u32;
                                fixed_u32::bit_and(a4240, a4241)
                            };
                            let a4243 = 4u32;
                            if equal(a4242, a4243) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4244, a4245)?
                    };
                    let a4257 = {
                        let a4254 = {
                            let a4248 = {
                                let a4246 = v2;
                                let a4247 = 2u32;
                                fixed_u32::bit_and(a4246, a4247)
                            };
                            let a4249 = 2u32;
                            if equal(a4248, a4249) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a4255 = {
                            let a4252 = {
                                let a4250 = v2;
                                let a4251 = 1u32;
                                fixed_u32::bit_and(a4250, a4251)
                            };
                            let a4253 = 1u32;
                            if equal(a4252, a4253) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4254, a4255)?
                    };
                    nat_add(a4256, a4257)?
                };
                nat_add(a4258, a4259)?
            };
            nat_add(a4260, a4261)?
        };
        nat_add(a4262, a4263)?
    }))
}

pub fn f1063() -> u64 {
    64u64
}

pub fn f1064(v0: Adt169) -> Adt170 {
    Adt170::C0(match v0.clone() {
        Adt169::C0(h4264, _, _, _, _, _, _, _) => {
            h4264
        }
    }, match v0.clone() {
        Adt169::C0(_, h4265, _, _, _, _, _, _) => {
            h4265
        }
    }, match v0.clone() {
        Adt169::C0(_, _, h4266, _, _, _, _, _) => {
            h4266
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, h4267, _, _, _, _) => {
            h4267
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, h4268, _, _, _) => {
            h4268
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, h4269, _, _) => {
            h4269
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, h4270, _) => {
            h4270
        }
    }, match v0 {
        Adt169::C0(_, _, _, _, _, _, _, h4271) => {
            h4271
        }
    }, f1072(), f1072(), f1072(), f1072(), f1072(), f1072(), f1072(), f1072())
}

pub fn f1065(v0: Bytes) -> R<Adt169> {
    let v1: u64 = {
        let a4272 = v0.clone();
        length_bytes(a4272)
    };
    let v2: u64 = {
        let a4278 = {
            let a4275 = {
                let a4273 = v1;
                let a4274 = 72u64;
                nat_add(a4273, a4274)?
            };
            let a4276 = 64u64;
            let a4277 = 0u64;
            nat_quot(a4275, a4276, a4277)
        };
        let a4279 = 64u64;
        nat_mul(a4278, a4279)?
    };
    let (_, h4280) = f1067(Fn12::F1075(v0, v1, v2), {
        let a4281 = v2;
        let a4282 = 64u64;
        let a4283 = 0u64;
        nat_quot(a4281, a4282, a4283)
    }, (0u64, f1057()))?;
    Ok(h4280)
}

pub fn f1066(v0: Bytes) -> R<Adt170> {
    Ok(Adt170::C0(f1076(v0.clone(), 0u64)?, f1076(v0.clone(), 4u64)?, f1076(v0.clone(), 8u64)?, f1076(v0.clone(), 12u64)?, f1076(v0.clone(), 16u64)?, f1076(v0.clone(), 20u64)?, f1076(v0.clone(), 24u64)?, f1076(v0.clone(), 28u64)?, f1076(v0.clone(), 32u64)?, f1076(v0.clone(), 36u64)?, f1076(v0.clone(), 40u64)?, f1076(v0.clone(), 44u64)?, f1076(v0.clone(), 48u64)?, f1076(v0.clone(), 52u64)?, f1076(v0.clone(), 56u64)?, f1076(v0, 60u64)?))
}

pub fn f1067(v0: Fn12, p4287: u64, p4288: (u64, Adt169)) -> R<(u64, Adt169)> {
    let m4286 = (p4287, p4288);
    {
        let mut m4286 = m4286;
        loop {
            let step = {
                let (v1, v2) = m4286;
                let m4284 = v1;
                if m4284 == 0 {
                    Err::<(u64, (u64, Adt169)), R<(u64, Adt169)>>(Ok(v2))
                } else {
                    let v3 = m4284 - 1;
                    Ok::<(u64, (u64, Adt169)), R<(u64, Adt169)>>((v3, {
                        let c4285: Fn12 = v0.clone();
                        c4285.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1068(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt169)) -> R<(u64, Adt169)> {
    Ok(({
        let a4290 = {
            let (h4289, _) = v4.clone();
            h4289
        };
        let a4291 = 64u64;
        nat_add(a4290, a4291)?
    }, f1056({
        let (_, h4292) = v4;
        h4292
    }, f1077(v0, v2, v3, {
        let a4295 = {
            let a4293 = v1;
            let a4294 = v2;
            nat_add(a4293, a4294)?
        };
        let a4296 = 8u64;
        nat_mul(a4295, a4296)?
    }, {
        let (h4297, _) = v4;
        h4297
    })?)?))
}

pub fn f1069(v0: Fn13, p4301: u64, p4302: Adt171) -> R<Adt171> {
    let m4300 = (p4301, p4302);
    {
        let mut m4300 = m4300;
        loop {
            let step = {
                let (v1, v2) = m4300;
                let m4298 = v1;
                if m4298 == 0 {
                    Err::<(u64, Adt171), R<Adt171>>(Ok(v2))
                } else {
                    let v3 = m4298 - 1;
                    Ok::<(u64, Adt171), R<Adt171>>((v3, {
                        let c4299: Fn13 = v0.clone();
                        c4299.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1070(v0: Adt171) -> R<Adt171> {
    f1078(v0)
}

pub fn f1071(v0: u32, v1: u32) -> u32 {
    let m4305 = {
        let a4303 = f1079(v0);
        let a4304 = f1079(v1);
        fixed_u64::checked_add(a4303, a4304)
    };
    match m4305 {
        None => {
            0u32
        }
        Some(v2) => {
            f1080(v2)
        }
    }
}

pub fn f1072() -> u32 {
    0u32
}

pub fn f1073(v0: Bytes, v1: u64) -> u8 {
    let m4308 = {
        let a4306 = v0;
        let a4307 = v1;
        index_bytes(a4306, a4307)
    };
    match m4308 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1074(v0: u32, v1: u32) -> u32 {
    let m4311 = {
        let a4309 = v0;
        let a4310 = v1;
        fixed_u32::shift_right(a4309, a4310)
    };
    match m4311 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1075(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt169)) -> R<(u64, Adt169)> {
    Ok(({
        let a4313 = {
            let (h4312, _) = v3.clone();
            h4312
        };
        let a4314 = 64u64;
        nat_add(a4313, a4314)?
    }, f1056({
        let (_, h4315) = v3;
        h4315
    }, f1081(v0, v1, v2, {
        let (h4316, _) = v3;
        h4316
    })?)?))
}

pub fn f1076(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f1082(f1083(v0.clone(), v1), f1083(v0.clone(), {
        let a4317 = v1;
        let a4318 = 1u64;
        nat_add(a4317, a4318)?
    }), f1083(v0.clone(), {
        let a4319 = v1;
        let a4320 = 2u64;
        nat_add(a4319, a4320)?
    }), f1083(v0, {
        let a4321 = v1;
        let a4322 = 3u64;
        nat_add(a4321, a4322)?
    })))
}

pub fn f1077(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt170> {
    Ok(Adt170::C0(f1084(v0.clone(), v1, v2, v3, v4)?, f1084(v0.clone(), v1, v2, v3, {
        let a4323 = v4;
        let a4324 = 4u64;
        nat_add(a4323, a4324)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4325 = v4;
        let a4326 = 8u64;
        nat_add(a4325, a4326)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4327 = v4;
        let a4328 = 12u64;
        nat_add(a4327, a4328)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4329 = v4;
        let a4330 = 16u64;
        nat_add(a4329, a4330)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4331 = v4;
        let a4332 = 20u64;
        nat_add(a4331, a4332)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4333 = v4;
        let a4334 = 24u64;
        nat_add(a4333, a4334)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4335 = v4;
        let a4336 = 28u64;
        nat_add(a4335, a4336)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4337 = v4;
        let a4338 = 32u64;
        nat_add(a4337, a4338)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4339 = v4;
        let a4340 = 36u64;
        nat_add(a4339, a4340)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4341 = v4;
        let a4342 = 40u64;
        nat_add(a4341, a4342)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4343 = v4;
        let a4344 = 44u64;
        nat_add(a4343, a4344)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4345 = v4;
        let a4346 = 48u64;
        nat_add(a4345, a4346)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4347 = v4;
        let a4348 = 52u64;
        nat_add(a4347, a4348)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4349 = v4;
        let a4350 = 56u64;
        nat_add(a4349, a4350)?
    })?, f1084(v0, v1, v2, v3, {
        let a4351 = v4;
        let a4352 = 60u64;
        nat_add(a4351, a4352)?
    })?))
}

pub fn f1078(v0: Adt171) -> R<Adt171> {
    let v1: u32 = f1085(match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, _, _, _, h4353) => {
            h4353
        }
    });
    let v2: u32 = f1071(match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, _, h4354, _, _) => {
            h4354
        }
    }, f1071(f1086(match v0.clone() {
        Adt171::C0(_, _, _, _, h4355, _, _, _, _, _) => {
            h4355
        }
    }), f1071(f1087(match v0.clone() {
        Adt171::C0(_, _, _, _, h4356, _, _, _, _, _) => {
            h4356
        }
    }, match v0.clone() {
        Adt171::C0(_, _, _, _, _, h4357, _, _, _, _) => {
            h4357
        }
    }, match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, h4358, _, _, _) => {
            h4358
        }
    }), f1071(v1, {
        let m4361 = match v0.clone() {
            Adt171::C0(_, _, _, _, _, _, _, _, h4360, _) => {
                h4360
            }
        };
        match m4361 {
            Adt170::C0(h4359, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4359
            }
        }
    }))));
    let v3: u32 = f1071(f1088(match v0.clone() {
        Adt171::C0(h4362, _, _, _, _, _, _, _, _, _) => {
            h4362
        }
    }), f1089(match v0.clone() {
        Adt171::C0(h4363, _, _, _, _, _, _, _, _, _) => {
            h4363
        }
    }, match v0.clone() {
        Adt171::C0(_, h4364, _, _, _, _, _, _, _, _) => {
            h4364
        }
    }, match v0.clone() {
        Adt171::C0(_, _, h4365, _, _, _, _, _, _, _) => {
            h4365
        }
    }));
    Ok(Adt171::C0(f1071(v2, v3), match v0.clone() {
        Adt171::C0(h4366, _, _, _, _, _, _, _, _, _) => {
            h4366
        }
    }, match v0.clone() {
        Adt171::C0(_, h4367, _, _, _, _, _, _, _, _) => {
            h4367
        }
    }, match v0.clone() {
        Adt171::C0(_, _, h4368, _, _, _, _, _, _, _) => {
            h4368
        }
    }, f1071(match v0.clone() {
        Adt171::C0(_, _, _, h4369, _, _, _, _, _, _) => {
            h4369
        }
    }, v2), match v0.clone() {
        Adt171::C0(_, _, _, _, h4370, _, _, _, _, _) => {
            h4370
        }
    }, match v0.clone() {
        Adt171::C0(_, _, _, _, _, h4371, _, _, _, _) => {
            h4371
        }
    }, match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, h4372, _, _, _) => {
            h4372
        }
    }, f1090(match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, _, _, h4373, _) => {
            h4373
        }
    }), {
        let a4375 = match v0 {
            Adt171::C0(_, _, _, _, _, _, _, _, _, h4374) => {
                h4374
            }
        };
        let a4376 = 1u64;
        nat_add(a4375, a4376)?
    }))
}

pub fn f1079(v0: u32) -> u64 {
    let m4378 = {
        let a4377 = v0;
        fixed_u64::convert(i128::from(a4377))
    };
    match m4378 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1080(v0: u64) -> u32 {
    let m4382 = {
        let a4381 = {
            let a4379 = v0;
            let a4380 = 4294967295u64;
            fixed_u64::bit_and(a4379, a4380)
        };
        fixed_u32::convert(i128::from(a4381))
    };
    match m4382 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1081(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt170> {
    Ok(Adt170::C0(f1091(v0.clone(), v1, v2, v3)?, f1091(v0.clone(), v1, v2, {
        let a4383 = v3;
        let a4384 = 4u64;
        nat_add(a4383, a4384)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4385 = v3;
        let a4386 = 8u64;
        nat_add(a4385, a4386)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4387 = v3;
        let a4388 = 12u64;
        nat_add(a4387, a4388)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4389 = v3;
        let a4390 = 16u64;
        nat_add(a4389, a4390)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4391 = v3;
        let a4392 = 20u64;
        nat_add(a4391, a4392)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4393 = v3;
        let a4394 = 24u64;
        nat_add(a4393, a4394)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4395 = v3;
        let a4396 = 28u64;
        nat_add(a4395, a4396)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4397 = v3;
        let a4398 = 32u64;
        nat_add(a4397, a4398)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4399 = v3;
        let a4400 = 36u64;
        nat_add(a4399, a4400)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4401 = v3;
        let a4402 = 40u64;
        nat_add(a4401, a4402)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4403 = v3;
        let a4404 = 44u64;
        nat_add(a4403, a4404)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4405 = v3;
        let a4406 = 48u64;
        nat_add(a4405, a4406)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4407 = v3;
        let a4408 = 52u64;
        nat_add(a4407, a4408)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4409 = v3;
        let a4410 = 56u64;
        nat_add(a4409, a4410)?
    })?, f1091(v0, v1, v2, {
        let a4411 = v3;
        let a4412 = 60u64;
        nat_add(a4411, a4412)?
    })?))
}

pub fn f1082(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a4417 = f1092(v0, 24u32);
    let a4418 = {
        let a4415 = f1092(v1, 16u32);
        let a4416 = {
            let a4413 = f1092(v2, 8u32);
            let a4414 = v3;
            fixed_u32::bit_or(a4413, a4414)
        };
        fixed_u32::bit_or(a4415, a4416)
    };
    fixed_u32::bit_or(a4417, a4418)
}

pub fn f1083(v0: Bytes, v1: u64) -> u32 {
    let m4421 = {
        let a4419 = v0;
        let a4420 = v1;
        index_bytes(a4419, a4420)
    };
    match m4421 {
        None => {
            0u32
        }
        Some(v2) => {
            let m4423 = {
                let a4422 = v2;
                fixed_u32::convert(i128::from(a4422))
            };
            match m4423 {
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

pub fn f1084(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a4426 = {
        let a4424 = v4;
        let a4425 = 8u64;
        nat_add(a4424, a4425)?
    };
    let a4427 = v2;
    if nat_eq(a4426, a4427) {
        Ok(f1093(v3, true))
    } else {
        let a4430 = {
            let a4428 = v4;
            let a4429 = 4u64;
            nat_add(a4428, a4429)?
        };
        let a4431 = v2;
        if nat_eq(a4430, a4431) {
            Ok(f1093(v3, false))
        } else {
            Ok(f1082(f1094(v0.clone(), v1, v4), f1094(v0.clone(), v1, {
                let a4432 = v4;
                let a4433 = 1u64;
                nat_add(a4432, a4433)?
            }), f1094(v0.clone(), v1, {
                let a4434 = v4;
                let a4435 = 2u64;
                nat_add(a4434, a4435)?
            }), f1094(v0, v1, {
                let a4436 = v4;
                let a4437 = 3u64;
                nat_add(a4436, a4437)?
            })))
        }
    }
}

pub fn f1085(v0: u64) -> u32 {
    let a4438 = v0;
    let a4439 = 32u64;
    if nat_lt(a4438, a4439) {
        let a4440 = v0;
        let a4441 = 16u64;
        if nat_lt(a4440, a4441) {
            let a4442 = v0;
            let a4443 = 8u64;
            if nat_lt(a4442, a4443) {
                let a4444 = v0;
                let a4445 = 4u64;
                if nat_lt(a4444, a4445) {
                    let a4446 = v0;
                    let a4447 = 2u64;
                    if nat_lt(a4446, a4447) {
                        let a4448 = v0;
                        let a4449 = 1u64;
                        if nat_lt(a4448, a4449) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a4450 = v0;
                        let a4451 = 3u64;
                        if nat_lt(a4450, a4451) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a4452 = v0;
                    let a4453 = 6u64;
                    if nat_lt(a4452, a4453) {
                        let a4454 = v0;
                        let a4455 = 5u64;
                        if nat_lt(a4454, a4455) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4456 = v0;
                        let a4457 = 7u64;
                        if nat_lt(a4456, a4457) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4458 = v0;
                let a4459 = 12u64;
                if nat_lt(a4458, a4459) {
                    let a4460 = v0;
                    let a4461 = 10u64;
                    if nat_lt(a4460, a4461) {
                        let a4462 = v0;
                        let a4463 = 9u64;
                        if nat_lt(a4462, a4463) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4464 = v0;
                        let a4465 = 11u64;
                        if nat_lt(a4464, a4465) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4466 = v0;
                    let a4467 = 14u64;
                    if nat_lt(a4466, a4467) {
                        let a4468 = v0;
                        let a4469 = 13u64;
                        if nat_lt(a4468, a4469) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4470 = v0;
                        let a4471 = 15u64;
                        if nat_lt(a4470, a4471) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4472 = v0;
            let a4473 = 24u64;
            if nat_lt(a4472, a4473) {
                let a4474 = v0;
                let a4475 = 20u64;
                if nat_lt(a4474, a4475) {
                    let a4476 = v0;
                    let a4477 = 18u64;
                    if nat_lt(a4476, a4477) {
                        let a4478 = v0;
                        let a4479 = 17u64;
                        if nat_lt(a4478, a4479) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4480 = v0;
                        let a4481 = 19u64;
                        if nat_lt(a4480, a4481) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4482 = v0;
                    let a4483 = 22u64;
                    if nat_lt(a4482, a4483) {
                        let a4484 = v0;
                        let a4485 = 21u64;
                        if nat_lt(a4484, a4485) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4486 = v0;
                        let a4487 = 23u64;
                        if nat_lt(a4486, a4487) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4488 = v0;
                let a4489 = 28u64;
                if nat_lt(a4488, a4489) {
                    let a4490 = v0;
                    let a4491 = 26u64;
                    if nat_lt(a4490, a4491) {
                        let a4492 = v0;
                        let a4493 = 25u64;
                        if nat_lt(a4492, a4493) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4494 = v0;
                        let a4495 = 27u64;
                        if nat_lt(a4494, a4495) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4496 = v0;
                    let a4497 = 30u64;
                    if nat_lt(a4496, a4497) {
                        let a4498 = v0;
                        let a4499 = 29u64;
                        if nat_lt(a4498, a4499) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4500 = v0;
                        let a4501 = 31u64;
                        if nat_lt(a4500, a4501) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4502 = v0;
        let a4503 = 48u64;
        if nat_lt(a4502, a4503) {
            let a4504 = v0;
            let a4505 = 40u64;
            if nat_lt(a4504, a4505) {
                let a4506 = v0;
                let a4507 = 36u64;
                if nat_lt(a4506, a4507) {
                    let a4508 = v0;
                    let a4509 = 34u64;
                    if nat_lt(a4508, a4509) {
                        let a4510 = v0;
                        let a4511 = 33u64;
                        if nat_lt(a4510, a4511) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4512 = v0;
                        let a4513 = 35u64;
                        if nat_lt(a4512, a4513) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4514 = v0;
                    let a4515 = 38u64;
                    if nat_lt(a4514, a4515) {
                        let a4516 = v0;
                        let a4517 = 37u64;
                        if nat_lt(a4516, a4517) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4518 = v0;
                        let a4519 = 39u64;
                        if nat_lt(a4518, a4519) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4520 = v0;
                let a4521 = 44u64;
                if nat_lt(a4520, a4521) {
                    let a4522 = v0;
                    let a4523 = 42u64;
                    if nat_lt(a4522, a4523) {
                        let a4524 = v0;
                        let a4525 = 41u64;
                        if nat_lt(a4524, a4525) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4526 = v0;
                        let a4527 = 43u64;
                        if nat_lt(a4526, a4527) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4528 = v0;
                    let a4529 = 46u64;
                    if nat_lt(a4528, a4529) {
                        let a4530 = v0;
                        let a4531 = 45u64;
                        if nat_lt(a4530, a4531) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4532 = v0;
                        let a4533 = 47u64;
                        if nat_lt(a4532, a4533) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4534 = v0;
            let a4535 = 56u64;
            if nat_lt(a4534, a4535) {
                let a4536 = v0;
                let a4537 = 52u64;
                if nat_lt(a4536, a4537) {
                    let a4538 = v0;
                    let a4539 = 50u64;
                    if nat_lt(a4538, a4539) {
                        let a4540 = v0;
                        let a4541 = 49u64;
                        if nat_lt(a4540, a4541) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4542 = v0;
                        let a4543 = 51u64;
                        if nat_lt(a4542, a4543) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4544 = v0;
                    let a4545 = 54u64;
                    if nat_lt(a4544, a4545) {
                        let a4546 = v0;
                        let a4547 = 53u64;
                        if nat_lt(a4546, a4547) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4548 = v0;
                        let a4549 = 55u64;
                        if nat_lt(a4548, a4549) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4550 = v0;
                let a4551 = 60u64;
                if nat_lt(a4550, a4551) {
                    let a4552 = v0;
                    let a4553 = 58u64;
                    if nat_lt(a4552, a4553) {
                        let a4554 = v0;
                        let a4555 = 57u64;
                        if nat_lt(a4554, a4555) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4556 = v0;
                        let a4557 = 59u64;
                        if nat_lt(a4556, a4557) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4558 = v0;
                    let a4559 = 62u64;
                    if nat_lt(a4558, a4559) {
                        let a4560 = v0;
                        let a4561 = 61u64;
                        if nat_lt(a4560, a4561) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4562 = v0;
                        let a4563 = 63u64;
                        if nat_lt(a4562, a4563) {
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

pub fn f1086(v0: u32) -> u32 {
    let a4566 = {
        let a4564 = f1095(v0, 6u32, 26u32);
        let a4565 = f1095(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4564, a4565)
    };
    let a4567 = f1095(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4566, a4567)
}

pub fn f1087(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4573 = {
        let a4568 = v0;
        let a4569 = v1;
        fixed_u32::bit_and(a4568, a4569)
    };
    let a4574 = {
        let a4571 = {
            let a4570 = v0;
            fixed_u32::bit_not(a4570)
        };
        let a4572 = v2;
        fixed_u32::bit_and(a4571, a4572)
    };
    fixed_u32::bit_xor(a4573, a4574)
}

pub fn f1088(v0: u32) -> u32 {
    let a4577 = {
        let a4575 = f1095(v0, 2u32, 30u32);
        let a4576 = f1095(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4575, a4576)
    };
    let a4578 = f1095(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4577, a4578)
}

pub fn f1089(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4587 = {
        let a4583 = {
            let a4579 = v0;
            let a4580 = v1;
            fixed_u32::bit_and(a4579, a4580)
        };
        let a4584 = {
            let a4581 = v0;
            let a4582 = v2;
            fixed_u32::bit_and(a4581, a4582)
        };
        fixed_u32::bit_xor(a4583, a4584)
    };
    let a4588 = {
        let a4585 = v1;
        let a4586 = v2;
        fixed_u32::bit_and(a4585, a4586)
    };
    fixed_u32::bit_xor(a4587, a4588)
}

pub fn f1090(v0: Adt170) -> Adt170 {
    Adt170::C0(match v0.clone() {
        Adt170::C0(_, h4589, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4589
        }
    }, match v0.clone() {
        Adt170::C0(_, _, h4590, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4590
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, h4591, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4591
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, h4592, _, _, _, _, _, _, _, _, _, _, _) => {
            h4592
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, h4593, _, _, _, _, _, _, _, _, _, _) => {
            h4593
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, h4594, _, _, _, _, _, _, _, _, _) => {
            h4594
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, h4595, _, _, _, _, _, _, _, _) => {
            h4595
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, h4596, _, _, _, _, _, _, _) => {
            h4596
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, h4597, _, _, _, _, _, _) => {
            h4597
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, h4598, _, _, _, _, _) => {
            h4598
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, h4599, _, _, _, _) => {
            h4599
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4600, _, _, _) => {
            h4600
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4601, _, _) => {
            h4601
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4602, _) => {
            h4602
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4603) => {
            h4603
        }
    }, f1071(f1096(match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4604, _) => {
            h4604
        }
    }), f1071(match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, h4605, _, _, _, _, _, _) => {
            h4605
        }
    }, f1071(f1097(match v0.clone() {
        Adt170::C0(_, h4606, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4606
        }
    }), match v0 {
        Adt170::C0(h4607, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4607
        }
    }))))
}

pub fn f1091(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4610 = {
        let a4608 = v3;
        let a4609 = 8u64;
        nat_add(a4608, a4609)?
    };
    let a4611 = v2;
    if nat_eq(a4610, a4611) {
        Ok(f1098({
            let a4614 = {
                let a4612 = v1;
                let a4613 = 8u64;
                nat_mul(a4612, a4613)?
            };
            let a4615 = 4294967296u64;
            let a4616 = 0u64;
            nat_quot(a4614, a4615, a4616)
        }))
    } else {
        let a4619 = {
            let a4617 = v3;
            let a4618 = 4u64;
            nat_add(a4617, a4618)?
        };
        let a4620 = v2;
        if nat_eq(a4619, a4620) {
            Ok(f1098({
                let a4623 = {
                    let a4621 = v1;
                    let a4622 = 8u64;
                    nat_mul(a4621, a4622)?
                };
                let a4624 = 4294967296u64;
                let a4625 = 0u64;
                nat_rem(a4623, a4624, a4625)
            }))
        } else {
            Ok(f1082(f1094(v0.clone(), v1, v3), f1094(v0.clone(), v1, {
                let a4626 = v3;
                let a4627 = 1u64;
                nat_add(a4626, a4627)?
            }), f1094(v0.clone(), v1, {
                let a4628 = v3;
                let a4629 = 2u64;
                nat_add(a4628, a4629)?
            }), f1094(v0, v1, {
                let a4630 = v3;
                let a4631 = 3u64;
                nat_add(a4630, a4631)?
            })))
        }
    }
}

pub fn f1092(v0: u32, v1: u32) -> u32 {
    let m4634 = {
        let a4632 = v0;
        let a4633 = v1;
        fixed_u32::shift_left(a4632, a4633)
    };
    match m4634 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1093(v0: u64, v1: bool) -> u32 {
    if v1 {
        f1098({
            let a4635 = v0;
            let a4636 = 4294967296u64;
            let a4637 = 0u64;
            nat_quot(a4635, a4636, a4637)
        })
    } else {
        f1098({
            let a4638 = v0;
            let a4639 = 4294967296u64;
            let a4640 = 0u64;
            nat_rem(a4638, a4639, a4640)
        })
    }
}

pub fn f1094(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4641 = v2;
    let a4642 = v1;
    if nat_lt(a4641, a4642) {
        let m4645 = {
            let a4643 = v0;
            let a4644 = v2;
            index_bytes(a4643, a4644)
        };
        match m4645 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4647 = {
                    let a4646 = v3;
                    fixed_u32::convert(i128::from(a4646))
                };
                match m4647 {
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
        let a4648 = v2;
        let a4649 = v1;
        if nat_eq(a4648, a4649) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f1095(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4650 = f1074(v0, v1);
    let a4651 = f1092(v0, v2);
    fixed_u32::bit_or(a4650, a4651)
}

pub fn f1096(v0: u32) -> u32 {
    let a4654 = {
        let a4652 = f1095(v0, 17u32, 15u32);
        let a4653 = f1095(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4652, a4653)
    };
    let a4655 = f1074(v0, 10u32);
    fixed_u32::bit_xor(a4654, a4655)
}

pub fn f1097(v0: u32) -> u32 {
    let a4658 = {
        let a4656 = f1095(v0, 7u32, 25u32);
        let a4657 = f1095(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4656, a4657)
    };
    let a4659 = f1074(v0, 3u32);
    fixed_u32::bit_xor(a4658, a4659)
}

pub fn f1098(v0: u64) -> u32 {
    let (_, h4660) = {
        let (_, h4661) = f1099(Fn14::F1100, 32u64, (v0, (1u32, 0u32)));
        h4661
    };
    h4660
}

pub fn f1099(v0: Fn14, p4665: u64, p4666: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4664 = (p4665, p4666);
    {
        let mut m4664 = m4664;
        loop {
            let step = {
                let (v1, v2) = m4664;
                let m4662 = v1;
                if m4662 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4662 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4663: Fn14 = v0.clone();
                        c4663.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1100(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4667, _) = v0;
        h4667
    };
    let v2: u32 = {
        let (h4668, _) = {
            let (_, h4669) = v0;
            h4669
        };
        h4668
    };
    let v3: u32 = {
        let (_, h4670) = {
            let (_, h4671) = v0;
            h4671
        };
        h4670
    };
    ({
        let a4672 = v1;
        let a4673 = 2u64;
        let a4674 = 0u64;
        nat_quot(a4672, a4673, a4674)
    }, (f1092(v2, 1u32), {
        let a4678 = {
            let a4675 = v1;
            let a4676 = 2u64;
            let a4677 = 0u64;
            nat_rem(a4675, a4676, a4677)
        };
        let a4679 = 1u64;
        if nat_eq(a4678, a4679) {
            let a4680 = v3;
            let a4681 = v2;
            fixed_u32::bit_or(a4680, a4681)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
