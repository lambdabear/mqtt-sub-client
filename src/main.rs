use rumqtt::{MqttClient, MqttOptions, QoS, ReconnectOptions};
use std::env;
use std::{io, io::Write};

fn main() {
    //pretty_env_logger::init();
    //let broker = "prod-mqtt-broker.atherengineering.in";
    let mut broker: String = "test.mosquitto.org".to_string();
    let mut port: u16 = 1883;

    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        broker = args[1].clone();
        port = args[2].clone().parse::<u16>().expect("invalid port");
        println!("start connecting broker {}:{}", broker, port);
    } else {
        println!(
            "need broker url and port, start connecting default broker {}:{}",
            broker, port
        );
    }

    let reconnection_options = ReconnectOptions::Always(10);
    let mqtt_options = MqttOptions::new("test-spb-msg", broker, port)
        .set_keep_alive(10)
        .set_reconnect_opts(reconnection_options)
        .set_clean_session(false);

    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    mqtt_client
        .subscribe("hello/world", QoS::AtLeastOnce)
        .unwrap();

    // thread::spawn(move || {
    //     for i in 0..100 {
    //         let payload = format!("publish {}", i);
    //         thread::sleep(Duration::from_millis(100));
    //         mqtt_client
    //             .publish("hello/world", QoS::AtLeastOnce, false, payload)
    //             .unwrap();
    //     }
    // });

    for notification in notifications {
        match notification {
            rumqtt::client::Notification::Publish(publish) => {
                io::stdout().write_all(&publish.payload).unwrap();
                print!("\n")
            }
            _ => (),
        }
    }
}
