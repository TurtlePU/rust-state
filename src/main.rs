mod article;

use article::Article;

fn main() {
    let mut article = Article::empty();

    article.add_text("Rust не принято считать ООП-языком");
    assert_eq!(None, article.content());

    article.send_to_moderators();
    assert_eq!(None, article.content());

    article.publish();
    assert_eq!(Some("Rust не принято считать ООП-языком"), article.content());
}
