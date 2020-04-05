extern crate clap;
extern crate dirs;

use clap::{App, Arg};
use std::fs::{self};
use std::fs::{remove_dir_all, remove_file};
use std::io;
use std::path::Path;

fn is_ignore(dir: &Path) -> bool {
	let file_name = dir.file_name().unwrap().to_str().unwrap();
	if file_name.starts_with('.') {
		return true;
	}
	false
}

fn clear_rust_project(dir: &Path, remove: bool) -> bool {
	let toml = dir.join("Cargo.toml");
	let lock = dir.join("Cargo.lock");
	let target = dir.join("target");

	if toml.is_file() && lock.is_file() && target.is_dir() {
		cl_remove_dir(remove, &target);
		return true;
	}

	false
}

fn clear_node_project(dir: &Path, remove: bool) -> bool {
	let package = dir.join("package.json");
	let node_modules = dir.join("node_modules");

	if package.is_file() && node_modules.is_dir() {
		cl_remove_dir(remove, &node_modules);
		return true;
	}

	false
}

fn remove_blend_file(file: &Path, remove: bool) -> bool {
	let file_name = file.file_name().unwrap().to_str().unwrap();

	if file_name.ends_with(".blend1") || file_name.ends_with(".blend2") {
		cl_remove_file(remove, file);
		return true;
	}

	false
}

fn visit_dir(dir: &Path, remove: bool) -> io::Result<()> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				if !is_ignore(&path)
					&& (!clear_rust_project(&path, remove) && !clear_node_project(&path, remove))
				{
					visit_dir(&path, remove)?;
				}
			} else {
				remove_blend_file(&path, remove);
			}
		}
	} else {
		remove_blend_file(&dir, remove);
	}
	Ok(())
}

fn cl_remove_dir(remove: bool, dir: &Path) {
	if !dir.is_dir() {return}

	println!("{:?}", dir);
	if remove { remove_dir_all(dir).unwrap()}
}

fn cl_remove_file(remove: bool, file: &Path) {
	if !file.is_file() {return}

	println!("{:?}", file);
	if remove {remove_file(file).unwrap()}
}


fn remove_local_dirs(remove: bool, home_dir: &Path) {
	// Trash
	let dir = dirs::data_local_dir().unwrap().join("Trash");
	cl_remove_dir(remove, &dir);

	// .cache
	let dir = home_dir.join(".cache");
	cl_remove_dir(remove, &dir);

	// .cache
	let dir = home_dir.join(".npm").join("_cacache");
	cl_remove_dir(remove, &dir);
}

fn main() {
	let matches = App::new("Cleaner Tool")
		.version("0.1")
		.about("remove stuff")
		.arg(
			Arg::with_name("remove")
				.short("r")
				.long("remove")
				.help("for remove"),
		)
		.get_matches();

	let remove = matches.is_present("remove");

	let my_home_dir = dirs::home_dir().expect("cant get home dir");

	remove_local_dirs(remove, &my_home_dir);

	visit_dir(&my_home_dir, remove).unwrap();
}
