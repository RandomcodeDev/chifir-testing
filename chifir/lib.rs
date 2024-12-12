#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]

#![feature(concat_idents)]
#![feature(macro_metavar_expr)]

#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

#[macro_use]
mod log_impl {
    macro_rules! make_log_function {
        ($level:ident) => {
            #[macro_export]
            macro_rules! $level {
                ($$($$args:tt)*) => {
                    $crate::Log::WriteSafe($crate::Log::Level::$level, line!(), false, file!(), $crate::function!(), format!($$($$args)*).as_str())
                };
            }
            pub(crate) use $level;
        }
    }

    make_log_function! {Trace}
    make_log_function! {Debug}
    make_log_function! {Info}
    make_log_function! {Warning}
    make_log_function! {Error}
    make_log_function! {FatalError}
}

pub mod Base;
pub mod Log;
pub mod Plat;
