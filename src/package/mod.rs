mod lua;
mod map;
pub use map::{Map, Value as MapValue};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub dependencies: Vec<String>,
    pub files: PackageFiles,
    pub hooks: PackageHooks,
    pub variables: Map,
}

#[derive(Clone, Debug)]
pub struct PackageFiles {
    pub trees: Vec<Tree>,
    pub extra: Vec<File>,
    pub templates: Vec<Template>,
    pub link_type: LinkType,
    pub replace_files: bool,
    pub replace_dirs: bool,
}

#[derive(Clone, Debug)]
pub struct PackageHooks {
    pub pre: Vec<Hook>,
    pub install: Option<HookBody>,
    pub post: Vec<Hook>,
}

#[derive(Clone, Debug)]
pub struct Tree {
    pub path: PathBuf,
    pub link_type: Option<LinkType>,
    pub ignore: Vec<String>,
    pub replace_files: Option<bool>,
    pub replace_dirs: Option<bool>,
}

impl Tree {
    pub fn file_path(&self, join: impl AsRef<Path>) -> PathBuf {
        self.path.join(join)
    }

    pub fn file_path_str(&self, join: impl AsRef<Path>) -> String {
        self.file_path(join).to_string_lossy().into_owned()
    }
}

#[derive(Clone, Debug)]
pub struct File {
    pub src: PathBuf,
    pub dest: PathBuf,
    pub link_type: Option<LinkType>,
    pub replace_files: Option<bool>,
    pub replace_dirs: Option<bool>,
}

#[derive(Clone, Debug)]
pub enum LinkType {
    Link,
    Copy,
}

#[derive(Clone, Debug)]
pub struct Template {
    pub src: PathBuf,
    pub dest: PathBuf,
    pub ty: TemplateType,
    pub replace_files: Option<bool>,
    pub replace_dirs: Option<bool>,
}

#[derive(Clone, Debug)]
pub enum TemplateType {
    Handlebars { partials: HashMap<String, PathBuf> },
    Gotmpl,
    Tera,
}

#[derive(Clone, Debug)]
pub struct Hook {
    pub name: String,
    pub body: HookBody,
}

#[derive(Clone, Debug)]
pub enum HookBody {
    Executable { command: String },
    LuaFunction { name: String },
}
