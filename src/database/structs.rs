use diesel::Queryable;
use diesel::table;

use serde::Serialize;

// response struct
#[derive(Serialize)]
pub struct Response<T> {
	#[serde(default = "unknown", rename(serialize = "lastUpdated"))]
	pub last_updated: u128,
	#[serde(default = "unknown")]
	pub data: Vec<T>,
}

// users struct and table
table! {
	gm_users {
		id -> Text,
		name -> Text,
		#[sql_name = "steamid"]
		steam_id -> Text,
		#[sql_name = "betatest"]
		beta_test -> Text,
		#[sql_name = "CreatedTime"]
		created_time -> Text,
		ip -> Text,
		levels -> Text,
		#[sql_name = "pvpweapons"]
		pvp_weapons -> Text,
		#[sql_name = "clisettings"]
		cli_settings -> Text,
		#[sql_name = "MaxItems"]
		max_items -> Text,
		inventory -> Text,
		#[sql_name = "BankLimit"]
		bank_limit -> Text,
		bank -> Text,
		#[sql_name = "plysize"]
		player_size -> Text,
		achivement -> Text, // spelling intended
		#[sql_name = "Roomdata"]
		room_data -> Text,
		#[sql_name = "rumData"]
		rum_data -> Text,
		#[sql_name = "romData"]
		rom_data -> Text,
		#[sql_name = "rooomData"]
		rooom_data -> Text, // ??
		money -> Integer,
		#[sql_name = "LastOnline"]
		last_online -> Text,
		hat -> Text,
		#[sql_name = "tetrisscore"]
		tetris_score -> Integer,
		time -> Text,
		ramk -> Integer, // ??
		ball -> Integer,
	}
}

#[derive(Clone, Queryable, Serialize)]
pub struct PlayerData {
	id: String,
	name: String,
	#[serde(rename(serialize = "steamID"))]
	steam_id: String,
	#[serde(skip_serializing)]
	beta_test: String,
	#[serde(rename(serialize = "joinedAt"))]
	created_time: String,
	#[serde(skip_serializing)]
	ip: String,
	#[serde(skip_serializing)]
	levels: String,
	#[serde(skip_serializing)]
	pvp_weapons: String,
	#[serde(skip_serializing)]
	cli_settings: String,
	#[serde(skip_serializing)]
	max_items: String,
	#[serde(skip_serializing)]
	inventory: String,
	#[serde(skip_serializing)]
	bank_limit: String,
	#[serde(skip_serializing)]
	bank: String,
	#[serde(skip_serializing)]
	player_size: String,
	#[serde(skip_serializing)]
	achivement: String, // spelling intended
	#[serde(skip_serializing)]
	room_data: String,
	#[serde(skip_serializing)]
	rum_data: String,
	#[serde(skip_serializing)]
	rom_data: String,
	#[serde(skip_serializing)]
	rooom_data: String, // ??
	money: i32,
	#[serde(skip_serializing)]
	last_online: String,
	#[serde(skip_serializing)]
	hat: String,
	#[serde(rename(serialize = "tetrisScore"))]
	tetris_score: i32,
	#[serde(rename(serialize = "timePlayed"))]
	time: String,
	#[serde(skip_serializing)]
	ramk: i32, // ??
	#[serde(skip_serializing)]
	ball: i32,
}

// ballrace struct and table
table! {
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
	id: i32,
	#[serde(default = "unknown", rename(serialize = "steamID64"))]
	steam_id64: String,
	#[serde(default = "unknown")]
	name: String,
	#[serde(default = "unknown", rename(serialize = "mapName"))]
	map: String,
	#[serde(default = "unknown", rename(serialize = "level"))]
	lvl: String,
	time: f32,
}
