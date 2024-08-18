# Project Title

Rust ZMQ Services

## Description

A simple implementation of many-to-many pub/sub of protobuf msg's over ZMQ. The implementations utilizes XSUB/XPUB proxy as described in this page: https://netmq.readthedocs.io/en/latest/xpub-xsub/

This allows an arbitrary number of publisher/subscribers to be added to the setup, without any needing to know about the other, addressing the dynamic discovery problem. And reduces the number of "bind" calls to only the proxy XSUB and XPUB.

It is best described in this image:

![image](https://github.com/user-attachments/assets/3e4c449b-aea0-49d3-8ec0-ec78b3b98020)


For this implementation, a ZMQ Service can be both a producer and consumer.
### Executing program

* A ZMQ service can me made like so:
* This will register Callback functions for the Protobuf message passed as template argument

```
    let mut service = Service::new();
    service.add_sub_callback::<GetResponse>(Arc::new(Mutex::new(move |x| SubCallback(x))));
    service.add_sub_callback::<GetRequest>(Arc::new(Mutex::new(move |x| SubCallbackB(x))));
    service.start_sub_threead();
```

