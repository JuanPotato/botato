extern crate tg_botapi;
extern crate rustyline;

use tg_botapi::args;
use tg_botapi::types;
use tg_botapi::BotApi;

use std::sync::Arc;
use std::thread;
use std::env;

fn main() {
    // Config file, should make it later, important stuff now
    let token = env::var("TOKEN").unwrap();
    let bot_arc = Arc::new(BotApi::new_debug(&token));

    let me_irl = bot_arc.get_me().expect("Could not establish a connection :\\");

    let mut update_args = args::GetUpdates::new().timeout(600).offset(0);

    'update_loop: loop {
        let updates = bot_arc.get_updates(&update_args).unwrap();

        for update in updates {
            update_args.offset = Some(update.update_id + 1);

            if let Some(message) = update.message {
                let bot = bot_arc.clone();

                thread::spawn(move || {
                    // // parse_message(bot,);
                    // I want to have a SQL db that has users information such as

                    // date_first_message
                    // last_message_date
                    // enabled_plugins
                    // default_setup

                    // I plan to make botato customizable to the point of what
                    // plugins you want to see in /help, what plugins you want
                    // enabled/disabled in groups (if admin)
                    // and ability to provide feedback
                });
            }
        }
    }

    update_args.limit = Some(0);
    update_args.timeout = Some(0);
    let _ = bot_arc.get_updates(&update_args);
}

fn parse_message(bot: BotApi, msg: types::Message) {
    // TODO Implement, or whatever
}
