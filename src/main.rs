use clap::Parser;
use dotenv::dotenv;
use std::env;

#[derive(Parser, Debug)]
#[command(name = "Telegram Sender")]
#[command(author = "MFU Tech")]
#[command(version = "1.0")]
#[command(about = "Sends a message to a Telegram channel", long_about = None)]
struct Args {
    /// Channel ID (e.g., -123456789)
    #[arg(short, long)]
    channel_id: Option<String>,

    /// Message to send
    #[arg(short, long)]
    message: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from the .env file
    dotenv().ok();

    // Get the Telegram bot token from the environment
    let token =
        env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set in the .env file");

    let args = Args::parse();

    let api_url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    let chat_id = match args.channel_id {
        Some(chat_id) => chat_id,
        None => env::var("DEFAULT_TELEGRAM_CHANNELID")
            .expect("DEFAULT_TELEGRAM_CHANNELID not set AND no channel id given in arguments, to either of these"),
    };

    let params = [
        ("chat_id", chat_id.as_str()),
        ("text", args.message.as_str()),
    ];

    let client = reqwest::blocking::Client::new();
    let response = client.post(api_url).form(&params).send()?;

    if response.status().is_success() {
        println!("Message sent successfully!");
    } else {
        eprintln!(
            "Failed to send message: {}",
            response.text().unwrap_or_default()
        );
    }

    Ok(())
}
