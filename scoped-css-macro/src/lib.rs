use proc_macro::{Literal, TokenStream, TokenTree};
use quote::quote;

#[proc_macro]
pub fn style(input: TokenStream) -> TokenStream {
    // expect exactly one literal token
    let css_lit = match input.into_iter().next() {
        Some(TokenTree::Literal(lit)) => lit,
        _ => panic!("expected a string literal as input"),
    };

    // convert literal to string, handling raw strings automatically
    let css_str = parse_literal_to_string(&css_lit);

    let mut fmt_str = String::new();
    let mut args = Vec::new();
    let mut rest = css_str.as_str();

    while let Some(start) = rest.find("[[") {
        let end = rest[start + 2..]
            .find("]]")
            .expect("unclosed [[ ]] placeholder")
            + start
            + 2;

        // escape braces and append chunk
        let chunk = &rest[..start];
        fmt_str.push_str(&chunk.replace("{", "{{").replace("}", "}}"));
        fmt_str.push_str("{}");

        // Everything inside [[...]] is treated as a raw Rust expression
        let expr_str = &rest[start + 2..end];
        let expr_tokens: proc_macro2::TokenStream = expr_str
            .parse()
            .expect("failed to parse expression inside [[ ]]");
        args.push(expr_tokens);

        rest = &rest[end + 2..];
    }

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

/// convert proc_macro::Literal to actual string content
fn parse_literal_to_string(lit: &Literal) -> String {
    let s = lit.to_string();
    if let Some(stripped) = s.strip_prefix("r#\"").and_then(|s| s.strip_suffix("\"#")) {
        stripped.to_string()
    } else if let Some(stripped) = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        stripped.to_string()
    } else {
        panic!("expected a string literal")
    }
}
