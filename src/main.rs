extern crate tg_botapi;
extern crate rusqlite;
extern crate time;
extern crate regex;

use tg_botapi::args;
use tg_botapi::types;
use tg_botapi::BotApi;

use std::sync::{Mutex, Arc};
use std::thread;
use std::path::Path;

use rusqlite::Connection;

use plugins::Plugin;

#[macro_use]
mod utils;

mod plugins;


// User
// id
// plugins # see Group

// Group
// id
// mode # 0 (normal), 1 (sed) more numbers for more ideas
// plugins # vec of u8 (to fit with sql BLOB) binary flipping on and of
//         # each plugin has a permenant id index
//         # but is just sorted alphabetically when shown.
//         # Has no effect when mode is not 0


const ADMIN_ID: i64 = 0;

#[derive(Debug)]
pub struct Database {
    conn: Mutex<Connection>
}

#[derive(Debug)]
pub struct Botato {
    pub db: Database,
    pub api: BotApi,
}

#[derive(Debug)]
struct User {
    id: i64,
    plugins: Vec<u8>,
}

#[derive(Debug)]
struct Group {
    id: i64,
    mode: i64,
    plugins: Vec<u8>,
}

impl Database {
    fn create_user(&self, user_id: i64) {
        let db = self.conn.lock().unwrap();

        db.execute("INSERT OR IGNORE INTO users (id, plugins) VALUES (?1, ?2)",
            &[&user_id, &vec![std::u8::MAX]]).unwrap();
    }

    fn create_group(&self, group_id: i64) {
        let db = self.conn.lock().unwrap();

        db.execute("INSERT OR IGNORE INTO users (id, plugins) VALUES (?1, ?2)",
            &[&group_id, &vec![std::u8::MAX]]).unwrap();
    }


    fn get_user_plugins(&self, user_id: i64) -> Vec<u8> {
        let db = self.conn.lock().unwrap();

        db.query_row("SELECT plugins FROM users WHERE id=?1", &[&user_id], |row| {
           row.get(0)
        }).unwrap()
    }

    fn get_group(&self, group_id: i64) -> Group {
        let db = self.conn.lock().unwrap();

        db.query_row("SELECT * FROM groups WHERE id=?1", &[&group_id], |row| {
            Group {
                id: row.get(0),
                mode: row.get(1),
                plugins: row.get(2),
            }
        }).unwrap()
    }

    fn get_group_plugins(&self, group_id: i64) -> Vec<u8> {
        let db = self.conn.lock().unwrap();

        db.query_row("SELECT plugins FROM groups WHERE id=?1", &[&group_id], |row| {
            row.get(0)
        }).unwrap()
    }

    fn get_group_mode(&self, group_id: i64) -> i64 {
        let db = self.conn.lock().unwrap();

        db.query_row("SELECT mode FROM groups WHERE id=?1", &[&group_id], |row| {
            row.get(0)
        }).unwrap()
    }


    fn inc_plugin_stat(&self, plugin_id: i64) {
        let now = time::get_time().sec;
        let hour = now - (now % 3600);

        let db = self.conn.lock().unwrap();

        db.execute(
            "UPDATE stats SET calls=calls+1 WHERE plugin_id=?1 AND hour=?2;
            
            INSERT INTO players (plugin_id, hour, calls) VALUES (?1, ?2, 1)
                WHERE (Select Changes() = 0);",
            &[&plugin_id, &hour]
        ).unwrap();
    }
    
    fn set_user_plugin(&self, plugin_id: i64, user_id: i64, on: bool) {
        let mut plugins = self.get_user_plugins(user_id);

        let index = ((plugin_id - 1) / 8) as usize; // because u32
        let bit_index = (plugin_id - 1) % 8;

        if plugins.len() <= index {
            plugins.resize(index + 1, std::u8::MAX); 
        }

        if on {
            plugins[index] |= 1 << bit_index;
        } else {
            plugins[index] &= !(1 << bit_index);
        }

        let db = self.conn.lock().unwrap();

        db.execute(
            "UPDATE users SET plugins=?2 WHERE user_id=?1",
            &[&user_id, &plugins]
        ).unwrap();
    }
    
