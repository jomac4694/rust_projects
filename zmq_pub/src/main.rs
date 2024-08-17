use zmq::*;
use std::time::Duration;
use std::thread;
fn make_publisher(topic: &String)
{
    //prepare context and publisher
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .connect("tcp://localhost:1234")
        .expect("failed binding publisher");

    loop {
        publisher
            .send(topic, zmq::SNDMORE)
            .expect("failed sending first envelope");
        publisher
            .send("We don't want to see this", 0)
            .expect("failed sending first message");
        thread::sleep(Duration::from_millis(10));
    }
}

fn make_publisher2()
{
    //prepare context and publisher
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .connect("tcp://localhost:1234")
        .expect("failed binding publisher");

    loop {
        publisher
            .send("C", zmq::SNDMORE)
            .expect("failed sending second envelope");
        publisher
            .send("C Message is coming in", 0)
            .expect("failed sending second message");
        thread::sleep(Duration::from_millis(1000));
    }
}
fn main() {
  //  make_publisher();
    println!("Hello, world!");
    let v = vec!["A", "B", "C", "D", "E", "F", "G", "H"];

    for val in v.iter()
    {
        let yo = val.clone();
        thread::spawn(move || {make_publisher(&yo.to_string());});
    }
    while true
    {

    }
}
