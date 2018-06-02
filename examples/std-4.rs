extern crate hashmap;
use hashmap::HashMap;

fn main() {
    let _timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
        .iter()
        .cloned()
        .collect();
}
