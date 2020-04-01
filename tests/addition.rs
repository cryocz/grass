#![cfg(test)]

#[macro_use]
mod macros;

test!(
    undefined_function_call_is_ident_adds,
    "a {\n  color: 1 + foo();\n}\n",
    "a {\n  color: 1foo();\n}\n"
);
test!(
    unitless_plus_null,
    "a {\n  color: 1 + null;\n}\n",
    "a {\n  color: 1;\n}\n"
);
test!(
    unitless_plus_null_plus_unitless,
    "a {\n  color: 1 + null + 1;\n}\n",
    "a {\n  color: 11;\n}\n"
);
test!(
    unit_plus_null,
    "a {\n  color: 1px + null;\n}\n",
    "a {\n  color: 1px;\n}\n"
);
test!(
    chain_ident_addition,
    "a {\n  color: a + b + c + d + e + f;\n}\n",
    "a {\n  color: abcdef;\n}\n"
);
test!(
    unquoted_plus_unquoted,
    "a {\n  color: foo + bar;\n}\n",
    "a {\n  color: foobar;\n}\n"
);
test!(
    dblquoted_plus_dblquoted,
    "a {\n  color: \"foo\" + \"bar\";\n}\n",
    "a {\n  color: \"foobar\";\n}\n"
);
test!(
    sglquoted_plus_sglquoted,
    "a {\n  color: 'foo' + 'bar';\n}\n",
    "a {\n  color: \"foobar\";\n}\n"
);
test!(
    dblquoted_plus_unquoted,
    "a {\n  color: \"foo\" + bar;\n}\n",
    "a {\n  color: \"foobar\";\n}\n"
);
test!(
    sglquoted_plus_unquoted,
    "a {\n  color: 'foo' + bar;\n}\n",
    "a {\n  color: \"foobar\";\n}\n"
);
test!(
    #[ignore]
    unquoted_plus_dblquoted,
    "a {\n  color: foo + \"bar\";\n}\n",
    "a {\n  color: foobar;\n}\n"
);
test!(
    #[ignore]
    unquoted_plus_sglquoted,
    "a {\n  color: foo + 'bar';\n}\n",
    "a {\n  color: foobar;\n}\n"
);
test!(
    sglquoted_plus_dblquoted,
    "a {\n  color: 'foo' + \"bar\";\n}\n",
    "a {\n  color: \"foobar\";\n}\n"
);
test!(
    dblquoted_plus_sglquoted,
    "a {\n  color: \"foo\" + 'bar';\n}\n",
    "a {\n  color: \"foobar\";\n}\n"
);
test!(
    unquoted_plus_true,
    "a {\n  color: foo + true;\n}\n",
    "a {\n  color: footrue;\n}\n"
);
test!(
    dblquoted_plus_true,
    "a {\n  color: \"foo\" + true;\n}\n",
    "a {\n  color: \"footrue\";\n}\n"
);
test!(
    sglquoted_plus_true,
    "a {\n  color: 'foo' + true;\n}\n",
    "a {\n  color: \"footrue\";\n}\n"
);
test!(
    unquoted_plus_false,
    "a {\n  color: foo + false;\n}\n",
    "a {\n  color: foofalse;\n}\n"
);
test!(
    dblquoted_plus_false,
    "a {\n  color: \"foo\" + false;\n}\n",
    "a {\n  color: \"foofalse\";\n}\n"
);
test!(
    sglquoted_plus_false,
    "a {\n  color: 'foo' + false;\n}\n",
    "a {\n  color: \"foofalse\";\n}\n"
);
test!(
    unquoted_plus_important,
    "a {\n  color: foo + !important;\n}\n",
    "a {\n  color: foo!important;\n}\n"
);
test!(
    unquoted_plus_important_uppercase,
    "a {\n  color: foo + !IMPORTANT;\n}\n",
    "a {\n  color: foo!important;\n}\n"
);
test!(
    unquoted_plus_null,
    "a {\n  color: foo + null;\n}\n",
    "a {\n  color: foo;\n}\n"
);
test!(
    dblquoted_plus_null,
    "a {\n  color: \"foo\" + null;\n}\n",
    "a {\n  color: \"foo\";\n}\n"
);
test!(
    sglquoted_plus_null,
    "a {\n  color: 'foo' + null;\n}\n",
    "a {\n  color: \"foo\";\n}\n"
);
test!(
    unquoted_plus_number_unitless,
    "a {\n  color: foo + 1;\n}\n",
    "a {\n  color: foo1;\n}\n"
);
test!(
    dblquoted_plus_number_unitless,
    "a {\n  color: \"foo\" + 1;\n}\n",
    "a {\n  color: \"foo1\";\n}\n"
);
test!(
    sglquoted_plus_number_unitless,
    "a {\n  color: 'foo' + 1;\n}\n",
    "a {\n  color: \"foo1\";\n}\n"
);
test!(
    unquoted_plus_number_unit,
    "a {\n  color: foo + 1px;\n}\n",
    "a {\n  color: foo1px;\n}\n"
);
test!(
    dblquoted_plus_number_unit,
    "a {\n  color: \"foo\" + 1px;\n}\n",
    "a {\n  color: \"foo1px\";\n}\n"
);
test!(
    sglquoted_plus_number_unit,
    "a {\n  color: 'foo' + 1px;\n}\n",
    "a {\n  color: \"foo1px\";\n}\n"
);
test!(
    true_plus_false,
    "a {\n  color: true + false;\n}\n",
    "a {\n  color: truefalse;\n}\n"
);
test!(
    false_plus_null,
    "a {\n  color: false + null;\n}\n",
    "a {\n  color: false;\n}\n"
);
test!(
    false_plus_null_is_string,
    "a {\n  color: type-of(false + null);\n}\n",
    "a {\n  color: string;\n}\n"
);
test!(
    null_plus_num,
    "a {\n  color: null + 1;\n}\n",
    "a {\n  color: 1;\n}\n"
);
test!(
    null_plus_num_is_string,
    "a {\n  color: type-of(null + 1);\n}\n",
    "a {\n  color: string;\n}\n"
);
test!(
    num_plus_list,
    "a {\n  color: 1 + (2 3);\n}\n",
    "a {\n  color: 12 3;\n}\n"
);
test!(
    list_plus_num,
    "a {\n  color: (1 2) + 3;\n}\n",
    "a {\n  color: 1 23;\n}\n"
);
test!(
    dblquoted_plus_list,
    "a {\n  color: \"1\" + (2 3);\n}\n",
    "a {\n  color: \"12 3\";\n}\n"
);
test!(
    list_plus_dblquoted,
    "a {\n  color: (1 2) + \"3\";\n}\n",
    "a {\n  color: \"1 23\";\n}\n"
);
test!(
    sglquoted_plus_list,
    "a {\n  color: 'a' + (b c);\n}\n",
    "a {\n  color: \"ab c\";\n}\n"
);
test!(
    list_plus_sglquoted,
    "a {\n  color: (b c) + 'a';\n}\n",
    "a {\n  color: \"b ca\";\n}\n"
);
test!(
    list_plus_list,
    "a {\n  color: (a b) + (1 2);\n}\n",
    "a {\n  color: a b1 2;\n}\n"
);
test!(
    multiple_ident_sum,
    "a {\n  color: foo + 1 + bar + 2;\n}\n",
    "a {\n  color: foo1bar2;\n}\n"
);
test!(
    dblquoted_plus_named_color,
    "a {\n  color: \"foo\" + red;\n}\n",
    "a {\n  color: \"foored\";\n}\n"
);
test!(
    sglquoted_plus_named_color,
    "a {\n  color: 'foo' + red;\n}\n",
    "a {\n  color: \"foored\";\n}\n"
);
test!(
    unquoted_plus_named_color,
    "a {\n  color: foo + red;\n}\n",
    "a {\n  color: foored;\n}\n"
);
test!(
    dblquoted_plus_hex_color,
    "a {\n  color: \"foo\" + #fff;\n}\n",
    "a {\n  color: \"foo#fff\";\n}\n"
);
test!(
    sglquoted_plus_hex_color,
    "a {\n  color: 'foo' + #fff;\n}\n",
    "a {\n  color: \"foo#fff\";\n}\n"
);
test!(
    unquoted_plus_hex_color,
    "a {\n  color: foo + #fff;\n}\n",
    "a {\n  color: foo#fff;\n}\n"
);
