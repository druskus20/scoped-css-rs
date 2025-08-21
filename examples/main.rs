use scoped_css::style;

fn main() {
    const RED: &str = "red";
    let (class, css) = style!(
        "
        & .example {
            color: [[RED]];
        }
        "
    );

    println!("Class: {}", class);

    println!("CSS: {}", css);
}
