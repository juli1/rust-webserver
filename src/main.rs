extern crate getopts;
extern crate mio;

use getopts::Options;
use std::str;
use std::env;

mod protocol;
mod server;
mod client;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} program [options]", program);
    print!("{}", opts.usage(&brief));
}

/// Constructs a new `Rc<T>`.
///
/// # Examples
///
/// ```
/// use std::rc::Rc;
///
/// let five = Rc::new(5);
/// ```
fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output filename", "NAME");
    opts.optopt("p", "port", "set listening port", "PORT");
    opts.optopt("r", "rootdir", "set webserver root directory", "ROOTDIR");
    opts.optopt("l", "hostname", "set hostname", "HOST");
    opts.optflag("h", "help", "print this help");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m}
        Err(f) => {print!("hello"); panic!(f.to_string())}
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if ! matches.opt_present("p") {
        print!("Has to specify the port");
        return;
    }

    if ! matches.opt_present("r") {
        print!("Has to specify root dir");
        return;
    }

    if ! matches.opt_present("l") {
        print!("Has to specify host");
        return;
    }


    let port : u32 = matches.opt_str("p").unwrap().parse::<u32>().unwrap();
    let hostname : String = matches.opt_str("l").unwrap().parse::<String>().unwrap();
    let root_dir : String = matches.opt_str("r").unwrap().parse::<String>().unwrap();

    print!("Starting web server on port {}", port);

    let server = server::Server::new(hostname, port, root_dir);
    server.run();
}
