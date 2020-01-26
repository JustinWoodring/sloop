use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_config(config : &str, default : bool) -> std::result::Result<(String, u32, String, String, String, u32), String>{
	let default_text = if default { " default" } else { "" };
	let default_file = 
"//This is the config file for dirigible.
	
//Client Configuration
//dirigible will look here when you only provide some arguments or just a message.
	
server : irc.freenode.net
port : 6667
nick : airship-bot
//FIXME: add password support, pass :
channel : #bot-testing
	
//Daemon Configuration
//dirigible will look here for settings relating to daemonizing.
	
//Listening Address
bindaddr : 127.0.0.1
	
//Listening Port
bindport : 6605
";
	
	let mut server = "".to_string();
	let mut port = 0;
	let mut nick = "".to_string();
	let mut channel = "".to_string();
	let mut bindaddr = "".to_string();
	let mut bindport = 0;
	
	if let Ok(mut config_file) = File::open(Path::new(&config)){
		let mut contents = String::new();
		if config_file.read_to_string(&mut contents).is_ok(){
			let lines = contents.split("\n");
			for line in lines {
				if !(line.contains("//") && line == ""){
					let mut lineparts = line.split(" : ");
					let part = lineparts.nth(0).unwrap_or_else(|| panic!("ERR: Could not parse {}config.", default_text));
					match part {
						"server" => {
							if let Some(cserver) = lineparts.nth(0){
								server = cserver.to_string();
							} else {
								println!("ERR: Server argument in the{} config file cannot be parsed.", default_text);
							}
						}
						"port" => {
							if let Some(cport) = lineparts.nth(0){
								if let Ok(cnport) = cport.to_string().parse::<u32>(){
									port = cnport;
								}else{
									println!("ERR: Port argument in the{} config file cannot be parsed.", default_text);
								}
							} else {
								println!("ERR: Port argument in the{} config file cannot be parsed.", default_text);
							}
						} 
						"nick" => {
							if let Some(cnick) = lineparts.nth(0){
								nick = cnick.to_string();
							} else {
								println!("ERR: Nickname argument in the{} config file cannot be parsed.", default_text);
							}
						} 
						"channel" => {
							if let Some(cchannel) = lineparts.nth(0){
								channel = cchannel.to_string();
							} else {
								println!("ERR: Channel argument in the{} config file cannot be parsed.", default_text);
							}
						} 
						"bindaddr" => {
							if let Some(cbindaddr) = lineparts.nth(0){
								bindaddr = cbindaddr.to_string();
							} else {
								println!("ERR: Bind address argument in the{} config file cannot be parsed.", default_text);
							}
						} 
						"bindport" => {
							if let Some(cbindport) = lineparts.nth(0){
								if let Ok(cnbindport) = cbindport.to_string().parse::<u32>(){
									bindport = cnbindport;
								}else{
									println!("ERR: Bind port argument in the{} config file cannot be parsed.", default_text);
								}
							} else {
								println!("ERR: Bind port argument in the{} config file cannot be parsed.", default_text);
							}
						} 
						_ => {}
					}
				}
			}
			return Ok((server,port,nick,channel,bindaddr,bindport))
		}
		else{
			if default{
				return Err("Could not open default config.".to_string());
			}else{
				return Err("Could not open specified config file. Attempting to use default config.".to_string());
			}
		}
	}else{
		if default {
			println!("INFO: Could not open default config. Attempting to create default config at {}...", config);
			if let Ok(mut file) = File::create(Path::new(&config)){
	    		if file.write_all(default_file.as_bytes()).is_ok(){
					println!("INFO: Default config creation successful");
				}
				else {
					println!("ERR: Failed to create default config.")
				}
			}
		}
		else{
			println!("ERR: Could not open config. Attempting to use default config...")
		}
		return Err("".to_string());
	}
}