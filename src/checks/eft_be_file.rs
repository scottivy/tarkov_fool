use std::{fs::File, path::Path};
use crate::file::CreateSpecificSizeFile;

const EFT_BE_SIZE: usize = 1024000;

pub(crate) fn eft_be_file_check(eft_path: &String) -> anyhow::Result<()> {
    println!("EscapeFromTarkov_BE.exe check");
    let eft_be_file_path = Path::new(eft_path).join("EscapeFromTarkov_BE.exe");
    if !eft_be_file_path.exists() {
        File::create_with_size(eft_be_file_path, EFT_BE_SIZE)?;
        println!("EscapeFromTarkov_BE.exe created with size: {}", EFT_BE_SIZE);
    } else {
        let eft_be_file = File::open(eft_be_file_path)?;
        if eft_be_file.metadata()?.len() < EFT_BE_SIZE as u64 {
            panic!("Wrong size of EscapeFromTarkov_BE.exe: current size is {} expected size is {}", eft_be_file.metadata()?.len(), EFT_BE_SIZE)
        }
    }
    println!("EscapeFromTarkov_BE.exe check passed");

    Ok(())
}
