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


#[derive(Clone)]
pub enum FillStyle {False, Origin, Start, End}

// Chart.js dataset
pub struct Dataset<V> {
    pub label: String,
    pub data: std::vec::Vec<V>,
    pub fill: FillStyle,
    pub line_tension: f32,
    pub border_color: String,
    pub background_color: Option<String>,
}

// Chart.js structure
pub struct Chart<T, V> {
    pub type_: String, //default: line
    pub labels : std::vec::Vec<T>,
    pub datasets : std::vec::Vec<Dataset<V>>,
}

#[derive(Clone)]
pub enum ImageType {Static}

pub struct ImageCrop {
    pub sx: u32,
    pub sy: u32,
    pub sw: u32,
    pub sh: u32,
}

pub struct ImageResize {
    pub w: u32,
    pub h: u32,
    pub crop: Option<ImageCrop>,
}

pub struct Image {
    pub type_: ImageType,
    pub source: String,
    pub x: i32,
    pub y: i32,
    pub resize: Option<ImageResize>,
}

impl JsGui {
    pub fn draw_image(&self, img: &Image) {
        let mut buf = String::new_json();
        buf.append_str("type", "img");
        buf.append_str("src", &img.source);
        buf.append_number("x", &img.x);
        buf.append_number("y", &img.y);
        if let Some(resize) = img.resize.as_ref() {
            buf.append_number("w", &resize.w);
            buf.append_number("h", &resize.h);
            buf.append_bool("resize", true);
            if let Some(crop) = resize.crop.as_ref() {
                buf.append_number("sx", &crop.sx);
                buf.append_number("sy", &crop.sy);
                buf.append_number("sw", &crop.sw);
                buf.append_number("sh", &crop.sh);
                buf.append_bool("crop", true);
            }
        }
        self.send(buf);
    }
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
    pub fn popup(&self, text: &str) {
        let mut buff = String::new_json();
        buff.append_str("type","alert");
        buff.append_str("text",text);
        self.send(buff);
    }
    pub fn draw_chart<T: std::string::ToString, V: std::string::ToString>(&self, chart: &Chart<T, V>) {
        let mut datasets_vec_json = vec![];

        for dataset in chart.datasets.iter() {
            let mut dataset_json = String::new_json();
            dataset_json.append_str("label", dataset.label.as_str());
            dataset_json.append_vec("data", &dataset.data);
            match &dataset.fill {
                FillStyle::False => dataset_json.append_str("fill", "false"),
                FillStyle::Origin => dataset_json.append_str("fill", "origin"),
                FillStyle::Start => dataset_json.append_str("fill", "start"),
                FillStyle::End => dataset_json.append_str("fill", "end")
            };
            dataset_json.append_str("borderColor", dataset.border_color.as_str());
            if let Some(background_color) = dataset.background_color.as_ref() {
                dataset_json.append_str("backgroundColor", background_color.as_str());
            }
            dataset_json.append_number("lineTension", &dataset.line_tension);
            datasets_vec_json.push(dataset_json);
        }

        let mut data_json = String::new_json();
        data_json.append_vec("labels", &chart.labels);
        data_json.append_vec("datasets", &datasets_vec_json);

        let mut chart_json = String::new_json();
        chart_json.append_str("type", chart.type_.as_str());
        chart_json.append_json("data", &data_json);
        chart_json.append_json("options", &String::new_json());

        let mut json = String::new_json();
        json.append_str("type", "chart");
        json.append_json("data", &chart_json);

        self.send(json);
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

