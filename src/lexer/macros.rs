macro_rules! iter_matcher {
    ($input:expr, $c:expr, {
        $($cust_ident:ident if $cust_if:expr => $cust_expr:expr,)*
        $(
            $lit1_literal:literal
            $(, $lit2_literal:literal)?
            => $lit_expr:expr,
        )*
        _ => $unhandled:expr $(,)?
    }) => {
        match $c {
            $($cust_ident if $cust_if => $cust_expr)*,
            $(
                $lit1_literal
                $(
                    if $input.next_if(|b|
                        matches!(b, Ok(b) if *b == $lit2_literal as u8))
                        .is_some()
                )?
                => $lit_expr,
            )*
            _ => $unhandled,
        }
    };
}

pub(crate) use iter_matcher;
