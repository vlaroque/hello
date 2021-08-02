use std::env;
use std::io;
use std::net::UdpSocket;
use std::string::String;
use std::str;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let send_to_port = match args.get(1) {
        Some(data) => String::from(data),
        None => String::from("0"),
    };
    let recv_from_port = match args.get(2) {
        Some(data) => String::from(data),
        None => String::from("0"),
    };

    println!("Args are send to :{} and receive from :{}", send_to_port, recv_from_port);

    //let src_ip_string = format!("{}:{}", "127.0.0.42", recv_from_port);
    let src_ip_string = recv_from_port;
    let src_socket = UdpSocket::bind(&src_ip_string).expect("Couldn't bind to adress");
    let clonned_src_socket = src_socket.try_clone().expect("cannot clone");
    //let dst_ip_string = format!("{}:{}", "127.0.0.42", send_to_port);
    let dst_ip_string =  send_to_port;

    let handle = thread::spawn( move || {
        // code to receive and show 
        for i in 1..10 {
            println!("i = {}", i);
            thread::sleep(Duration::from_millis(10));
        }
        println!("receiver initialized!");
        loop {
            let mut received_buff = vec![0; 100];
            let (n, src_adress) = clonned_src_socket.recv_from(&mut received_buff).expect("nothing received");
            let resp_str = str::from_utf8(&received_buff[0..n]);
            println!("from {}[{}]=>{}", src_adress, n, resp_str.unwrap());
        }
    });

    loop {
        let mut msg_to_send = String::new();
        io::stdin()
            .read_line(&mut msg_to_send)
            .expect("Failed to read line");
        let end_msg = String::from("bye\n");
        if msg_to_send.eq(&end_msg) {
            println!("exits");
            return;
        }
        println!("You : {}", msg_to_send);

        src_socket.send_to(&msg_to_send.into_bytes(), &dst_ip_string).expect("send failed");
    }
    //handle.join().unwrap();
}
