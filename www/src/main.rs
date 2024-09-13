use ::www::{ErrorOnly, ThreadPool};
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() -> ErrorOnly {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream?;

        pool.execute(|| {
            handle_connection(stream)
                .unwrap_or_else(|err| eprintln!("Error from request handler: {:?}", err));
        })?;
    }

    Ok(())
}

// Define a function to handle incoming connections.
fn handle_connection(mut stream: TcpStream) -> ErrorOnly {
    let buf_reader = BufReader::new(&stream);
    let mut http_request = Vec::new();

    // read request line by line
    for line in buf_reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        http_request.push(line);
    }

    // find longest line
    let longest_line = http_request.iter().max_by_key(|line| line.len()).unwrap();

    // log incoming request
    println!(
        "Incoming request:\n┌{}┐\n{}\n└{}┘\n",
        "─".repeat(longest_line.len() + 2),
        http_request
            .iter()
            .map(|line| format!("| {:<width$} |", line, width = longest_line.len()))
            .collect::<Vec<_>>()
            .join("\n"),
        "─".repeat(longest_line.len() + 2),
    );

    // determine response based on request
    let (status_line, response_body) = match http_request.get(0) {
        Some(line) if line.starts_with("GET / HTTP/1.1") => {
            ("HTTP/1.1 200 OK", include_str!("200.html"))
        }
        Some(line) if line.starts_with("GET /sleep HTTP/1.1") => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", include_str!("200.html"))
        }
        _ => ("HTTP/1.1 404 NOT FOUND", include_str!("404.html")),
    };

    // send response
    let response = format!(
        "{}\nContent-Length: {}\nContent-Type: text/html\n\n{}",
        status_line,
        response_body.len(),
        response_body
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
