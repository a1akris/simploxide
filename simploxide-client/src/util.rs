pub fn cast_file_size(file_size: u64) -> std::io::Result<usize> {
    file_size.try_into().map_err(file_is_too_large)
}

pub fn file_is_too_large<E>(reason: E) -> std::io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    std::io::Error::new(std::io::ErrorKind::FileTooLarge, reason)
}
