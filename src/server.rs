extern crate arsehole;
extern crate bincode;

use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

use arsehole::game::Game;
use arsehole::mp::{ClientPacket, ServerPacket};

fn main() -> std::io::Result<()> {
    let game = Arc::new(Mutex::new(Game::<String>::new()));
    let thread_game = game.clone();
    let peers: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    let thread_peers = peers.clone();
    let accept_more = Arc::new(Mutex::new(true));
    let accept_more_thread = accept_more.clone();

    let listener = TcpListener::bind("0.0.0.0:8008")?;

    thread::spawn(move || {
        let peers = thread_peers;
        let game = thread_game;
        let accept_more = accept_more_thread;

        println!("Waiting for host");
        while peers.lock().unwrap().is_empty() {
            thread::sleep(Duration::from_millis(10));
        }
        println!("Host found: {:?}", game.lock().unwrap().players);
        peers.lock().unwrap()[0].set_read_timeout(Some(Duration::from_millis(10))).unwrap();

        loop {
            match bincode::deserialize_from(&peers.lock().unwrap()[0]) {
                Ok(ClientPacket::BeginGame) => break,
                _ => (),
            }
        }
        *accept_more.lock().unwrap() = false;
        drop(accept_more);



    });

    for stream in listener.incoming() {
        if !*accept_more.lock().unwrap() {
            break
        }
        match stream {
            Ok(stream) => {
                let hello: ClientPacket = bincode::deserialize_from(&stream).unwrap();

                match hello {
                    ClientPacket::HelloIm(name) => {
                        bincode::serialize_into(&stream, &ServerPacket::Welcome).unwrap();

                        for peer in peers.lock().unwrap().iter() {
                            bincode::serialize_into(peer, &ServerPacket::NewPlayer(name.clone())).unwrap();
                        }

                        game.lock().unwrap().add_player(name);
                        peers.lock().unwrap().push(stream);
                    }
                    _ => stream.shutdown(Shutdown::Both).unwrap(),
                }

            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    Ok(())
}
