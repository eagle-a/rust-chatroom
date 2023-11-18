use std::net::TcpStream;
use std::sync::mpsc::{self};
const LOCAL_HOST: &str = "127.0.0.1:8080"; //服务端地址
const MESSAGE_SIZE: usize = 32; //缓冲区大小
struct IO_Array {

}
impl IO_Array {}
fn send_init()  {
    let mut client = TcpStream::connect(LOCAL_HOST).expect("Failed to connect"); //连接服务端

    client
        .set_nonblocking(true)
        .expect("Failed to intiate non-blocking"); //设置为非阻塞模式

    let (sender, receiver) = mpsc::channel::<String>();

    use std::io::{self};

    let mut buf = [0; MESSAGE_SIZE];
    let mut stream = io::BufReader::new(&mut client);
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let message = String::from_utf8_lossy(&buf[0..n]);
                sender.send(message.to_string()).unwrap();
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
}
