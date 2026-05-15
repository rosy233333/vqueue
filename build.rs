fn main() {
    vdso_helper::mut_cfg! {
        /// 单条队列长度
        const QUEUE_LEN: usize = 4096;
        /// 数组长度，决定同时可用的队列数量
        const ARRAY_LEN: usize = 64;
    }
}
