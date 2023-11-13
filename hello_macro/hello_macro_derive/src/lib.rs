use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST(Abstract Syntax Tree)
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(MyDefault)]
pub fn my_default_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_my_default(&ast)
}

fn impl_my_default(ast: &DeriveInput) -> TokenStream {
    let id = &ast.ident;
    let Data::Struct(s) = &ast.data else {
        panic!("MyDefault derive macro must use in struct");
    };

    // 声明一个新的ast，用于动态构建字段赋值的token
    let mut field_ast = quote!();

    // 这里就是要动态添加token的地方了，需要动态完成Self的字段赋值
    for (idx, f) in s.fields.iter().enumerate() {
        let (field_id, field_ty) = (&f.ident, &f.ty);

        if field_id.is_none() {
            //没有ident表示是匿名字段，对于匿名字段，都需要添加 `#field_idx: #field_type::default(),` 这样的代码
            let field_idx = syn::Index::from(idx);
            field_ast.extend(quote! {
                #field_idx: #field_ty::default(),
            });
        } else {
            //对于命名字段，都需要添加 `#field_name: #field_type::default(),` 这样的代码
            field_ast.extend(quote! {
                #field_id: #field_ty::default(),
            });
        }
    }

    let gen = quote! {
        impl MyDefault for #id {
            fn default() -> Self {
                Self { #field_ast }
            }
        }
    };
    gen.into()
}
