#![feature(proc_macro_hygiene)]

use ink_lang as ink;
use ink_core::{
    env::DefaultSrmlTypes,
    storage,
};

#[ink::contract(
    env = DefaultSrmlTypes,
    version = [0, 1, 0],
)]
mod noop {
    #[ink(storage)]
    struct Noop {}

    impl Noop {
        #[ink(constructor)]
        fn new(&mut self) {}
    }
}

fn main() {}
