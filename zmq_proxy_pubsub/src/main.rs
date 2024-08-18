use zmq::*;
use std::env::args;
use protobuf::{EnumOrUnknown, Message};
include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
use example::GetRequest;

type Event = String;

pub trait IZmqService
{
    fn send_request(&self, msg: String)
    {
        println!("sending request");
    }
}

pub struct ZmqService
{

}

impl IZmqService for ZmqService
{

}

fn make_proxy()
{
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::XSUB).unwrap();
    let backend = context.socket(zmq::XPUB).unwrap();

    frontend
        .bind("tcp://127.0.0.1:1234")
        .expect("failed connecting frontend");
    backend
        .bind("tcp://127.0.0.1:5563")
        .expect("failed binding backend");
    zmq::proxy(&frontend, &backend).expect("failed proxying");
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
