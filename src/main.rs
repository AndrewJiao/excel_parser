use serde_json::Value;

fn main() {

        let value = Value::from(23);
        println!("j={}",value.to_string())
}
