/// Macro which declares give constants of same type and also declares an
/// additional constant which contains all the constants in a list
#[macro_export]
macro_rules! declare_all_consts {
    ($module_name: ident, $list_name: ident, $const_type: ty ,{ $($const_name: ident, $camel_case: ident : $const_value: expr), *$(,) ? } ) => {
        pub mod $module_name {
            $(
                pub const $const_name: $const_type = $const_value;
            )*

            pub const $list_name: &[$const_type] = &[
                $($const_name),*
            ];

            #[derive(Clone, Copy, Debug)]
            pub enum Chars {
                $(
                    $camel_case,
                )*
            }
        }
    };

    ($module_name: ident, $list_name: ident, $const_prefix: ident : $camel_case: ident, $const_type: ty, { $($const_name: ident: $const_value: expr), *$(,) ?  } ) => {
        paste::paste! {
            pub mod $module_name {
                $(
                    pub const [<$const_prefix _ $const_name>]: $const_type = $const_value;
                )*

                pub const $list_name: &[$const_type] = &[
                    $([<$const_prefix _ $const_name>]),*
                ];

                #[derive(Clone, Copy, Debug)]
                pub enum Chars {
                    $(
                        [<$camel_case $const_name> ],
                    )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! catch_error {
    ($result: expr => $err: expr) => {
        if $result.is_err() {
            return Err($err);
        }
    };
}
