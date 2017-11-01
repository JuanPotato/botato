use Botato;

use tg_botapi::types;
use std::sync::Arc;

mod sed;
mod start;
mod about;
mod help;
mod whoami;

pub use self::sed::Sed;
pub use self::start::Start;
pub use self::about::About;
// pub use self::help::Help;
pub use self::whoami::WhoAmI;

pub trait Plugin {
    const ID: i64;

    fn parse(bot: &Arc<Botato>, msg: types::Message) -> Result<Option<String>, String>;
}

