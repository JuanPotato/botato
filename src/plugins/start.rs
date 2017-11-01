use tg_botapi::types;
use std::sync::Arc;
use Botato;
use Plugin;

#[derive(Debug)]
pub struct Start;

impl Plugin for Start {
    const ID: i64 = 2;
    fn parse(_bot: &Arc<Botato>, _msg: types::Message) -> Result<Option<String>, String> {
        Ok(Some(String::from("You are now running Botat OS v0.0")))
    }
}
