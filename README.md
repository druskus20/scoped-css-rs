# scoped-css-rs

> [WARN]
> Do not use this.

However. 

It works (more or less):


```rs
use scoped_css_core::style;

fn my_random_color() -> String {
    const POSSIBLE: [&str; 6] = [
        "#a123ff", "#ff1234", "#12ff34", "#1234ff", "#34ff12", "#ff3412",
    ];
    POSSIBLE[std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        % POSSIBLE.len()]
    .to_string()
}

fn main() {
    let red = "#ff0000";
    let (class, sytle) = style!(
        r#"
        & {
            background_color:  [[red]];
            color: [[my_random_color()]];
            height: 100px;
        }
        "#
    );

    println!("class: {class}");
    println!("style: {sytle}");
}
```
