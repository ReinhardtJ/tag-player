
pub enum DecoderCommand {
    Seek(u64), // seek to sample position
    Stop,
}