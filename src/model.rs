#[derive(Serialize, Deserlialize, Debug)]
pub struct Article {
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub published_at: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Deserlialize, Debug)]
pub struct ApiResponse {
    pub articles: Vec<Article>,
}
