use tg_botapi::args;
use tg_botapi::types;
use tg_botapi::BotApi;

use regex::Regex;

use std::sync::Arc;


pub fn parse_sed(bot: Arc<BotApi>, msg: types::Message, msg_text: String) {
    if msg.reply_to_message.is_none() {
        return;
    }

    let mut reply_msg = msg.reply_to_message.unwrap();


    if reply_msg.caption.is_some() {
        reply_msg.text = reply_msg.caption.clone();
    }

    if reply_msg.text.is_none() {
        return;
    }

    if msg.from.is_none() {
        return;
    }

    let reply_msg_id = reply_msg.message_id;
    let reply_msg_text = reply_msg.text.unwrap();

    if msg_text.starts_with("s/") || msg_text.starts_with("/s/") {
        let boundaries = get_boundaries(&msg_text);
        let len = boundaries.len();

        match len {
            2 | 3 => {
                let pattern = &msg_text[boundaries[0] + 1 .. boundaries[1]].replace("\\/", "/");

                let to = if len == 3 {
                    msg_text[boundaries[1] + 1 .. boundaries[2]].to_string().replace("\\/", "/")
                } else {
                    String::new()
                };

                let re = Regex::new(pattern);

                match re {
                    Ok(result) => {
                        let after = result.replace_all(&reply_msg_text, to.as_str());

                        let new_msg = if after == "" {
                            args::SendMessageBuilder::default()
                                .text("`java.lang.NullPointerException: Empty Message`")
                                .chat_id(msg.chat.id)
                                .reply_to_message_id(reply_msg_id)
                                .parse_mode(String::from("Markdown"))
                                .build().unwrap()
                        } else {
                            args::SendMessageBuilder::default()
                                .text(after.into_owned())
                                .chat_id(msg.chat.id)
                                .reply_to_message_id(reply_msg_id)
                                .build().unwrap()
                        };

                        let _ = bot.send_message(&new_msg);
                    }

                    Err(err) => {
                        let new_msg = args::SendMessageBuilder::default()
                            .text(err.to_string())
                            .chat_id(msg.chat.id)
                            .reply_to_message_id(msg.message_id)
                            .build().unwrap();

                        let _ = bot.send_message(&new_msg);
                    }
                }
            }
            _ => {
                let new_msg = args::SendMessageBuilder::default()
                    .text("Invalid number of delimiters!")
                    .chat_id(msg.chat.id)
                    .reply_to_message_id(msg.message_id)
                    .build().unwrap();

                let _ = bot.send_message(&new_msg);
            }
        }
    }
}

fn get_boundaries(string: &str) -> Vec<usize> { // Better than regex
    let mut boundaries = Vec::new();
    let mut previous_char = '/';

    for (index,cha) in string.char_indices() {
        if '/' == cha && previous_char != '\\' {
            boundaries.push(index);
        }

        previous_char = cha;
        
        if cha == '\\' && previous_char == '\\' {
            previous_char = ' ';
        }
    }

    if boundaries[0] == 0 {
        let _ = boundaries.remove(0);
    }

    if boundaries[boundaries.len() - 1] != string.len() - 1 {
        boundaries.push(string.len());
    }

    boundaries
}
