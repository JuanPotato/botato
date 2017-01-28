extern crate tg_botapi;
extern crate rustyline;

use tg_botapi::args;
use tg_botapi::types;
use tg_botapi::BotApi;

use std::sync::Arc;
use std::thread;
use std::env;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("config.json");

    match file {
        Ok(f) => {
            println!("Could not find config file. Making one...");
            let mut rl = rustyline::Editor::<()>::new();
            let redline = rl.readline(">> ");
            let mut token = "";

            match readline {
                Ok(line) => {
                    token = line;
                }

                Err(_)   => {
                    println!("No input, thanks.");
                    process::exit(1);
                },

            }
        }

        Err(e) => {
        }
    }

    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    let token = &env::var("TOKEN");
    let bot = Arc::new(BotApi::new_debug(token));

    let me_irl = bot.get_me().expect("Could not establish a connection :\\");

    let mut update_args = args::GetUpdates::new().timeout(600).offset(0);

    'update_loop: loop {
        let updates = bot.get_updates(&update_args).unwrap();

        for update in updates {
            update_args.offset = Some(update.update_id + 1);

            if let Some(message) = update.message {
                let message_text = message.text.unwrap_or(String::new());
            }
        }
    }
    update_args.limit = Some(0);
    update_args.timeout = Some(0);
    let _ = bot.get_updates(&update_args);
}
