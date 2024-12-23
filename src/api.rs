use gemini_rs::Conversation;

pub async fn ai_query(request : &String, api_key : &String) -> Result<String,()> {
	//let client = Client::new(api_key.to_string());
	let mut convo = Conversation::new(api_key.clone(), "gemini-1.5-flash".to_string());
    let query = &request[..];
    let res = convo.prompt(&query).await;
        
	return Ok(String::from(format!("{:#?}", res)))
}