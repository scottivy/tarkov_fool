extern crate rand;

use core::panic;
use std::io::{BufWriter, ErrorKind, Write};
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;
use std::fs::{self, File};
use rand::Rng;
use std::cmp;

const EFT_REG_PATH: &str = "Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\EscapeFromTarkov";
const EFT_SIZE: usize = 647 * 1000;
const EFT_BE_SIZE: usize = 1024000;


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

        Ok(_) => {
            hklm.delete_subkey(EFT_REG_PATH)?;
            (eft_key,_) = hklm.create_subkey(EFT_REG_PATH)?;
        }
    }
    println!("Regestry check passed");


    println!("EFT regestry value check");
    let eft_path_wrapped: Result<String, std::io::Error> = eft_key.get_value("InstallLocation");
    let eft_path: String;
    match eft_path_wrapped {
        Err(why) => {
            if why.kind() == ErrorKind::NotFound {
                let path = std::env::current_dir()?.into_os_string();
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


    println!("EscapeFromTarkov.exe check");
    let eft_file_path = Path::new(&eft_path).join("EscapeFromTarkov.exe");
    if !eft_file_path.exists() {
        let path = std::env::current_dir()?.join("EscapeFromTarkov.exe");
        if !path.exists() {
            panic!("EscapeFromTarkov.exe doesn't exist");
        }
        eft_key.set_value("InstallLocation", &path.into_os_string())?;
        println!("EFT regestry InstallLocation created");
    } else {
        let eft_file = File::open(&eft_file_path)?;
        if eft_file.metadata().unwrap().len() < EFT_SIZE as u64 {
            panic!("Wrong size of EscapeFromTarkov.exe: current size is {} expected size is {}", eft_file.metadata().unwrap().len(), EFT_SIZE)
        }
        eft_key.set_value("DisplayIcon", &eft_file_path.into_os_string())?;
        println!("EFT regestry DisplayIcon created or sets to new value");
    }
    println!("EscapeFromTarkov.exe passed");


    println!("EscapeFromTarkov_BE.exe check");
    let eft_be_file_path = Path::new(&eft_path).join("EscapeFromTarkov_BE.exe");
    if !eft_be_file_path.exists() {
        create_file(eft_be_file_path, EFT_BE_SIZE)?;
        println!("EscapeFromTarkov_BE.exe created with size: {}", EFT_BE_SIZE);
    } else {
        let eft_be_file = File::open(eft_be_file_path)?;
        if eft_be_file.metadata().unwrap().len() < EFT_BE_SIZE as u64 {
            panic!("Wrong size of EscapeFromTarkov_BE.exe: current size is {} expected size is {}", eft_be_file.metadata().unwrap().len(), EFT_BE_SIZE)
        }
    }
    println!("EscapeFromTarkov_BE.exe check passed");


    println!("BattleEye folder check");
    let be_path = Path::new(&eft_path).join("BattlEye");
    if !be_path.exists() || !be_path.is_dir() {
        fs::create_dir(&be_path)?;
    }
    println!("BattleEye folder check passed");

    create_file(be_path.join("BEClient_x64.dll"), 100)?;
    create_file(be_path.join("BEService_x64.exe"), 100)?;

    println!("All checks passed");
    Ok(())
}

fn create_file<P>(path: P, size: usize) -> anyhow::Result<()>
    where
        P: AsRef<std::path::Path>
{
    let f = File::create(path).unwrap();
    let mut writer = BufWriter::new(f);
    let mut rng = rand::thread_rng();
    let mut buffer = [0; 1024];
    let mut remaining_size = size;

    while remaining_size > 0 {
        let to_write = cmp::min(remaining_size, buffer.len());
        let buffer=  &mut buffer[..to_write];
        rng.fill(buffer);
        writer.write(buffer).unwrap();

        remaining_size -= to_write;
    }

    Ok(())
}
