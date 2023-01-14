use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use rust_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //error is not handled here; if error occurs, the application will panic.

    //multi-thread & concurrency: fixed number of threads in a thread pool - fixed number to avoid attacks

    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
            println!("Connection established");
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    //to read data from TcpStream & print it out
    //first, create a buffer to hold the data
    let mut buffer = [0; 1024]; //1024=size of the data which the buffer will hold

    stream.read(&mut buffer).unwrap();

    //REQUEST

    let get = b"GET / HTTP/1.1\r\n"; //b returns bytes array representing the string

    let sleep = b"GET /sleep HTTP/1.1\r\n"; //this will simulate a request that's taking long to process

    //verify the request is as expected, otherwise return 404 page
    let (status_line, filename) = 
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) { 
        thread::sleep(Duration::from_secs(5)); //let the thread sleeps for 5 secs
        ("HTTP/1.1 200 OK", "index.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    //RESPONSE

    //structure of a response:
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    // eg.: HTTP/1.1 200 OK\r\n\r\n

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!(
        "Request: {}",
        String::from_utf8_lossy(&buffer[..]) //converts slice of bytes to a string incl. invalid characters
    )
}
