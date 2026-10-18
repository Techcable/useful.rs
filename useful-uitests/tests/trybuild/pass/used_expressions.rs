#![deny(unused)]

use core::fmt::Debug;

fn foo(_x: impl Debug) {}
fn bar(_x: impl Debug, _y: impl Debug) {}
fn baz(_x: impl Debug) {}
fn helper(_x: impl Debug) {}
useful::used!(let foo(""), let bar(0u8, ()), let { baz as fn(&'static str) },);
useful::used_for_tests!(let helper(""));

fn main() {}
