use attr::Attribute;
use file::FileTrait;
use impl_object::impl_object;
use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;
mod attr;
mod file;
mod gen_rc;
mod impl_object;
use crate::gen_rc::GenImpl;

/// # 转换 File trait 的各种方法到 Object 结构体中
/// 例如 `Client` 结构体中有 `put_file` 方法，通过这个 macro ，可以让 Object 结构体支持 `put_file` 方法
///
/// 注意，之前的方法签名是这样的 `put_file(filename, path)`，由于 Object 本身有 path 属性，转换后的方法是这样的
/// `put_file(filename, &filer)`，其中 filer 可以传入实现 `File` trait 的结构体，在 oss-rs 项目中，有 `Client`, `Bucket`, `ObjectList`
/// 等结构体已实现了该trait，可以直接传入使用，其他的也可以
#[proc_macro_attribute]
pub fn oss_file(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Attribute);
    let mut item = parse_macro_input!(input as FileTrait);
    impl_object(&mut item, attr.send);
    TokenStream::from(quote!(#item))
}

/// # 根据 Arc 自动生成 Rc 代码
/// 目前支持的转换为：
///
/// ArcPointer => RcPointer
///
/// Arc => Rc
///
/// Arc::clone() => Rc::clone()
///
/// ClientArc => ClientRc
///
/// 还会在新生成的 `impl {}` 语句块之前添加 `#[cfg(feature = "blocking")]` 标记
///
#[proc_macro_attribute]
pub fn oss_gen_rc(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as GenImpl);
    TokenStream::from(quote!(#item))
}
