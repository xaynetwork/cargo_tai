use std::{
    fs::{create_dir_all, File},
    path::{Path, PathBuf},
};

use handlebars::{to_json, Handlebars};
use serde::Serialize;
use serde_json::value::Map;
use tracing::instrument;

use crate::{
    common::{bundle::copy_resources, task::Task},
    ios::{
        platform::APP_ID,
        tools::{Rsync, XCodeGenGenerate},
    },
    TaiResult,
};

use super::Context;

// needs to be otherwise the tests will not work https://github.com/yonaskolb/XcodeGen/issues/408#issuecomment-458639126
const APP_NAME: &str = "cargoTai";

pub struct CreateXCodeProject;

impl Task<Context> for CreateXCodeProject {
    #[instrument(name = "create_xcode_project", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let lib_name = context
            .take_built_units()?
            .first()
            .ok_or_else(|| anyhow::anyhow!("no built units"))?
            .name
            .clone();

        let project_meta = context.project_metadata()?;
        let template_dir = &context.build()?.template_dir;
        let ios_working_dir = project_meta.ios_working_dir.to_owned();

        create_dir_all(&ios_working_dir)?;
        let mut cmd = Rsync::new(&template_dir, &ios_working_dir);
        cmd.archive().delete().only_content();
        if context.opts.cli.verbose {
            cmd.verbose();
        }
        cmd.execute()?;

        let resources_path = ios_working_dir.join(tai_util::DATA_DIR_NAME);
        create_dir_all(&resources_path)?;

        if let Some(resources) = &context.opts.resources {
            copy_resources(resources_path, resources)?;
        }

        let tpl_path = ios_working_dir.join("project.yml.hbs");
        let project_path = ios_working_dir.join("project.yml");

        let (app_bundle_id, team_id) = if let Ok(sig_settings) = context.signing_settings() {
            (
                sig_settings.app_id.to_owned(),
                Some(sig_settings.team_id.to_owned()),
            )
        } else {
            (APP_ID.to_owned(), None)
        };

        let data = Data {
            app_name: APP_NAME.into(),
            app_bundle_id,
            target_dir: project_meta
                .meta
                .target_directory
                .clone()
                .into_std_path_buf(),
            template_dir: template_dir.clone().canonicalize()?,
            lib_name,
            team_id,
        };

        generate_project_file(&tpl_path, &project_path, &data)?;
        let mut cmd = XCodeGenGenerate::new();
        cmd.spec(&project_path).project(&ios_working_dir);
        if context.opts.cli.verbose {
            cmd.verbose();
        }

        cmd.execute()?;

        let xcode_project = XCodeProject {
            root: ios_working_dir,
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
    pub team_id: Option<String>,
}

pub struct XCodeProject {
    pub root: PathBuf,

    pub app_name: String,
}

impl XCodeProject {
    pub fn path(&self) -> PathBuf {
        self.root.join(format!("{}.xcodeproj", self.app_name))
    }

    pub fn xctest_name(&self) -> String {
        format!("{}Test", self.app_name)
    }
}
