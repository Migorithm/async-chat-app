use std::sync::Arc;

use async_chatting_service::{self, utils, ChatResult};
use async_chatting_service::{FromClient, FromServer};
use async_std::net;

use async_std::prelude::*;
use async_std::{io, task};

async fn send_commands(mut to_server: net::TcpStream) -> ChatResult<()> {
    println!(
        "Commands:\n\
              join GROUP\n\
              post GROUP MESSAGE...\n\
              Type Control-D (on Unix) or Control-Z (on Windows) \
              to close the connection."
    );
    let mut command_lines = io::BufReader::new(io::stdin()).lines();
    while let Some(command_result) = command_lines.next().await {
        let command = command_result?;

        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue,
        };
        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await;
    }
    Ok(())
}

async fn handle_replies(from_server: net::TcpStream) -> ChatResult<()> {
    //* wraps a BufReader around socket
    let buffered = io::BufReader::new(from_server);
    let mut reply_stream = utils::receive_as_json(buffered);
    while let Some(reply) = reply_stream.next().await {
        match reply? {
            FromServer::Message {
                group_name,
                message,
            } => {
                println!("Message posted to {}: {}", group_name, message);
            }
            FromServer::Error(message) => {
                println!("Error from server: {}", message);
            }
        }
    }
    Ok(())
}

/// Parse a line (presumably read from the standard input) as a `Request`.
fn parse_command(line: &str) -> Option<FromClient> {
    let (command, rest) = get_next_token(line)?;
    if command == "post" {
        let (group, rest) = get_next_token(rest)?;
        let message = rest.trim_start().to_string();
        Some(FromClient::Post {
            group_name: Arc::new(group.to_string()),
            message: Arc::new(message),
        })
    } else if command == "join" {
        let (group, rest) = get_next_token(rest)?;
        if !rest.trim_start().is_empty() {
            return None;
        }
        Some(FromClient::Join {
            group_name: Arc::new(group.to_string()),
        })
    } else {
        //* If the user enters an unrecognizable command, print an error message and return None
        eprintln!("Unrecognized command: {:?}", line);
        None
    }
}

fn get_next_token(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();

    if input.is_empty() {
        return None;
    }

    match input.find(char::is_whitespace) {
        Some(space) => Some((&input[0..space], &input[space..])),
        None => Some((input, "")),
    }
}
fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: client ADDRESS:PORT");

    task::block_on(async {
        let socket = net::TcpStream::connect(address).await?;
        socket.set_nodelay(true)?;

        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);

        //* future::select! has been replaced by Future::race which make the program exits as soon as either one has finished.
        to_server.race(from_server).await?;

        Ok(())
    })
}
