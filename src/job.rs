use crate::cache::Cache;
use crate::model::{ApiResponse, Article};
use dotenv::dotenv;
use serde_json::Value;
use std::env;
use std::time::Duration;
use tokio_cron_schedular::{Job, JobScheduler};

async fn fetch_news() -> Vec<Article> {
    dotenv().ok();
    let key = env::var("API_KEY").unwrap();

    let url = format!(
        "https://newsapi.org/v2/everything?q=tesla&from=2025-07-18&sortBy=publishedAt&apiKey={}",
        key
    );

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("User-Agent", "News-Feed-App")
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    println!("{:?}", response);

    let api_response = serde_json::from_value(response).unwrap();
    let news = api_response
        .articles
        .into_iter()
        .take(20)
        .map(|articles| Article {
            title: articles.title,
            description: articles.description,
            author: articles.author,
            published_at: articles.published_at,
            url: articles.url,
            image_url: articles.image_url,
        })
        .collect();

    news
}

pub async fn run() {
    let scheduler = JobScheduler::new().await.unwrap();

    let job = Job::new_repeated_async(Duration::from_secs(30), |_uuid, _l| {
        Box::pin(async {
            let news = fetch_news().await;
            println!("Get News {:?}", news.iter().count());
            let mut cache = Cache::new();
            cache.store_news(news).unwrap()
        })
    })
    .unwrap();

    scheduler.add(job).await.unwrap();
    scheduler.start().await.unwrap();
}
