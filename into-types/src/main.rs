mod conversion;
mod custom_type;
mod dst;
mod enum_int;

fn main() {
    conversion::run();
    custom_type::run();
    dst::run();
    enum_int::run();
}
