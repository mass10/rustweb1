extern crate rustc_serialize;
#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, JsonBody, Nickel, StaticFilesHandler};
extern crate chrono;

/// ユーティリティ
struct Util;

impl Util {
	/// タイムスタンプ
	pub fn get_current_timestamp0() -> String {
		let date = chrono::Local::now();
		return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
	}
}

///
// #[derive(RustcDecodable, RustcEncodable)]
#[derive(serde::Deserialize, Default)]
struct LogStruct {
	/// アプリケーションの名前
	pub application_name: String,
	/// タイムスタンプ
	pub timestamp: String,
	/// 属性
	pub field01: String,
	/// 属性
	pub field02: String,
	/// 属性
	pub field03: String,
	/// 属性
	pub field04: String,
	/// 属性
	pub field05: String,
	/// 属性
	pub field06: String,
	/// 属性
	pub field07: String,
	/// 属性
	pub field08: String,
	/// 属性
	pub field09: String,
}

/// アプリケーション本体
struct Application;
impl Application {
	fn new() -> Application {
		Application {}
	}
	fn run(self: &Application) {
		let mut server = Nickel::new();
		server.utilize(StaticFilesHandler::new("static/assets/"));
		server.get(
			"/",
			middleware! { |_req, _res|
				println!("{} [TRACE] {:?}: [/]", Util::get_current_timestamp0(), std::thread::current().id());
				format!("{:?}: [/] Hello world!", std::thread::current().id())
			},
		);
		server.get(
			"/hello1",
			middleware! { |_req, _res|
				println!("{} [TRACE] {:?}: [/hello1]", Util::get_current_timestamp0(), std::thread::current().id());
				format!("{:?}: [/hello1]", std::thread::current().id())
			},
		);
		server.post(
			"/addon-log",
			middleware! { |request, response|
				let log = request.json_as::<LogStruct>().unwrap_or(LogStruct{..Default::default()});
				println!("{} [TRACE] {:?}: [/addon-log] -> accepted post data. type: application_name: [{}], timestamp: [{}]", Util::get_current_timestamp0(), std::thread::current().id(), log.application_name, log.timestamp);
				format!("{:?}: [/addon-log] -> accepted post data. type: application_name: [{}], timestamp: [{}]", std::thread::current().id(), log.application_name, log.timestamp)
			},
		);
		let _result = server.listen("127.0.0.1:6767");
	}
}

/// エントリーポイント
fn main() {
	let app = Application::new();
	app.run();
}
