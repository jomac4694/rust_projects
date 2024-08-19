# Project Title

Rust ZMQ Services

## Description

A simple implementation of many-to-many pub/sub of protobuf msg's over ZMQ. The implementations utilizes XSUB/XPUB proxy as described in this page: https://netmq.readthedocs.io/en/latest/xpub-xsub/

This allows an arbitrary number of publisher/subscribers to be added to the setup, without any needing to know about the other, addressing the dynamic discovery problem. And reduces the number of "bind" calls to only the proxy XSUB and XPUB.

It is best described in this image:

![image](https://github.com/user-attachments/assets/3e4c449b-aea0-49d3-8ec0-ec78b3b98020)


For this implementation, a ZMQ Service can be both a producer and consumer.
### Executing program

* First, you must run the zmq_proxy_pubsub service, which acts as the intermediary described in the image above. Simply navigate to the directory and run:
  ```
  cargo run
  ```
* A ZMQ service can me made like so, using example from zmq_service main.rs:

```
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
```

