extern crate proc_macro;
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

use cargo_metadata::PackageId;
use fasthash::city;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Resource {
    package_id: PackageId,
    resource_source: PathBuf,
    resource_destination: PathBuf,
}

#[proc_macro]
pub fn include_file(input: TokenStream) -> TokenStream {
    let tokens: Vec<_> = input.into_iter().collect();

    let resource_path = match tokens.as_slice() {
        [TokenTree::Literal(lit)] => unwrap_string_literal(lit),
        _ => panic!("This macro only accepts a single, non-empty string argument"),
    };

    let mut resource_path = PathBuf::from(&resource_path);

    if let Ok(out_dir) = std::env::var("CARGO_TAI_RESOURCE_DIR") {
        let _recompile = option_env!("CARGO_TAI_RECOMPILE");
        let crate_root = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to find manifest dir");

        let mut cmd = cargo_metadata::MetadataCommand::new();
        let crate_meta = cmd
            .manifest_path(PathBuf::from(&crate_root).join("Cargo.toml"))
            .exec()
            .expect("Failed to read manifest");

        let package_id = crate_meta
            .root_package()
            .expect("Failed to read the metadata for the root package")
            .id
            .clone();

        let resource_source = PathBuf::from(&crate_root).join(&resource_path);

        // add crate scope
        let package_id_hash = city::hash64(&package_id.repr).to_string();
        resource_path = PathBuf::from(&package_id_hash).join(&resource_path);

        let resource_meta = Resource {
            package_id,
            // for copying
            resource_source,
            // path inside the app bundle
            resource_destination: resource_path.clone(),
        };

        let hash = city::hash64(&resource_path.display().to_string());
        let resource_crate_meta_dir = PathBuf::from(&out_dir).join(&package_id_hash);
        create_dir_all(&resource_crate_meta_dir).expect("Failed to create resource directory");
        let resource_meta_file =
            PathBuf::from(&resource_crate_meta_dir).join(format!("{}.json", hash));
        let meta_file =
            File::create(&resource_meta_file).expect("Failed to create resource info file");
        serde_json::ser::to_writer(&meta_file, &resource_meta)
            .expect("Failed to write resource info");
    }

    let resource_path = resource_path.display().to_string();

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
