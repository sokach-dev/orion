use std::process::Command;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .type_attribute(
            "orion.VocabularyQuery",
            "#[derive(to_sql_condition::ToSqlCondition, derive_builder::Builder)]",
        )
        .type_attribute("orion.VocabularyQuery", "#[builder(setter(into), default)]")
        .compile(&["protos/orion.proto"], &["protos"])
        .unwrap();

    // remove file, so that help check code
    // std::fs::remove_file("src/pb/google.protobuf.rs").unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();

    println!("cargo:rerun-if-changed=protos/orion.proto");
}
