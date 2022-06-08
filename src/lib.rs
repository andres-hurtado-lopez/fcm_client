use serde_derive::{Deserialize, Serialize};


#[derive(Serialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Notification{
    	pub title: String,
	pub body: String,
	pub click_action: String,
	pub icon: String,
}

#[derive(Serialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct FirebaseMessage {
    pub notification: Notification,
    pub to: String,
}


#[derive(Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ResponseMessageIds{
    #[allow(dead_code)]
    message_id: String,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[allow(dead_code)]
pub struct FirebaseResponse {
    multicast_id: i64,
    success: i64,
    failure: i64,
    canonical_ids: i64,
    results: Vec<ResponseMessageIds>,
}




pub struct FCMClient{
    url: String,
    key: String,
}

impl FCMClient{
    pub fn build(url:&str, key: &str) -> Self{
	Self{
	    url:url.to_string(),
	    key: key.to_string(),
	}
    }

    pub fn send_notification(&self, message: FirebaseMessage) -> Result<FirebaseResponse, Box<dyn std::error::Error>>{

	let mut headers = reqwest::header::HeaderMap::new();

	headers.insert("Authorization", format!("key={}",self.key).try_into()? );

	let json_message = serde_json::json!(&message);

	println!("mensaje de salida: {}", json_message);

	//return Ok(json_message);
	
	let request = reqwest::blocking::Client::new()
            .post(&self.url)
	    //.bearer_auth(&self.key)
	    .headers(headers)
            .json(&message)
            .build()?;

	#[cfg(feature="debug")]
	{ println!("request: {:?}",request); }

	let response = reqwest::blocking::Client::new().execute(request)
	    .map_err(|why| format!("error enviando el mensaje http a firebase: {}",why))?;

	let status = response.status();

	#[cfg(feature="debug")]
	{
	    let status = response.status();
	    let headers = response.headers().clone();
	    let content_length = response.content_length();

	    println!("status: {:?}\n\nheaders: {:?}.\n\ncontent_lenght: {:?}\n\n ", status, headers, content_length);
	}
		
	if status.is_client_error() || status.is_server_error() {
	    let text = response.text()?;
	    return Err(format!("Error en la respuesta de firebase: {:?} {} {}",&status, status.canonical_reason().unwrap_or("UNKNOWN ERROR"), text))?;
	}

	let decoded_response : FirebaseResponse;

	if cfg!(feature="debug")
	{

	    let debug_buffer : String = response.text().unwrap();
	    decoded_response = serde_json::from_str(&debug_buffer).unwrap();
	    
	}else{

	    decoded_response = response.json()
		.map_err(|why| format!("error decodificando de json el mensaje http desde firebase: {}",why))?;
	    
	}


	

	#[cfg(feature="debug")]
	{
	    println!("parsed response: {:?}", decoded_response);
	}

	Ok(decoded_response)
	    
    }

}

