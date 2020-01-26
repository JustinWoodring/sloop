mod parsefile;
use parsefile::read_config;

pub struct Arguments {
	config : String,
	channel : String,
	daemon : bool,
	message : String,
	nick : String,
	port : u32,
	pass : String,
	server : String,
	bindaddr : String,
	bindport : u32,
	die : bool,
}

impl Arguments{
	pub fn parse(mut args : Vec<String>) -> Self{
		
		//Define defaults
		
		
		let mut local_config = "".to_string();
		if cfg!(windows) { //Check if we are compiling for windows or linux
			let home = std::env::var("%HOMEDRIVE%%HOMEPATH%");
			match home{
				Ok(home)=> {local_config = home+&"\\.sloop.conf".to_string();}
				Err(_) => {println!("Could not find user home directory. Using current directory."); local_config = ".dirigible.conf".to_string();}
			}
		} else if cfg!(unix) {
			let home = std::env::var("HOME");
			match home{
				Ok(home)=> {local_config = home+&"/.sloop.conf".to_string();}
				Err(_) => {println!("Could not find user home directory. Using current directory."); local_config = ".dirigible.conf".to_string();}
			}
		}
		
		let mut local_channel = "".to_string();
		let mut local_daemon = false;
		let mut local_message = "".to_string();
		let mut local_nick = "".to_string();
		let mut local_port = 0;
		let mut local_pass = "".to_string();
		let mut local_server = "".to_string();
		let mut local_bindaddr = "".to_string();
		let mut local_bindport = 0;
		let mut local_die = false;
		
		//Attempt to read default config.
		let read_default_result = read_config(&local_config, true);
		match read_default_result{
			Ok((server,port,nick,channel,bindaddr,bindport)) => { 
				local_server = server;
				local_port = port;
				local_nick = nick;
				local_channel = channel;
				local_bindaddr = bindaddr;
				local_bindport = bindport;
			}
			Err(string) => {println!("{}",string);local_die = true;} 
		}
		//Define all possible argument indicators
		//"-c", "-C", "-d", "-m", "-n", "-p", "-a", "-h", "-s", "-x", "-z"
		
		//Get number of arguments
		let num_args = args.iter().count();
		
		if num_args < 2{	
			local_die=true;
		}
		
		for arg in 0..num_args{
			match args.get(arg).expect("Out of bounds on arguments array.").as_str() {
				"-c" => {
					if !(arg > num_args){
						local_config = args.get(arg+1).expect("ERR: Out of bounds retreiving arg").to_string();
						let read_result = read_config(&local_config, false);
						match read_result{
							Ok((server,port,nick,channel,bindaddr,bindport)) => { 
								local_server = server;
								local_port = port;
								local_nick = nick;
								local_channel = channel;
								local_bindaddr = bindaddr;
								local_bindport = bindport;
							}
							Err(string) => {println!("{}",string);local_die = true;} 
						} 
					} else{
						local_die = true;
					}		
				}
				"-C" => {
					if !(arg > num_args){
						local_channel = args.get(arg+1).expect("ERR: Out of bounds retreiving arg").to_string();
					} else{
						local_die = true;
					}	
				}
				"-d" => {
						local_daemon = true;	
				}
				"-m" => {
					if !(arg > num_args){
						local_message = args.get(arg+1).expect("ERR: Out of bounds retreiving arg").to_string();
					} else{
						local_die = true;
					}		
				}
				"-n" => {
					if !(arg > num_args){
						local_nick = args.get(arg+1).expect("ERR: Out of bounds retreiving arg").to_string();
					} else{
						local_die = true;
					}		
				}
				"-p" => {
					if !(arg > num_args){
						local_port = args.get(arg+1).expect("ERR: Out of bounds retreiving arg").to_string().parse::<u32>().expect("Could not parse port!");
					} else{
						local_die = true;
					}		
				}
				"-a" => {
					if !(arg > num_args){
						local_pass = args.get(arg+1).expect("ERR: Out of bounds retreiving arg").to_string();
					} else{
						local_die = true;
					}		
				}
				"-h" => {
						local_die = true;	
				}
				"-s" => {
					if !(arg > num_args){
						local_server = args.get(arg+1).expect("ERR: Out of bounds retreiving arg").to_string();
					} else{
						local_die = true;
					}		
				}
				"-x" => {
					if !(arg > num_args){
						local_bindaddr = args.get(arg+1).expect("ERR: Out of bounds retreiving arg").to_string();
					} else{
						local_die = true;
					}		
				}
				"-z" => {
					if !(arg > num_args){
						local_bindport = args.get(arg+1).expect("ERR: Out of bounds retreiving arg").to_string().parse::<u32>().expect("ERR: Could not parse port!");
					} else{
						local_die = true;
					}		
				}
				_ => {
				}
			}
		}
		println!("\nPRINTING RUNTIME ARGUMENTS\nConfig: {}\nServer: {}\nPort: {}\nChannel: {}\nNick: {}\nMessage: {}\n---ONLY APPLIES IF DAEMONIZED---\nDaemon: {}\nBind Address: {}\nBind Port: {}\nEND OF ARGUMENTS\n\n", local_config,local_server,local_port,local_channel,local_nick,local_message,local_daemon,local_bindaddr,local_port);
		if local_server == "" || local_message == "" || local_port == 0 || local_channel == "" || local_nick == ""{
			local_die = true;
		}
		if local_daemon {
			if local_bindaddr == ""{
				local_die = true;
			}
			if local_bindport == 0{
				local_die = true;
			}
		}
		if local_die{
			print_help();
		}
		Arguments {
			config : local_config,
			channel : local_channel,
			daemon : local_daemon,
			message : local_message,
			nick : local_nick,
			port : local_port,
			pass : local_pass,
			server : local_server,
			bindaddr : local_bindaddr,
			bindport : local_bindport,
			die : local_die,
		}
	}
	
	pub fn is_daemon(&self) -> bool{
		return self.daemon;
	}
	
	pub fn should_quit(&self) -> bool{
		return self.die;
	}
	
	pub fn get_server(&self) -> &str{
		return self.server.as_str();
	}
	
	pub fn get_port(&self) -> u32{
		return self.port;
	}
	
	pub fn get_channel(&self) -> &str{
		return self.channel.as_str();
	}
	
	pub fn get_nick(&self) -> &str{
		return self.nick.as_str();
	}
	
	pub fn get_pass(&self) -> &str{
		return self.pass.as_str();
	}
	
	pub fn get_message(&self) -> &str{
		return self.message.as_str();
	}
	
	pub fn get_bind_addr(&self) -> &str{
		return self.bindaddr.as_str();
	}
	
	pub fn get_bind_port(&self) -> u32{
		return self.bindport;
	}
}

fn print_help(){
	let help = vec![
		"HELP:",
		"-a: Specify a plain-text password for SASL auth. [NOT AVAILABLE YET]", 
		"-c: Specify an alternate config file to be used.",
		"-C: Specify the irc channel to be entered after connection.", 
		"-d: Daemonize the session to allow the dirigible to receive HTTP post requests. [NOT AVAILABLE YET]",
		"-h: Print this help.",
		"-m: Specify the message to be sent.",
		"-n: Specify a nickname for this bot when it connects to the irc network.",
		"-p: Specify the port to be used when connecting to the server.",
		"-s: Specify the server to which dirigible should connect.",
		"-x: Set the address to listen to HTTP requests on when daemonized.",
		"-z: Set the port to listen to HTTP requests on when daemonized.",
	];
	for line in help {
		println!("{}",line);
	}
}
