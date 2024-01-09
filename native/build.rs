use std::{fs, path::Path};

fn main() {
    let mut config = prost_build::Config::new();
    config.btree_map(&["."]);
    let path = Path::new("../proto");

    if !path.exists() {
        panic!(
            "Protos directory does not exist at \"../proto\", please sync + update the submodule."
        )
    }

    if let Ok(contents) = path.read_dir() {
        if contents
            .filter_map(|item| item.ok())
            .filter_map(|item| item.file_name().into_string().ok())
            .filter(|file_name| file_name.ends_with(".proto"))
            .count()
            == 0
        {
            panic!("Protos directory does exist at \"../proto\" but does not contain protobuf files. Please sync + update the submodule.");
        }
    }

    let files = fs::read_dir(path).unwrap();
    let files = files
        .filter_map(|item| item.ok())
        .filter_map(|item| item.file_name().into_string().ok())
        .filter(|file_name| file_name.ends_with(".proto"))
        .map(|file_name| path.join(file_name))
        .map(|file| file.to_str().unwrap().to_string())
        .collect::<Vec<String>>();

    tonic_build::configure()
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Duration", "::prost_wkt_types::Duration")
        .extern_path(".google.protobuf.Value", "::prost_wkt_types::Value")
        .build_server(!cfg!(target_arch = "wasm32"))
        .compile(&files, &["../proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {e:#?}"));
}
