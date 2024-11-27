mod model;
mod lexer;
use serde_json::json;

fn main() {
    let example = json!({
        "type": "doc",
        "version": 1,
        "content": [
            {
                "type": "paragraph",
                "content": [
                  {
                    "type": "text",
                    "text": "Hello world"
                  }
                ]
            }
        ]
    });

    // Check root node
    let root = lexer::from_value(example);
    println!("{:#?}", root);

    // TODO: Make a parser to verify the ADF doc
    // TODO: Make a renderer to convert ADF doc to HTML
    // TODO: Make a renderer to convert ADF doc to Markdown
}