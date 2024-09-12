use ::blogger::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today.");
    println!("{}", post);

    post.request_review();
    println!("{}", post);

    post.reject();
    println!("{}", post);

    post.edit_text("I ate a salad for lunch today and it was great.");
    println!("{}", post);

    post.request_review();
    println!("{}", post);

    post.approve();
    println!("{}", post);

    println!("Next post:");
    let mut post = post.next();
    post.add_text("I ate a salad for dinner today and it was great again.");
    println!("{}", post);

    post.request_review();
    println!("{}", post);

    post.approve();
    println!("{}", post);

    if let Some(prev) = post.prev() {
        println!("Previous post:\n{}", prev);
    }
}
