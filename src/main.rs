extern crate rand;

mod checks;
mod file;

use core::panic;
use std::io::ErrorKind;
use std::path::Path;
use file::CreateSpecificSizeFile;
use winreg::enums::*;
use winreg::RegKey;
use std::fs::{self, File};
use crate::checks::*;

const EFT_REG_PATH: &str = "Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\EscapeFromTarkov";


fn main() -> anyhow::Result<()> {
    if !is_elevated::is_elevated() {
        panic!("program must be runned with admins rights");
    }

    println!("Regestry check");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let eft_key_wrapped = hklm.open_subkey(EFT_REG_PATH);
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

        Ok(reg_key) => {
           	reg_key.delete_subkey_all("")?;
            eft_key = reg_key;
        }
    }
    println!("Regestry check passed");

    //TODO: Add Recursive search

    let eft_path: String = eft_folder_check(&eft_key)?;
    eft_file_check(&eft_key, &eft_path)?;
    eft_be_file_check(&eft_path)?;

    println!("BattleEye folder check");
    let be_path = Path::new(&eft_path).join("BattlEye");
    if !be_path.exists() || !be_path.is_dir() {
        fs::create_dir(&be_path)?;
    }
    println!("BattleEye folder check passed");

    File::create_with_size(be_path.join("BEClient_x64.dll"), 100)?;
    File::create_with_size(be_path.join("BEService_x64.exe"), 100)?;

    println!("All checks passed");
    Ok(())
}
