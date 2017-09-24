extern crate tg_botapi;
extern crate rusqlite;
extern crate time;
extern crate regex;

use tg_botapi::args;
use tg_botapi::types;
use tg_botapi::BotApi;

use std::sync::Arc;
use std::thread;

#[macro_use]
mod utils;

mod plugins;


// User
// id
// plugins # see Group

// Group
// id
// mode # 0 (normal), 1 (sed) more numbers for more ideas
// plugins # vec of u32? binary flipping on and of
//         # each plugin has a permenant id index
//         # but is just sorted alphabetically when shown.
//         # Has no effect when mode is not 0


const ADMIN_ID: i64 = 0;

fn main() {
    // Config file, should make it later, important stuff now
    let token = env::var("TOKEN").unwrap();

    // let bot_arc = Arc::new(BotApi::new_debug(&token));
    let bot = BotApi::new_debug(&token);

    let me_irl = bot.get_me().expect("Could not establish a connection :\\");

    get_update_listener(bot);
}

fn get_update_listener(bot: BotApi) {
    let bot_arc = Arc::new(bot);

    let mut update_args = args::GetUpdatesBuilder::default()
        .timeout(600)
        .offset(0)
        .build()
        .unwrap();

    loop {
        let res_updates = bot_arc.get_updates(&update_args);

        match res_updates {
            Ok(updates) => {
                for update in updates {
                    update_args.offset = Some(update.update_id + 1);

                    parse_update(bot_arc.clone(), update);
                }
            }

            Err(err) => {
                let new_msg = args::SendMessageBuilder::default()
                    .text(format!("```{}```", err.to_string()))
                    .chat_id(ADMIN_ID)
                    .parse_mode(String::from("Markdown"))
                    .build().unwrap();

                let _ = bot_arc.send_message(&new_msg);
            }
        }
    }
}

fn parse_update(bot: Arc<BotApi>, update: types::Update) {
    if let Some(mut message) = update.message {
        let now = time::get_time();

        if message.caption.is_some() {
            message.text = message.caption.clone();
        }

        if message.text.is_none() || message.date < now.sec - 10 {
            // we only care about recent text messages
            return;
        }

        let new_bot = bot.clone();

        thread::spawn(move || {
            parse_message(new_bot, message);
        });
    }
}

fn parse_message(bot: Arc<BotApi>, msg: types::Message) {
    // Increment message / hour counter
    // increment command / hour counter if start / or s/
    
    // no need to try and check the message if it isnt sed not a command
    let index = ref_or_return!(msg.text).find("/").unwrap_or(3);

    match index {
        0 => {
            parse_command(bot, msg);
        }

        1 => {
            plugins::parse_sed(bot, msg);
        }

        _ => { }
    }
}

fn parse_command(bot: Arc<BotApi>, msg: types::Message) {
    let text = msg.text.clone().unwrap();
    let parts = text.split(" ").collect::<Vec<&str>>();

    match parts[0] {
        "/start" => plugins::start(bot, msg),
        // "/about" => plugins::about();
        // "/help" => plugins::help();
        // "/whoami" | "/who" | "/me" => plugins::whoami();

        _ => {}
    }
}
