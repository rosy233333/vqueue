#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IPCItem {
    pub sender: u64,
    pub msg_type: u64,
    pub data: [u64; 8],
}
