use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Article {
    author: String,
    title: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        author: String::from("Duncan Macharia"),
        title: String::from("This is the title of this article"),
        paragraph: vec![
            Paragraph {
                name: String::from("This is the name of the first paragraph"),
            },
            Paragraph {
                name: String::from("This is the name of the second paragraph"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("This is the json object: {}", json);
}
