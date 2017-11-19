extern crate mio;
use std::fs::File;
use std::str;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::metadata;
use server::Server;
use protocol::Request;
use protocol::Reply;
use protocol::HttpMethod;
use std::os::unix::io::{AsRawFd, RawFd};
use std::io::Write;
use std::io::Read;
use std;


pub struct Client<'a> {
    pub stream: mio::net::TcpStream,
    addr: std::net::SocketAddr,
    server: &'a Server,
}

impl<'a> Client<'a> {
    pub fn new(stream: mio::net::TcpStream, addr: std::net::SocketAddr, serv: &'a Server) -> Self {
        Client { stream: stream, addr: addr, server: serv }
    }
 
    pub fn get_fd(&self) -> RawFd {
        self.stream.as_raw_fd()
    }

    pub fn send_reply(&mut self, reply: &Reply) {
        print!("Client {} send reply\n", self.addr);
        match self.stream.write(reply.get_preamble().as_bytes())
        {
            Ok(n) => {
                print!("send preamble successful {:?}", n);
            },
            Err(n) => {
                print!("error sending preamble {:?}", n);
            }
        }
        match self.stream.write(&reply.content)
        {
            Ok(n) => {
                print!("send content successful {:?}", n);
            },
            Err(n) => {
                print!("error content preamble {:?}", n);
            }
        }


    }

    pub fn handle_request (&self, req: &Request) -> Reply {
        let mut rep = Reply::new_from_request(req);
        match req.get_method()
        {
            HttpMethod::GET =>
            {
                let mut filename = self.server.get_root();
                filename.push_str(req.get_resource());
                if metadata(&filename).unwrap().is_dir() {
                    filename.push_str("/index.html");
                }
                print!("Handle a GET to get resource {}\n", filename);
                let mut file = match File::open(&filename) {
                    Ok(f) => f,
                    Err(e) =>
                    {
                        print!("Cannot open {} error {}", filename, e);
                        rep.set_code(404);
                        return rep;
                    }
                };

                // We assume we have the file and we put the content
                // in the reply.
                rep.set_code(200);

                let mut slices : Vec<&str> = filename.split('.').collect();
                let extension = slices.pop().unwrap();
                print!("EXTENSION for file {}: {}\n", filename, extension);

                if extension == "html" || extension == "css"
                {
                    let mut content = String::new();
                    let buf_reader = BufReader::new(&file);
                    for line in buf_reader.lines()
                    {
                        let l = line.unwrap();
                        content.push_str(l.as_str());
                    }
                    let mut c = &mut content.into_bytes();
                    rep.content.append(c);
                }
                else
                {
                    match file.read_to_end(&mut rep.content)
                    {
                        Ok(n) =>
                        {
                            print!("successfully read the file {}", n);
                        },
                        Err(n) =>
                        {
                            print!("error while reading the file {}", n);
                        }

                    }
                }
                print!("Done reading resource {}\n", filename);
            },
            _ =>
            {
                print!("Not handled at that time");
            }
        }

        return rep;
    }

}

impl<'a> mio::Evented for Client<'a> {
    fn register(&self, poll: &mio::Poll, token: mio::Token, interest: mio::Ready, opts: mio::PollOpt)
        -> std::io::Result<()>
    {
        self.stream.register(poll, token, interest, opts)
    }
 
    fn reregister(&self, poll: &mio::Poll, token: mio::Token, interest: mio::Ready, opts: mio::PollOpt)
        -> std::io::Result<()>
    {
        self.stream.reregister(poll, token, interest, opts)
    }
 
    fn deregister(&self, poll: &mio::Poll)
        -> std::io::Result<()>
    {
        print!("Unregister");
        self.stream.deregister(poll)
    }
}



