use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> std::io::Result<()> {
    let mut stack = vec![PathBuf::from(from.as_ref())];

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if let Some(filename) = path.file_name() {
                let dest_path = dest.join(filename);
                fs::copy(&path, &dest_path)?;
            }
        }
    }

    Ok(())
}

pub fn exist_config_dir() -> Result<bool, std::io::Error> {
    let config_dir = dirs::config_dir()
        .expect("No config directories found... exiting.");
    let result = fs::read_dir(config_dir)?.fold(
        false,
        |acc, e| {match e {
            Ok(entry) => acc || (entry.file_name() == "olagem"),
            Err(_) => false,
        }}
    );
    Ok(result)
}

// #[allow(unused_must_use)]
fn main() -> std::io::Result<()> {

    match exist_config_dir() {
        Ok(false) => {
            let install_path = dirs::config_dir()
                .expect("Couldn't find a configuration directory to install to.")
                .join("olagem");
            // let install_path = Path::new("/usr/share/olagem/");
            fs::create_dir_all(&install_path).expect("Couldn't create the directory");

            let resources_path = env::current_dir()
                .expect("Couldn't find the source directory.")
                .join("assets");
            // .join("runtime");
            copy(resources_path, &install_path).expect("Couldn't copy the assets files");
        },
        _ => (),
    }

    Ok(())
}
