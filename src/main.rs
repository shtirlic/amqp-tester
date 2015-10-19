extern crate amqp;
extern crate env_logger;
extern crate argparse;

use amqp::{Session, Table, Basic, protocol, Channel, ConsumerCallBackFn};
use argparse::{ArgumentParser, Store, Print};

fn consumer_function(channel: &mut Channel, deliver: protocol::basic::Deliver, _headers: protocol::basic::BasicProperties, _body: Vec<u8>){
    // println!("[function] Got a delivery:");
    // println!("[function] Deliver info: {:?}", deliver);
    // println!("[function] Content headers: {:?}", headers);
    // println!("[function] Content body: {:?}", body);
    // println!("[function] Content body(as string): {:?}", String::from_utf8(body));
    let _ = channel.basic_ack(deliver.delivery_tag, false);
}

fn main() {
    let mut url = "amqp://guest:guest@docker.local//".to_string();
    let mut queue_name = "test_queue".to_string();
    let mut prefetch_count = 1000;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Fetch messages form rabbitmq.");
        ap.add_option(&["-V", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()), "Show version");
        ap.refer(&mut url)
            .add_option(&["-U","--url"], Store,
            "Url to rabbitmq");
        ap.refer(&mut queue_name)
            .add_option(&["-Q","--queue"], Store,
            "Queue name");
        ap.refer(&mut prefetch_count)
            .add_option(&["-C","--count"], Store,
            "Prefetch count");
        ap.parse_args_or_exit();
    }
    env_logger::init().unwrap();
    let mut session = match Session::open_url(&url) {
        Ok(session) => session,
        Err(error) => panic!("Can't create session: {:?}", error)
    };
    println!("Connected to {:?}", url);

    let mut channel = session.open_channel(1).ok().expect("Error openning channel 1");
    println!("Openned channel: {:?}", channel.id);

    //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
    let queue_declare = channel.queue_declare(&queue_name, false, false, false, false, false, Table::new());

    println!("Queue declare: {:?}", queue_declare);

    channel.basic_prefetch(prefetch_count).ok().expect("Failed to prefetch");
    println!("Prefetch count: {:?}", prefetch_count);

    let consumer_name = channel.basic_consume(consumer_function  as ConsumerCallBackFn, &queue_name, "", false, false, false, false, Table::new());

    println!("Starting consumer {:?}", consumer_name);
    channel.start_consuming();

    channel.close(200, "Bye".to_string()).unwrap();
    session.close(200, "Good Bye".to_string());
}
