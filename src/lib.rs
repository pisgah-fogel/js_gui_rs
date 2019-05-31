extern crate websocket;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

mod json;
use crate::json::Jsonify;

pub struct JsGui {
    main_to_server_tx: std::sync::mpsc::Sender<OwnedMessage>,
    server_to_main_rx: std::sync::mpsc::Receiver<OwnedMessage>,
}

// Chart.js dataset
pub struct Dataset<V> {
    pub label: String,
    pub data: std::vec::Vec<V>,
    pub fill: bool,
    pub line_tension: f32,
}

// Chart.js structure
pub struct Chart<T, V> {
    pub type_: String, //default: line
    pub labels : std::vec::Vec<T>,
    pub datasets : std::vec::Vec<Dataset<V>>,
}

impl JsGui {
    pub fn new(address: &str) -> JsGui {

        let (server_to_main_tx, server_to_main_rx) = std::sync::mpsc::channel();
        let (main_to_server_tx, main_to_server_rx) = std::sync::mpsc::channel();

        let server = Server::bind(address).unwrap();
        // spawn a thread for non blocking connection
        thread::spawn(move || {
            println!("Ready to accept one client");

            if let Some(request) = server.filter_map(Result::ok).next() {
                // client reception to server channel
                if !request.protocols().contains(&"rust-websocket".to_string()) {
                    request.reject().unwrap();
                    return;
                }

                let client = request.use_protocol("rust-websocket").accept().unwrap();

                let ip = client.peer_addr().unwrap();

                println!("Connection from {}", ip);

                let (mut receiver, mut sender) = client.split().unwrap();

                let (recv_tx, server_rx) = std::sync::mpsc::channel();

                // client's recv thread to client's thread channel
                // spawn thread to recev
                thread::spawn(move || for message in receiver.incoming_messages() {
                    let message = message.unwrap();
                    recv_tx.send(message).unwrap();
                });

                // server main loop
                // handle send command comming from the main app
                loop {
                    if let Ok(message) = server_rx.try_recv() {
                        // kinf of non-blockinf reception
                        match message {
                            OwnedMessage::Close(_) => {
                                let message = OwnedMessage::Close(None);
                                sender.send_message(&message).unwrap();
                                println!("Client {} disconnected", ip);
                                return;
                            }
                            OwnedMessage::Ping(_ping) => {
                                let message = OwnedMessage::Pong(_ping);
                                sender.send_message(&message).unwrap();
                                println!("Ping reveived");
                            }
                            _ => server_to_main_tx.send(message).unwrap(),
                        }
                    }
                    if let Ok(message) = main_to_server_rx.try_recv() {
                        sender.send_message(&message).unwrap();
                    }
                }
            }
            println!("[v] Websocket server done");
        });
        return JsGui {
            main_to_server_tx: main_to_server_tx,
            server_to_main_rx: server_to_main_rx,
        };
    }
    fn send(&self, cmd: String) -> bool {
        self.main_to_server_tx.send(OwnedMessage::Text(cmd)).is_ok()
    }
    pub fn receive(&self) -> Option<String> {
        if let Ok(message) = self.server_to_main_rx.try_recv() {
            match message {
                OwnedMessage::Text(txt) => return Some(txt),
                _ => return Some(String::new())
            }
        } else {
            return None;
        }
    }
    pub fn draw_rect(&self, x: i32, y:i32, w: i32, h:i32) {
        let mut buf = String::new_json();
        buf.append_str("type", "rec");
        buf.append_number("x", &x);
        buf.append_number("y", &y);
        buf.append_number("w", &w);
        buf.append_number("h", &h);
        self.send(buf);
    }
    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.send(format!("{{\"type\":\"line\",\"x1\":{},\"y1\":{},\"x2\":{},\"y2\":{}}}", x1, y1, x2 ,y2));
    }
    pub fn draw_arc(&self, x: i32, y: i32, r: u32, start_angle: f32, end_angle: f32) {
        self.send(format!("{{\"type\":\"arc\",\"x\":{},\"y\":{},\"r\":{},\"s\":{},\"e\":{}}}", x, y, r, start_angle, end_angle));
    }
    pub fn draw_text(&self, x: i32, y: i32, text: &str, font: &str) {
        self.send(format!("{{\"type\":\"text\",\"x\":{},\"y\":{},\"text\":\"{}\",\"font\":\"{}\"}}", x, y, text, font));
    }
    pub fn set_fill_style(&self, color: &str) {
        self.send(format!("{{\"type\":\"color\",\"color\":\"{}\"}}", color));
    }
    pub fn clear(&self) {
        self.send(format!("{{\"type\":\"clear\"}}"));
    }

    pub fn draw_chart<T: std::string::ToString, V: std::string::ToString>(&self, chart: &Chart<T, V>) {
        let mut cmd = String::new();
        cmd.push_str("{\"type\":\"chart\",\"data\":"); // wrapped
        cmd.push('{');
        // type
        cmd.push_str("\"type\":\"");
        cmd.push_str(chart.type_.as_str());
        cmd.push_str("\",");
        // data
        cmd.push_str("\"data\":{");
        //data.labels
        cmd.push_str("\"labels\":[");
        for iter in chart.labels.iter() {
            cmd.push('"');
            cmd.push_str(iter.to_string().as_str());
            cmd.push('"');
            cmd.push(',');
        }
        cmd.pop(); // remove last '"'
        cmd.push_str("],"); // close label
        //datasets
        cmd.push_str("\"datasets\":[");
        for dataset in chart.datasets.iter() {
        cmd.push('{');
        //data.datasets.label
        cmd.push_str("\"label\":\"");
        cmd.push_str(dataset.label.as_str());
        cmd.push_str("\","); // close label
        cmd.push_str("\"data\":[");
        for iter in dataset.data.iter() {
            cmd.push_str(iter.to_string().as_str());
            cmd.push(',');
        }
        cmd.pop(); // remove last '"'
        cmd.push_str("],");
        cmd.push_str("\"fill\":false,"); // TODO
        // TODO borderColor
        cmd.push_str("\"lineTension\":");
        cmd.push_str(dataset.line_tension.to_string().as_str());
        cmd.push_str("},"); // close dataset
        }
        cmd.pop(); // remove last '"'
        cmd.push(']'); // close dataset collection
        //iterate for datas
        cmd.push_str("},"); // close data
        // options
        cmd.push_str("\"options\":{}");
        cmd.push('}');
        cmd.push_str("}"); // close wrapper
        self.send(cmd);
    }
}

pub fn print_link() {
    let srcdir = std::path::PathBuf::from("frontend/demo.html");
    let path = std::fs::canonicalize(&srcdir);
    match path {
        Ok(path) => {
            let path = path.to_str();
            if path.is_none() {
                println!("The path to the frontend can't be printed ! Do you use non-utf8 character ?");
            } else {
                println!("Frontend at file://{:}", path.unwrap());
            }
        },
        Err(_e) => println!("Please copy the frontend folder in your project")
    };
}

