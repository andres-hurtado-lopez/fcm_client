use fcm_client::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let url = "https://fcm.googleapis.com/fcm/send";
    let key = "here the key for the server should go";
    
    let _response = FCMClient::build(url, key)
	.send_notification(
	    FirebaseMessage{
		notification: Notification {
		    title: "This is a test title message".to_string(),
		    body: "This is a body title message".to_string(),
		    click_action: "FLUTTER_NOTIFICATION_CLICK".to_string(),
		    icon: "http://url-to-an-icon/icon.png".to_string(),
		},
		to: "here the token of the mobile device should go".to_string(),
	    }
	).await.map_err(|why| format!("Fail sending message : {}",why))?;

    #[cfg(feature="debug")]
    { println!("Firebase response: {:?}", _response); }
    
    Ok(())
}
