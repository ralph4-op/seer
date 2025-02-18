use std::collections::HashMap;

#[derive(Debug)]
struct Post {
    id: String,
    user: String,
    content: String,
    status: String,
    created_at: i64,
}

fn filter_restricted_content(mut post: Post) -> Post {
    let restricted_keywords = vec!["gore", "chipper"];
    
    if restricted_keywords.iter().any(|&keyword| post.content.contains(keyword)) {
        post.status = "flagged".to_string();
    }
    
    post
}