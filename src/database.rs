mod structs;
use structs::*;

use serde::Serialize;

use diesel::prelude::*;
use diesel::dsl::sql;
use dotenv::dotenv;

use std::env;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

fn connect_to_database() -> MysqlConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL")
		.expect("DATABASE_URL was not set!");
	MysqlConnection::establish(&database_url)
		.expect(&format!("Couldn't connect to {}", database_url))
}

pub struct Db {
	// levelname, then level
	ballrace_results: HashMap<String, HashMap<usize, Vec<BallraceRecord>>>,
	// another thing to do would be to load all players into memory
	// then do sorting/filtering in rust rather than in the sql query
	// but i like this better for now - it'll be better when there
	// are more people
	richest_players: Vec<PlayerData>,
	most_active_players: Vec<PlayerData>,
	top_tetris_players: Vec<PlayerData>,
	// understandably having one timestamp for everything makes
	// it operate in a way that is not as apparent and probably
	// not ideal at all.
	updated: SystemTime,
	connect: MysqlConnection,
}

impl Db {
	pub fn connection() -> Db {
		Db {
			ballrace_results: HashMap::new(),
			richest_players: Vec::new(),
			most_active_players: Vec::new(),
			top_tetris_players: Vec::new(),
			updated: SystemTime::UNIX_EPOCH,
			connect: connect_to_database(),
		}
	}

	pub fn get_ballrace_records(&mut self, map: &str, lvl: usize) -> String {
		if !self.ballrace_results.entry(map.to_string()).or_insert(HashMap::new()).contains_key(&lvl) ||
			self.is_cache_outdated() {
			self.ballrace_results.entry(map.to_string())
				.or_insert(HashMap::new())
				.insert(lvl, (&gm_ballrace::table
						.filter(gm_ballrace::map.eq(map).and(sql(&format!("lvl = '{}'", lvl))))
						.order(gm_ballrace::time)
						.limit(5)
						.load::<BallraceRecord>(&self.connect)
						.unwrap())
					.to_vec());
			self.updated = SystemTime::now();
		}
		self.to_response_string(self.ballrace_results.get(map)
			.unwrap()
			.get(&lvl)
			.unwrap()
			.to_vec())
	}

	pub fn get_richest_players(&mut self) -> String {
		if self.richest_players.len() == 0 || self.is_cache_outdated() {
			self.richest_players = (&gm_users::table
				.order(gm_users::money.desc())
				.limit(15)
				.load::<PlayerData>(&self.connect)
				.unwrap()).to_vec();
			self.updated = SystemTime::now();
		}
		self.to_response_string(self.richest_players.to_vec())
	}

	pub fn get_most_active_players(&mut self) -> String {
		if self.most_active_players.len() == 0 || self.is_cache_outdated() {
			self.most_active_players = (&gm_users::table
				.order(gm_users::money.desc())
				.limit(15)
				.load::<PlayerData>(&self.connect)
				.unwrap()).to_vec();
			self.updated = SystemTime::now();
		}
		self.to_response_string(self.most_active_players.to_vec())
	}

	pub fn get_top_tetris_players(&mut self) -> String {
		if self.top_tetris_players.len() == 0 || self.is_cache_outdated() {
			self.top_tetris_players = (&gm_users::table
				.order(gm_users::tetris_score.desc())
				.limit(15)
				.load::<PlayerData>(&self.connect)
				.unwrap()).to_vec();
			self.updated = SystemTime::now();
		}
		self.to_response_string(self.top_tetris_players.to_vec())
	}

	fn is_cache_outdated(&self) -> bool {
		SystemTime::now().duration_since(self.updated).unwrap() >= Duration::new(60 * 60, 0)
	}

	fn to_response_string<T: Serialize>(&self, data: Vec<T>) -> String {
		serde_json::to_string_pretty(
			&Response {
				last_updated: self.updated.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis(),
				data,
			})
			.unwrap()
	}
}

