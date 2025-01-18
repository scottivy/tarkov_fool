extern crate rand;

mod checks;
mod file;

use core::panic;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use file::CreateSpecificSizeFile;
use winreg::enums::*;
use winreg::RegKey;
use std::fs::{self, File};
use crate::checks::*;

const EFT_REG_PATH: &str = "Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\EscapeFromTarkov";

struct OnExit;

impl Drop for OnExit {
    fn drop(&mut self) {
	    let mut input = String::new();
		println!("\nPress Enter to exit...");
	    io::stdin()
	        .read_line(&mut input)
	        .expect("Failed to read line");
    }
}

fn main() -> anyhow::Result<()> {
	let _on_exit = OnExit;

    if !is_elevated::is_elevated() {
        panic!("program must be runned with admins rights");
    }

    println!("Regestry check");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let eft_key_wrapped = hklm.open_subkey_with_flags(EFT_REG_PATH, KEY_WRITE|KEY_READ);
    let eft_key: RegKey;

    match eft_key_wrapped {

        Err(why) => {
            if why.kind() == ErrorKind::NotFound {
                (eft_key,_) = hklm.create_subkey(EFT_REG_PATH)?;
                println!("Regestry subkey created\n");
            } else {
                panic!("{:?}",why);
            }
        }

        Ok(_) => {
			hklm.delete_subkey_all(EFT_REG_PATH)?;
            (eft_key,_) = hklm.create_subkey(EFT_REG_PATH)?;
        }
    }
    println!("Regestry check passed");

    //TODO: Add Recursive search

    let eft_path_string: String = eft_folder_check(&eft_key)?;
    eft_file_check(&eft_key, &eft_path_string)?;

	let eft_path = Path::new(&eft_path_string);

	File::create_with_size(eft_path.join("ConsistencyInfo"), 100)?;
	File::create_with_size(eft_path.join("Uninstall.exe"), 100)?;
	File::create_with_size(eft_path.join("UnityCrashHandler64.exe"), 100)?;

    eft_be_file_check(&eft_path_string)?;

    println!("BattleEye folder check");
    let be_path = Path::new(&eft_path_string).join("BattlEye");
    if !be_path.exists() || !be_path.is_dir() {
        fs::create_dir(&be_path)?;
    }
    println!("BattleEye folder check passed");

    File::create_with_size(be_path.join("BEClient_x64.dll"), 100)?;
    File::create_with_size(be_path.join("BEService_x64.exe"), 100)?;

    println!("All checks passed");

    Ok(())
}
