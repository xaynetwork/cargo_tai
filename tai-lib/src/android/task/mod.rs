pub mod build_build_units;
pub mod context;
pub mod create_bundles;
pub mod find_android_sdk;
pub mod list_devices;
pub mod run_on_devices;
pub mod task;

pub use self::{
    build_build_units::BuildBuildUnit,
    context::{Context, Options},
    create_bundles::CreateBundles,
    find_android_sdk::FindAndroidSdk,
    list_devices::ListDevices,
    run_on_devices::RunOnDevices,
    task::Task,
};

pub struct GetProjectMetadata;

impl crate::task::Task for GetProjectMetadata {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> crate::TaiResult<Self::Context> {
        let mut cargo_args = context.requested.general.compiler.cargo_args.iter();

        // https://docs.rs/cargo_metadata/0.14.0/cargo_metadata/#examples
        let manifest_path = match cargo_args.next() {
            Some(p) if p == "--manifest-path" => cargo_args
                .next()
                .ok_or_else(|| anyhow::anyhow!("no manifest"))?
                .into(),
            Some(p) => p.trim_start_matches("--manifest-path=").into(),
            None => std::env::current_dir()?.join("Cargo.toml"),
        };

        context.project_metadata = Some(crate::task::project_metadata(manifest_path)?);

        Ok(context)
    }
}
