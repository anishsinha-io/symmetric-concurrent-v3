use positioned_io::{ReadAt, WriteAt};

pub fn write_bytes(abs_path: &str, bytes: [u8; 512], offset: u64) -> Option<usize> {
    use std::path::Path;
    let path = Path::new(abs_path);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(false)
        .open(path)
        .unwrap();

    let bytes_written = file.write_at(offset, &bytes);
    match bytes_written {
        Err(_) => None,
        Ok(bytes_written) => Some(bytes_written),
    }
}

pub fn read_bytes(abs_path: &str, buffer: &mut [u8; 512], offset: u64) -> () {
    let path = std::path::Path::new(abs_path);
    let file = std::fs::OpenOptions::new()
        .read(true)
        .create(false)
        .truncate(false)
        .open(path)
        .unwrap();
    file.read_at(offset, buffer).unwrap();
}

pub fn truncate(abs_path: &str) {
    let path = std::path::Path::new(abs_path);
    std::fs::OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(true)
        .open(path)
        .unwrap();
}
