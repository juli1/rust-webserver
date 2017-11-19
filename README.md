# A webserver in Rust!

This is a simple webserver in rust. It uses the [mio](https://docs.rs/mio) crate
to handle the networking aspect of the code. It uses select/poll to handle
client connections.

## Disclaimer
This code was written to learn rust. **This is not safe or even ready to use in
production**. This was an educational project. I just hope it can help people
who are starting to learn rust and are looking for sample code.

You can have more details about the motification for this project
on [http://julien.gunnm.org/rust/2017/11/18/learning-rust-in-practice-webserver-in-rust/](http://julien.gunnm.org/rust/2017/11/18/learning-rust-in-practice-webserver-in-rust/])

## Compilation and usage
 
```
# git clone https://github.com/juli1/rust-webserver.git
# cd rust-webserver
# cargo build && cargo run -- -p 9065 -l 127.0.0.1 --rootdir `pwd`
```

Then, open your browser and go to
[http://localhost:9065/Cargo.toml](http://localhost:9065/Cargo.toml). You will
be able to see the file from the source directory.

## Contribute
Push requests and patches are welcome! But please fork if you want/need.

## Contact
You check out my homepage: [http://julien.gunnm.org](http://julien.gunnm.org).
