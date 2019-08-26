use diesel::prelude::*;
use diesel::dsl::sql;
use diesel::Queryable;
use diesel::table;

use dotenv::dotenv;

use serde::Serialize;

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
	connect: MysqlConnection,
	updated: SystemTime,
}

impl Db {
	pub fn connection() -> Db {
		Db {
			ballrace_results: HashMap::new(),
			connect: connect_to_database(),
			updated: SystemTime::now(),
		}
	}

	pub fn get_ballrace_records(&mut self, map: &str, lvl: usize) -> String {
		let now = SystemTime::now();
		if !self.ballrace_results.entry(map.to_string()).or_insert(HashMap::new()).contains_key(&lvl) ||
			now.duration_since(self.updated).unwrap() >= Duration::new(60 * 60, 0) {
			println!("Querying information for {} {}", map, lvl);
			let json =
					&gm_ballrace::table
						.filter(gm_ballrace::map.eq(map).and(sql(&format!("lvl = '{}'", lvl))))
						.order(gm_ballrace::time)
						.limit(5)
						.load::<BallraceRecord>(&self.connect)
						.unwrap();
			self.ballrace_results.entry(map.to_string())
				.or_insert(HashMap::new())
				.insert(lvl, json.to_vec());
			self.updated = now;
		}
		serde_json::to_string_pretty(
			&Response {
				last_updated: now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis(),
				data: self.ballrace_results.get(map)
					.unwrap()
					.get(&lvl)
					.unwrap()
					.to_vec()
			}
		).unwrap()
	}
}

#[derive(Serialize)]
pub struct Response<T> {
	#[serde(default = "unknown", rename(serialize = "lastUpdated"))]
	last_updated: u128,
	#[serde(default = "unknown")]
	data: Vec<T>,
}

table! {
	use diesel::sql_types::*;
	gm_ballrace {
		id -> Integer,
		#[sql_name = "ply"]
		steam_id64 -> Tinytext,
		name -> Tinytext,
		map -> Tinytext,
		lvl -> Tinytext,
		time -> Float,
	}
}

#[derive(Clone, Queryable, Serialize)]
pub struct BallraceRecord {
	#[serde(skip_serializing)]
	pub id: i32,
	#[serde(default = "unknown", rename(serialize = "steamID64"))]
	pub steam_id64: String,
	#[serde(default = "unknown")]
	pub name: String,
	#[serde(default = "unknown", rename(serialize = "mapName"))]
	pub map: String,
	#[serde(default = "unknown", rename(serialize = "level"))]
	pub lvl: String,
	pub time: f32,
}
