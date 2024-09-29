use std::{io::ErrorKind, path::Path};

use winreg::RegKey;

pub(crate) fn eft_folder_check(eft_key: &RegKey) -> anyhow::Result<String> {
    println!("EFT regestry value check");
    let eft_path_wrapped: Result<String, std::io::Error> = eft_key.get_value("InstallLocation");
    let eft_path: String;
    match eft_path_wrapped {
        Err(why) => {
            if why.kind() == ErrorKind::NotFound {
                let path = std::env::current_dir()?.join("").into_os_string();
                eft_key.set_value("InstallLocation", &path)?;
                eft_path = path.into_string().unwrap();
                println!("EFT regestry InstallLocation created");
            } else {
                panic!("{:?}",why);
            }
        }
        Ok(key) => eft_path = key
    }
    if !Path::new(&eft_path).exists() {
        panic!("EFT folder doesn't exist")
    }
    println!("EFT regestry value check passed");

    Ok(eft_path)
}
