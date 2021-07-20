extern crate proc_macro;
use proc_macro::*;
use quote::quote;
use syn;

// 标识为 procedural 扩展宏
#[proc_macro_derive(Hello)]
pub fn say_hello_derive(input: TokenStream) -> TokenStream {
    // 从 tokenstream 中提取出类型名称
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
      impl SayHello for #name {
        fn say_hello(&self) {
          println!("{} say hello", stringify!(#name));
        }
      }
    };
    gen.into()
}

// procedural macros 注解宏
#[proc_macro_attribute]
pub fn say_hello(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let name_new = attr.to_string();
    let gen = quote! {
      #ast
      impl SayHello for #name {
        fn say_hello(&self) {
          println!("{} say hello", #name_new);
        }
      }
    };
    gen.into()
}

#[proc_macro]
pub fn hello_custer(_: TokenStream) -> TokenStream {
    r#"
      pub fn hello_custer() {
        println!("Hello from Custer!");
      }
  "#
    .parse()
    .unwrap()
}
