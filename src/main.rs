use std::{env, process::exit};
mod persistence;

use matrix_sdk::{
    self,
    config::SyncSettings,
    room::Room,
    ruma::events::room::message::{
        MessageType, RoomMessageEventContent, SyncRoomMessageEvent, TextMessageEventContent, FormattedBody,
    },
    Client,
};
use url::Url;

async fn on_room_message(event: SyncRoomMessageEvent, room: Room) {
    if let Room::Joined(room) = room {
        println!("YEEEEEEEEEEEEEEEEEEEHAAAAAAAAAAAAAAAAAAA");
        println!("-----------------Event is {:#?}", event);
        if let SyncRoomMessageEvent {
            content:
                RoomMessageEventContent {
                    msgtype: MessageType::Text(TextMessageEventContent { 
                        body: msg_body, 
                        formatted: 
                            Some(FormattedBody {body: formatted_body, ..} )
                        , ..
                        }),
                    ..
                },
            sender,
            ..
        } = event
        {
            let member = room.get_member(&sender).await.unwrap().unwrap();
            let name = member
                .display_name()
                .unwrap_or_else(|| member.user_id().as_str());

            if msg_body.contains("!party") && formatted_body.contains("@bot-shodan:ionescu.net") {
                let content = RoomMessageEventContent::text_plain("🎉🎊🥳 let's PARTY!! 🥳🎊🎉");

                println!("sending");

                // send our message to the room we found the "!party" command in
                // the last parameter is an optional transaction id which we don't
                // care about.
                room.send(content, None).await.unwrap();

                println!("message sent");
            }
        }
    }
}

async fn login(
    homeserver_url: String,
    username: &str,
    password: &str,
) -> Result<(), matrix_sdk::Error> {
    let homeserver_url = Url::parse(&homeserver_url).expect("Couldn't parse the homeserver URL");
    let client = Client::new(homeserver_url).await.unwrap();

    client.register_event_handler(on_room_message).await;

    client
        .login(username, password, None, Some("rust-sdk"))
        .await?;
    client.sync(SyncSettings::new()).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), matrix_sdk::Error> {
    tracing_subscriber::fmt::init();

    let (homeserver_url, username, password) =
        match (env::args().nth(1), env::args().nth(2), env::args().nth(3)) {
            (Some(a), Some(b), Some(c)) => (a, b, c),
            _ => {
                eprintln!(
                    "Usage: {} <homeserver_url> <username> <password>",
                    env::args().next().unwrap()
                );
                exit(1)
            }
        };

    login(homeserver_url, &username, &password).await
}
