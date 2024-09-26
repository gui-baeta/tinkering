mod bot;
mod db;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = db::setup();
    let db_client = rt.block_on(future).unwrap();
    println!("going to search ");
    db::search(&db_client, "The Dark Knight");

    let future = bot::setup();
    rt.block_on(future);
}
