use mongodb::bson::{self, doc};
use serde::{Deserialize, Serialize};

use imdb_id::omdb::search_by_title;

use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::Client;

use bson::DateTime;

use tokio;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<bson::oid::ObjectId>,
    imdb_link: String,
    name: String,
    date_of_addition: DateTime,
    likes: i32,
    dislikes: i32,
}

pub async fn setup() -> Result<Client> {
    // Load the MongoDB connection string from an environment variable
    let client_uri =
        env::var("MONGODB_URI").expect("Couldn't get token from environment variables");

    // An extra line of code to work around a DNS issue on Windows
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();
    // A Client is needed to connect to MongoDB
    let client = mongodb::Client::with_options(options).unwrap();

    // Print the databases in our MongoDB cluster:
    println!("Databases:");

    for name in client.list_database_names(None, None).await.unwrap() {
        println!("- {}", name);
    }

    Ok(client)
}

async fn insert_movie(db_client: &Client, movie: Movie) -> Result<()> {
    let movies = db_client
        .database("discord_bot")
        .collection("movies_to_watch");
    let ser_movie = bson::to_bson(&movie).unwrap();
    let movie_doc = ser_movie.as_document().unwrap();
    let _insert_res = movies.insert_one(movie_doc.to_owned(), None).await.unwrap();
    println!("yellow");
    Ok(())
}

pub fn search(db_client: &Client, name: &str) {
    println!("Searching for movie title");

    let search_results = search_by_title("f9134778", &name).unwrap();
    if search_results.total_results == 0 {
        println!("No results for movie: {}", &name);
    }
    for search_result in &search_results.entries {
        println!(
            "Name: {}\nIMDB ID: {}",
            search_result.title, search_result.imdb_id
        );
    }

    let movie_result = search_results.entries.get(0).unwrap();

    let movie = Movie {
        id: None,
        imdb_link: format!(
            "https://www.imdb.com/title/{}/",
            movie_result.imdb_id.to_owned()
        ),
        name: movie_result.title.to_owned(),
        date_of_addition: DateTime::from_system_time(std::time::SystemTime::now()),
        likes: 0,
        dislikes: 0,
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = insert_movie(db_client, movie);
    rt.block_on(future).unwrap();
}
