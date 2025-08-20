use redis::Client;

pub struct Cache {
    client: Client,
}

impl Cache {
    pub fn new() -> Cache {
        let client = Client::open("redis://127.0.0.1/").unwrap();

        Cache { client }
    }

    pub fn store_news(&mut self, news: Vec<Article>) -> RedisResult<()> {
        let key = String::from("LATEST_NEWS");

        let mut con = self.client.get_connection().unwrap();

        let data = serde_json::to_string(&news).unwrap();

        con.set_ex(key, data, 600)
    }

    pub fn get_news(&mut self) -> Option<String> {
        let key = String::from("LATEST_NEWS");

        let mut con = self.client.get_connection().unwrap();

        let value = con.get(key).unwrap();

        value
    }
}
