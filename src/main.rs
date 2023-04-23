use std::env;
use std::collections::HashMap;
use std::error::Error;

#[macro_use] extern crate rocket;

async fn send_message(token: String, channel_id: String) -> Result<(), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder()
        .build()?;

    let mut body: HashMap<_, _> = HashMap::new();
    body.insert("chat_id", channel_id);
    body.insert("text", "You've got mail".to_string());


    // Perform the actual execution of the network request
    let res: reqwest::Response = client
        .post(format!("https://api.telegram.org/{}/sendMessage", token))
        .json(&body)
        .send()
        .await?;

    // Parse the response body as Json in this case
    let response: HashMap<String, String> = res
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:?}", response);
    Ok(())
}


#[get("/post")]
fn get_post_log() -> &'static str {
    "Hello, world!"
}

#[post("/post")]
async fn notify_post() -> () {
    let bot_token = env::var("BOT_TOKEN").expect("Env variable `BOT_TOKEN` not set");
    let channel_id = env::var("CHANNEL_ID").expect("Env variable `CHANNEL_ID` not set");

    let _r: Result<(), Box<dyn Error>> = send_message(bot_token, channel_id).await;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_post_log, notify_post])
}
