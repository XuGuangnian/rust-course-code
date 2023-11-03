mod array;
mod enumeration;
mod string_slice;
mod structs;
mod tuple;

fn main() {
    string_slice::run();
    tuple::run();
    structs::run();
    enumeration::run();
    array::run();
}
