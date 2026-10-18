#![expect(
    unused_imports,
    reason = "because used! macro fails",
)]

use core::fmt::Debug;
use std::collections::HashMap;

useful::used!(Debug, HashMap);

fn main() {}
