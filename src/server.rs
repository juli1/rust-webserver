extern crate mio;
use mio::*;
use std::str;
use std::io::Read;
use mio::net::{TcpListener};
use std::net::SocketAddr;
use std;
use client::*;
use protocol::Request;

pub struct Server {
    hostname : String,
    port : u32,
    rootdir : String
}

fn handler(client: &mut Client) {
    let mut buf : [u8; 1024] = [0;  1024];
    match client.stream.read(& mut buf) {
        Ok(n) => {
            if n > 0
            {
                let s = str::from_utf8(&buf).expect("bla");

                // Get first all the string before the line break
                let find_req = s.split("\n").collect::<Vec<&str>>();

                // Separates the request in different parts
                let parts = find_req[0].split(" ").collect::<Vec<&str>>();

                if parts.len() < 3 {
                    print!("invalid request\n");
                    return;
                }
                let req = Request::new(parts[0], parts[1], parts[2]);
                let rep = client.handle_request(&req);
                let ret = client.send_reply(&rep);
                print!("send_reply returns {:?}", ret);
            }
        },
        Err(f) => {
            print!("cannot read {:?}", f);
        }
    }
}


impl Server {
    pub fn new(h : String, p : u32, r : String) -> Self {
        Server { hostname: h, port : p, rootdir : r }
    }

    pub fn get_root(&self) -> String
    {
        self.rootdir.clone()
    }


    pub fn run(&self) -> Self {
        // build a string like "hostname:port"
        let mut connect_string : String = String::new();
        let port_str : String = self.port.to_string().to_owned();
        connect_string.push_str(&self.hostname);
        connect_string.push_str(":");
        connect_string.push_str(&port_str);

        // build the SocketAddr part so that we can connect
        let socket_addr : SocketAddr = connect_string.parse().unwrap();

        const SERVER_TOKEN : Token = Token(0);
        let server = TcpListener::bind(&socket_addr).unwrap();
        let poll = Poll::new().unwrap();
        poll.register(&server, SERVER_TOKEN, Ready::readable(),
        PollOpt::edge()).unwrap();
        let mut events = Events::with_capacity(1024);
        let mut clients: std::collections::HashMap<mio::Token, Client> = std::collections::HashMap::new();
        loop {
            poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                match event.token() {
                    SERVER_TOKEN => {
                        if clients.len() < 100
                        {
                            let (newstream, newsocket) = server.accept().unwrap();
                            let client = Client::new(newstream, newsocket, self);
                            let token = mio::Token((client.get_fd() as usize));
                            poll.register(&client, token, Ready::readable(), PollOpt::edge()).unwrap();

                            clients.insert(token, client);
                        }
                        else
                        {
                            print!("out of capacity");
                        }

                    },
                    token => {
                        // we need to do this to avoid having to
                        // borrow clients twice.
                        {
                            let client = clients.get_mut(&token).unwrap();
                            handler(client);
                            poll.deregister(client).is_ok();
                        }
                        // when we remove the client from the list, its
                        // value is dropped and the connection is closed.
                        clients.remove(&token);
                    }
                }
            }
        }

    }
}

