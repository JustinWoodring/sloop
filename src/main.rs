mod irc;
mod parseargs;
use irc::Connector;
use parseargs::Arguments;
use std::env;

fn main() {
	let mut args: Vec<String> = env::args().collect();
	let arguments = Arguments::parse(args);

	if !(arguments.should_quit()){
		if arguments.is_daemon(){
			//Daemon::new();
		}
		else{
			let myconnector = Connector::new(arguments.get_server(), arguments.get_port(), arguments.get_channel(), arguments.get_nick(), arguments.get_pass());
			match myconnector.connect() {
				Ok(mut mystream) => {
					mystream.send(arguments.get_message());
					mystream.quit();
				}
				Err(errmsg) => {
					println!("{}",errmsg);
				}
			}
		}
	}
}