
use std::{
    sync::{Arc, Mutex, RwLock},
};
use std::thread;
use std::collections::HashMap;
use std::time::Duration;
use protobuf::{EnumOrUnknown, Message, MessageFull};
include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
use example::{TestMsg, TestMsg2, TestMsg3};
use std::sync::mpsc::{channel, Sender, Receiver};
type Arg = Vec<u8>;
type SafeCallback = Arc<Mutex<dyn 'static + FnMut(Arg) + Send + Sync>>;

fn TestMsgCB(arg: Arg)
{
    let msg = TestMsg::parse_from_bytes(&arg).unwrap();

    println!("Got Response with name {}", msg.str_val);
    println!("Got Response with age {}", msg.int_val);
}

fn TestMsg2CB(arg: Arg)
{
    let msg = TestMsg2::parse_from_bytes(&arg).unwrap();

    println!("Got Response with name {}", msg.name);
    println!("Got Response with age {}", msg.age);
}

fn TestMsg3CB(arg: Arg)
{
    let msg = TestMsg3::parse_from_bytes(&arg).unwrap();

    println!("Got Response with address {}", msg.address);
    println!("Got Response with zipcode {}", msg.zipcode);
}

pub struct Service {
    pub name: String,
    pub sub_callbacks: HashMap<String, SafeCallback>,
    pub zmq_context: zmq::Context,
    pub sender: Arc<Sender<Vec<u8>>>,
    pub receiver: Arc<Mutex<Receiver<Vec<u8>>>>,
    pub publisher: zmq::Socket,
}

unsafe impl Sync for Service
{
}

unsafe impl Send for Service
{
}

// This represents a ZMQ Service capable of publishing and subscribing to arbtirary Proto messages
impl Service
{
    pub fn new(name: String) -> Self {
        let context = zmq::Context::new();
        let sock = context.socket(zmq::PUB).unwrap();
        sock.connect("tcp://localhost:1234").expect("could not connect to publisher");
        thread::sleep(Duration::from_millis(1000));
        let (send, receive) = channel();
        Self {
            name: name,
            sub_callbacks: HashMap::new(),
            zmq_context: context,
            sender: Arc::new(send),
            receiver: Arc::new(Mutex::new(receive)),
            publisher: sock,
        }
    }

    // Add a callback for some Protobuf message
    pub fn add_sub_callback<T: MessageFull>(&mut self, cb: SafeCallback)
    {
        let desc = T::descriptor();
        let name = desc.name();
        self.sub_callbacks.insert(name.to_string(), cb);
    }

    // Publish any Protobuf message
    pub fn publish_msg<T: MessageFull>(&self, msg: T)
    {
        let desc = T::descriptor();
        let topic = desc.name();
        println!("PUBLISHING TOPIC {}", topic);
        self.sender.send(topic.to_string().into_bytes()).unwrap();
        self.sender.send(msg.write_to_bytes().unwrap()).unwrap();
    }

    // Publishing thread. Uses channels to receive messages and publish them using ZMQ socket
    pub fn start_pub_thread(&self)
    {
        let mut cont = self.zmq_context.clone();
        let mut rec = self.receiver.clone();
        thread::spawn(move || {
            let pubber = cont.socket(zmq::PUB).unwrap();
            pubber.connect("tcp://localhost:1234");
            thread::sleep(Duration::from_millis(25));
            loop
            {
                let topic = rec.lock().unwrap().recv().unwrap();
                let res = rec.lock().unwrap().recv().unwrap();
                pubber
                .send(topic, zmq::SNDMORE)
                .expect("failed sending first envelope");
                pubber
                .send(res, 0)
                .expect("failed sending first message"); 
            }
        });
    }
    // The subscriber thread simply waits for incoming Proto Msgs (from proxy server)
    // Then calls the associated callback function, if it exists
    pub fn start_sub_thread(&self)
    {
        let mut cb_map = self.sub_callbacks.clone();
        let mut cont = self.zmq_context.clone();
        let n = self.name.clone();
        thread::spawn(move || {
            let subber = cont.socket(zmq::SUB).unwrap();
            subber.connect("tcp://localhost:5563").expect("failed connecting subscriber");
            for (key, value) in cb_map.clone().into_iter() {
                println!("adding KEY {}", key);
                subber.set_subscribe(key.as_bytes());
            }
            loop {
                println!("listening");
                let envelope = subber
                    .recv_bytes(0)
                    .expect("failed receiving envelope");
                let topic = String::from_utf8(envelope).unwrap();
                println!("GOT TOPIC {}", topic);
                let message = subber
                    .recv_bytes(0)
                    .expect("failed receiving message");
                println!("{} GOT A MSG", n);
                if let Some(cb) = cb_map.get(&topic) {
                    (cb.lock().unwrap())(message.clone());
                }
                else
                {
                    println!("found no CALLBACK");
                }
            }
        });
    }

    pub fn Start(&self)
    {
        self.start_pub_thread();
        self.start_sub_thread();
        // Give em a bit to initialize
        thread::sleep(Duration::from_millis(30));
    }
}

// Publish n amount of messages
fn send_n<T: MessageFull>(srv: &Service, msg: T, n: i32)
{
    for i in 0..n
    {
        srv.publish_msg::<T>(msg.clone());
    }  
}

fn main() {
    let mut service = Service::new(String::from("Service_1"));

    // Add any number of subscription callbacks
    service.add_sub_callback::<TestMsg3>(Arc::new(Mutex::new(move |x| TestMsg3CB(x))));
    service.add_sub_callback::<TestMsg>(Arc::new(Mutex::new(move |x| TestMsgCB(x))));
    service.Start();

    // Prepare a proto msg for publishing, this is received by the service itself
    let mut resp = TestMsg3::new();
    resp.address = String::from("1616 Random Street");
    resp.zipcode = 8080;
    let to_send = 1;

    send_n::<TestMsg3>(&service, resp.clone(), to_send);

    // Prepare a proto msg for publishing, this should be ignored by Service1 subscriber in this setup
    let mut req = TestMsg2::new();
    req.name = String::from("Joseph");
    req.age = 30;

    send_n::<TestMsg2>(&service, req.clone(), to_send);

    // Run a second service instance because why not, sub to TestMsg2 msg
    let mut service2 = Service::new(String::from("Service_2"));
    service2.add_sub_callback::<TestMsg2>(Arc::new(Mutex::new(move |x| TestMsg2CB(x))));
    service2.Start();

    // Send from Service1 to Service2
    req.name = String::from("TestMsg2 from Service 1");
    req.age = 30;
    send_n::<TestMsg2>(&service, req.clone(), 5);


    // Send from Service2 to Service1
    resp.address = String::from("TestMsg3 from Service 2");
    resp.zipcode = 1000;

    send_n::<TestMsg3>(&service2, resp.clone(), to_send);

    // Send from Service2 to Service1
    let mut test = TestMsg::new();
    test.str_val = String::from("TestMsg from Service 2");
    test.int_val = 123;

    send_n::<TestMsg>(&service2, test.clone(), to_send);

    while true {}
}
