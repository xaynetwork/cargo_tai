use std::{
    fs::{create_dir_all, File},
    path::{Path, PathBuf},
};

use handlebars::{to_json, Handlebars};
use serde::Serialize;
use serde_json::value::Map;

use crate::{
    common::task::Task,
    ios::{
        platform::APP_ID,
        tools::{rsync::rsync, xcodegen},
    },
    TaiResult,
};

use super::Context;

// needs to be otherwise the tests will not work https://github.com/yonaskolb/XcodeGen/issues/408#issuecomment-458639126
const APP_NAME: &str = "cargoTai";

pub struct CreateXCodeProject;

impl Task<Context> for CreateXCodeProject {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let lib_name = context
            .take_build_units()?
            .first()
            .ok_or(anyhow::anyhow!("no units"))?
            .name
            .clone();

        let project_meta = context.project_metadata()?;
        let template_dir = PathBuf::from("/Users/robert/projects/cargo-tai/project-templates/ios");

        let ios_dir = project_meta.ios_dir();

        create_dir_all(&ios_dir)?;
        rsync(&template_dir, &ios_dir)?;

        let tpl_path = ios_dir.join("project.yml.hbs");
        let project_path = ios_dir.join("project.yml");

        let data = Data {
            app_name: APP_NAME.into(),
            app_bundle_id: APP_ID.into(),
            target_dir: project_meta
                .meta
                .target_directory
                .clone()
                .into_std_path_buf(),
            template_dir,
            lib_name,
        };

        generate_project_file(&tpl_path, &project_path, &data)?;
        xcodegen::generate(&project_path, &ios_dir)?;

        let xcode_project = XCodeProject {
            root: ios_dir,
            app_name: APP_NAME.into(),
        };

        context.xcode_project = Some(xcode_project);

        Ok(context)
    }
}

fn generate_project_file<P1: AsRef<Path>, P2: AsRef<Path>>(
    tpl_path: P1,
    project_path: P2,
    data: &Data,
) -> TaiResult<()> {
    let mut handlebars = Handlebars::new();

    let mut data_map = Map::new();
    data_map.insert("data".to_string(), to_json(data));

    let tpl_name = "project";
    handlebars
        .register_template_file(tpl_name, tpl_path)
        .unwrap();

    let mut output_file = File::create(project_path)?;
    handlebars.render_to_write(tpl_name, &data_map, &mut output_file)?;
    Ok(())
}

#[derive(Serialize)]
struct Data {
    pub app_name: String,
    pub app_bundle_id: String,
    pub target_dir: PathBuf,
    pub template_dir: PathBuf,
    pub lib_name: String,
}

pub struct XCodeProject {
    pub root: PathBuf,

    pub app_name: String,
}

impl XCodeProject {
    pub fn path(&self) -> PathBuf {
        self.root.join(format!("{}.xcodeproj", self.app_name))
    }
}
