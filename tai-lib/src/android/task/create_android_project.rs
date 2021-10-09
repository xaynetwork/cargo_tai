use std::{
    fs::{self, copy, create_dir, create_dir_all, remove_file, File},
    path::Path,
};

use handlebars::{to_json, Handlebars};
use serde::Serialize;
use serde_json::Map;
use walkdir::WalkDir;

use crate::{
    android::tools::adb::CpuArch,
    common::{bundle::copy_resources, compiler::BuiltUnit, task::Task, tools::Rsync},
    TaiResult,
};

use super::Context;

pub struct CreateAndroidProject;

impl Task<Context> for CreateAndroidProject {
    fn run(&self, context: Context) -> TaiResult<Context> {
        let project_meta = context.project_metadata()?;
        let template_dir = &context.build()?.template_dir;
        let android_working_dir = project_meta.android_working_dir.to_owned();

        // copy template into working directory
        let mut cmd = Rsync::new(&template_dir, &android_working_dir);
        cmd.archive().delete().only_content();
        if context.opts.cli.verbose {
            cmd.verbose();
        }
        cmd.execute()?;

        // copy resources into working directory
        copy_resources(
            &android_working_dir,
            context.opts.resources.as_ref().unwrap_or(&Vec::new()),
        )?;

        let lib_name = context
            .built_units()?
            .first()
            .ok_or_else(|| anyhow::anyhow!("no built units"))?
            .name
            .as_str();

        let data = TemplateData {
            app_name: "RUST",
            application_id: "com.example.rust",
            min_sdk: context
                .opts
                .android
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("missing api lvl "))?
                .api_lvl,
            target_sdk: 30,
            version_code: 1,
            version_name: "1.0",
            lib_name,
            sdk_path: &format!("{}", &context.android_sdk()?.sdk.display()),
        };

        generate_project(&android_working_dir, &data)?;

        let jni_libs = &android_working_dir
            .join("app")
            .join("src")
            .join("main")
            .join("jniLibs");
        create_dir_all(&jni_libs)?;

        context
            .built_units()?
            .iter()
            .try_for_each(|unit| copy_lib(jni_libs, unit))?;

        Ok(context)
    }
}

#[derive(Serialize)]
struct TemplateData<'a> {
    pub app_name: &'a str,
    pub application_id: &'a str,
    pub min_sdk: u8,
    pub target_sdk: u8,
    pub version_code: u32,
    pub version_name: &'a str,
    pub lib_name: &'a str,
    pub sdk_path: &'a str,
}

fn generate_project<P: AsRef<Path>>(working_dir: P, data: &TemplateData) -> TaiResult<()> {
    let handlebars = Handlebars::new();

    let mut data_map = Map::new();
    data_map.insert("data".to_string(), to_json(data));

    WalkDir::new(working_dir.as_ref())
        .into_iter()
        .filter_map(|e| e.ok())
        .try_for_each(|entry| {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if "hbs" == ext {
                        let path = entry
                            .path()
                            .parent()
                            .map(|p| p.join(entry.path().file_stem().unwrap()))
                            .unwrap();
                        let output_file = File::create(path)?;

                        let template = fs::read_to_string(entry.path())?;
                        handlebars.render_template_to_write(&template, &data_map, output_file)?;
                        remove_file(entry.path())?;
                    }
                }
            }
            Ok(())
        })
}

fn copy_lib(jni_libs: &Path, unit: &BuiltUnit) -> TaiResult<()> {
    let arch: CpuArch = unit.target.arch.into();
    let arch: &'static str = arch.into();
    let lib_path = jni_libs.join(arch);
    create_dir(&lib_path)?;

    let lib_path = lib_path.join(unit.artifact.file_name().unwrap());
    copy(&unit.artifact, lib_path)?;

    Ok(())
}
