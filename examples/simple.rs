use scoped_css::style;

fn main() {
    let (button_class, button_css) = style!(
        r#"
        & {
            background-color: #007bff;
            color: white;
            border: none;
            padding: 12px 24px;
            border-radius: 6px;
            font-size: 16px;
            cursor: pointer;
            transition: all 0.2s ease;
        }
        &:hover {
            background-color: #0056b3;
            transform: translateY(-2px);
            box-shadow: 0 4px 8px rgba(0,0,0,0.2);
        }
        & .title { 
            font-weight: bold;
            font-size: 18px;
            margin-bottom: 8px;
        }
        @media (max-width: 768px) {
            & {
                font-size: 14px;
            }
        }
    "#
    );

    let html = maud::html! {
        style { (button_css) }
        div class = (button_class) {
            h2 class = "title" { "Click Me!" }
            h2 class = "title" { "Not a title" }
            p { "This is a simple button." }
        }
    };

    println!("Generated HTML:");
    println!("--------------------------");
    println!("{}", html.into_string());
    println!("--------------------------");
}
