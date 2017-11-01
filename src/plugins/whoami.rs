use tg_botapi::types;
use std::sync::Arc;
use Botato;
use Plugin;
use utils::{make_name, escape_md};

#[derive(Debug)]
pub struct WhoAmI;

impl Plugin for WhoAmI {
    const ID: i64 = 3;
    fn parse(_bot: &Arc<Botato>, msg: types::Message) -> Result<Option<String>, String> {
        if let Some(from) = msg.from {
            Ok(Some(format!("{} \\[{}]", escape_md(&make_name(&from)), from.id)))
        } else {
            Ok(None)
        }
    }
}
