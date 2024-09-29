use std::{cmp, fs::File, io::{BufWriter, Write}};

use rand::Rng;

pub(crate) trait CreateSpecificSizeFile {
    fn create_with_size<P>(path: P, size: usize) -> anyhow::Result<()>
        where
            P: AsRef<std::path::Path>;
}

impl CreateSpecificSizeFile for File {
    fn create_with_size<P>(path: P, size: usize) -> anyhow::Result<()>
        where
            P: AsRef<std::path::Path>
    {
        let f = File::create(path)?;
        let mut writer = BufWriter::new(f);
        let mut rng = rand::thread_rng();
        let mut buffer = [0; 1024];
        let mut remaining_size = size;

        while remaining_size > 0 {
            let to_write = cmp::min(remaining_size, buffer.len());
            let buffer=  &mut buffer[..to_write];
            rng.fill(buffer);
            writer.write(buffer)?;

            remaining_size -= to_write;
        }

        Ok(())
    }
}
