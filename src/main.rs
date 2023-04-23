use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{ Read, Write };
use chrono::Utc;

#[macro_use]
extern crate rocket;

static DATABASE_FILE: &str = "database.txt";

async fn send_message(
    token: String,
    channel_id: String,
    msg: String
) -> Result<(), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder().build()?;

    let mut body: HashMap<_, _> = HashMap::new();
    body.insert("chat_id", channel_id);
    body.insert("text", msg);

    // Perform the actual execution of the network request
    client
        .post(format!("https://api.telegram.org/{}/sendMessage", token))
        .json(&body)
        .send().await?;

    Ok(())
}

#[get("/post")]
async fn get_post_log() -> String {
    let bot_token = env::var("BOT_TOKEN").expect("Env variable `BOT_TOKEN` not set");
    let channel_id = env::var("CHANNEL_ID").expect("Env variable `CHANNEL_ID` not set");
    let mut data_file = File::open(DATABASE_FILE).unwrap();

    let mut file_content = String::new();
    data_file.read_to_string(&mut file_content).unwrap();

    send_message(bot_token, channel_id, format!("{}", file_content)).await.expect(
        "Sending message failed"
    );

    format!("{}", file_content)
}

#[post("/post")]
async fn notify_post() -> &'static str {
    let bot_token = env::var("BOT_TOKEN").expect("Env variable `BOT_TOKEN` not set");
    let channel_id = env::var("CHANNEL_ID").expect("Env variable `CHANNEL_ID` not set");

    send_message(bot_token, channel_id, "You've got mail".to_string()).await.expect(
        "Sending message failed"
    );

    // Log to file
    let mut file = OpenOptions::new()
        .append(true)
        .open(DATABASE_FILE)
        .expect("Opening file failed.");
    let date = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    write!(file, "{}: Received mail\n", date).expect("Writing failed.");

    "Post notification received\n"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_post_log, notify_post])
}