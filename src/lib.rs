use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use lightningcss::targets::Targets;

#[macro_export]
macro_rules! style {
    ($css:literal $(, $($name:ident = $value:expr),* )?) => {{
        // Replace [[...]] with {…} for format!
        let fmt_str = $css.replace("[[", "{").replace("]]", "}");

        // Handle optional arguments
        #[allow(unused_mut)]
        let formatted_css = {
            #[allow(unused)]
            let s = {
                $( format!(fmt_str, $($name = $value),*) )?
                #[cfg(not(any($($($name),*),*)))]
                fmt_str.clone()
            };
            s
        };

        use sha2::{Digest, Sha256};
        // Generate deterministic class name from hash
        let mut hasher = sha2::Sha256::new();
        hasher.update(formatted_css.as_bytes());
        let hash = hasher.finalize();
        let class_name = format!("css-{:x}", &hash[..4].iter().fold(0u32, |acc, &b| acc << 8 | b as u32));

        // Replace & with .<class_name>
        let final_css_with_class = formatted_css.replace("&", &format!(".{}", class_name));

        // Process with lightningcss
        let processed_css = $crate::process_css_with_lightning(&final_css_with_class).unwrap();

        (class_name, processed_css)
    }};
}

pub fn process_css_with_lightning(css: &str) -> Result<String, Box<dyn std::error::Error>> {
    let css_static = Box::leak(css.to_string().into_boxed_str());

    let stylesheet = StyleSheet::parse(
        css_static,
        lightningcss::stylesheet::ParserOptions {
            filename: "inline.css".to_string(),
            css_modules: None,
            source_index: 0,
            error_recovery: true,
            warnings: None,
            flags: Default::default(),
        },
    )?;

    let printer_options = PrinterOptions {
        minify: false,
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

    // Constants for tests
    pub const RED: &str = "#ff0000";

    #[test]
    fn test_simple_replacement() {
        let (class_name, css) = style!(
            "
        & {
            background-color: [[super::RED]];
        }"
        );

        assert!(class_name.starts_with("css-"));
        assert!(css.contains(&format!(".{}", class_name)));
        assert!(css.contains(RED));
    }

    #[test]
    fn test_multiple_properties() {
        let (class_name, css) = style!(
            "
        & {
            background-color: [[super::RED]];
            color: [[super::RED]];
        }"
        );

        assert!(css.contains("background-color"));
        assert!(css.contains("color"));
        assert!(css.contains(&format!(".{}", class_name)));
    }

    #[test]
    fn test_class_determinism() {
        let (class1, _) = style!("& { color: [[super::RED]]; }");
        let (class2, _) = style!("& { color: [[super::RED]]; }");
        assert_eq!(
            class1, class2,
            "Class name should be deterministic from CSS"
        );
    }

    #[test]
    fn test_lightningcss_minification() {
        let (_, css) = style!(
            "
        & {
            background-color: [[super::RED]];
            margin: 10px 20px;
        }"
        );

        // Should be minified
        assert!(!css.contains("\n"));
        assert!(css.contains("background-color"));
    }
}
