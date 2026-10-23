#![forbid(unsafe_code)]
use core::cmp::Ordering;

use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;

pub type R<T> = Result<T, Overflow>;
 // hand edit
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
    C0,
    C1(bool),
    C2(Adt1),
    C3(Str),
    C4(List<Adt0>),
    C5(List<(Str, Adt0)>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(i64, i64),
}

#[derive(Clone)]
pub enum Adt2 {
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
pub enum Adt3 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt5 {
    C0(List<u64>),
    C1(u64, Adt0),
    C2(u64, List<Adt0>),
    C3(u64, List<Adt0>),
    C4(u64, List<(Str, Adt0)>),
    C5(u64, List<(Str, Adt0)>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Adt7, List<Adt11>, u64),
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt8, List<u64>, bool),
    C7(u64, u64),
    C8(Adt9),
    C9(Adt0),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(bool, Adt10, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt10 {
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
pub enum Adt11 {
    C0(List<Adt0>),
    C1(List<(Str, Adt0)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Adt2),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Adt2),
    C1(List<Adt5>, List<u64>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0,
    C1,
    C2(Adt0),
}

pub fn f0(v0: u64, v1: Bytes) -> R<List<u64>> {
    let a1 = v0;
    let a2 = 0u64;
    if nat_eq(a1, a2) {
        f1(f2(v1)?)
    } else {
        let a3 = v0;
        let a4 = 1u64;
        if nat_eq(a3, a4) {
            Ok(f3(f4(f5(v1)?)?))
        } else {
            let a5 = v0;
            let a6 = 2u64;
            if nat_eq(a5, a6) {
                f6(v1)
            } else {
                let a7 = v0;
                let a8 = 3u64;
                if nat_eq(a7, a8) {
                    f7(f2(v1)?)
                } else {
                    f1(f8(f5(v1)?)?)
                }
            }
        }
    }
}

pub fn f1(v0: Result<Adt0, Adt2>) -> R<List<u64>> {
    let m9 = v0;
    match m9 {
        Ok(v2) => {
            f10(v2)
        }
        Err(v1) => {
            Ok(f9(1u64, v1))
        }
    }
}

pub fn f2(v0: Bytes) -> R<Result<Adt0, Adt2>> {
    f11(f5(v0)?)
}

pub fn f3(v0: Adt3) -> List<u64> {
    let m10 = v0;
    match m10 {
        Adt3::C0 => {
            List::cons(0u64, List::<u64>::nil())
        }
        Adt3::C1(v1) => {
            List::cons(1u64, List::cons(0u64, List::cons(v1, List::<u64>::nil())))
        }
        Adt3::C2(v2) => {
            List::cons(1u64, List::cons(1u64, List::cons(v2, List::<u64>::nil())))
        }
    }
}

pub fn f4(v0: List<u64>) -> R<Adt3> {
    Ok(f12({
        let a11 = v0.clone();
        length_list(a11)
    }, f13(Adt4::C0, 0u64, 0u64, v0)?))
}

pub fn f5(v0: Bytes) -> R<List<u64>> {
    f14(v0.clone(), {
        let a12 = v0;
        length_bytes(a12)
    }, List::<u64>::nil())
}

pub fn f6(v0: Bytes) -> R<List<u64>> {
    let v1: u64 = f15(v0.clone(), 0u64)?;
    let v2: u64 = {
        let a15 = {
            let a13 = f15(v0.clone(), 1u64)?;
            let a14 = 256u64;
            nat_mul(a13, a14)?
        };
        let a16 = f15(v0, 2u64)?;
        nat_add(a15, a16)?
    };
    let m19 = f16({
        let a17 = v1;
        let a18 = 5u64;
        if nat_eq(a17, a18) {
            v2
        } else {
            f17()
        }
    }, f18(v1, v2)?)?;
    match m19 {
        Ok(v4) => {
            Ok(List::cons(0u64, v4))
        }
        Err(v3) => {
            Ok(f9(2u64, v3))
        }
    }
}

pub fn f7(v0: Result<Adt0, Adt2>) -> R<List<u64>> {
    let m20 = v0;
    match m20 {
        Ok(v2) => {
            let m21 = v2;
            match m21 {
                Adt0::C0 => {
                    Ok(List::cons(4u64, List::<u64>::nil()))
                }
                Adt0::C1(_) => {
                    Ok(List::cons(4u64, List::<u64>::nil()))
                }
                Adt0::C2(_) => {
                    Ok(List::cons(4u64, List::<u64>::nil()))
                }
                Adt0::C3(_) => {
                    Ok(List::cons(4u64, List::<u64>::nil()))
                }
                Adt0::C4(_) => {
                    Ok(List::cons(4u64, List::<u64>::nil()))
                }
                Adt0::C5(v7) => {
                    let m22 = f19(Str::lit("a"), v7);
                    match m22 {
                        None => {
                            Ok(List::cons(3u64, List::<u64>::nil()))
                        }
                        Some(v8) => {
                            f10(v8)
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Ok(f9(1u64, v1))
        }
    }
}

pub fn f8(v0: List<u64>) -> R<Result<Adt0, Adt2>> {
    f20(f21(), 0u64, v0)
}

pub fn f9(v0: u64, v1: Adt2) -> List<u64> {
    List::cons(v0, List::cons(f22(v1.clone()), List::cons(f23(v1), List::<u64>::nil())))
}

pub fn f10(v0: Adt0) -> R<List<u64>> {
    let m23 = f24(v0)?;
    match m23 {
        Ok(v2) => {
            Ok(List::cons(0u64, v2))
        }
        Err(v1) => {
            Ok(f9(2u64, v1))
        }
    }
}

pub fn f11(v0: List<u64>) -> R<Result<Adt0, Adt2>> {
    let m24 = f4(v0.clone())?;
    match m24 {
        Adt3::C0 => {
            f20(f21(), 0u64, v0)
        }
        Adt3::C1(v1) => {
            Ok(Err::<Adt0, Adt2>(Adt2::C0(v1)))
        }
        Adt3::C2(v2) => {
            Ok(Err::<Adt0, Adt2>(Adt2::C1(v2)))
        }
    }
}

pub fn f12(v0: u64, v1: u64) -> Adt3 {
    let a25 = v1;
    let a26 = 0u64;
    if nat_eq(a25, a26) {
        Adt3::C0
    } else {
        let a27 = v1;
        let a28 = 2u64;
        if nat_eq(a27, a28) {
            Adt3::C2(v0)
        } else {
            Adt3::C1({
                let a32 = v0;
                let a33 = {
                    let a29 = v1;
                    let a30 = 3u64;
                    let a31 = 0u64;
                    nat_quot(a29, a30, a31)
                };
                nat_sub(a32, a33)
            })
        }
    }
}

pub fn f13(p61: Adt4, p62: u64, p63: u64, p64: List<u64>) -> R<u64> {
    let m60 = (p61, (p62, (p63, p64)));
    {
        let mut m60 = m60;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m60;
                let m34 = v3;
                match m34.uncons() {
                    None => {
                        let m35 = v0;
                        match m35 {
                            Adt4::C0 => {
                                Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt4::C1 => {
                                Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt4::C2 => {
                                Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt4::C3 => {
                                Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m36 = v0;
                        match m36 {
                            Adt4::C0 => {
                                let a37 = v4;
                                let a38 = 128u64;
                                if nat_lt(a37, a38) {
                                    Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a39 = v4;
                                    let a40 = 194u64;
                                    if nat_lt(a39, a40) {
                                        let v6: u64 = f25({
                                            let a41 = List::cons(v4, v5);
                                            length_list(a41)
                                        })?;
                                        Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a42 = v4;
                                        let a43 = 224u64;
                                        if nat_lt(a42, a43) {
                                            Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a44 = v4;
                                            let a45 = 224u64;
                                            if nat_eq(a44, a45) {
                                                Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a46 = v4;
                                                let a47 = 237u64;
                                                if nat_eq(a46, a47) {
                                                    Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a48 = v4;
                                                    let a49 = 240u64;
                                                    if nat_lt(a48, a49) {
                                                        Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a50 = v4;
                                                        let a51 = 240u64;
                                                        if nat_eq(a50, a51) {
                                                            Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a52 = v4;
                                                            let a53 = 244u64;
                                                            if nat_eq(a52, a53) {
                                                                Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a54 = v4;
                                                                let a55 = 244u64;
                                                                if nat_lt(a54, a55) {
                                                                    Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f25({
                                                                        let a56 = List::cons(v4, v5);
                                                                        length_list(a56)
                                                                    })?;
                                                                    Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt4::C1 => {
                                if f26(v4, v1, v2) {
                                    Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f25({
                                        let a57 = List::cons(v4, v5);
                                        length_list(a57)
                                    })?;
                                    Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt4::C2 => {
                                if f26(v4, v1, v2) {
                                    Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f25({
                                        let a58 = List::cons(v4, v5);
                                        length_list(a58)
                                    })?;
                                    Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt4::C3 => {
                                if f26(v4, v1, v2) {
                                    Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f25({
                                        let a59 = List::cons(v4, v5);
                                        length_list(a59)
                                    })?;
                                    Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m60 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Bytes, p67: u64, p68: List<u64>) -> R<List<u64>> {
    let m66 = (p67, p68);
    {
        let mut m66 = m66;
        loop {
            let step = {
                let (v1, v2) = m66;
                let m65 = v1;
                if m65 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m65 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f15(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m66 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: Bytes, v1: u64) -> R<u64> {
    let m71 = {
        let a69 = v0;
        let a70 = v1;
        index_bytes(a69, a70)
    };
    match m71 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a116 = {
                let a92 = {
                    let a80 = {
                        let a74 = {
                            let a72 = v3;
                            let a73 = 128u8;
                            fixed_u8::bit_and(a72, a73)
                        };
                        let a75 = 128u8;
                        if equal(a74, a75) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a81 = {
                        let a78 = {
                            let a76 = v3;
                            let a77 = 64u8;
                            fixed_u8::bit_and(a76, a77)
                        };
                        let a79 = 64u8;
                        if equal(a78, a79) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a80, a81)?
                };
                let a93 = {
                    let a90 = {
                        let a84 = {
                            let a82 = v3;
                            let a83 = 32u8;
                            fixed_u8::bit_and(a82, a83)
                        };
                        let a85 = 32u8;
                        if equal(a84, a85) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a91 = {
                        let a88 = {
                            let a86 = v3;
                            let a87 = 16u8;
                            fixed_u8::bit_and(a86, a87)
                        };
                        let a89 = 16u8;
                        if equal(a88, a89) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a90, a91)?
                };
                nat_add(a92, a93)?
            };
            let a117 = {
                let a114 = {
                    let a102 = {
                        let a96 = {
                            let a94 = v3;
                            let a95 = 8u8;
                            fixed_u8::bit_and(a94, a95)
                        };
                        let a97 = 8u8;
                        if equal(a96, a97) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a103 = {
                        let a100 = {
                            let a98 = v3;
                            let a99 = 4u8;
                            fixed_u8::bit_and(a98, a99)
                        };
                        let a101 = 4u8;
                        if equal(a100, a101) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a102, a103)?
                };
                let a115 = {
                    let a112 = {
                        let a106 = {
                            let a104 = v3;
                            let a105 = 2u8;
                            fixed_u8::bit_and(a104, a105)
                        };
                        let a107 = 2u8;
                        if equal(a106, a107) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a113 = {
                        let a110 = {
                            let a108 = v3;
                            let a109 = 1u8;
                            fixed_u8::bit_and(a108, a109)
                        };
                        let a111 = 1u8;
                        if equal(a110, a111) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a112, a113)?
                };
                nat_add(a114, a115)?
            };
            nat_add(a116, a117)
        }
    }
}

pub fn f16(v0: u64, v1: Adt0) -> R<Result<List<u64>, Adt2>> {
    let m118 = f27(v0, List::cons(Adt5::C1(0u64, v1), List::<Adt5>::nil()), List::<u64>::nil())?;
    match m118 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt2>(f28(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt2>(v3))
        }
    }
}

pub fn f17() -> u64 {
    67108864u64
}

pub fn f18(v0: u64, v1: u64) -> R<Adt0> {
    let a119 = v0;
    let a120 = 0u64;
    if nat_eq(a119, a120) {
        Ok(f29(v1, Adt0::C0))
    } else {
        let a121 = v0;
        let a122 = 1u64;
        if nat_eq(a121, a122) {
            Ok(f30(f31({
                let a123 = v1;
                let a124 = 1u64;
                nat_sub(a123, a124)
            }, 1i64)?, 0i64))
        } else {
            let a125 = v0;
            let a126 = 2u64;
            if nat_eq(a125, a126) {
                Ok(f30(1i64, f32(v1, 0i64)?))
            } else {
                let a127 = v0;
                let a128 = 3u64;
                if nat_eq(a127, a128) {
                    Ok(f30(1i64, {
                        let a129 = f32(v1, 0i64)?;
                        int_neg(a129)?
                    }))
                } else {
                    let a130 = v0;
                    let a131 = 4u64;
                    if nat_eq(a130, a131) {
                        Ok(Adt0::C3(f33()))
                    } else {
                        Ok(f29(5u64, Adt0::C0))
                    }
                }
            }
        }
    }
}

pub fn f19(v0: Str, p138: List<(Str, Adt0)>) -> Option<Adt0> {
    let m137 = p138;
    {
        let mut m137 = m137;
        loop {
            let step = {
                let v1 = m137;
                let m132 = v1;
                match m132.uncons() {
                    None => {
                        Err::<List<(Str, Adt0)>, Option<Adt0>>(None::<Adt0>)
                    }
                    Some((v2, v3)) => {
                        let a134 = {
                            let (h133, _) = v2;
                            h133
                        };
                        let a135 = v0.clone();
                        if equal(a134, a135) {
                            Err::<List<(Str, Adt0)>, Option<Adt0>>(Some({
                                let (_, h136) = v2;
                                h136
                            }))
                        } else {
                            Ok::<List<(Str, Adt0)>, Option<Adt0>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(p145: Adt6, p146: u64, p147: List<u64>) -> R<Result<Adt0, Adt2>> {
    let m144 = (p145, (p146, p147));
    {
        let mut m144 = m144;
        loop {
            let step = {
                let (v0, (v1, v2)) = m144;
                let m139 = v2;
                match m139.uncons() {
                    None => {
                        let m140 = f34(v0, v1)?;
                        match m140 {
                            Ok(v3) => {
                                Err::<(Adt6, (u64, List<u64>)), R<Result<Adt0, Adt2>>>(Ok(Ok::<Adt0, Adt2>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt6, (u64, List<u64>)), R<Result<Adt0, Adt2>>>(Ok(Err::<Adt0, Adt2>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m141 = f35(v0, v1, v5)?;
                        match m141 {
                            Adt12::C0(v7) => {
                                Err::<(Adt6, (u64, List<u64>)), R<Result<Adt0, Adt2>>>(Ok(Err::<Adt0, Adt2>(v7)))
                            }
                            Adt12::C1(v8) => {
                                Ok::<(Adt6, (u64, List<u64>)), R<Result<Adt0, Adt2>>>((v8, ({
                                    let a142 = v1;
                                    let a143 = 1u64;
                                    nat_add(a142, a143)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21() -> Adt6 {
    Adt6::C0(Adt7::C0, List::<Adt11>::nil(), 0u64)
}

pub fn f22(v0: Adt2) -> u64 {
    let m148 = v0;
    match m148 {
        Adt2::C0(_) => {
            0u64
        }
        Adt2::C1(_) => {
            1u64
        }
        Adt2::C2(_) => {
            2u64
        }
        Adt2::C3 => {
            3u64
        }
        Adt2::C4(_) => {
            4u64
        }
        Adt2::C5(_) => {
            5u64
        }
        Adt2::C6(_) => {
            6u64
        }
        Adt2::C7(_) => {
            7u64
        }
        Adt2::C8(_) => {
            8u64
        }
        Adt2::C9(_) => {
            9u64
        }
        Adt2::C10(_) => {
            10u64
        }
        Adt2::C11 => {
            11u64
        }
        Adt2::C12 => {
            12u64
        }
        Adt2::C13 => {
            13u64
        }
        Adt2::C14 => {
            14u64
        }
    }
}

pub fn f23(v0: Adt2) -> u64 {
    let m149 = v0;
    match m149 {
        Adt2::C0(v1) => {
            v1
        }
        Adt2::C1(v2) => {
            v2
        }
        Adt2::C2(v3) => {
            v3
        }
        Adt2::C3 => {
            0u64
        }
        Adt2::C4(v4) => {
            v4
        }
        Adt2::C5(v5) => {
            v5
        }
        Adt2::C6(v6) => {
            v6
        }
        Adt2::C7(v7) => {
            v7
        }
        Adt2::C8(v8) => {
            v8
        }
        Adt2::C9(v9) => {
            v9
        }
        Adt2::C10(v10) => {
            v10
        }
        Adt2::C11 => {
            0u64
        }
        Adt2::C12 => {
            0u64
        }
        Adt2::C13 => {
            0u64
        }
        Adt2::C14 => {
            0u64
        }
    }
}

pub fn f24(v0: Adt0) -> R<Result<List<u64>, Adt2>> {
    f16(f17(), v0)
}

pub fn f25(v0: u64) -> R<u64> {
    let a152 = {
        let a150 = v0;
        let a151 = 3u64;
        nat_mul(a150, a151)?
    };
    let a153 = 1u64;
    nat_add(a152, a153)
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> bool {
    let a154 = v1;
    let a155 = v0;
    if nat_le(a154, a155) {
        let a156 = v0;
        let a157 = v2;
        nat_le(a156, a157)
    } else {
        false
    }
}

pub fn f27(p162: u64, p163: List<Adt5>, p164: List<u64>) -> R<Result<List<u64>, Adt2>> {
    let m161 = (p162, (p163, p164));
    {
        let mut m161 = m161;
        loop {
            let step = {
                let (v0, (v1, v2)) = m161;
                let m158 = v0;
                if m158 == 0 {
                    Err::<(u64, (List<Adt5>, List<u64>)), R<Result<List<u64>, Adt2>>>(Ok(Err::<List<u64>, Adt2>(Adt2::C13)))
                } else {
                    let v3 = m158 - 1;
                    let m159 = v1;
                    match m159.uncons() {
                        None => {
                            Err::<(u64, (List<Adt5>, List<u64>)), R<Result<List<u64>, Adt2>>>(Ok(Ok::<List<u64>, Adt2>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m160 = f36(v4, v5, v2)?;
                            match m160 {
                                Adt13::C0(v6) => {
                                    Err::<(u64, (List<Adt5>, List<u64>)), R<Result<List<u64>, Adt2>>>(Ok(Err::<List<u64>, Adt2>(v6)))
                                }
                                Adt13::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt5>, List<u64>)), R<Result<List<u64>, Adt2>>>((v3, (v7, v8)))
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

pub fn f28(p167: List<u64>, p168: List<u64>) -> List<u64> {
    let m166 = (p167, p168);
    {
        let mut m166 = m166;
        loop {
            let step = {
                let (v0, v1) = m166;
                let m165 = v0;
                match m165.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(p171: u64, p172: Adt0) -> Adt0 {
    let m170 = (p171, p172);
    {
        let mut m170 = m170;
        loop {
            let step = {
                let (v0, v1) = m170;
                let m169 = v0;
                if m169 == 0 {
                    Err::<(u64, Adt0), Adt0>(v1)
                } else {
                    let v2 = m169 - 1;
                    Ok::<(u64, Adt0), Adt0>((v2, Adt0::C4(List::cons(v1, List::<Adt0>::nil()))))
                }
            };
            match step {
                Ok(next) => { m170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: i64, v1: i64) -> Adt0 {
    Adt0::C2(Adt1::C0(v0, v1))
}

pub fn f31(p177: u64, p178: i64) -> R<i64> {
    let m176 = (p177, p178);
    {
        let mut m176 = m176;
        loop {
            let step = {
                let (v0, v1) = m176;
                let m173 = v0;
                if m173 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m173 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a174 = v1;
                        let a175 = 10i64;
                        int_mul(a174, a175)?
                    }))
                }
            };
            match step {
                Ok(next) => { m176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(p181: u64, p182: i64) -> R<i64> {
    let m180 = (p181, p182);
    {
        let mut m180 = m180;
        loop {
            let step = {
                let (v0, v1) = m180;
                let m179 = v0;
                if m179 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m179 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f37(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m180 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33() -> Str {
    let m184 = {
        let a183 = f38(f39(), f40(128u64, List::<u64>::nil()));
        utf8_decode(a183)
    };
    match m184 {
        None => {
            Str::lit("")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f34(v0: Adt6, v1: u64) -> R<Result<Adt0, Adt2>> {
    let m186 = match v0.clone() {
        Adt6::C0(h185, _, _) => {
            h185
        }
    };
    match m186 {
        Adt7::C0 => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C1 => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C2 => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C3 => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C4 => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C5 => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C6(_, _, _) => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C7(_, _) => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C8(v3) => {
            let m187 = f41(v3)?;
            match m187 {
                Adt14::C0 => {
                    Ok(Err::<Adt0, Adt2>(Adt2::C7(v1)))
                }
                Adt14::C1 => {
                    Ok(Err::<Adt0, Adt2>(Adt2::C8(v1)))
                }
                Adt14::C2(v4) => {
                    let m192 = {
                        let m191 = f42(v4, match v0.clone() {
                            Adt6::C0(_, h189, _) => {
                                h189
                            }
                        }, match v0 {
                            Adt6::C0(_, _, h190) => {
                                h190
                            }
                        });
                        match m191 {
                            Adt6::C0(h188, _, _) => {
                                h188
                            }
                        }
                    };
                    match m192 {
                        Adt7::C0 => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C1 => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C2 => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C3 => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C4 => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C5 => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C6(_, _, _) => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C7(_, _) => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C8(_) => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C9(v5) => {
                            Ok(Ok::<Adt0, Adt2>(v5))
                        }
                    }
                }
            }
        }
        Adt7::C9(v2) => {
            Ok(Ok::<Adt0, Adt2>(v2))
        }
    }
}

pub fn f35(v0: Adt6, v1: u64, v2: u64) -> R<Adt12> {
    let m194 = match v0.clone() {
        Adt6::C0(h193, _, _) => {
            h193
        }
    };
    match m194 {
        Adt7::C0 => {
            if f43(v2) {
                Ok(Adt12::C1(v0))
            } else {
                f44(v0, v1, v2)
            }
        }
        Adt7::C1 => {
            if f43(v2) {
                Ok(Adt12::C1(v0))
            } else {
                let a195 = v2;
                let a196 = 93u64;
                if nat_eq(a195, a196) {
                    Ok(f45(v0, v1))
                } else {
                    f44(v0, v1, v2)
                }
            }
        }
        Adt7::C2 => {
            Ok(f46(v0, v1, v2))
        }
        Adt7::C3 => {
            if f43(v2) {
                Ok(Adt12::C1(v0))
            } else {
                let a197 = v2;
                let a198 = 125u64;
                if nat_eq(a197, a198) {
                    Ok(f47(v0, v1))
                } else {
                    let a199 = v2;
                    let a200 = 34u64;
                    if nat_eq(a199, a200) {
                        Ok(f48(v0, Adt7::C6(Adt8::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f49(Adt2::C2(v1)))
                    }
                }
            }
        }
        Adt7::C4 => {
            if f43(v2) {
                Ok(Adt12::C1(v0))
            } else {
                let a201 = v2;
                let a202 = 34u64;
                if nat_eq(a201, a202) {
                    Ok(f48(v0, Adt7::C6(Adt8::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f49(Adt2::C2(v1)))
                }
            }
        }
        Adt7::C5 => {
            if f43(v2) {
                Ok(Adt12::C1(v0))
            } else {
                let a203 = v2;
                let a204 = 58u64;
                if nat_eq(a203, a204) {
                    Ok(f48(v0, Adt7::C0))
                } else {
                    Ok(f49(Adt2::C2(v1)))
                }
            }
        }
        Adt7::C6(v3, v4, v5) => {
            f50(v0, v1, v2, v3, v4, v5)
        }
        Adt7::C7(v6, v7) => {
            f51(v0, v1, v2, v6, v7)
        }
        Adt7::C8(v8) => {
            f52(v0, v1, v2, v8)
        }
        Adt7::C9(_) => {
            Ok(f46(v0, v1, v2))
        }
    }
}

pub fn f36(v0: Adt5, v1: List<Adt5>, v2: List<u64>) -> R<Adt13> {
    let m205 = v0;
    match m205 {
        Adt5::C0(v3) => {
            Ok(Adt13::C1(v1, f28(v3, v2)))
        }
        Adt5::C1(v4, v5) => {
            f53(v4, v5, v1, v2)
        }
        Adt5::C2(v6, v7) => {
            let m206 = v7;
            match m206.uncons() {
                None => {
                    Ok(Adt13::C1(List::cons(Adt5::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt13::C1(List::cons(Adt5::C1(v6, v8), List::cons(Adt5::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt5::C3(v10, v11) => {
            let m207 = v11.clone();
            match m207.uncons() {
                None => {
                    Ok(Adt13::C1(List::cons(Adt5::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt13::C1(List::cons(Adt5::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt5::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt5::C4(v14, v15) => {
            let m208 = v15;
            match m208.uncons() {
                None => {
                    Ok(Adt13::C1(List::cons(Adt5::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt13::C1(List::cons(Adt5::C0({
                        let a210 = f54({
                            let (h209, _) = v16;
                            h209
                        })?;
                        let a211 = List::cons(58u64, List::<u64>::nil());
                        append_list(a210, a211)
                    }), List::cons(Adt5::C1(v14, {
                        let (_, h212) = v16;
                        h212
                    }), List::cons(Adt5::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt5::C5(v18, v19) => {
            let m213 = v19.clone();
            match m213.uncons() {
                None => {
                    Ok(Adt13::C1(List::cons(Adt5::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt13::C1(List::cons(Adt5::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt5::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f37(v0: i64, v1: i64) -> R<i64> {
    let a215 = v0;
    let a216 = {
        let a214 = v1;
        int_neg(a214)?
    };
    int_sub(a215, a216)
}

pub fn f38(v0: Bytes, v1: List<u64>) -> Bytes {
    let a217 = v0;
    let a218 = f55(f39(), f56(List::<Bytes>::nil(), v1));
    append_bytes(a217, a218)
}

pub fn f39() -> Bytes {
    Bytes::lit(&[])
}

pub fn f40(p221: u64, p222: List<u64>) -> List<u64> {
    let m220 = (p221, p222);
    {
        let mut m220 = m220;
        loop {
            let step = {
                let (v0, v1) = m220;
                let m219 = v0;
                if m219 == 0 {
                    Err::<(u64, List<u64>), List<u64>>(v1)
                } else {
                    let v2 = m219 - 1;
                    Ok::<(u64, List<u64>), List<u64>>((v2, List::cons(v2, v1)))
                }
            };
            match step {
                Ok(next) => { m220 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: Adt9) -> R<Adt14> {
    if f57(match v0.clone() {
        Adt9::C0(_, h223, _, _, _, _, _) => {
            h223
        }
    }) {
        let v1: i64 = {
            let a229 = if match v0.clone() {
                Adt9::C0(_, _, _, _, _, h224, _) => {
                    h224
                }
            } {
                let a226 = match v0.clone() {
                    Adt9::C0(_, _, _, _, _, _, h225) => {
                        h225
                    }
                };
                int_neg(a226)?
            } else {
                match v0.clone() {
                    Adt9::C0(_, _, _, _, _, _, h227) => {
                        h227
                    }
                }
            };
            let a230 = match v0.clone() {
                Adt9::C0(_, _, _, _, h228, _, _) => {
                    h228
                }
            };
            int_sub(a229, a230)?
        };
        let m233 = {
            let a231 = v1;
            let a232 = -9999i64;
            compare(a231, a232)
        };
        if if match m233 {
            Ordering::Less => {
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
            let m236 = {
                let a234 = 9999i64;
                let a235 = v1;
                compare(a234, a235)
            };
            match m236 {
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
            Ok(Adt14::C1)
        } else {
            Ok(Adt14::C2(Adt0::C2(Adt1::C0(if match v0.clone() {
                Adt9::C0(h237, _, _, _, _, _, _) => {
                    h237
                }
            } {
                let a239 = match v0 {
                    Adt9::C0(_, _, h238, _, _, _, _) => {
                        h238
                    }
                };
                int_neg(a239)?
            } else {
                match v0 {
                    Adt9::C0(_, _, h240, _, _, _, _) => {
                        h240
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt14::C0)
    }
}

pub fn f42(v0: Adt0, v1: List<Adt11>, v2: u64) -> Adt6 {
    let m241 = v1.clone();
    match m241.uncons() {
        None => {
            Adt6::C0(Adt7::C9(v0), List::<Adt11>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m242 = v3;
            match m242 {
                Adt11::C0(v5) => {
                    Adt6::C0(Adt7::C2, List::cons(Adt11::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt11::C1(v6, v7) => {
                    let m243 = v7;
                    match m243 {
                        None => {
                            Adt6::C0(Adt7::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt6::C0(Adt7::C2, List::cons(Adt11::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f43(v0: u64) -> bool {
    let a244 = v0;
    let a245 = 32u64;
    if nat_eq(a244, a245) {
        true
    } else {
        let a246 = v0;
        let a247 = 10u64;
        if nat_eq(a246, a247) {
            true
        } else {
            let a248 = v0;
            let a249 = 13u64;
            if nat_eq(a248, a249) {
                true
            } else {
                let a250 = v0;
                let a251 = 9u64;
                nat_eq(a250, a251)
            }
        }
    }
}

pub fn f44(v0: Adt6, v1: u64, v2: u64) -> R<Adt12> {
    let a252 = v2;
    let a253 = 34u64;
    if nat_eq(a252, a253) {
        Ok(f48(v0, Adt7::C6(Adt8::C0, List::<u64>::nil(), false)))
    } else {
        let a254 = v2;
        let a255 = 123u64;
        if nat_eq(a254, a255) {
            f58(v0, Adt11::C1(List::<(Str, Adt0)>::nil(), None::<Str>), Adt7::C3, v1)
        } else {
            let a256 = v2;
            let a257 = 91u64;
            if nat_eq(a256, a257) {
                f58(v0, Adt11::C0(List::<Adt0>::nil()), Adt7::C1, v1)
            } else {
                let a258 = v2;
                let a259 = 116u64;
                if nat_eq(a258, a259) {
                    Ok(f48(v0, Adt7::C7(0u64, 1u64)))
                } else {
                    let a260 = v2;
                    let a261 = 102u64;
                    if nat_eq(a260, a261) {
                        Ok(f48(v0, Adt7::C7(1u64, 1u64)))
                    } else {
                        let a262 = v2;
                        let a263 = 110u64;
                        if nat_eq(a262, a263) {
                            Ok(f48(v0, Adt7::C7(2u64, 1u64)))
                        } else {
                            let a264 = v2;
                            let a265 = 45u64;
                            if nat_eq(a264, a265) {
                                Ok(f48(v0, f59(true, Adt10::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a266 = v2;
                                let a267 = 48u64;
                                if nat_eq(a266, a267) {
                                    Ok(f48(v0, f59(false, Adt10::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f26(v2, 49u64, 57u64) {
                                        Ok(f48(v0, f59(false, Adt10::C2, f60({
                                            let a268 = v2;
                                            let a269 = 48u64;
                                            nat_sub(a268, a269)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f49(Adt2::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f45(v0: Adt6, v1: u64) -> Adt12 {
    let m271 = match v0.clone() {
        Adt6::C0(_, h270, _) => {
            h270
        }
    };
    match m271.uncons() {
        None => {
            f49(Adt2::C2(v1))
        }
        Some((v2, v3)) => {
            let m272 = v2;
            match m272 {
                Adt11::C0(v4) => {
                    Adt12::C1(f42(Adt0::C4(f61(v4, List::<Adt0>::nil())), v3, {
                        let a274 = match v0 {
                            Adt6::C0(_, _, h273) => {
                                h273
                            }
                        };
                        let a275 = 1u64;
                        nat_sub(a274, a275)
                    }))
                }
                Adt11::C1(_, _) => {
                    f49(Adt2::C2(v1))
                }
            }
        }
    }
}

pub fn f46(v0: Adt6, v1: u64, v2: u64) -> Adt12 {
    let m277 = match v0.clone() {
        Adt6::C0(h276, _, _) => {
            h276
        }
    };
    match m277 {
        Adt7::C0 => {
            f49(Adt2::C2(v1))
        }
        Adt7::C1 => {
            f49(Adt2::C2(v1))
        }
        Adt7::C2 => {
            if f43(v2) {
                Adt12::C1(v0)
            } else {
                let a278 = v2;
                let a279 = 44u64;
                if nat_eq(a278, a279) {
                    f62(v0, v1)
                } else {
                    let a280 = v2;
                    let a281 = 93u64;
                    if nat_eq(a280, a281) {
                        f45(v0, v1)
                    } else {
                        let a282 = v2;
                        let a283 = 125u64;
                        if nat_eq(a282, a283) {
                            f47(v0, v1)
                        } else {
                            f49(Adt2::C2(v1))
                        }
                    }
                }
            }
        }
        Adt7::C3 => {
            f49(Adt2::C2(v1))
        }
        Adt7::C4 => {
            f49(Adt2::C2(v1))
        }
        Adt7::C5 => {
            f49(Adt2::C2(v1))
        }
        Adt7::C6(_, _, _) => {
            f49(Adt2::C2(v1))
        }
        Adt7::C7(_, _) => {
            f49(Adt2::C2(v1))
        }
        Adt7::C8(_) => {
            f49(Adt2::C2(v1))
        }
        Adt7::C9(_) => {
            if f43(v2) {
                Adt12::C1(v0)
            } else {
                f49(Adt2::C10(v1))
            }
        }
    }
}

pub fn f47(v0: Adt6, v1: u64) -> Adt12 {
    let m285 = match v0.clone() {
        Adt6::C0(_, h284, _) => {
            h284
        }
    };
    match m285.uncons() {
        None => {
            f49(Adt2::C2(v1))
        }
        Some((v2, v3)) => {
            let m286 = v2;
            match m286 {
                Adt11::C0(_) => {
                    f49(Adt2::C2(v1))
                }
                Adt11::C1(v5, _) => {
                    Adt12::C1(f42(Adt0::C5(f63(v5, List::<(Str, Adt0)>::nil())), v3, {
                        let a288 = match v0 {
                            Adt6::C0(_, _, h287) => {
                                h287
                            }
                        };
                        let a289 = 1u64;
                        nat_sub(a288, a289)
                    }))
                }
            }
        }
    }
}

pub fn f48(v0: Adt6, v1: Adt7) -> Adt12 {
    Adt12::C1(f64(v0, v1))
}

pub fn f49(v0: Adt2) -> Adt12 {
    Adt12::C0(v0)
}

pub fn f50(v0: Adt6, v1: u64, v2: u64, v3: Adt8, v4: List<u64>, v5: bool) -> R<Adt12> {
    let m290 = v3;
    match m290 {
        Adt8::C0 => {
            let a291 = v2;
            let a292 = 34u64;
            if nat_eq(a291, a292) {
                Ok(f65(v0, v1, v4, v5))
            } else {
                let a293 = v2;
                let a294 = 92u64;
                if nat_eq(a293, a294) {
                    Ok(f48(v0, Adt7::C6(Adt8::C1, v4, v5)))
                } else {
                    let a295 = v2;
                    let a296 = 32u64;
                    if nat_lt(a295, a296) {
                        Ok(f49(Adt2::C4(v1)))
                    } else {
                        Ok(f48(v0, Adt7::C6(Adt8::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt8::C1 => {
            let a297 = v2;
            let a298 = 117u64;
            if nat_eq(a297, a298) {
                Ok(f48(v0, Adt7::C6(Adt8::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a299 = f66(v2);
                let a300 = 0u64;
                if nat_eq(a299, a300) {
                    Ok(f49(Adt2::C5(v1)))
                } else {
                    Ok(f48(v0, Adt7::C6(Adt8::C0, List::cons(f66(v2), v4), v5)))
                }
            }
        }
        Adt8::C2(v6, v7, v8) => {
            let a301 = 15u64;
            let a302 = f67(v2);
            if nat_lt(a301, a302) {
                Ok(f49(Adt2::C5(v1)))
            } else {
                let a305 = {
                    let a303 = v6;
                    let a304 = 1u64;
                    nat_add(a303, a304)?
                };
                let a306 = 4u64;
                if nat_eq(a305, a306) {
                    f68(v0, v1, {
                        let a309 = {
                            let a307 = v7;
                            let a308 = 16u64;
                            nat_mul(a307, a308)?
                        };
                        let a310 = f67(v2);
                        nat_add(a309, a310)?
                    }, v8, v4, v5)
                } else {
                    Ok(f48(v0, Adt7::C6(Adt8::C2({
                        let a311 = v6;
                        let a312 = 1u64;
                        nat_add(a311, a312)?
                    }, {
                        let a315 = {
                            let a313 = v7;
                            let a314 = 16u64;
                            nat_mul(a313, a314)?
                        };
                        let a316 = f67(v2);
                        nat_add(a315, a316)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt8::C3(v9) => {
            let a317 = v2;
            let a318 = 92u64;
            if nat_eq(a317, a318) {
                Ok(f48(v0, Adt7::C6(Adt8::C4(v9), v4, v5)))
            } else {
                Ok(f49(Adt2::C6(v1)))
            }
        }
        Adt8::C4(v10) => {
            let a319 = v2;
            let a320 = 117u64;
            if nat_eq(a319, a320) {
                Ok(f48(v0, Adt7::C6(Adt8::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f49(Adt2::C6(v1)))
            }
        }
    }
}

pub fn f51(v0: Adt6, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt12> {
    let a321 = v2;
    let a322 = f69(v3, v4);
    if nat_eq(a321, a322) {
        let a326 = {
            let a323 = v4;
            let a324 = 1u64;
            nat_add(a323, a324)?
        };
        let a327 = {
            let a325 = f70(v3);
            length_list(a325)
        };
        if nat_eq(a326, a327) {
            Ok(f71(v0, f72(v3)))
        } else {
            Ok(f48(v0, Adt7::C7(v3, {
                let a328 = v4;
                let a329 = 1u64;
                nat_add(a328, a329)?
            })))
        }
    } else {
        Ok(f49(Adt2::C2(v1)))
    }
}

pub fn f52(v0: Adt6, v1: u64, v2: u64, v3: Adt9) -> R<Adt12> {
    let m331 = match v3.clone() {
        Adt9::C0(_, h330, _, _, _, _, _) => {
            h330
        }
    };
    match m331 {
        Adt10::C0 => {
            let a332 = v2;
            let a333 = 48u64;
            if nat_eq(a332, a333) {
                Ok(f73(v0, v3, Adt10::C1))
            } else {
                if f26(v2, 49u64, 57u64) {
                    f74(v0, v1, v2, v3, false)
                } else {
                    Ok(f49(Adt2::C7(v1)))
                }
            }
        }
        Adt10::C1 => {
            let a334 = v2;
            let a335 = 46u64;
            if nat_eq(a334, a335) {
                Ok(f73(v0, v3, Adt10::C3))
            } else {
                if f75(v2) {
                    Ok(f73(v0, v3, Adt10::C5))
                } else {
                    if f76(v2) {
                        Ok(f49(Adt2::C7(v1)))
                    } else {
                        f77(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt10::C2 => {
            if f76(v2) {
                f74(v0, v1, v2, v3, false)
            } else {
                let a336 = v2;
                let a337 = 46u64;
                if nat_eq(a336, a337) {
                    Ok(f73(v0, v3, Adt10::C3))
                } else {
                    if f75(v2) {
                        Ok(f73(v0, v3, Adt10::C5))
                    } else {
                        f77(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt10::C3 => {
            if f76(v2) {
                f74(v0, v1, v2, v3, true)
            } else {
                Ok(f49(Adt2::C7(v1)))
            }
        }
        Adt10::C4 => {
            if f76(v2) {
                f74(v0, v1, v2, v3, true)
            } else {
                if f75(v2) {
                    Ok(f73(v0, v3, Adt10::C5))
                } else {
                    f77(v0, v1, v2, v3)
                }
            }
        }
        Adt10::C5 => {
            let a338 = v2;
            let a339 = 43u64;
            if nat_eq(a338, a339) {
                Ok(f48(v0, f59(match v3.clone() {
                    Adt9::C0(h340, _, _, _, _, _, _) => {
                        h340
                    }
                }, Adt10::C6, match v3.clone() {
                    Adt9::C0(_, _, h341, _, _, _, _) => {
                        h341
                    }
                }, match v3.clone() {
                    Adt9::C0(_, _, _, h342, _, _, _) => {
                        h342
                    }
                }, match v3 {
                    Adt9::C0(_, _, _, _, h343, _, _) => {
                        h343
                    }
                }, false, 0i64)))
            } else {
                let a344 = v2;
                let a345 = 45u64;
                if nat_eq(a344, a345) {
                    Ok(f48(v0, f59(match v3.clone() {
                        Adt9::C0(h346, _, _, _, _, _, _) => {
                            h346
                        }
                    }, Adt10::C6, match v3.clone() {
                        Adt9::C0(_, _, h347, _, _, _, _) => {
                            h347
                        }
                    }, match v3.clone() {
                        Adt9::C0(_, _, _, h348, _, _, _) => {
                            h348
                        }
                    }, match v3 {
                        Adt9::C0(_, _, _, _, h349, _, _) => {
                            h349
                        }
                    }, true, 0i64)))
                } else {
                    if f76(v2) {
                        f78(v0, v1, v2, Adt9::C0(match v3.clone() {
                            Adt9::C0(h350, _, _, _, _, _, _) => {
                                h350
                            }
                        }, match v3.clone() {
                            Adt9::C0(_, h351, _, _, _, _, _) => {
                                h351
                            }
                        }, match v3.clone() {
                            Adt9::C0(_, _, h352, _, _, _, _) => {
                                h352
                            }
                        }, match v3.clone() {
                            Adt9::C0(_, _, _, h353, _, _, _) => {
                                h353
                            }
                        }, match v3 {
                            Adt9::C0(_, _, _, _, h354, _, _) => {
                                h354
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f49(Adt2::C7(v1)))
                    }
                }
            }
        }
        Adt10::C6 => {
            if f76(v2) {
                f78(v0, v1, v2, v3.clone(), match v3 {
                    Adt9::C0(_, _, _, _, _, h355, _) => {
                        h355
                    }
                })
            } else {
                Ok(f49(Adt2::C7(v1)))
            }
        }
        Adt10::C7 => {
            if f76(v2) {
                f78(v0, v1, v2, v3.clone(), match v3 {
                    Adt9::C0(_, _, _, _, _, h356, _) => {
                        h356
                    }
                })
            } else {
                f77(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f53(v0: u64, v1: Adt0, v2: List<Adt5>, v3: List<u64>) -> R<Adt13> {
    let m357 = v1;
    match m357 {
        Adt0::C0 => {
            Ok(Adt13::C1(v2, f28(f79(2u64), v3)))
        }
        Adt0::C1(v4) => {
            Ok(Adt13::C1(v2, f28(if v4 {
                f79(0u64)
            } else {
                f79(1u64)
            }, v3)))
        }
        Adt0::C2(v5) => {
            if f80(v5.clone())? {
                Ok(Adt13::C1(v2, f28(f81(v5)?, v3)))
            } else {
                Ok(Adt13::C0(Adt2::C11))
            }
        }
        Adt0::C3(v6) => {
            Ok(Adt13::C1(v2, f28(f54(v6)?, v3)))
        }
        Adt0::C4(v7) => {
            let a358 = v0;
            let a359 = f82();
            if nat_lt(a358, a359) {
                Ok(Adt13::C1(f83(91u64, Adt5::C2({
                    let a360 = v0;
                    let a361 = 1u64;
                    nat_add(a360, a361)?
                }, v7), v2), v3))
            } else {
                Ok(Adt13::C0(Adt2::C12))
            }
        }
        Adt0::C5(v8) => {
            let a362 = v0;
            let a363 = f82();
            if nat_lt(a362, a363) {
                Ok(Adt13::C1(f83(123u64, Adt5::C4({
                    let a364 = v0;
                    let a365 = 1u64;
                    nat_add(a364, a365)?
                }, v8), v2), v3))
            } else {
                Ok(Adt13::C0(Adt2::C12))
            }
        }
    }
}

pub fn f54(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a367 = f28(f84(f5({
            let a366 = v0;
            utf8_encode(a366)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a368 = List::cons(34u64, List::<u64>::nil());
        append_list(a367, a368)
    }))
}

pub fn f55(p373: Bytes, p374: List<Bytes>) -> Bytes {
    let m372 = (p373, p374);
    {
        let mut m372 = m372;
        loop {
            let step = {
                let (v0, v1) = m372;
                let m369 = v1;
                match m369.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a370 = v2;
                            let a371 = v0;
                            append_bytes(a370, a371)
                        }, v3))
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

pub fn f56(p377: List<Bytes>, p378: List<u64>) -> List<Bytes> {
    let m376 = (p377, p378);
    {
        let mut m376 = m376;
        loop {
            let step = {
                let (v0, v1) = m376;
                let m375 = v1;
                match m375.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f85(f86(v2), v0), v3))
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

pub fn f57(v0: Adt10) -> bool {
    let m379 = v0;
    match m379 {
        Adt10::C0 => {
            false
        }
        Adt10::C1 => {
            true
        }
        Adt10::C2 => {
            true
        }
        Adt10::C3 => {
            false
        }
        Adt10::C4 => {
            true
        }
        Adt10::C5 => {
            false
        }
        Adt10::C6 => {
            false
        }
        Adt10::C7 => {
            true
        }
    }
}

pub fn f58(v0: Adt6, v1: Adt11, v2: Adt7, v3: u64) -> R<Adt12> {
    let a381 = match v0.clone() {
        Adt6::C0(_, _, h380) => {
            h380
        }
    };
    let a382 = f82();
    if nat_lt(a381, a382) {
        Ok(Adt12::C1(Adt6::C0(v2, List::cons(v1, match v0.clone() {
            Adt6::C0(_, h383, _) => {
                h383
            }
        }), {
            let a385 = match v0 {
                Adt6::C0(_, _, h384) => {
                    h384
                }
            };
            let a386 = 1u64;
            nat_add(a385, a386)?
        })))
    } else {
        Ok(f49(Adt2::C9(v3)))
    }
}

pub fn f59(v0: bool, v1: Adt10, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt7 {
    Adt7::C8(Adt9::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f60(v0: u64) -> i64 {
    let a387 = v0;
    let a388 = 0u64;
    if nat_eq(a387, a388) {
        0i64
    } else {
        let a389 = v0;
        let a390 = 1u64;
        if nat_eq(a389, a390) {
            1i64
        } else {
            let a391 = v0;
            let a392 = 2u64;
            if nat_eq(a391, a392) {
                2i64
            } else {
                let a393 = v0;
                let a394 = 3u64;
                if nat_eq(a393, a394) {
                    3i64
                } else {
                    let a395 = v0;
                    let a396 = 4u64;
                    if nat_eq(a395, a396) {
                        4i64
                    } else {
                        let a397 = v0;
                        let a398 = 5u64;
                        if nat_eq(a397, a398) {
                            5i64
                        } else {
                            let a399 = v0;
                            let a400 = 6u64;
                            if nat_eq(a399, a400) {
                                6i64
                            } else {
                                let a401 = v0;
                                let a402 = 7u64;
                                if nat_eq(a401, a402) {
                                    7i64
                                } else {
                                    let a403 = v0;
                                    let a404 = 8u64;
                                    if nat_eq(a403, a404) {
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

pub fn f61(p407: List<Adt0>, p408: List<Adt0>) -> List<Adt0> {
    let m406 = (p407, p408);
    {
        let mut m406 = m406;
        loop {
            let step = {
                let (v0, v1) = m406;
                let m405 = v0;
                match m405.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m406 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: Adt6, v1: u64) -> Adt12 {
    let m410 = match v0.clone() {
        Adt6::C0(_, h409, _) => {
            h409
        }
    };
    match m410.uncons() {
        None => {
            f49(Adt2::C2(v1))
        }
        Some((v2, _)) => {
            let m411 = v2;
            match m411 {
                Adt11::C0(_) => {
                    f48(v0, Adt7::C0)
                }
                Adt11::C1(_, _) => {
                    f48(v0, Adt7::C4)
                }
            }
        }
    }
}

pub fn f63(p414: List<(Str, Adt0)>, p415: List<(Str, Adt0)>) -> List<(Str, Adt0)> {
    let m413 = (p414, p415);
    {
        let mut m413 = m413;
        loop {
            let step = {
                let (v0, v1) = m413;
                let m412 = v0;
                match m412.uncons() {
                    None => {
                        Err::<(List<(Str, Adt0)>, List<(Str, Adt0)>), List<(Str, Adt0)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt0)>, List<(Str, Adt0)>), List<(Str, Adt0)>>((v3, List::cons(v2, v1)))
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

pub fn f64(v0: Adt6, v1: Adt7) -> Adt6 {
    Adt6::C0(v1, match v0.clone() {
        Adt6::C0(_, h416, _) => {
            h416
        }
    }, match v0 {
        Adt6::C0(_, _, h417) => {
            h417
        }
    })
}

pub fn f65(v0: Adt6, v1: u64, v2: List<u64>, v3: bool) -> Adt12 {
    let m419 = {
        let a418 = f38(f39(), f28(v2, List::<u64>::nil()));
        utf8_decode(a418)
    };
    match m419 {
        None => {
            f49(Adt2::C14)
        }
        Some(v4) => {
            if v3 {
                f87(v0, v1, v4)
            } else {
                f71(v0, Adt0::C3(v4))
            }
        }
    }
}

pub fn f66(v0: u64) -> u64 {
    let a420 = v0;
    let a421 = 34u64;
    if nat_eq(a420, a421) {
        34u64
    } else {
        let a422 = v0;
        let a423 = 92u64;
        if nat_eq(a422, a423) {
            92u64
        } else {
            let a424 = v0;
            let a425 = 47u64;
            if nat_eq(a424, a425) {
                47u64
            } else {
                let a426 = v0;
                let a427 = 98u64;
                if nat_eq(a426, a427) {
                    8u64
                } else {
                    let a428 = v0;
                    let a429 = 102u64;
                    if nat_eq(a428, a429) {
                        12u64
                    } else {
                        let a430 = v0;
                        let a431 = 110u64;
                        if nat_eq(a430, a431) {
                            10u64
                        } else {
                            let a432 = v0;
                            let a433 = 114u64;
                            if nat_eq(a432, a433) {
                                13u64
                            } else {
                                let a434 = v0;
                                let a435 = 116u64;
                                if nat_eq(a434, a435) {
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

pub fn f67(v0: u64) -> u64 {
    let a436 = v0;
    let a437 = 48u64;
    if nat_lt(a436, a437) {
        16u64
    } else {
        let a438 = v0;
        let a439 = 58u64;
        if nat_lt(a438, a439) {
            let a440 = v0;
            let a441 = 48u64;
            nat_sub(a440, a441)
        } else {
            let a442 = v0;
            let a443 = 65u64;
            if nat_lt(a442, a443) {
                16u64
            } else {
                let a444 = v0;
                let a445 = 71u64;
                if nat_lt(a444, a445) {
                    let a446 = v0;
                    let a447 = 55u64;
                    nat_sub(a446, a447)
                } else {
                    let a448 = v0;
                    let a449 = 97u64;
                    if nat_lt(a448, a449) {
                        16u64
                    } else {
                        let a450 = v0;
                        let a451 = 103u64;
                        if nat_lt(a450, a451) {
                            let a452 = v0;
                            let a453 = 87u64;
                            nat_sub(a452, a453)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f68(v0: Adt6, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt12> {
    let a454 = v3;
    let a455 = 0u64;
    if nat_eq(a454, a455) {
        if f26(v2, 55296u64, 56319u64) {
            Ok(f48(v0, Adt7::C6(Adt8::C3(v2), v4, v5)))
        } else {
            if f26(v2, 56320u64, 57343u64) {
                Ok(f49(Adt2::C6(v1)))
            } else {
                Ok(f48(v0, Adt7::C6(Adt8::C0, f88(v2, v4)?, v5)))
            }
        }
    } else {
        if f26(v2, 56320u64, 57343u64) {
            Ok(f48(v0, Adt7::C6(Adt8::C0, f88({
                let a464 = 65536u64;
                let a465 = {
                    let a462 = {
                        let a458 = {
                            let a456 = v3;
                            let a457 = 55296u64;
                            nat_sub(a456, a457)
                        };
                        let a459 = 1024u64;
                        nat_mul(a458, a459)?
                    };
                    let a463 = {
                        let a460 = v2;
                        let a461 = 56320u64;
                        nat_sub(a460, a461)
                    };
                    nat_add(a462, a463)?
                };
                nat_add(a464, a465)?
            }, v4)?, v5)))
        } else {
            Ok(f49(Adt2::C6(v1)))
        }
    }
}

pub fn f69(v0: u64, v1: u64) -> u64 {
    let m468 = {
        let a466 = f70(v0);
        let a467 = v1;
        index_list(a466, a467)
    };
    match m468 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f70(v0: u64) -> List<u64> {
    let a469 = v0;
    let a470 = 0u64;
    if nat_eq(a469, a470) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a471 = v0;
        let a472 = 1u64;
        if nat_eq(a471, a472) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f71(v0: Adt6, v1: Adt0) -> Adt12 {
    Adt12::C1(f42(v1, match v0.clone() {
        Adt6::C0(_, h473, _) => {
            h473
        }
    }, match v0 {
        Adt6::C0(_, _, h474) => {
            h474
        }
    }))
}

pub fn f72(v0: u64) -> Adt0 {
    let a475 = v0;
    let a476 = 0u64;
    if nat_eq(a475, a476) {
        Adt0::C1(true)
    } else {
        let a477 = v0;
        let a478 = 1u64;
        if nat_eq(a477, a478) {
            Adt0::C1(false)
        } else {
            Adt0::C0
        }
    }
}

pub fn f73(v0: Adt6, v1: Adt9, v2: Adt10) -> Adt12 {
    f48(v0, Adt7::C8(Adt9::C0(match v1.clone() {
        Adt9::C0(h479, _, _, _, _, _, _) => {
            h479
        }
    }, v2, match v1.clone() {
        Adt9::C0(_, _, h480, _, _, _, _) => {
            h480
        }
    }, match v1.clone() {
        Adt9::C0(_, _, _, h481, _, _, _) => {
            h481
        }
    }, match v1.clone() {
        Adt9::C0(_, _, _, _, h482, _, _) => {
            h482
        }
    }, match v1.clone() {
        Adt9::C0(_, _, _, _, _, h483, _) => {
            h483
        }
    }, match v1 {
        Adt9::C0(_, _, _, _, _, _, h484) => {
            h484
        }
    })))
}

pub fn f74(v0: Adt6, v1: u64, v2: u64, v3: Adt9, v4: bool) -> R<Adt12> {
    let v5: u64 = {
        let a486 = match v3.clone() {
            Adt9::C0(_, _, _, h485, _, _, _) => {
                h485
            }
        };
        let a487 = 0u64;
        if if nat_eq(a486, a487) {
            let a488 = v2;
            let a489 = 48u64;
            nat_eq(a488, a489)
        } else {
            false
        } {
            0u64
        } else {
            let a491 = match v3.clone() {
                Adt9::C0(_, _, _, h490, _, _, _) => {
                    h490
                }
            };
            let a492 = 1u64;
            nat_add(a491, a492)?
        }
    };
    let v6: i64 = if v4 {
        f37(match v3.clone() {
            Adt9::C0(_, _, _, _, h493, _, _) => {
                h493
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt9::C0(_, _, _, _, h494, _, _) => {
                h494
            }
        }
    };
    let a495 = f89();
    let a496 = v5;
    if nat_lt(a495, a496) {
        Ok(f49(Adt2::C8(v1)))
    } else {
        let m499 = {
            let a497 = 99999i64;
            let a498 = v6;
            compare(a497, a498)
        };
        if match m499 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f49(Adt2::C8(v1)))
        } else {
            Ok(f48(v0, f59(match v3.clone() {
                Adt9::C0(h500, _, _, _, _, _, _) => {
                    h500
                }
            }, if v4 {
                Adt10::C4
            } else {
                Adt10::C2
            }, f37({
                let a502 = match v3.clone() {
                    Adt9::C0(_, _, h501, _, _, _, _) => {
                        h501
                    }
                };
                let a503 = 10i64;
                int_mul(a502, a503)?
            }, f60({
                let a504 = v2;
                let a505 = 48u64;
                nat_sub(a504, a505)
            }))?, v5, v6, match v3.clone() {
                Adt9::C0(_, _, _, _, _, h506, _) => {
                    h506
                }
            }, match v3 {
                Adt9::C0(_, _, _, _, _, _, h507) => {
                    h507
                }
            })))
        }
    }
}

pub fn f75(v0: u64) -> bool {
    let a508 = v0;
    let a509 = 101u64;
    if nat_eq(a508, a509) {
        true
    } else {
        let a510 = v0;
        let a511 = 69u64;
        nat_eq(a510, a511)
    }
}

pub fn f76(v0: u64) -> bool {
    f26(v0, 48u64, 57u64)
}

pub fn f77(v0: Adt6, v1: u64, v2: u64, v3: Adt9) -> R<Adt12> {
    let m512 = f41(v3)?;
    match m512 {
        Adt14::C0 => {
            Ok(f49(Adt2::C7(v1)))
        }
        Adt14::C1 => {
            Ok(f49(Adt2::C8(v1)))
        }
        Adt14::C2(v4) => {
            Ok(f46(f42(v4, match v0.clone() {
                Adt6::C0(_, h513, _) => {
                    h513
                }
            }, match v0 {
                Adt6::C0(_, _, h514) => {
                    h514
                }
            }), v1, v2))
        }
    }
}

pub fn f78(v0: Adt6, v1: u64, v2: u64, v3: Adt9, v4: bool) -> R<Adt12> {
    let v5: i64 = f37({
        let a516 = match v3.clone() {
            Adt9::C0(_, _, _, _, _, _, h515) => {
                h515
            }
        };
        let a517 = 10i64;
        int_mul(a516, a517)?
    }, f60({
        let a518 = v2;
        let a519 = 48u64;
        nat_sub(a518, a519)
    }))?;
    let m522 = {
        let a520 = 99999i64;
        let a521 = v5;
        compare(a520, a521)
    };
    if match m522 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f49(Adt2::C8(v1)))
    } else {
        Ok(f48(v0, f59(match v3.clone() {
            Adt9::C0(h523, _, _, _, _, _, _) => {
                h523
            }
        }, Adt10::C7, match v3.clone() {
            Adt9::C0(_, _, h524, _, _, _, _) => {
                h524
            }
        }, match v3.clone() {
            Adt9::C0(_, _, _, h525, _, _, _) => {
                h525
            }
        }, match v3 {
            Adt9::C0(_, _, _, _, h526, _, _) => {
                h526
            }
        }, v4, v5)))
    }
}

pub fn f79(v0: u64) -> List<u64> {
    f70(v0)
}

pub fn f80(v0: Adt1) -> R<bool> {
    let a544 = {
        let m530 = {
            let a528 = f90();
            let a529 = match v0.clone() {
                Adt1::C0(h527, _) => {
                    h527
                }
            };
            compare(a528, a529)
        };
        if match m530 {
            Ordering::Less => {
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
            let m535 = {
                let a533 = match v0.clone() {
                    Adt1::C0(h531, _) => {
                        h531
                    }
                };
                let a534 = {
                    let a532 = f90();
                    int_neg(a532)?
                };
                compare(a533, a534)
            };
            if match m535 {
                Ordering::Less => {
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
                let m539 = {
                    let a537 = 9999i64;
                    let a538 = match v0.clone() {
                        Adt1::C0(_, h536) => {
                            h536
                        }
                    };
                    compare(a537, a538)
                };
                if match m539 {
                    Ordering::Less => {
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
                    let m543 = {
                        let a541 = match v0 {
                            Adt1::C0(_, h540) => {
                                h540
                            }
                        };
                        let a542 = -9999i64;
                        compare(a541, a542)
                    };
                    match m543 {
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
    Ok(bool_not(a544))
}

pub fn f81(v0: Adt1) -> R<List<u64>> {
    let m548 = {
        let a546 = match v0.clone() {
            Adt1::C0(_, h545) => {
                h545
            }
        };
        let a547 = 0i64;
        compare(a546, a547)
    };
    if if match m548 {
        Ordering::Less => {
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
        let m552 = {
            let a550 = 0i64;
            let a551 = match v0.clone() {
                Adt1::C0(_, h549) => {
                    h549
                }
            };
            compare(a550, a551)
        };
        match m552 {
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
        let a555 = f91(match v0.clone() {
            Adt1::C0(h553, _) => {
                h553
            }
        })?;
        let a556 = List::cons(101u64, f91(match v0 {
            Adt1::C0(_, h554) => {
                h554
            }
        })?);
        Ok(append_list(a555, a556))
    } else {
        f91(match v0 {
            Adt1::C0(h557, _) => {
                h557
            }
        })
    }
}

pub fn f82() -> u64 {
    512u64
}

pub fn f83(v0: u64, v1: Adt5, v2: List<Adt5>) -> List<Adt5> {
    List::cons(Adt5::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f84(p560: List<u64>, p561: List<u64>) -> R<List<u64>> {
    let m559 = (p560, p561);
    {
        let mut m559 = m559;
        loop {
            let step = {
                let (v0, v1) = m559;
                let m558 = v0;
                match m558.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f28(f92(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m559 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(p570: Bytes, p571: List<Bytes>) -> List<Bytes> {
    let m569 = (p570, p571);
    {
        let mut m569 = m569;
        loop {
            let step = {
                let (v0, v1) = m569;
                let m562 = v1;
                match m562.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a565 = {
                            let a563 = v2.clone();
                            length_bytes(a563)
                        };
                        let a566 = {
                            let a564 = v0.clone();
                            length_bytes(a564)
                        };
                        if nat_eq(a565, a566) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a567 = v2;
                                let a568 = v0;
                                append_bytes(a567, a568)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
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

pub fn f86(v0: u64) -> Bytes {
    let m575 = {
        let a572 = f93();
        let a573 = v0;
        let a574 = 1u64;
        slice_bytes(a572, a573, a574)
    };
    match m575 {
        None => {
            f39()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f87(v0: Adt6, v1: u64, v2: Str) -> Adt12 {
    let m577 = match v0.clone() {
        Adt6::C0(_, h576, _) => {
            h576
        }
    };
    match m577.uncons() {
        None => {
            f49(Adt2::C2(v1))
        }
        Some((v3, v4)) => {
            let m578 = v3;
            match m578 {
                Adt11::C0(_) => {
                    f49(Adt2::C2(v1))
                }
                Adt11::C1(v6, _) => {
                    Adt12::C1(Adt6::C0(Adt7::C5, List::cons(Adt11::C1(v6, Some(v2)), v4), match v0 {
                        Adt6::C0(_, _, h579) => {
                            h579
                        }
                    }))
                }
            }
        }
    }
}

pub fn f88(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a580 = v0;
    let a581 = 128u64;
    if nat_lt(a580, a581) {
        Ok(List::cons(v0, v1))
    } else {
        let a582 = v0;
        let a583 = 2048u64;
        if nat_lt(a582, a583) {
            Ok(List::cons({
                let a584 = 128u64;
                let a585 = f94(v0, 64u64);
                nat_add(a584, a585)?
            }, List::cons({
                let a586 = 192u64;
                let a587 = f95(v0, 64u64);
                nat_add(a586, a587)?
            }, v1)))
        } else {
            let a588 = v0;
            let a589 = 65536u64;
            if nat_lt(a588, a589) {
                Ok(List::cons({
                    let a590 = 128u64;
                    let a591 = f94(v0, 64u64);
                    nat_add(a590, a591)?
                }, List::cons({
                    let a592 = 128u64;
                    let a593 = f94(f95(v0, 64u64), 64u64);
                    nat_add(a592, a593)?
                }, List::cons({
                    let a594 = 224u64;
                    let a595 = f95(v0, 4096u64);
                    nat_add(a594, a595)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a596 = 128u64;
                    let a597 = f94(v0, 64u64);
                    nat_add(a596, a597)?
                }, List::cons({
                    let a598 = 128u64;
                    let a599 = f94(f95(v0, 64u64), 64u64);
                    nat_add(a598, a599)?
                }, List::cons({
                    let a600 = 128u64;
                    let a601 = f94(f95(v0, 4096u64), 64u64);
                    nat_add(a600, a601)?
                }, List::cons({
                    let a602 = 240u64;
                    let a603 = f95(v0, 262144u64);
                    nat_add(a602, a603)?
                }, v1)))))
            }
        }
    }
}

pub fn f89() -> u64 {
    18u64
}

pub fn f90() -> i64 {
    999999999999999999i64
}

pub fn f91(v0: i64) -> R<List<u64>> {
    let m606 = {
        let a604 = v0;
        let a605 = 0i64;
        compare(a604, a605)
    };
    if match m606 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f96(20u64, {
            let a607 = v0;
            int_neg(a607)?
        }, List::<u64>::nil())?))
    } else {
        f96(20u64, v0, List::<u64>::nil())
    }
}

pub fn f92(v0: u64) -> R<List<u64>> {
    let a608 = v0;
    let a609 = 34u64;
    if nat_eq(a608, a609) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a610 = v0;
        let a611 = 92u64;
        if nat_eq(a610, a611) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a612 = v0;
            let a613 = 8u64;
            if nat_eq(a612, a613) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a614 = v0;
                let a615 = 9u64;
                if nat_eq(a614, a615) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a616 = v0;
                    let a617 = 10u64;
                    if nat_eq(a616, a617) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a618 = v0;
                        let a619 = 12u64;
                        if nat_eq(a618, a619) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a620 = v0;
                            let a621 = 13u64;
                            if nat_eq(a620, a621) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a622 = v0;
                                let a623 = 32u64;
                                if nat_lt(a622, a623) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f97(f95(v0, 16u64))?, List::cons(f97(f94(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f93() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f94(v0: u64, v1: u64) -> u64 {
    let a624 = v0;
    let a625 = v1;
    let a626 = 0u64;
    nat_rem(a624, a625, a626)
}

pub fn f95(v0: u64, v1: u64) -> u64 {
    let a627 = v0;
    let a628 = v1;
    let a629 = 0u64;
    nat_quot(a627, a628, a629)
}

pub fn f96(p641: u64, p642: i64, p643: List<u64>) -> R<List<u64>> {
    let m640 = (p641, (p642, p643));
    {
        let mut m640 = m640;
        loop {
            let step = {
                let (v0, (v1, v2)) = m640;
                let m630 = v0;
                if m630 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m630 - 1;
                    let v4: i64 = {
                        let a631 = v1;
                        let a632 = 10i64;
                        let a633 = 0i64;
                        int_quot(a631, a632, a633)?
                    };
                    let v5: i64 = {
                        let a634 = v1;
                        let a635 = 10i64;
                        let a636 = 0i64;
                        int_rem(a634, a635, a636)
                    };
                    let m639 = {
                        let a637 = v4;
                        let a638 = 1i64;
                        compare(a637, a638)
                    };
                    if match m639 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f98(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f98(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m640 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: u64) -> R<u64> {
    let a644 = v0;
    let a645 = 10u64;
    if nat_lt(a644, a645) {
        let a646 = 48u64;
        let a647 = v0;
        nat_add(a646, a647)
    } else {
        let a648 = 87u64;
        let a649 = v0;
        nat_add(a648, a649)
    }
}

pub fn f98(v0: i64) -> u64 {
    let m652 = {
        let a650 = v0;
        let a651 = 1i64;
        compare(a650, a651)
    };
    if match m652 {
        Ordering::Less => {
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
        let m655 = {
            let a653 = v0;
            let a654 = 2i64;
            compare(a653, a654)
        };
        if match m655 {
            Ordering::Less => {
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
            let m658 = {
                let a656 = v0;
                let a657 = 3i64;
                compare(a656, a657)
            };
            if match m658 {
                Ordering::Less => {
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
                let m661 = {
                    let a659 = v0;
                    let a660 = 4i64;
                    compare(a659, a660)
                };
                if match m661 {
                    Ordering::Less => {
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
                    let m664 = {
                        let a662 = v0;
                        let a663 = 5i64;
                        compare(a662, a663)
                    };
                    if match m664 {
                        Ordering::Less => {
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
                        let m667 = {
                            let a665 = v0;
                            let a666 = 6i64;
                            compare(a665, a666)
                        };
                        if match m667 {
                            Ordering::Less => {
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
                            let m670 = {
                                let a668 = v0;
                                let a669 = 7i64;
                                compare(a668, a669)
                            };
                            if match m670 {
                                Ordering::Less => {
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
                                let m673 = {
                                    let a671 = v0;
                                    let a672 = 8i64;
                                    compare(a671, a672)
                                };
                                if match m673 {
                                    Ordering::Less => {
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
                                    let m676 = {
                                        let a674 = v0;
                                        let a675 = 9i64;
                                        compare(a674, a675)
                                    };
                                    if match m676 {
                                        Ordering::Less => {
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

pub fn run(p0: u64, p1: Bytes) -> R<List<u64>> {
    f0(p0, p1)
}
