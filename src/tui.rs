mod secrets;
mod api;
use std::io;
use gemini_rs::Conversation;
use colored::Colorize;
#[tokio::main]
async fn main() {
    let api_key : String = secrets::getAPIKey();
    let mut history = Vec::new();
    loop {
        let mut query : String = String::new();
        println!("{}", ">>>> ".red());
        let _ = io::stdin().read_line(&mut query);
        println!("==== Sending ====\n{query}");
        //let res = api::ai_query(&query, &api_key).await;
        history.push(format!("<User> : {}",query.clone()));
        let mut convo = Conversation::new(api_key.clone(), "gemini-1.5-flash".to_string());
        let qs = format!("[ CONVERSATION HISTORY ]");
        let mut que : String = String::new();
        let qe = format!("[ END CONVERSATION HISTORY ]");
        for x in &history {
            que = format!("{que}\n- {x}");   
        }
        query = format!("{qs}\n- {que} {qe}\n {query}");
        let q = &query[..];
        println!("{q}");
        let res = convo.prompt(&q).await;
        history.push(format!("<Gemini> : {}", res.clone()));
        println!("{res}");
    }
}