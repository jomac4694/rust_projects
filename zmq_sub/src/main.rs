use zmq::*;
use std::thread;
fn make_sub(topic: &String)
{
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber
        .connect("tcp://localhost:5563")
        .expect("failed connecting subscriber");
    subscriber.set_subscribe(topic.as_bytes()).expect("failed subscribing");

    loop {
        let envelope = subscriber
            .recv_string(0)
            .expect("failed receiving envelope")
            .unwrap();
        let message = subscriber
            .recv_string(0)
            .expect("failed receiving message")
            .unwrap();
        println!("[{}] {}", envelope, message);
    }
}

fn make_sub2()
{
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber
        .connect("tcp://localhost:5563")
        .expect("failed connecting subscriber");
    subscriber.set_subscribe(b"A").expect("failed subscribing");

    loop {
        let envelope = subscriber
            .recv_string(0)
            .expect("failed receiving envelope")
            .unwrap();
        let message = subscriber
            .recv_string(0)
            .expect("failed receiving message")
            .unwrap();
        println!("[{}] {}", envelope, message);
    }
}

fn make_sub3()
{
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber
        .connect("tcp://localhost:5563")
        .expect("failed connecting subscriber");
    subscriber.set_subscribe(b"C").expect("failed subscribing");

    loop {
        let envelope = subscriber
            .recv_string(0)
            .expect("failed receiving envelope")
            .unwrap();
        let message = subscriber
            .recv_string(0)
            .expect("failed receiving message")
            .unwrap();
        println!("[{}] {}", envelope, message);
    }
}


fn main() {
    println!("Hello, world!");
    let v = vec!["A", "B", "C", "D", "E", "F", "G", "H"];

    for val in v.iter()
    {
        let yo = val.clone();
        thread::spawn(move || {make_sub(&yo.to_string());});
    }
    
    while true
    {

    }
}
