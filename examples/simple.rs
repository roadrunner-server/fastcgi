use fastcgi::{RequestResult, Requests};

fn main() {
    let addr = "0.0.0.0:3000";
    let listener = std::net::TcpListener::bind(addr).unwrap();

    loop {
        let connection = listener.accept();

        match connection {
            Err(err) => {
                println!("Establishing connection failed: {}", err);
                break;
            }
            Ok((stream, address)) => {
                dbg!("Connection from {}", address);
                dbg!(stream.try_clone().expect("failed"));
                let mut request = Requests::from_split_socket(
                    (
                        stream.try_clone().expect("failed"),
                        stream.try_clone().expect("failed"),
                    ),
                    1,
                    1,
                );

                if let Ok(Some(rr)) = request.read_next_request() {
                    if let Err(err) = rr.process(|reqq| {
                        if let Some(uri) = reqq.get_str_param("REQUEST_URI") {
                            dbg!(uri);
                        }
                        reqq.get_stdout().write(b"Status: 200 OK\r\n").unwrap();
                        reqq.get_stdout().write(b"\r\n").unwrap();

                        RequestResult::Complete(0)
                    }){
                        eprint!("{err}");
                    }
                }
            }
        }
    }
}
