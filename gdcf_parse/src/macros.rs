macro_rules! __parsing {
    (@ $value: expr, index = $idx: expr, parse = $func: path) => {
        match $func($value) {
            Err(err) => return Err(ValueError::Parse(stringify!($idx), $value, Box::new(err))),
            Ok(v) => Some(v),
        }
    };
    (@ $value: expr, index = $idx: expr, parse = $func: path, $($also_tokens:tt)*) => {
        match $func($value) {
            Err(err) => return Err(ValueError::Parse(stringify!($idx), $value, Box::new(err))),
            Ok(v) => Some(v),
        }
    };

    (@ $value: expr, index = $idx: expr, with = $func: path) => {
        parse(stringify!($idx), $value)?.map($func)
    };

    (@ $value: expr, index = $idx: expr, with = $func: path, $($also_tokens:tt)*) => {
        parse(stringify!($idx), $value)?.map($func)
    };

    (@ $value: expr, index = $idx: expr, parse_infallible = $func: path) => {
        Some($func($value))
    };

    (@ $value: expr, index = $idx: expr, parse_infallible = $func: path, $($also_tokens:tt)*) => {
        Some($func($value))
    };

    (@ $value: expr, index = $idx: expr, $($also_tokens:tt)*) => {
        parse(stringify!($idx), $value)?
    };

    (@ $value: expr, index = $idx: expr) => {
        parse(stringify!($idx), $value)?
    };
}

macro_rules! __index {
    (index = $idx: expr) => {
        stringify!($idx)
    };

    (index = $idx: expr, $($also_tokens:tt)*) => {
        stringify!($idx)
    };
}

macro_rules! __unwrap {
    ($field_name: ident(index = $idx: expr)) => {
        $field_name.ok_or(ValueError::NoValue(stringify!($idx)))?
    };

    ($field_name: ident(index = $idx: expr, default)) => {
        $field_name.unwrap_or_default()
    };

    ($field_name: ident(index = $idx: expr, default = $default_func: path)) => {
        $field_name.unwrap_or_self($default_func)
    };

    ($field_name: ident(index = $idx: expr, with = $p: path $(, $($crap:tt)*)?)) => {
        __unwrap!($field_name(index = $idx $(, $($crap)*)?))
    };

    ($field_name: ident(index = $idx: expr, parse_infallible = $p: path $(, $($crap:tt)*)?)) => {
        __unwrap!($field_name(index = $idx $(, $($crap)*)?))
    };

    ($field_name: ident(index = $idx: expr, parse = $p: path $(, $($crap:tt)*)?)) => {
        __unwrap!($field_name(index = $idx $(, $($crap)*)?))
    };
}

