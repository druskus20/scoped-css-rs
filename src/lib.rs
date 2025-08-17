use lightningcss::stylesheet::{ParserFlags, ParserOptions, PrinterOptions, StyleSheet};
use lightningcss::targets::Targets;
use proc_macro::TokenStream;
use quote::quote;
use sha2::{Digest, Sha256};
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn style(input: TokenStream) -> TokenStream {
    let input_lit = parse_macro_input!(input as LitStr);
    let css_content = input_lit.value();

    let class_name = generate_class_name(&css_content);
    let processed_css = css_content.replace("&", &format!(".{class_name}"));
    let final_css = match process_css_with_lightning(&processed_css) {
        Ok(css) => css,
        Err(e) => {
            return syn::Error::new(input_lit.span(), format!("CSS parsing error: {e}"))
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        (
            #class_name,
            #final_css
        )
    };

    expanded.into()
}

fn generate_class_name(css: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(css.as_bytes());
    let result = hasher.finalize();
    format!(
        "css-{:x}",
        &result[..4].iter().fold(0u32, |acc, &b| acc << 8 | b as u32)
    )
}

fn process_css_with_lightning(css: &str) -> Result<String, Box<dyn std::error::Error>> {
    let css_static = Box::leak(css.to_string().into_boxed_str());

    let stylesheet = StyleSheet::parse(
        css_static,
        ParserOptions {
            filename: "inline.css".to_string(),
            css_modules: None,
            source_index: 0,
            error_recovery: true,
            warnings: None,
            flags: ParserFlags::default(),
        },
    )?;

    let printer_options = PrinterOptions {
        minify: true,
        source_map: None,
        project_root: None,
        targets: Targets::default(),
        analyze_dependencies: None,
        pseudo_classes: None,
    };

    let result = stylesheet.to_css(printer_options)?;

    Ok(result.code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_css_generation() {
        let css = "& { color: red; background: blue; }";
        let class_name = generate_class_name(css);

        assert!(class_name.starts_with("css-"));
        assert!(class_name.len() > 4);
    }

    #[test]
    fn test_hex_color_parsing() {
        let css = "& { background: #4ecdc4; color: #123e45; }";
        let processed = css.replace("&", ".test-class");

        let result = process_css_with_lightning(&processed);
        assert!(result.is_ok());

        let final_css = result.unwrap();
        assert!(final_css.contains("4ecdc4") || final_css.contains("4ECDC4"));
    }

    #[test]
    fn test_complex_css() {
        let css = r#"
            & {
                background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
                transform: translateY(-2px) scale(1.05);
                transition: all 0.3s ease-in-out;
            }
            &:hover {
                transform: translateY(-4px) scale(1.1);
            }
            & .title {
                font-weight: bold;
                color: #333;
            }
        "#;

        let class_name = generate_class_name(css);
        let processed = css.replace("&", &format!(".{class_name}"));

        let result = process_css_with_lightning(&processed);
        assert!(result.is_ok());

        let final_css = result.unwrap();
        assert!(final_css.contains("linear-gradient"));
        assert!(final_css.contains("box-shadow"));
        assert!(final_css.contains(&format!(".{class_name}")));
    }

    #[test]
    fn test_class_name_consistency() {
        let css = "& { color: red; }";
        let class1 = generate_class_name(css);
        let class2 = generate_class_name(css);

        assert_eq!(class1, class2, "Same CSS should generate same class name");
    }

    #[test]
    fn test_class_name_uniqueness() {
        let css1 = "& { color: red; }";
        let css2 = "& { color: blue; }";
        let class1 = generate_class_name(css1);
        let class2 = generate_class_name(css2);

        assert_ne!(
            class1, class2,
            "Different CSS should generate different class names"
        );
    }

    #[test]
    fn test_media_queries() {
        let css = r#"
            & {
                font-size: 16px;
            }
            @media (max-width: 768px) {
                & {
                    font-size: 14px;
                }
            }
        "#;

        let class_name = generate_class_name(css);
        let processed = css.replace("&", &format!(".{class_name}"));

        let result = process_css_with_lightning(&processed);
        assert!(result.is_ok());

        let final_css = result.unwrap();
        assert!(final_css.contains("@media"));
    }

    #[test]
    fn test_css_variables() {
        let css = r#"
            & {
                --primary-color: #007bff;
                --hover-color: #0056b3;
                background-color: var(--primary-color);
            }
            &:hover {
                background-color: var(--hover-color);
            }
        "#;

        let class_name = generate_class_name(css);
        let processed = css.replace("&", &format!(".{}", class_name));

        let result = process_css_with_lightning(&processed);
        assert!(result.is_ok());

        let final_css = result.unwrap();
        assert!(final_css.contains("--primary-color"));
        assert!(final_css.contains("var(--primary-color)"));
    }

    #[test]
    fn test_empty_css() {
        let css = "";
        let class_name = generate_class_name(css);

        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_css_minification() {
        let css = r#"
            & {
                background-color: #007bff;
                color: white;
                border: none;
                padding: 12px 24px;
            }
        "#;

        let class_name = generate_class_name(css);
        let processed = css.replace("&", &format!(".{class_name}"));

        let result = process_css_with_lightning(&processed);
        assert!(result.is_ok());

        let final_css = result.unwrap();

        assert!(!final_css.contains("  "), "CSS should be minified");
        assert!(!final_css.contains("\n\n"), "CSS should be minified");
    }
}
