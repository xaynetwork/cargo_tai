mod build_app;
mod build_build_units;
mod build_xcode_test;
mod context;
mod create_bundles;
mod create_signed_bundles;
mod create_xcode_project;
mod list_physical_devices;
mod list_simulators;
mod read_signing_settings;
mod run_on_physical_device;
mod run_on_simulators;
mod task;

pub use self::{
    build_build_units::BuildBuildUnit,
    context::Context,
    create_bundles::CreateBundles,
    create_signed_bundles::CreateSignedBundles,
    list_physical_devices::ListPhysicalDevices,
    list_simulators::ListSimulators,
    read_signing_settings::ReadSigningSettings,
    run_on_physical_device::RunOnPhysicalDevice,
    run_on_simulators::RunOnSimulators,
    task::Task,
};

// fn build_for_native_tests() -> TaiResult<()>{
//     // build_dir
//     //     template
//     //         bindings
//     //         ...
//     //     xc_project
//     //         build-app
//     //         build-test

//     const XCODEGEN_PROJECT_SPEC: &str = "project.yml";
//     // compile -> lib_name
//     let lib_name = "test-paradise.a";

//     let build_dir = std::path::Path::new("");
//     let ios_template_dir = std::path::Path::new("");

//     // copy template into build dir
//     std::fs::copy(ios_template_dir, build_dir)?;

//     // generate project spec
//     let mut spec = build_dir.to_path_buf();
//     spec.push("template");
//     spec.push(XCODEGEN_PROJECT_SPEC);

//     let mut project_dir = build_dir.to_path_buf();
//     project_dir.push("xc_project");

//     // generate xcode project
//     xcodegen::generate(&spec, &project_dir)?;

//     // build app
//     let mut data_path_build_app = project_dir.clone();
//     data_path_build_app.push("build");
//     xcodebuild::build(&project_dir, lib_name, data_path_build_app)?;

//     // build test
//     let mut data_path_build_test = project_dir.clone();
//     data_path_build_test.push("build_test");
//     xcodebuild::build_for_testing(&project_dir, lib_name, data_path_build_test)?;

//     // build ipa
//     // Build/Products/Release-iphoneos/

//     // copy to output dir
//     Ok(())
// }
