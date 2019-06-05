extern crate js_gui_rs;

fn main() {
    let js_gui = js_gui_rs::JsGui::new("127.0.0.1:2794");

    js_gui_rs::print_link();

    js_gui.set_fill_style("#FFFF00");

    js_gui.clear();

    std::thread::sleep(std::time::Duration::from_millis(5000));

    js_gui.draw_rect(10,10, 20, 30);

    std::thread::sleep(std::time::Duration::from_millis(200));

    js_gui.draw_line(10, 10, 50, 100);

    std::thread::sleep(std::time::Duration::from_millis(200));

    js_gui.draw_arc(100, 100, 20, 0f32, 6.2832);

    std::thread::sleep(std::time::Duration::from_millis(200));

    js_gui.draw_text(100, 100, "Hello world!", "30px Arial");

    std::thread::sleep(std::time::Duration::from_millis(200));

    let data = js_gui_rs::Dataset::<f32> {
        label: String::from("Foo"),
        data: vec![0.5, 0.75, 1.0],
        fill: js_gui_rs::FillStyle::False,
        line_tension: 0.1,
        border_color: String::from("#8e5ea2"),
        background_color: None
    };

    let data2 = js_gui_rs::Dataset::<f32> {
        label: String::from("Bar"),
        data: vec![10.5, 10.75, 10.0],
        fill: js_gui_rs::FillStyle::Start,
        line_tension: 0.1,
        border_color: String::from("rgba(60,150,90,0.9)"),
        background_color: Some(String::from("rgba(60,150,90,0.1)"))
    };

    let chart = js_gui_rs::Chart {
        type_: String::from("line"),
        labels: vec![1.0, 2.0, 3.0],
        datasets: vec![data, data2]
    };

    js_gui.draw_chart(&chart);

    std::thread::sleep(std::time::Duration::from_millis(1000));
    js_gui.clear();
    std::thread::sleep(std::time::Duration::from_millis(200));

    let mut img1 = js_gui_rs::Image {
        type_: js_gui_rs::ImageType::Static,
        source: String::from("img1.jpg"),
        x: 150,
        y: 100,
        resize: Some(js_gui_rs::ImageResize {
            w: 70,
            h: 50,
            crop: Some(js_gui_rs::ImageCrop {
                sx: 10,
                sy: 10,
                sw: 10,
                sh: 10,
            }),
        }),
    };

    js_gui.draw_image(&img1);
    std::thread::sleep(std::time::Duration::from_millis(200));

    img1.resize.as_mut().unwrap().crop = None;
    js_gui.draw_image(&img1);
    std::thread::sleep(std::time::Duration::from_millis(200));

    img1.resize = None;
    js_gui.draw_image(&img1);
    std::thread::sleep(std::time::Duration::from_millis(200));

    let img2 = js_gui_rs::RawImage {
        data: vec![
                    255, 0, 0, 255, // RGBA pixels' value
                    0, 255, 0, 255,
                    0, 0, 255, 255,
                    0, 0, 0, 255,
                    ],
        width: 2,
        height: 2,
        x: 100,
        y: 100,
        resize: Some(js_gui_rs::ImageResize {
            w: 200,
            h: 200,
            crop: None,
        }),
    };

    if let Err(e) = js_gui.draw_image_vec(&img2) {
        match e {
            js_gui_rs::JsGuiError::WrongDataSize => {
                println!("[x] JsGui::draw_image_vec: Error wrong image size");
            },
        }
    }
    std::thread::sleep(std::time::Duration::from_millis(200));

    js_gui.popup("Wow !");

}
