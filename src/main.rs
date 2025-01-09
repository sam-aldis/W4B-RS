mod secrets;
mod api;

#[tokio::main]
async fn main() {
    let api_key : String = secrets::getAPIKey();
    let query : String = String::from("Does this work, say yes if you got this query");
    let res = api::ai_query(&query, &api_key).await;
    println!("{}",res.unwrap());
}