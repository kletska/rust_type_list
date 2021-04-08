mod type_list;
mod type_tree;

use crate::type_list::*;
use crate::type_tree::*;

fn main() {
    let tree = "I am root"
        .root()
        .add_leaf("I am daughter")
        .add_child("I am child")
            .add_child("I am grandson")
                .add_leaf("I am grandgrandson")
            .back()
            .add_leaf("I am grandson to")
        .back()
        ;
    let list = ("first",)
        .add("second")
        .add("third");
    println!("{}", list.start_print())
}