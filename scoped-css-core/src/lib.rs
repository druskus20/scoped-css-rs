use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{ParserFlags, ParserOptions, StyleSheet},
    targets::Targets,
};

pub mod macros {
    pub use scoped_css_macro::style;
}

/// CSS post-processing with lightningcss
pub fn process_css_with_lightning(css: &str) -> Result<String, Box<dyn std::error::Error>> {
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

pub fn generate_class_name(css_content: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(css_content.as_bytes());
    let result = hasher.finalize();
    format!(
        "css-{:x}",
        &result[..4].iter().fold(0u32, |acc, &b| acc << 8 | b as u32)
    )
}
