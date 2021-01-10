extern crate proc_macro;//Rust 自带 proc_macro crate，因此无需将其加到 Cargo.toml 文件的依赖中。
//proc_macro crate 是编译器用来读取和操作我们 Rust 代码的 API。

use crate::proc_macro::TokenStream;
use quote::quote;//quote 则将 syn 解析的数据结构转换回 Rust 代码
use syn;//syn crate 将字符串中的 Rust 代码解析成为一个可以操作的数据结构

#[proc_macro_derive(HelloMacro)]//当用户在一个类型上指定 #[derive(HelloMacro)] 时，hello_macro_derive 函数将会被调用
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {//原因在于我们已经使用 proc_macro_derive 及其指定名称对 hello_macro_derive 函数进行了注解：HelloMacro
    // 构建 Rust 代码所代表的语法树
    // 以便可以进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        //这个宏也提供了一些非常酷的模板机制；我们可以写 #name ，然后 quote! 会以名为 name 的变量值来替换它。
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));//此处所使用的 stringify! 为 Rust 内置宏。其接收一个 Rust 表达式，如 1 + 2 ， 然后在编译时将表达式转换为一个字符串常量，如 "1 + 2" 
            }
        }
    };
    gen.into()//quote! 宏执行的直接结果并不是编译器所期望的并需要转换为 TokenStream。为此需要调用 into 方法，它会消费这个中间表示（intermediate representation，IR）并返回所需的 TokenStream 类型值
}
