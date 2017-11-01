use tg_botapi::types;
use std::sync::Arc;
use Botato;
use Plugin;

#[derive(Debug)]
pub struct About;

impl Plugin for About {
    const ID: i64 = 0;

    fn parse(_bot: &Arc<Botato>, _msg: types::Message) -> Result<Option<String>, String> {
        Ok(Some(String::from("Hi! I'm Botato.

        I have tons of plugins that you can disable if you don't need (check out /plugins). \
        You can also disable my features for entire groups. If you just want my regex \
        replacement functionality, you can disable everything with /sed\\_mode

        I'm written in Rust by @JuanPotato

        Botato v0.4 [Github](https://github.com/JuanPotato/Botato)")))
    }
}
