#[derive(Debug)]
pub struct Database {
    pub url: String,
}

#[derive(Debug)]
pub struct Config {
    pub database: Database,
}

pub fn load_config() -> Config {
    println!("Loading config...");
    dotenv::dotenv().expect("Failed to read .env file");

    let database = Database {
        url: std::env::var("DATABASE_URL").unwrap_or(String::from("test.db")),
    };

    Config {
        database
    }
}