
use std::{
    sync::{Arc, Mutex, RwLock},
};
use std::thread;
use std::collections::HashMap;
use std::time::Duration;
use protobuf::{EnumOrUnknown, Message, MessageFull};
include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
use example::{GetRequest, GetResponse};

type Arg = String;
type SafeCallback = Arc<Mutex<dyn 'static + FnMut(Arg) + Send + Sync>>;


fn SubCallback(arg: Arg)
{
    println!("Got callback for Topic A {}", arg);
}

fn SubCallbackB(arg: Arg)
{
    println!("Got callback for Topic B {}", arg);
}

fn SubCallbackC(arg: Arg)
{
    println!("Got callback for Topic C {}", arg);
}

fn callback_proto<T: MessageFull>()
{
    let desc = T::descriptor();
    println!("Register callback for proto msg {}",desc.name());
}
pub struct Service {
    pub sub_callbacks: HashMap<String, SafeCallback>,
    pub zmq_context: zmq::Context,
  //  pub zmq_subber: zmq::Socket,
}
unsafe impl Sync for Service
{

}

unsafe impl Send for Service
{

}


impl Service
{
    pub fn new() -> Self {
        let context = zmq::Context::new();

        Self {
            sub_callbacks: HashMap::new(),
            zmq_context: context,
          //  zmq_subber: subber,
        }
    }

    pub fn add_sub_callback(&mut self, topic: String, cb: SafeCallback)
    {
        self.sub_callbacks.insert(topic.clone(), cb);
    }

    pub fn start_sub_thread(&self)
    {
        let mut cb_map = self.sub_callbacks.clone();
        let mut cont = self.zmq_context.clone();
        thread::spawn(move || {
            let subber = cont.socket(zmq::SUB).unwrap();
            subber.connect("tcp://localhost:5563").expect("failed connecting subscriber");
      //      subber.set_subscribe("A".as_bytes());
            for (key, value) in cb_map.clone().into_iter() {
                println!("adding KEY {}", key);
                subber.set_subscribe(key.as_bytes());
            }

            loop {
                println!("listening");
                let envelope = subber
                    .recv_string(0)
                    .expect("failed receiving envelope")
                    .unwrap();
                let message = subber
                    .recv_string(0)
                    .expect("failed receiving message")
                    .unwrap();
                println!("[{}] {}", envelope, message);
                if let Some(cb) = cb_map.get(&envelope) {
                    (cb.lock().unwrap())(message.clone());
                }
                else
                {
                    println!("found no CALLBACK");
                }
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }
}

fn main() {
    let mut service = Service::new();

//    service.add_sub_callback(String::from("A"), Arc::new(Mutex::new(move |x| SubCallback(x))));
//    service.add_sub_callback(String::from("B"), Arc::new(Mutex::new(move |x| SubCallbackB(x))));
//    service.add_sub_callback(String::from("C"), Arc::new(Mutex::new(move |x| SubCallbackC(x))));
//    service.start_sub_thread();

    callback_proto::<GetResponse>();

    /*
    let mut service = Service::new();
    let mut s = String::from("A");
    let mut cb_map = service.sub_callbacks.clone();
    cb_map.insert(s.clone(), Arc::new(Mutex::new(move |x| SubCallback(x))));
    cb_map.insert(String::from("B"), Arc::new(Mutex::new(move |x| SubCallback2(x))));

    thread::spawn(move || {
        let context = zmq::Context::new();
        let subscriber = context.socket(zmq::SUB).unwrap();
        subscriber
            .connect("tcp://localhost:5563")
            .expect("failed connecting subscriber");
        subscriber.set_subscribe(b"A").expect("failed subscribing");
        subscriber.set_subscribe(b"B").expect("failed subscribing");
        if let Some(cb) = cb_map.get(&s) {
            (cb.lock().unwrap())("Here's a callback".to_string());
        }
        loop {
            println!("listening");
            let envelope = subscriber
                .recv_string(0)
                .expect("failed receiving envelope")
                .unwrap();
            let message = subscriber
                .recv_string(0)
                .expect("failed receiving message")
                .unwrap();
            if let Some(cb) = cb_map.get(&s) {
                (cb.lock().unwrap())(message.clone());
            }
            println!("[{}] {}", envelope, message);
        }
    });
    */
    while true {}
}
