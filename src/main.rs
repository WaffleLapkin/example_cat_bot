use serde::Deserialize;
use teloxide::Bot;
use teloxide::dispatching::updater::polling;
use teloxide::types::{UpdateKind, InputFile};
use teloxide::types::ParseMode::Markdown;
use futures::StreamExt;

use reqwest::Client;

#[tokio::main]
async fn main() {
    let bot = Bot::new(env!("TOKEN", "Example cat bot require bot-token to be set at compile-time, slease re-run build command with `TOKEN=*bot token here*`"));
    let http = Client::new();
    let updater = polling(&bot);

    updater.for_each(|update| async {
        match update {
            Ok(update) => match update.kind {
                UpdateKind::Message(msg) => {
                    match msg.text() {
                        Some("/cat") | Some("/pic") => {
                            if let Ok(url) = kitten_url(&http).await {
                                bot
                                    .send_photo(
                                        msg.chat.id,
                                        InputFile::Url(url),
                                    )
                                    .send()
                                    .await
                                    .map(|_| ())
                                    .unwrap_or_else(|err| { dbg!(err); });
                            }
                        }
                        _ => {
                            bot
                                .send_message(
                                    msg.chat.id,
                                    "Это бот демонстрирующий работу библиотеки [teloxide](https://github.com/teloxide/teloxide).\n\nИспользуй команду /cat чтобы получить картинку котика!"
                                )
                                .parse_mode(Markdown)
                                .send()
                                .await
                                .map(|_| ())
                                .unwrap_or_else(|err| { dbg!(err); });
                        }
                    }
                },
                _ => { /* ignore all non-message updates */ },
            },
            Err(error) => {
                dbg!(error);
            }
        }
    }).await
}

#[derive(Deserialize)]
struct ApiResp {
    url: String,
    // other field are omitted
}

async fn kitten_url(client: &Client) -> Result<String, reqwest::Error> {
    let resp = client.get("https://api.thecatapi.com/v1/images/search").send().await?;
    let mut result: Vec<ApiResp> = serde_json::from_str(&resp.text().await?).unwrap();
    Ok(
        result
            .pop()
            .map(|api_resp| api_resp.url)
            .unwrap_or_else(|| {
                eprintln!("no kitten in response");
                String::new()
            })
    )
}
