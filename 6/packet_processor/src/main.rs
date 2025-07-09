#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
#[allow(unused)]
struct PacketData {
    source_ip: String,
    destination_ip: String,
    payload: Vec<u8>,
}

#[allow(unused)]
impl PacketData {
    const MAX_PAYLOAD_SIZE: usize = 256;
    fn new(source_ip: String, destination_ip: String) -> Self {
        Self {
            source_ip,
            destination_ip,
            payload: Vec::with_capacity(Self::MAX_PAYLOAD_SIZE),
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
struct PacketAck {
    packet_id: u32,
    receiver_ip: String,
}

#[allow(unused)]
#[derive(Debug)]
struct PacketSyn {
    sender_ip: String,
    sequence_number: u32,
}

#[allow(unused)]
#[derive(Debug)]
struct PacketFin {
    sender_ip: String,
}

#[allow(unused)]
#[derive(Debug)]
enum Packet {
    Data(PacketData),
    Ack(PacketAck),
    Syn(PacketSyn),
    Fin(PacketFin),
    Error(String),
}

#[allow(unused)]
impl Packet {
    fn process_packet(&self) {
        match self {
            Packet::Data(packet_data) => {
                println!("This is a data packet.");
                // Cannot borrow `packet_data.payload` as mutable, as it is behind a `&` reference
                // Cannot modify immutable variable, we could instead receive &mut self
                // packet_data.payload.push(8u8);
                println!(
                    "source ip: {}\ndestination ip: {}\npayload: {:#?}",
                    packet_data.source_ip, packet_data.destination_ip, packet_data.payload
                );
            }
            Packet::Ack(packet_ack) => {
                println!(
                    "Ack: packet with id: {} to receiver ip: {}",
                    packet_ack.packet_id, packet_ack.receiver_ip
                );
            }
            Packet::Syn(packet_syn) => {
                println!(
                    "Syn: sender ip: {}, seq number: {}",
                    packet_syn.sender_ip, packet_syn.sequence_number
                );
            }
            Packet::Fin(packet_fin) => {
                println!("Fin: sender ip: {}", packet_fin.sender_ip);
                println!("Connection closed.");
            }
            Packet::Error(e) => {
                println!("Error: {e}");
            }
        }
    }

    fn get_sender_ip(&self) -> Option<&String> {
        match self {
            Packet::Data(data) => Some(&data.source_ip),
            Packet::Syn(data) => Some(&data.sender_ip),
            Packet::Fin(data) => Some(&data.sender_ip),
            _ => None,
        }
    }
}

#[allow(unused)]
impl Packet {
    fn process_all_packets(packets: Vec<Packet>) {
        let mut errors = 0;

        for packet in packets {
            match packet {
                Packet::Error(_) => {
                    errors += 1;
                }
                other => other.process_packet(),
            }
        }

        println!("Total Errors: {errors}");
    }
}

fn main() {
    let packet = Packet::Data(PacketData::new(
        String::from("127.8.6.12"),
        String::from("123.8.2.45"),
    ));

    packet.process_packet();
    if let Some(sender_ip) = packet.get_sender_ip() {
        println!("Sender IP: {sender_ip}");
    } else {
        println!("No sender IP could be found");
    }

    let packet_err_1 = Packet::Error(String::from("Couldn't read the data!!!"));
    let packets = vec![packet, packet_err_1];

    Packet::process_all_packets(packets);
    // borrow of moved value: `packets`
    // Because process_all_packets took owner ship and dropped these packets
    // println!("{packets:#?}");
    // println!("{packet:#?}");
}

// Conceptual Questions
/*
    Enum vs. Struct
    I would chose an enum when I am faced with finite number of possibilities that I can exhaust without needing specific data structure.
    I would use enum when I need a way of grouping some data together
    Let's say I need to keep track of the reptiles in my terrarium (oddly specific, isn't it?) and I have snakes with snake genome name, turtles and lizards, 
    In this case I have no use for a struct, I would use an enum with Snake(String), Turtle(String) and Lizard(String), this wat I can count them easily or perform some actions on groups of them.

    Option<T> and null
    Coming from Java, I can tell how exhausting is to catch everything and guard against a null.
    Option allows us to use data if data exists, otherwise, we have None, which is not null, but an Optional type.
    This makes handling the absence of data much easier.
    Rust does not have null due to the safety and agronomics of the language and to prevent serious runtime errors.

    match Exhaustiveness
    Rust match ensures before compile time, that all the enum's options are covered, thus preventing unwanted or undefined behavior.

    if let Benefits
    If we have an optional and need to perform some action only if data exists, or we have an enum bu need only 1 specific outcome, we could write less verbose code use if let Some(val) = a ...
    
*/
