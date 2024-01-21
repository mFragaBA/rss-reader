use super::rss_item::RssItem;

pub struct RssDocument {
    // Required Fields
    pub title: String,
    pub link: String,
    pub description: String,

    // Optional Fields
    pub language: Option<String>,
    pub copyright: Option<String>,
    pub managing_editor_email: Option<String>,
    pub web_master_email: Option<String>,
    pub publication_date: Option<String>, // TODO: change this to some other date format maybe?
    pub last_build_date: Option<String>, // TODO: change this to some other date format maybe?
    pub category: Option<Category>,
    pub generator: Option<String>,
    pub docs_url: Option<String>,
    pub cloud: Option<Cloud>,
    pub tll: Option<TTL>,
    pub image: Option<Image>,
    // pub rating: Option<>, TODO: find what type this is
    pub text_input: Option<TextInput>,
    pub skip_hours: Option<Vec<Hour>>,
    pub skip_days: Option<Vec<Day>>,
    pub items: Vec<RssItem>
}

pub struct Category {
    pub domain: Option<String>,
    pub text: String
}

// Web service that supports the rssCloud interface
pub struct Cloud {
    pub domain: String,
    pub port: u32,
    pub path: String,
    pub register_procedure: String,
    pub protocol: String
}

// It's a number of minutes that indicate how long a channel cna be cached before refreshing from
// the source.
pub struct TTL {
    pub ttl_value: u32,
}

pub struct Image {
    // Required Fields
    pub url: String,
    pub title: String,
    pub link: String,
    // Optional Fields
    pub width: Option<u32>, // Maximum value is 144, default is 88
    pub height: Option<u32>, // Maximum value is 400, default is 31
    pub description: Option<u32>,
}

pub struct TextInput {
    pub title: String,
    pub description: String,
    pub name: String,
    pub link: String
}

// Value is a number between 0 and 23 representing a time in GMT
pub struct Hour {
    pub value: u8,
}

// Value specifies the day where aggregators shouldn't read this channel
pub struct Day {
    pub value: String,
}
