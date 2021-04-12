/// Functions for constructing paths to files, based on the standard FreeSurfer recon-all ouput
/// directory strucure.

use std::path::{Path};
use anyhow::{Context, Result, bail};

fn fs_file_from_base(base_path : &str, fs_file : &str, subdir: &str, file_type: &str, must_exist : bool) -> Result<String> {
    let base_path : &Path = &Path::new(base_path);
    let fs_file : &Path = &Path::new(fs_file);

    let fs_full_path = if subdir.is_empty() { base_path.join(fs_file) } else { base_path.join(&Path::new(subdir)).join(fs_file) };
    let fs_full_path = fs_full_path.to_str().context("Converting OS path for FreeSurfer file to String.").expect("Path cannot be converted to UTF-8 string");

    if must_exist {
        if ! std::path::Path::new(&fs_full_path).exists() {
            bail!("Cannot read fs {} file '{}'.", file_type, fs_full_path);
        }
    }
    return Ok(String::from(fs_full_path));   
}


pub fn fs_surf_file_from_base(base_path : &str, fs_file : &str, must_exist : bool) -> Result<String> {
    fs_file_from_base(base_path, fs_file, "surf", "surface", must_exist)
}

pub fn fs_curv_file_from_base(base_path : &str, fs_file : &str, must_exist : bool) -> Result<String> {
    fs_file_from_base(base_path, fs_file, "surf", "curv", must_exist)
}

pub fn fs_label_file_from_base(base_path : &str, fs_file : &str, must_exist : bool) -> Result<String> {
    fs_file_from_base(base_path, fs_file, "label", "label", must_exist)
}

pub fn fs_annot_file_from_base(base_path : &str, fs_file : &str, must_exist : bool) -> Result<String> {
    fs_file_from_base(base_path, fs_file, "label", "annot", must_exist)
}

