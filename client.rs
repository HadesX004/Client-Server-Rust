use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::io::stdin;


fn main(){
    let mut buffer = [0; 1024];
    let mut data = String::new();

    match TcpStream::connect("127.0.0.1:3333"){
        Ok(mut stream) => {
            loop{
                
                stdin().read_line(&mut data);

                stream.write(&mut data.as_bytes());

                match stream.read(&mut buffer){
                    Ok(response) => {
                        println!("{}", from_utf8(&buffer[0..response]).unwrap());

                    },

                    Err(e) => {
                        println!("Error on received the information");
                        println!("{}", e);

                    },
                }
            }
        },
        
        Err(e) => {
            println!("Error on connection !");
            println!("{}", e);
        },
    }
}