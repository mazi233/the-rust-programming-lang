extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
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

// 第一个用于属性内容本身，也就是 GET, "/" 部分。
// 第二个是属性所标记的项：在本例中，是 fn index() {} 和剩下的函数体
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
  let gen = quote! {};
  gen.into()
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
  let gen = quote! {};
  gen.into()
}
