extern crate proc_macro;
use std::{fs::File, path::PathBuf};

use fasthash::city;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Resource {
    package_id: String,
    resource_path_absolute: String,
    resource_path_relative: String,
}

#[proc_macro]
pub fn include_file(input: TokenStream) -> TokenStream {
    let tokens: Vec<_> = input.into_iter().collect();

    let resource_path = match tokens.as_slice() {
        [TokenTree::Literal(lit)] => unwrap_string_literal(lit),
        _ => panic!("This macro only accepts a single, non-empty string argument"),
    };

    if let Ok(out_dir) = std::env::var("CARGO_TAI_RESOURCE_DIR") {
        let _recompile = option_env!("CARGO_TAI_RECOMPILE");
        let crate_root = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to find manifest dir");

        let mut cmd = cargo_metadata::MetadataCommand::new();
        let meta = cmd
            .manifest_path(PathBuf::from(&crate_root).join("Cargo.toml"))
            .exec()
            .expect("Failed to read manifest");

        let resource = Resource {
            package_id: meta.root_package().unwrap().id.repr.to_owned(),
            resource_path_absolute: PathBuf::from(crate_root)
                .join(&resource_path)
                .display()
                .to_string(),
            resource_path_relative: resource_path.clone(),
        };

        let hash = city::hash64(&resource_path.as_bytes());
        let resource_file = PathBuf::from(out_dir).join(hash.to_string());
        let file = File::create(resource_file).expect("Failed to create resource info");
        serde_json::ser::to_writer(file, &resource).expect("Failed to write resource info");
    }

    quote! {
        {
            use std::{env, path::PathBuf};
            if cfg!(any(target_os = "ios", target_os = "android")) {
                let current_exe = env::current_exe().expect("Failed to access path of executable");
                current_exe.parent().map(|p| p.join(#resource_path)).expect("Failed to access parent of executable path")
            } else {
                PathBuf::from(#resource_path)
            }
        }
    }
    .into()
}

fn unwrap_string_literal(lit: &proc_macro::Literal) -> String {
    let mut repr = lit.to_string();
    if !repr.starts_with('"') || !repr.ends_with('"') {
        panic!("This macro only accepts a single, non-empty string argument")
    }

    repr.remove(0);
    repr.pop();

    repr
}