macro_rules! parser {
    ($struct_name: ty => {$($tokens:tt)*}$(, $($tokens2:tt)*)?) => {
        parser!(@ $struct_name [] [] [] [$($tokens)*] [$($($tokens2)*)?]);
    };

    (@ $struct_name: ty [$($crap:tt)*] [] [$($crap3:tt)*] [$field_name: ident(custom $($data: tt)*), $($tokens:tt)*] [$($rest:tt)*]) => {
        parser!(@ $struct_name [$($crap)*] [] [$($crap3)*, $field_name(custom $($data)*)] [$($tokens)*] [$($rest)*]);
    };

    (@ $struct_name: ty [$($crap:tt)*] [] [$($crap3:tt)*] [$field_name: ident(delegate), $($tokens:tt)*] [$($rest:tt)*]) => {
        parser!(@@ $struct_name, $field_name [$($crap)*] [] [$($crap3)*] [$($tokens)*] [$($rest)*]);
    };

    (@ $struct_name: ty [$($crap:tt)*] [] [$($crap3:tt)*] [$field_name: ident($($data: tt)*), $($tokens:tt)*] [$($rest:tt)*]) => {
        parser!(@ $struct_name [$($crap)*, $field_name($($data)*)] [] [$($crap3)*] [$($tokens)*] [$($rest)*]);
    };

    (@ $struct_name: ty [$($crap:tt)*] [$($crap2:tt)*] [$($crap3:tt)*] [] [$field_name: ident($($data: tt)*), $($rest:tt)*]) => {
        parser!(@ $struct_name [$($crap)*] [$($crap2)*, $field_name($($data)*)] [$($crap3)*] [] [$($rest)*]);
    };

    (@ $struct_name: ty
        [$(, $field_name: ident($($tokens:tt)*))*]
        [$(, $helper_field: ident($($tokens2:tt)*))*]
        [$(, $custom_field: ident(custom = $func: path, depends_on = [$($field: expr),*]))*]
        [] []
    ) => {
        impl Parse for $struct_name {
            #[inline]
            fn parse<'a, I, F>(iter: I, mut f: F) -> Result<Self, ValueError<'a>>
            where
                I: Iterator<Item = (&'a str, &'a str)> + Clone,
                F: FnMut(&'a str, &'a str) -> Result<(), ValueError<'a>>
            {
                use $crate::util::parse;

                trace!("Parsing {}", stringify!($struct_name));

                $(
                    let mut $field_name = None;
                )*

                $(
                    let mut $helper_field = None;
                )*

                for (idx, value) in iter.into_iter() {
                    match idx {
                        $(
                            __index!($($tokens)*) => $field_name = __parsing!(@ value, $($tokens)*),
                        )*
                        $(
                            __index!($($tokens2)*) => {
                                $helper_field = __parsing!(@ value, $($tokens2)*);

                                f(idx, value)?
                            },
                        )*
                        _ => f(idx, value)?
                    }
                }

                $(
                    let $field_name = __unwrap!($field_name($($tokens)*));
                )*

                $(
                    let $helper_field = __unwrap!($helper_field($($tokens2)*));
                )*

                Ok(Self {
                    $(
                        $field_name,
                    )*
                    $(
                        $custom_field: $func($($field,)*),
                    )*
                })
            }
        }
    };

    (@@ $struct_name: ty, $delegated: ident [$($crap:tt)*] [] [$($crap3:tt)*] [$field_name: ident(custom $($data: tt)*), $($tokens:tt)*] [$($rest:tt)*]) => {
        parser!(@@ $struct_name, $delegated [$($crap)*] [] [$($crap3)*, $field_name(custom $($data)*)] [$($tokens)*] [$($rest)*]);
    };

    (@@ $struct_name: ty, $delegated: ident [$($crap:tt)*] [] [$($crap3:tt)*] [$field_name: ident($($data: tt)*), $($tokens:tt)*] [$($rest:tt)*]) => {
        parser!(@@ $struct_name, $delegated [$($crap)*, $field_name($($data)*)] [] [$($crap3)*] [$($tokens)*] [$($rest)*]);
    };

    (@@ $struct_name: ty, $delegated: ident [$($crap:tt)*] [$($crap2:tt)*] [$($crap3:tt)*] [] [$field_name: ident($($data: tt)*), $($rest:tt)*]) => {
        parser!(@@ $struct_name, $delegated [$($crap)*] [$($crap2)*, $field_name($($data)*)] [$($crap3)*] [] [$($rest)*]);
    };

    (@@ $struct_name: ty, $delegated: ident
        [$(, $field_name: ident($($tokens:tt)*))*]
        [$(, $helper_field: ident($($tokens2:tt)*))*]
        [$(, $custom_field: ident(custom = $func: path, depends_on = [$($field: expr),*]))*]
        [] []
    ) => {
        impl Parse for $struct_name {
            #[inline]
            fn parse<'a, I, F>(iter: I, mut f: F) -> Result<Self, ValueError<'a>>
            where
                I: Iterator<Item = (&'a str, &'a str)> + Clone,
                F: FnMut(&'a str, &'a str) -> Result<(), ValueError<'a>>
            {
                use $crate::util::parse;

                trace!("Parsing {}", stringify!($struct_name));

                $(
                    let mut $field_name = None;
                )*

                $(
                    let mut $helper_field = None;
                )*

                let closure = |idx: &'a str, value: &'a str| -> Result<(), ValueError<'a>> {
                    match idx {
                        $(
                            __index!($($tokens)*) => $field_name = __parsing!(@ value, $($tokens)*),
                        )*
                        $(
                            __index!($($tokens2)*) => {
                                $helper_field = __parsing!(@ value, $($tokens2)*);

                                f(idx, value)?
                            },
                        )*
                        _ => f(idx, value)?
                    }

                    Ok(())
                };

                let $delegated = Parse::parse(iter, closure)?;

                $(
                    let $field_name = __unwrap!($field_name($($tokens)*));
                )*

                $(
                    let $helper_field = __unwrap!($helper_field($($tokens2)*));
                )*

                Ok(Self {
                    $delegated,
                    $(
                        $field_name,
                    )*
                    $(
                        $custom_field: $func($($field,)*),
                    )*
                })
            }
        }
    };
}
