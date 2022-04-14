mod image_generator;
use image_generator::{Generator, ImageGenerator};
use std::fs;
use std::sync::Arc;
use tbot::prelude::*;
use tbot::types::message::Kind;
use tbot::types::{input_file, message, Message, chat};
use tbot::contexts::Command;

pub mod env {
    pub const CHAT_ID: &str = env!("CHAT_ID");
    pub const MESSAGE_ID: &str = env!("MESSAGE_ID");
}

pub async fn command_handler(context: Arc<Command>) {

    context.delete_message(context.message_id).call().await.unwrap();
    
    let (message, from_user) = if let Some( Message {
        kind: Kind::Text(ref text),
        from: Some( message::From::User (
                ref user
            )
        ),  
        ..
    }) = context.reply_to {
        (text.value.clone(), user)
    } else {
        return;
    };

    match context.from {
        Some(message::From::User(ref user)) => {
            let superusers = context.get_chat_administrators().call().await.unwrap();
            let member = superusers
            .into_iter()
            .find(|m| m.user.username == user.username );

            if member.is_some() {
                if  Generator::generate(message).is_some() {
                    let file = fs::read("tmp/render.png").unwrap();
                    let input = input_file::Photo::with_bytes(file).caption(
                        {
                            let name = match from_user.username {
                                Some(ref username) => username,
                                None => &from_user.first_name,
                            };
                            format!("from: {}", name)
                        }
                    );
                    context.bot.edit_message_media( 
                        chat::Id::from(env::CHAT_ID.parse::<i64>().unwrap()), 
                        message::Id::from(env::MESSAGE_ID.parse::<u32>().unwrap()),
                        input
                    )
                    .call().await.unwrap();
    
                } else {
                    context
                        .send_message_in_reply("error".to_string())
                        .call().await.unwrap();
                }
            }
            else {
                return
            }
        }
        _ => (),
    }
}
