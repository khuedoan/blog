include!(concat!(env!("OUT_DIR"), "/posts_data.rs"));

#[derive(Debug)]
pub struct PostMetadata {
    pub id: String,
    pub title: String,
    pub date: String,
}

pub fn list_posts() -> Vec<PostMetadata> {
    POSTS
        .iter()
        .map(|&(id, title, date, _content)| PostMetadata {
            id: id.to_string(),
            title: title.to_string(),
            date: date.to_string(),
        })
        .collect()
}

pub fn get_post(path: String) -> Option<(PostMetadata, String)> {
    POSTS
        .iter()
        .filter_map(|(id, title, date, content)| {
            if id == &path {
                Some((
                    PostMetadata {
                        id: id.to_string(),
                        title: title.to_string(),
                        date: date.to_string(),
                    },
                    content.to_string(),
                ))
            } else {
                None
            }
        })
        .next()
}
