use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Request {
    pub action: String,
    pub params: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub time: i64,
    pub self_id: i64,
    #[serde(flatten)]
    pub inner: EventInner,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "message_type", rename_all = "snake_case")]
pub enum MessageInner {
    Guild { message_id: String },
    Group { group_id: i64 },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "post_type", rename_all = "snake_case")]
pub enum EventInner {
    Message {
        // TODO enum
        #[serde(flatten)]
        inner: MessageInner,
        sub_type: String,
        // message_id: i32,
        // user_id: i64,
        message: String,
        // raw_message: String,
        // font: i64,
        // sender: Sender,
    },
    Request {
        request_type: String,
    },
    Notice {
        notice_type: String,
    },
    MetaEvent {
        meta_event_type: String,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Sender {
    GroupUpload,
    GroupAdmin,
    GroupDecrease,
    GroupIncrease,
    GroupBan,
    FriendAdd,
    GroupRecall,
    FriendRecall,
    GroupCard,
    OfflineFile,
    ClientStatus,
    Essense,
    Notify,
}

#[derive(Deserialize, Debug)]
pub struct Message {}
