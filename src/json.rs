use std::string::String;


#[cfg(test)]
mod tests {
    #[test]
    fn append_str() {
        use crate::json::Jsonify;
        let mut a = String::new_json();
        a.append_str("label","value");
        println!("Result: {}", a);
        assert_eq!(a, String::from("{\"label\":\"value\"}"));
    }
    #[test]
    fn append_json() {
        use crate::json::Jsonify;
        let expected = "{\"data\":{\"label\":\"value\"}}";
        let mut a = String::new_json();
        a.append_str("label","value");
        let mut b = String::new_json();
        b.append_json("data", &a);
        println!("Result: {}", b);
        assert_eq!(b, String::from(expected));
    }
    #[test]
    fn append_vec() {
        use crate::json::Jsonify;
        let expected = "{\"array\":[1,2,3,4,5]}";
        let mut a = String::new_json();
        a.append_vec("array",&vec![1, 2, 3, 4, 5]);
        println!("Result: {}", a);
        assert_eq!(a, String::from(expected));
    }
    #[test]
    fn append_bool() {
        use crate::json::Jsonify;
        let expected = "{\"label1\":true,\"label2\":false}";
        let mut a = String::new_json();
        a.append_bool("label1",true);
        a.append_bool("label2",false);
        println!("Result: {}", a);
        assert_eq!(a, String::from(expected));
    }
    #[test]
    fn jschart_ex() {
        use crate::json::Jsonify;
        let expected = "{\"type\":\"line\",\"data\":{\"labels\":[January,February,March,April,May,June,July],\"datasets\":[{\"label\":\"My First Dataset\",\"data\":[65,59,80,81,56,55,40],\"fill\":false,\"borderColor\":\"rgb(75, 192, 192)\",\"lineTension\":\"0.1\"}]},\"options\":{}}";

        let type_ = "line";
        let labels = vec!["January", "February", "March", "April", "May", "June", "July"];
        let label = "My First Dataset";
        let data = vec![65,59,80,81,56,55,40];


        let mut dataset_json = String::new_json();
        dataset_json.append_str("label", label);
        dataset_json.append_vec("data", &data);
        dataset_json.append_bool("fill", false);
        dataset_json.append_str("borderColor", "rgb(75, 192, 192)");
        dataset_json.append_str("lineTension", "0.1");

        let mut data_json = String::new_json();
        data_json.append_vec("labels", &labels);
        data_json.append_vec("datasets", &vec![dataset_json]);

        let mut json = String::new_json();
        json.append_str("type", type_);
        json.append_json("data", &data_json);
        json.append_json("options", &String::new_json());

        println!("Result: {}", json);
        assert_eq!(json, String::from(expected));
    }
}

pub trait Jsonify {
    fn new_json() -> String;
    fn append_str(&mut self, label: &str, value: &str);
    fn append_json(&mut self, label: &str, json: &String);
    fn append_vec <T: std::string::ToString> (&mut self, label: &str, array: &std::vec::Vec<T>);
    fn append_number <T: std::string::ToString> (&mut self, label: &str, value: &T);
    fn append_bool(&mut self, label: &str, value: bool);
}

impl Jsonify for String {
    // Returns a blank json ("{}")
    fn new_json() -> String {
        return String::from("{}");
    }
    fn append_str(&mut self, label: &str, value: &str) {
        self.pop(); // remove '}'
        if self.len() > 2 {
            self.push(',');
        }

        self.push('"');
        self.push_str(label);
        self.push_str("\":\"");
        self.push_str(value);
        self.push('"');

        self.push('}'); // close json object
    }
    fn append_json(&mut self, label: &str, json: &String) {
        self.pop(); // remove '}'
        if self.len() > 2 {
            self.push(',');
        }

        self.push('"');
        self.push_str(label);
        self.push_str("\":");
        self.push_str(json.as_str());

        self.push('}'); // close json object
    }
    fn append_vec <T: std::string::ToString> (&mut self, label: &str, array: &std::vec::Vec<T>) {
        self.pop(); // remove '}'
        if self.len() > 2 {
            self.push(',');
        }

        self.push('"');
        self.push_str(label);
        self.push_str("\":[");
        for val in array.iter() {
            self.push_str(val.to_string().as_str());
            self.push(',');
        }
        self.pop(); // remove ','
        self.push(']');

        self.push('}'); // close json object
    }
    fn append_bool(&mut self, label: &str, value: bool) {
        self.pop(); // remove '}'
        if self.len() > 2 {
            self.push(',');
        }

        self.push('"');
        self.push_str(label);
        self.push_str("\":");
        if value {
            self.push_str("true");
        } else {
            self.push_str("false");
        }
        self.push('}'); // close json object
    }
    fn append_number <T: std::string::ToString> (&mut self, label: &str, value: &T) {
        self.pop(); // remove '}'
        if self.len() > 2 {
            self.push(',');
        }

        self.push('"');
        self.push_str(label);
        self.push_str("\":");
        self.push_str(value.to_string().as_str());
        self.push('}'); // close json object
    }
}
