extern crate serde;
extern crate serde_json;
extern crate telegram_bot_raw;

use telegram_bot_raw::types::message::MessageKind;
use telegram_bot_raw::types::update::{Update, UpdateKind};

macro_rules! make_test {
    ($asset: ident, $test: expr) => {
        #[test]
        fn $asset() {
        	let data = include_str!(concat!("update_assets/", stringify!($asset), ".json"));
            let update = serde_json::from_str::<Update>(&data).unwrap();
            $test(update)
        }
    };
}

make_test!(migrate_from_chat_id, |update: Update| {
let t = format!("{:?}", update);
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::MigrateFromChatId { .. } = message.kind {
            return ()
        }
    }
    assert!(false, t)
});

make_test!(migrate_to_chat_id, |update: Update| {
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::MigrateToChatId { .. } = message.kind {
            return ()
        }
    }
    assert!(false)
});
