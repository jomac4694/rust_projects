use zmq::*;
use std::time::Duration;
use std::thread;

fn make_publisher(topic: &String, msg: &String)
{
    //prepare context and publisher
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .connect("tcp://localhost:1234")
        .expect("failed binding publisher");

    loop {
        publisher
            .send(topic.as_bytes(), zmq::SNDMORE)
            .expect("failed sending first envelope");
        publisher
            .send(msg.as_bytes(), 0)
            .expect("failed sending first message");
        thread::sleep(Duration::from_millis(1000));
    }
}

fn single_publish(topic: &String, msg: &String)
{
    //prepare context and publisher
    println!("doing single send");
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .connect("tcp://localhost:1234")
        .expect("failed binding publisher");
    loop {
        publisher
            .send(topic.as_bytes(), zmq::SNDMORE)
            .expect("failed sending first envelope");
        publisher
            .send("im so confused", 0)
            .expect("failed sending first message");
    }
}

fn make_req(topic: &String)
{
    let context = zmq::Context::new();

    let requester = context.socket(zmq::REQ).unwrap();
    assert!(requester.connect("tcp://localhost:5560").is_ok());

    for request_nbr in 0..10 {
        requester.send("Hello", 0).unwrap();
        let string = requester.recv_string(0).unwrap().unwrap();
        println!("Received reply {} {}", request_nbr, string);
    }
}

fn main() {
  //  make_publisher();
    println!("Hello, world!");
    let v = vec!["A", "B", "C"]; //"C", "D", "E", "F", "G", "H"];
  //  make_publisher(&"A".to_string());
   // let a = "A";
   // let b = "B";
  //  single_publish(&a.to_string(), &b.to_string());
    for val in v.iter()
    {
        let yo = val.clone();
        thread::spawn(move || {make_publisher(&yo.to_string(), &"yeerrr".to_string());});
    }
    while true
    {

    }

 //  make_req(&"Sending the request!!".to_string());
 //   while true
 //   {
//
  //  }
}
