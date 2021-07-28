use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpListener;
use std::io::Read;

    pub struct Server {
        address: String,
    
    }

    fn arr(a: &[u8]){

    }
    
    impl Server {
        pub fn new(address: String) -> Self {
           Self {
               address
           }     
        }
    
        pub fn run(self){
            println!("Listening on {}", self.address);
            let listener = TcpListener::bind(&self.address).unwrap();

            loop {

                match listener.accept(){
                   Ok((mut stream,_address))=> {
                        
                        let mut buffer = [0; 1024];

                        match stream.read(&mut buffer){
                            Ok(_) => {
                                println!("Recieve the requets {}", String::from_utf8_lossy(&buffer));
                                match Request::try_from(&buffer[..] ) {
                                    Ok(reqest) => {}
                                    Err(e) => println!("failed")
                                }





                                // let res: &Result<Request, _> = &buffer[..].try_into();
                            }
                            Err(e) => {
                                println!("Falied to tread from connection {}", e);
                            }
                        }
                        //stream.read();
                   }
                   Err(_)=>{
                       println!("Failed");
                   }
                }


            }
        }
    }
