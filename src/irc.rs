use std::io::prelude::*;
use std::net::TcpStream;
use std::result::Result;
use std::time::Duration;

pub struct Connector {
	server: String,
	port : u32,
	channel : String,
	nick: String,
	pass: String,
}

impl Connector{
	pub fn new(sserver : &str, sport : u32, schannel: &str, snick : &str, spass :&str) -> Self {
		Connector{
			server : sserver.to_string(),
			port : sport,
			channel : schannel.to_string(),
			nick : snick.to_string(),
			pass : spass.to_string(),
		}
	}
	pub fn connect(&self) -> Result<IrcStream, String>{
		match TcpStream::connect(format!("{}:{}", self.server, self.port)){
			Ok(mut stream) => {
				stream.set_nonblocking(true).expect("set_nonblocking call failed");
				stream.set_read_timeout(Some(Duration::new(5, 0))).expect("Failure");
				write!(stream, "NICK {} \r\n USER {} 0 * : {} \r\n", self.nick, self.nick, self.nick).expect("Connection stream-write failed.");
				println!("{}",self.nick);
				let mut stream_clone = stream.try_clone().unwrap();
				let mut connected = false;
				loop{
					let mut buf =  [0; 512];
					if stream_clone.read(&mut buf).is_ok(){
						let string = String::from_utf8(buf.to_vec()).unwrap();
						if string != ""{
							println!("{}",string);
						}
						let mut search = format!("001 {} :", self.nick);
						if !string.contains("End of /NAMES list.") && connected{
							writeln!(stream_clone, "JOIN :{}", self.channel).expect("Connection stream-write failed.");
							println!("Attempting to Join {}", self.channel);
						}
						else if connected && string.contains("End of /NAMES list."){
							let irc_stream = IrcStream { stream: stream_clone, authed: false, nick: self.nick.clone(), channel: self.channel.clone()};
							return Ok(irc_stream);
						}
						else if string.contains(&search){
							connected = true;
						}
						if string.contains(&format!("{} :Erroneous Nickname", self.nick)){
							panic!("Nickname cannot be used!!");
						}
						search = "PING".to_string();
						let mut pong = false;
						if string.contains(&search){
							let split = string.split(":");
							for s in split {
								if s.contains(&search){
									pong = true;
								}
								else if pong {
									println!("PONG :{}", s);
									writeln!(stream_clone, "PONG :{}", s).expect("Pong reply failed.");
									break;
								}
							}
						}
					}
				}
			}
			Err(_) => {
				return Err(format!("Error connecting to {} on port {}", self.server, self.port));
			}
		}
	}
}

pub struct IrcStream {
	stream: TcpStream,
	authed: bool,
	nick: String,
	channel: String,
}

impl IrcStream {
	pub fn send(&mut self, message: &str){
		for s in message.replace("\r","").split("\n"){
			writeln!(self.stream, "PRIVMSG {} :{}",self.channel, s).expect("Stream-write failed.");
		}
	}
	pub fn quit(&mut self){
		writeln!(self.stream, "QUIT :No more messages").expect("Stream-write failed.");
	}
}