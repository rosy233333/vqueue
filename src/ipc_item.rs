#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IPCItem {
    pub sender: usize,
    pub msg_type: usize,
    pub data: [usize; 8],
}
