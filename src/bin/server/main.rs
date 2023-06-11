mod connection;
mod group;
mod group_table;

use async_chatting_service::utils::ChatResult;
use async_std::{net, prelude::*, task};
use group_table::GroupTable;

use std::sync::Arc;

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: server ADDRESS");
    let chat_group_table = Arc::new(group_table::GroupTable::new());

    task::block_on(async {
        let listener = net::TcpListener::bind(address).await?;
        let mut new_connections = listener.incoming();
        while let Some(socket_result) = new_connections.next().await {
            let socket = socket_result?;
            let groups = chat_group_table.clone();
            task::spawn(async {
                log_error(serve(socket, groups).await);
            });
        }
        Ok(())
    })
}

fn log_error(result: ChatResult<()>) {
    if let Err(err) = result {
        eprintln!("Error: {}", err)
    }
}

// ! subject to change
async fn serve(_S: net::TcpStream, _groups: Arc<GroupTable>) -> ChatResult<()> {
    Ok(())
}
