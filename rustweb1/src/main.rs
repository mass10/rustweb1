#[macro_use]
extern crate nickel;

use nickel::Nickel;

fn main() {
	let mut server = Nickel::new();

	server.utilize(router! {
		// 全ての GET をひっかけるようだ。
		get "**" => |_req, _res| {
			format!("{:?}: [/] Hello world!\n", std::thread::current().id())
		}
		// 有効なルーティング。完全一致のようだ。
		post "/test1" => |_req, _res| {
			format!("{:?}: [/test1] -> accepted post data.\n", std::thread::current().id())
		}
		// 無効。これは何もひっかけられなかった。
		post "test2" => |_req, _res| {
			format!("{:?}: [test2]: accepted post data.\n", std::thread::current().id())
		}
	});

	let _result = server.listen("0.0.0.0:6767");
}
