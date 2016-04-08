use std::io::Write;
use std::net::TcpListener;
use std::thread;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::str;

pub fn listen_tcp(addr: &str, text: Arc<Mutex<String>>) {
    let listener = TcpListener::bind(addr).unwrap();
    println!("listening started, ready to accept");
    let mut i = 0;
    for stream in listener.incoming() {
        i += 1;
        println!("got a stream");
        let mut stream = stream.unwrap();
        let text = text.clone();
        let i = i;
        thread::spawn(move || {
            let _ = stream.write(b"Hello World\n");
            let buf = &mut [0; 128];
            while let Ok(n) = stream.read(buf) {
                if n == 0 {
                    continue
                }

                let mut string = text.lock().unwrap();
                string.push_str(format!("{}: {}", i, str::from_utf8(&buf[0..n]).unwrap()).as_str());
            }
            // TODO: not being called =( would like to handle connection terminated
            println!("Connection terminated");
        });
    }
}
