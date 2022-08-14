use std::net::{TcpListener};
use std::io::{Read, Write};
use std::io::stdin;
use std::str::from_utf8;

fn main(){

    let server = TcpListener::bind("0.0.0.0:3333");
    let mut buffer = [0; 1024];

    match server {
        Ok(server) => {
            for client in server.incoming(){
                match client {
                    Ok(mut client) => {  // <- se nn for usar thread tem quer ter MUT>
                        println!("New Client: {}", client.peer_addr().unwrap());

                        loop{
                            let mut data = client.read(&mut buffer).unwrap();
                            println!("{}", from_utf8(&buffer[0..data]).unwrap());

                            let mut response = String::new();
                            stdin().read_line(&mut response);

                            client.write(&mut response.as_bytes()); 

                        }               
                    
                    },

                    Err(_) => {
                        println!("Error to receive client !");

                    },
                }
            }
        },

        Err(_) => {
            println!("Error on create the server !");

        }
    }
}