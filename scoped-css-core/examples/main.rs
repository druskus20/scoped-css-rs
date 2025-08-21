use scoped_css_core::style;

fn main() {
    let red = "#ff0000";
    let (class, sytle) = style!(
        r#"
        & {
            background_color:  [[red]]
        }
        "#
    );

    println!("class: {class}");
    println!("style: {sytle}");
}
