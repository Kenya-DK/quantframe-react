use chrono::{format, Duration};
use directories::{BaseDirs, UserDirs};
use eyre::eyre;
use once_cell::sync::Lazy;
use polars::{
    lazy::dsl::col,
    prelude::{DataFrame, Expr, IntoLazy, SortOptions},
    series::Series,
};
use serde_json::{json, Value};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::{api::file, Window};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::{
    error::AppError,
    logger::{self},
    structs::WarframeLanguage,
    PACKAGEINFO,
};
/// Asynchronously sends a message to a Discord channel via a webhook.
///
/// The function takes a webhook URL, a title, content, and an optional list of user IDs to mention.
/// It constructs a JSON payload with the provided information and sends a POST request to the webhook URL.
/// If the webhook URL is empty, the function logs a warning and returns early.
/// If the request is successful, it logs an info message. If the request fails, it prints the error.
///
/// # Arguments
///
/// * `webhook` - The URL of the Discord webhook to send the message to.
/// * `title` - The title of the message.
/// * `content` - The content of the message.
/// * `user_ids` - An optional list of user IDs to mention in the message.
pub fn send_message_to_discord(
    webhook: String,
    title: String,
    content: String,
    user_ids: Option<Vec<String>>,
) {
    // Check if the webhook is empty
    if webhook.is_empty() {
        logger::warning_con("Helper", "Discord webhook is empty");
        return;
    }
    tauri::async_runtime::spawn(async move {
        let client = reqwest::Client::new();

        let mut body = json!({
            "username": "Quantframe",
            "avatar_url": "https://i.imgur.com/bgR6vAd.png",
            "embeds": [
                {
                    "title": title,
                    "description": content,
                    "color": 5814783,
                    "footer": {
                        "text": format!("Quantframe v{}", PACKAGEINFO.lock().unwrap().clone().unwrap().version.to_string()),
                        "timestamp": chrono::Local::now()
                        .naive_utc()
                        .to_string()
                    }
                }
            ]
        });

        let mut pings: Vec<String> = Vec::new();
        if let Some(user_ids) = user_ids {
            for user_id in user_ids {
                pings.push(format!("<@{}>", user_id));
            }
        }
        if pings.len() > 0 {
            body["content"] = json!(format!("{}", pings.join(" ")).replace("\"", ""));
        } else {
            body["content"] = json!("");
        }

        let res = client.post(webhook).json(&body).send().await;
        match res {
            Ok(_) => {
                logger::info_con("Helper", "Message sent to discord");
            }
            Err(e) => {
                println!("Error while sending message to discord {:?}", e);
            }
        }
    });
}
