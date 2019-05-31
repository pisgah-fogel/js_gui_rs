extern crate js_gui_rs;

fn main() {
    let js_gui = js_gui_rs::JsGui::new("127.0.0.1:2794");

    js_gui_rs::print_link();

    js_gui.set_fill_style("#FFFF00");

    std::thread::sleep(std::time::Duration::from_millis(2000));

    js_gui.draw_rect(10,10, 20, 30);

    std::thread::sleep(std::time::Duration::from_millis(1000));

    js_gui.draw_line(10, 10, 50, 100);

    std::thread::sleep(std::time::Duration::from_millis(1000));

    js_gui.draw_arc(100, 100, 20, 0f32, 6.2832);

    std::thread::sleep(std::time::Duration::from_millis(1000));

    js_gui.draw_text(100, 100, "Hello world!", "30px Arial");

    std::thread::sleep(std::time::Duration::from_millis(1000));

    js_gui.clear();

    std::thread::sleep(std::time::Duration::from_millis(1000));

    js_gui.clear();

    let data = js_gui_rs::Dataset::<f32> {
        label: String::from("Foo"),
        data: vec![0.5, 0.75, 1.0],
        fill: false,
        line_tension: 0.1
    };

    let data2 = js_gui_rs::Dataset::<f32> {
        label: String::from("Bar"),
        data: vec![10.5, 10.75, 10.0],
        fill: false,
        line_tension: 0.1
    };

    let chart = js_gui_rs::Chart {
        type_: String::from("line"),
        labels: vec![1.0, 10f32, 11f32],
        datasets: vec![data, data2]
    };

    js_gui.draw_chart(&chart);
}
