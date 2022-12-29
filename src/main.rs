use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string, env::args};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Operation {
    operation_id: String,
}

#[derive(Deserialize, Debug)]
struct Swagger {
    paths: HashMap<String, HashMap<String, Operation>>,
}
fn main() {
    // cargo run -- "../Untitled-1.json" "../Untitled-2.json"
    let files_paths = args().skip(1).take(2).collect::<Vec<_>>();
    if files_paths.len() < 2 {
        panic!("you have to pass the files to compare, example: 'cargo run -- ../Untitled-1.json ../Untitled-2.json'")
    }
    let first_operations = get_swagger_operation_ids(&files_paths[0]);
    let second_operations = get_swagger_operation_ids(&files_paths[1]);
    let difference: Vec<_> = second_operations
        .into_iter()
        .filter(|item| !first_operations.contains(item))
        .collect();
    println!("{difference:?}");
}

fn get_swagger_operation_ids(file_path: &str) -> Vec<String> {
    let file_str = read_to_string(file_path).unwrap();
    let swagger = serde_json::from_str::<Swagger>(file_str.as_str()).unwrap();
    swagger
        .paths
        .values() //
        .flat_map(|path| path.values().map(|op| op.operation_id.clone()))
        .collect()
}
