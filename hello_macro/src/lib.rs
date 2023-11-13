pub trait HelloMacro {
    fn hello_macro();
}

pub trait MyDefault: Sized {
    fn default() -> Self;
}
