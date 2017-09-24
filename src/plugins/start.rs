use tg_botapi::args;
use tg_botapi::types;
use tg_botapi::BotApi;

use regex::Regex;

use std::sync::Arc;

pub fn start(bot: Arc<BotApi>, msg: types::Message) {
    let new_msg = args::SendMessageBuilder::default()
        .text("You are now running Botat OS v0.0")
        .chat_id(msg.chat.id)
        .reply_to_message_id(msg.message_id)
        .build().unwrap();

    let _ = bot.send_message(&new_msg);
}
