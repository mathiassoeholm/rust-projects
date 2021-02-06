use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Rust is a pretty cool language");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    post.approve();
    assert_eq!("Rust is a pretty cool language", post.content());
}
