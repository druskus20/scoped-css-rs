use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn style(input: TokenStream) -> TokenStream {
    use syn::{Expr, LitStr, parse_macro_input};

    let css_lit = parse_macro_input!(input as LitStr);
    let css_str = css_lit.value();

    let mut fmt_str = String::new();
    let mut args = Vec::new();
    let mut rest = css_str.as_str();

    while let Some(start) = rest.find("[[") {
        let end = rest[start + 2..]
            .find("]]")
            .expect("unclosed [[ ]] placeholder")
            + start
            + 2;

        // Escape braces in the chunk before [[ ... ]]
        let chunk = rest[..start].replace("{", "{{").replace("}", "}}");
        fmt_str.push_str(&chunk);
        fmt_str.push_str("{}"); // placeholder

        // Parse the placeholder as a full Rust expression
        let expr_str = &rest[start + 2..end];
        let expr: Expr =
            syn::parse_str(expr_str.trim()).expect("failed to parse expression inside [[ ]]");
        args.push(expr);

        rest = &rest[end + 2..];
    }

    // Escape braces in the final remainder
    fmt_str.push_str(&rest.replace("{", "{{").replace("}", "}}"));

    let expanded = quote! {
        {
            let raw_css = format!(#fmt_str, #( #args ),*);
            let class = scoped_css_core::generate_class_name(&raw_css);
            let scoped_css = raw_css.replace("&", &format!(".{}", class));
            let css = scoped_css_core::process_css_with_lightning(&scoped_css)
                .expect("CSS parsing failed");
            (class, css)
        }
    };
    expanded.into()
}
