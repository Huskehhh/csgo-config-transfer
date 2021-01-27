extern crate dotenv;

use std::error::Error;
use std::{env, ffi::OsString, fs, path::Path};

use dotenv::dotenv;
use fs_extra::dir::CopyOptions;

use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let account =
        env::var("STEAM_MAIN_ACCOUNT").expect("Missing STEAM_MAIN_ACCOUNT environment variable!");
    let steam_userdata_location =
        env::var("STEAM_INSTALL_LOC").expect("Missing STEAM_INSTALL_LOC environment variable!");

    let path_to_copy = Path::new(&steam_userdata_location)
        .join(&account)
        .join("730");

    let account_os_string = OsString::from_str(&account).unwrap();

    let dirs = fs::read_dir(&Path::new(&steam_userdata_location))?
        .filter_map(|f| {
            let result = f.ok()?;
            if !result.file_name().eq(&account_os_string) {
                Some(result)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let copy_options = CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
        copy_inside: false,
        content_only: false,
        depth: 0,
    };

    for dir in dirs {
        let dir_path = dir.path();
        println!(
            "Copying {} -> {}",
            path_to_copy.display(),
            dir_path.display()
        );
        fs_extra::dir::copy(&path_to_copy, dir_path, &copy_options)?;
    }

    Ok(())
}
