use std::{fs::{self, File}, path::Path};

use winreg::RegKey;

const EFT_SIZE: usize = 647 * 1000;

pub(crate) fn eft_file_check(eft_key: &RegKey, eft_path: &String) -> anyhow::Result<()> {
    println!("EscapeFromTarkov.exe check");
    let eft_file_path = Path::new(&eft_path).join("EscapeFromTarkov.exe");
    if !eft_file_path.exists() {
        let path = std::env::current_dir()?.join("EscapeFromTarkov.exe");
        if !path.exists() {
            panic!("EscapeFromTarkov.exe doesn't exist");
        }
		fs::copy(&path, &eft_file_path)?;
        println!("EFT regestry InstallLocation created");
    } else {
        let eft_file = File::open(&eft_file_path)?;
        if eft_file.metadata()?.len() < EFT_SIZE as u64 {
            panic!("Wrong size of EscapeFromTarkov.exe: current size is {} expected size is {}", eft_file.metadata()?.len(), EFT_SIZE)
        }
        eft_key.set_value("DisplayIcon", &eft_file_path.into_os_string())?;
        println!("EFT regestry DisplayIcon created or sets to new value");
    }
    println!("EscapeFromTarkov.exe passed");

    Ok(())
}
