use gemini_rs::Conversation;
use reqwest;
pub struct Connection{
	pub url : String,
	pub response : String
}


pub enum API {
	Query(String,String),
	Connect(String),
}
pub enum Message {
	Response(String),
	Connection(Connection),
}
impl Message {
	pub fn response(&self) -> Result<&String,String> {
		match self {
			Message::Response(msg) => Ok(msg),
			Message::Connection(_con) => Err("Canot call unwrap on this".to_string())
		}
	}
	pub fn connection(&self) -> Result<&Connection,String> {
		match self {
			Message::Connection(con) => Ok(con),
			Message::Response(_msg) => Err("Cannot call unwrap on this".to_string())
		} 
	}
}

impl Connection {
    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn response(&self) -> &String {
        &self.response
    }
}
impl API {
	pub async fn call(&self) -> Message {
		match self {
			API::Query(request, api_key) => {
				let res = ai_query(&request, &api_key).await;
				Message::Response(res)
			},
			API::Connect(url) =>{
				let con = connect(&url).await;
				Message::Connection(con)
			},
		}
	}
}
pub async fn open_connection(url: &String) -> Result<String, String> {
let response: Result<reqwest::Response, reqwest::Error> = match reqwest::get(url.to_string()).await {
    		Ok(resp) => Ok(resp),
    		Err(_) => return Err(String::from("Failed to connect")),
		};
		let text = match response.unwrap().text().await {
			Result::Ok(txt) => Ok(txt),
			Err(_) => Err("Failed to collect".to_string())
		};
		Ok(text.unwrap())
}
pub async fn connect(url : &String) -> Connection {
	let con = Connection  {
		url : url.to_string(),
		response: open_connection(url).await.unwrap()
	};
	return con
}
pub async fn ai_query(request : &String, api_key : &String) -> String {
	//let client = Client::new(api_key.to_string());
	let mut convo = Conversation::new(api_key.clone(), "gemini-1.5-flash".to_string());
    let query = &request[..];
    let res = convo.prompt(&query).await;
	return String::from(format!("{:#?}", res))
}
// Example Usage:
// async fn main() {
// 	let connection = API::Connect("/test".to_string());
// 	let con = connection.call().await;
// 	let _url = &con.connection().unwrap().url.clone();
// 	let ai = API::Query("request".to_string(),"Api Key".to_string());
// 	let res = ai.call().await;
// 	let _msg = &res.response().unwrap();
// }