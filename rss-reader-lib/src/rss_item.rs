use super::rss::Category;

pub struct RssItem {
    // All items are optional, however at least either title or description must be present
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<Description>,
    pub author: Option<Author>,
    pub category: Option<Category>,
    pub comments: Option<Comments>,
    pub enclosure: Option<Enclosure>,
    pub guid: Option<Guid>,
    pub publication_date: Option<Date>,
    pub source: Option<Source>,
}

pub struct Description {
    pub text: String,
}

pub struct Author {
    pub email: String,
}

pub struct Comments {
    pub url: String,
}

pub struct Enclosure {
    pub url: String,
    pub length: usize,
    pub mime_type: String,
}

pub struct Guid {
    pub identifier: String,
}

pub struct Date {
    pub date_str: String,
}

pub struct Source {
    pub url: String, // links to the XMLization of the source
}