    fn set_group_plugin(&self, plugin_id: i64, group_id: i64, on: bool) {
        let mut plugins = self.get_group_plugins(group_id);

        let index = ((plugin_id - 1) / 8) as usize; // because u32
        let bit_index = (plugin_id - 1) % 8;

        if plugins.len() <= index {
            plugins.resize(index + 1, std::u8::MAX); 
        }

        if on {
            plugins[index] |= 1 << bit_index;
        } else {
            plugins[index] &= !(1 << bit_index);
        }

        let db = self.conn.lock().unwrap();

        db.execute(
            "UPDATE groups SET plugins=?2 WHERE group_id=?1",
            &[&group_id, &plugins]
        ).unwrap();
    }
   
    fn enable_user_plugin(&self, plugin_id: i64, user_id: i64) {
        self.set_user_plugin(plugin_id, user_id, true);
    }

    fn disable_user_plugin(&self, plugin_id: i64, user_id: i64) {
        self.set_user_plugin(plugin_id, user_id, false);
    }

    fn enable_group_plugin(&self, plugin_id: i64, group_id: i64) {
        self.set_group_plugin(plugin_id, group_id, true);
    }

    fn disable_group_plugin(&self, plugin_id: i64, group_id: i64) {
        self.set_group_plugin(plugin_id, group_id, false);
    }
     
    fn set_group_mode(&self, group_id: i64, mode: i64) {
        let db = self.conn.lock().unwrap();

        db.execute(
            "UPDATE groups SET mode=?2 WHERE group_id=?1",
            &[&group_id, &mode]
        ).unwrap();
    }
}

fn main() {
    // Config file, should make it later, important stuff now
    // let token = env::var("TOKEN").unwrap();
    let token = "";

    // let bot_arc = Arc::new(BotApi::new_debug(&token));
    let bot_api = BotApi::new_debug(&token);

    let me_irl = bot_api.get_me().expect("Could not establish a connection :\\");

    let db_path = Path::new("./database.db");
    let exists = db_path.exists();

    let conn = Connection::open(db_path).unwrap();

    if !exists {
        conn.execute("
            CREATE TABLE users (
                id              INTEGER NOT NULL PRIMARY KEY,
                plugins         BLOB
            );
            
            CREATE TABLE groups (
                id              INTEGER NOT NULL PRIMARY KEY,
                mode            INTEGER NOT NULL DEFAULT 0,
                plugins         BLOB
            );
            
            CREATE TABLE stats (
                plugin_id       INTEGER NOT NULL,
                hour            INTEGER NOT NULL,
                amount_called   INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (plugin_id, hour)
            )", &[]).unwrap();
    }

    let botato = Botato {
        db: Database { conn: Mutex::new(conn) },
        api: bot_api,
    };

    get_update_listener(botato);
}

fn get_update_listener(bot: Botato) {
    let bot_arc = Arc::new(bot);

    let mut update_args = args::GetUpdatesBuilder::default()
        .timeout(600)
        .offset(0)
        .build()
        .unwrap();

    loop {
        let res_updates = bot_arc.api.get_updates(&update_args);

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

                let _ = bot_arc.api.send_message(&new_msg);
            }
        }
    }
}

fn parse_update(bot: Arc<Botato>, update: types::Update) {
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

fn parse_message(bot: Arc<Botato>, msg: types::Message) {
    // Increment message / hour counter
    // increment command / hour counter if start / or s/
    
    // no need to try and check the message if it isnt sed not a command
    let index = ref_or_return!(msg.text).find("/").unwrap_or(3);

    match index {
        0 => {
            parse_command(bot, msg);
        }

        1 => {
            plugins::Sed::parse(&bot, msg);
        }

        _ => { }
    }
}

fn parse_command(bot: Arc<Botato>, msg: types::Message) {
    let text = msg.text.clone().unwrap();
    let parts = text.split(" ").collect::<Vec<&str>>();

    let chat_id = msg.chat.id;
    let msg_id = msg.message_id;

    let res = match parts[0] {
        "/start" => plugins::Start::parse(&bot, msg),
        "/about" => plugins::About::parse(&bot, msg),
        // "/help" => plugins::Help::parse(&bot, msg),
        "/whoami" | "/who" | "/me" => plugins::WhoAmI::parse(&bot, msg),

        _ => { Ok(None) }
    };

    match res {
        Ok(opt) => {
            if let Some(send_str) = opt {
               
                let new_msg = args::SendMessageBuilder::default()
                    .text(send_str)
                    .chat_id(chat_id)
                    .reply_to_message_id(msg_id)
                    .parse_mode(String::from("markdown"))
                    .build().unwrap();

                let _ = bot.api.send_message(&new_msg);

            }
        }

        Err(err_str) => {

        }
    }
}
