use zmq::*;
use std::env::args;
type Event = String;


fn make_proxy()
{
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::ROUTER).unwrap();
    let backend = context.socket(zmq::DEALER).unwrap();
    assert!(frontend.bind("tcp://*:5559").is_ok());
    assert!(backend.bind("tcp://*:5560").is_ok());

    let items = &mut [
        frontend.as_poll_item(zmq::POLLIN),
        backend.as_poll_item(zmq::POLLIN),
    ];

    loop {
        zmq::poll(items, -1).unwrap();
        if items[0].is_readable() {
            loop {
                let message = frontend.recv_msg(0).unwrap();
                let more = if frontend.get_rcvmore().unwrap() {
                    zmq::SNDMORE
                } else {
                    0
                };
                backend.send(message, more).unwrap();
                if more == 0 {
                    break;
                };
            }
        }
        if items[1].is_readable() {
            loop {
                let message = backend.recv_msg(0).unwrap();
                let more = if backend.get_rcvmore().unwrap() {
                    zmq::SNDMORE
                } else {
                    0
                };
                frontend.send(message, more).unwrap();
                if more == 0 {
                    break;
                }
            }
        }
    }
}
fn main() {
    /*
  println!("hey yall");
  let ctx = Context::new();
  let addr = "tcp://127.0.0.1:1234";
  let mut sock = ctx.socket(SocketType::REQ).unwrap();
  let _ = sock.connect(addr);
  let payload = "Hello world!".to_string();
  println!("-> {}", payload);
 // let mut msg = Message::new(payload.len());
  let mut msg = Message::from(&payload.into_bytes());
  let _ = sock.send_msg(msg, 0);
  if let Ok(msg) = sock.recv_msg(0) {
      let contents = msg.as_str().unwrap();
      println!("<- {}", contents);
  }
      */
    make_proxy();
}
