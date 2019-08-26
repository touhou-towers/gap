#![recursion_limit = "128"]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

mod database;

use database::*;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::str;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let mut connection = Db::connection();

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		// TODO: threads
		handle_connection(stream, &mut connection);
	}
}

fn handle_connection(mut stream: TcpStream, connection: &mut Db) {
	let mut buffer = [0; 512];
	stream.read(&mut buffer).unwrap();

	let mut req = str::from_utf8(&buffer).unwrap().split_whitespace();

	let response = match [req.next(), req.next(), req.next()] {
		[Some("GET"), Some(path), Some(_ver)] => {
			let mut path = path.split("/");
			path.next(); // starts with a / so get rid of that
			match [path.next(), path.next(), path.next()] {
				[Some("ballrace"), Some(map), Some(level)] => {
					match level.parse::<usize>() {
						Ok(lvl) => connection.get_ballrace_records(map, lvl),
						Err(_e) => "{}".to_string(),
					}
				},
				[Some("players"), Some("richest"), None] => connection.get_richest_players(),
				[Some("players"), Some("active"), None] => connection.get_most_active_players(),
				[Some("players"), Some("blockles"), None] => connection.get_top_tetris_players(),
				_ => "{}".to_string(),
			}
		}
		_ => "{}".to_string(),
	};

	let response = format!("{}{}", "HTTP/1.1 200 OK \r\n\r\n", response);
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}
