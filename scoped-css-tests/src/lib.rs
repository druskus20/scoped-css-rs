#[cfg(test)]
mod tests {

    use scoped_css_core::macros::style;

    #[test]
    fn test_basic_css_generation() {
        let (_class_name, css) = style!("& { color: #1f0000; }");
        assert!(css.contains("color: #1f0000;"));
    }

    #[test]
    fn test_hex_color_parsing() {
        let bg = "#4ecdc4";
        let fg = "#123e45";
        let (class_name, css) = style!("& { background: [[bg]]; color: [[fg]]; }");
        assert!(css.contains("4ecdc4"));
        assert!(css.contains("123e45"));
        assert!(css.contains(&class_name));
    }

    #[test]
    fn test_complex_css() {
        let (class_name, css) = style!(
            r#"
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
        "#
        );

        assert!(css.contains("linear-gradient"));
        assert!(css.contains("box-shadow"));
        assert!(css.contains(&class_name));
    }

    #[test]
    fn test_media_queries() {
        let bp = 768;
        let (class_name, css) = style!(
            r#"
            & { font-size: 16px; }
            @media (max-width: [[bp]]px) {
                & { font-size: 14px; }
            }
        "#
        );

        assert!(css.contains("@media"));
        assert!(css.contains("768px"));
        assert!(css.contains(&class_name));
    }

    #[test]
    fn test_css_variables() {
        let primary = "#007bff";
        let hover = "#0056b3";
        let (_class_name, css) = style!(
            r#"
            & {
                --primary-color: [[primary]];
                --hover-color: [[hover]];
                background-color: var(--primary-color);
            }
            &:hover {
                background-color: var(--hover-color);
            }
        "#
        );
        dbg!(&css);

        assert!(css.contains("--primary-color"));
        assert!(css.contains("var(--primary-color)"));
        assert!(css.contains("007bff"));
    }
}
