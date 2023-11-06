mod deref;
mod drop;
mod rc_arc;
mod smart_pointer;

fn main() {
    smart_pointer::run();
    deref::run();
    drop::run();
    rc_arc::run();
}
