extern crate nsq;

use nsq::config::NsqConfig;
use nsq::consumer::Consumer;
use nsq::message::{Handler, Message, MessageReply};

struct TestHandler;

impl Handler for TestHandler {
    fn handle_message(&self, message: &Message) -> MessageReply {
        println!("the message is {:?}", message.body());
        MessageReply::Fin(message.id())
    }
}

fn main() {
    let conf = NsqConfig::default();
    let mut reader = Consumer::new("test", "chan", conf);

    reader.connect_to_nsqd("127.0.0.1:4150").expect("could not connect to nsqd");
    reader.add_handler(TestHandler);
    reader.begin_consuming().expect("invalid nsqd connection");
}
