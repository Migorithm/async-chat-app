use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utils;
pub use utils::*;

// * FromClient Represent the packets a client can send to the server
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Join {
        //* Arc helps the server avoid making copies of strings as it manages groups and distributes messages
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

// * FromServer Represent what the server can send back.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromServer {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

#[test]
fn test_from_client_json() {
    let from_client = FromClient::Post {
        group_name: Arc::new("A".to_string()),
        message: Arc::new("Hello Guys!".to_string()),
    };

    let jsonified = serde_json::to_string(&from_client).unwrap();

    //* Note that Arc pointers have no effect on the serialized form.
    assert_eq!(
        jsonified,
        r#"{"Post":{"group_name":"A","message":"Hello Guys!"}}"#
    );
    assert_eq!(
        serde_json::from_str::<FromClient>(&jsonified).unwrap(),
        from_client
    );
}
