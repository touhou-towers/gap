#![recursion_limit = "128"]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate lazy_static;

mod database;

use lazy_static::lazy_static;
use database::*;
use std::collections::HashSet;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::str;

lazy_static! {
	static ref BALLRACE_MAPS: HashSet<&'static str> = {
		let mut m = HashSet::new();
		m.insert("gmt_ballracer_skyworld01");
		m.insert("gmt_ballracer_grassworld01");
		m.insert("gmt_ballracer_memories02");
		m.insert("gmt_ballracer_khromidro02");
		m.insert("gmt_ballracer_paradise03");
		m.insert("gmt_ballracer_sandworld02");
		m.insert("gmt_ballracer_iceworld03");
		m.insert("gmt_ballracer_nightball");
		m.insert("gmt_ballracer_facile");
		m.insert("gmt_ballracer_flyinhigh01");
		m.insert("gmt_ballracer_metalworld");
		m.insert("gmt_ballracer_neonlights");
		m.insert("gmt_ballracer_waterworld02");
		m.insert("gmt_ballracer_spaceworld");
		m.insert("gmt_ballracer_rainbowworld");
		m.insert("gmt_ballracer_midorib5");
		m
	};
}

fn main() {
	let listener = TcpListener::bind("0.0.0.0:80").unwrap();
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
						Ok(lvl) if BALLRACE_MAPS.contains(map) => connection.get_ballrace_records(map, lvl),
						_ => "{\"err\":\"invalid map or level\"}".to_string(),
					}
				},
				[Some("players"), Some("richest"), None] => connection.get_richest_players(),
				[Some("players"), Some("active"), None] => connection.get_most_active_players(),
				[Some("players"), Some("blockles"), None] => connection.get_top_tetris_players(),
				_ => "{\"err\":\"invalid endpoint\"}".to_string(),
			}
		}
		_ => "{\"err\":\"invalid request\"}".to_string(),
	};

	let response = format!("{}{}", "HTTP/1.1 200 OK \r\n\r\n", response);
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}
