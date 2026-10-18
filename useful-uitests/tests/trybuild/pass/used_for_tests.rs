#![deny(unused)]

fn only_used_in_tests() {}
useful::used_for_tests!(use only_used_in_tests);

fn main() {}
