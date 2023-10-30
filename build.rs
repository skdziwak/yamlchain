use std::env;
use std::fs;
use std::str::FromStr;
use proc_macro2::TokenStream;
use walkdir::WalkDir;

struct Visitor {
    macros: Vec<Macro>,
}

struct Macro {
    runner: String,
    info: String,
    enum_name: String,
}

impl Visitor {
    fn new() -> Self {
        Self {
            macros: Vec::new(),
        }
    }

    fn visit(&mut self, path: &str) {
        let content = fs::read_to_string(path).unwrap();
        let syntax = syn::parse_file(&content).unwrap();
        let path_from_src = path.strip_prefix("src/").unwrap();
        let path_without_rs = path_from_src.strip_suffix(".rs").unwrap();
        let rust_path = &format!("crate::{}", path_without_rs.replace("/", "::"));
        for item in syntax.items {
            match item {    
                syn::Item::Struct(s) => {
                    self.visit_struct(rust_path, s);
                },
                _ => {}
            }
        }
    }

    fn visit_struct(&mut self, path: &str, item: syn::ItemStruct) {
        for attr in item.attrs {
            if attr.path().is_ident("stage") {
                let runner_name = item.ident.to_string();
                let runner_path = format!("{}::{}", path, &runner_name);
                let info_name = attr.parse_args_with(|input: syn::parse::ParseStream| {
                    let info: syn::Ident = input.parse()?;
                    let info = info.to_string();
                    Ok(info)
                }).unwrap();
                let info_path = format!("{}::{}", path, info_name);
                let enum_name = runner_name.strip_suffix("StageRunner").unwrap_or(runner_name.as_str());
                self.macros.push(Macro {
                    runner: runner_path,
                    info: info_path,
                    enum_name: enum_name.to_string(),
                });
            }
        }

    }
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src = format!("{}/src", dir);
    let mut visitor = Visitor::new();
    for entry in WalkDir::new(src) {
        let entry = entry.unwrap();
        if entry.path().extension() == Some(std::ffi::OsStr::new("rs")) {
            let relative_path = entry.path().strip_prefix(&dir).unwrap();
            visitor.visit(relative_path.to_str().unwrap());
        }
    }
    let enum_variants = (&visitor.macros).iter()
        .map(|m| {
            let info = &m.info;
            let enum_name = &m.enum_name;
            TokenStream::from_str(&format!("{}({})", enum_name, info)).unwrap()
        }).collect::<Vec<_>>();
    let match_arms = (&visitor.macros).iter()
        .map(|m| {
            let runner = &m.runner;
            let enum_name = &m.enum_name;
            TokenStream::from_str(&format!("WorkflowStage::{}(d) => Box::new({}::new(&d))", enum_name, runner)).unwrap()
        }).collect::<Vec<_>>();

    let generated_source = quote::quote! {
        use schemars::JsonSchema;
        use serde::{Deserialize, Serialize};
        use crate::schema::WorkflowStageData;
        use crate::workflows::stages::StageRunner;

        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
        #[serde(tag = "type", rename_all = "snake_case")]
        pub enum WorkflowStage {
            #(#enum_variants),* 
        }
        pub fn get_runner<'a>(stage: &'a WorkflowStageData) -> Box<dyn StageRunner + 'a> {
            match &stage.stage {
                #(#match_arms),*
            }
        }
    };
    let generated_source = generated_source.to_string();
    let output_path = std::path::Path::new(&out_dir).join("generated.rs");
    fs::write(&output_path, generated_source).unwrap();
}
