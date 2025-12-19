use std::{
    mem,
    sync::{
        Arc,
        atomic::{AtomicIsize, AtomicUsize, Ordering},
    },
};

use crate::map::map_vdso;
use libvqueue::*;

mod map;

const QUEUE_NUM: usize = 16;
const WORKERS_PER_QUEUE: usize = 16;
const DATA_PER_WORKER: usize = 128;

fn main() {
    assert!(QUEUE_NUM <= ARRAY_LEN);
    assert!(WORKERS_PER_QUEUE * DATA_PER_WORKER < QUEUE_LEN);

    env_logger::init();
    log::info!("Starting VDSO test...");
    let map = map_vdso().expect("Failed to map VDSO");

    let mut handles = Vec::new();
    for queue_id in 0..QUEUE_NUM {
        let slot_ref = crate::api::register_queue().expect("Failed to register queue");
        assert!(slot_ref.into_id() == queue_id); // into_id prevents drop
    }
    for queue_id in 0..QUEUE_NUM {
        let data_num: Arc<AtomicIsize> = Arc::new(AtomicIsize::new(0));
        for worker_id in 0..WORKERS_PER_QUEUE {
            let data_num_c = data_num.clone();
            let handle = std::thread::spawn(move || {
                for i in 0..DATA_PER_WORKER {
                    let data = IPCItem {
                        sender: worker_id as u64,
                        msg_type: 0,
                        data: [i as u64; 8],
                    };
                    push(queue_id, data).expect(
                        format!(
                            "Failed to push data in queue {}, worker {}, iter {}",
                            queue_id, worker_id, i
                        )
                        .as_str(),
                    );
                    data_num_c.fetch_add(1, Ordering::AcqRel);
                }
                for i in 0..DATA_PER_WORKER {
                    let data_num = data_num_c.fetch_sub(1, Ordering::AcqRel);
                    if data_num < 0 {
                        println!("data_num < 0 in queue {}, worker {}", queue_id, worker_id);
                        while data_num_c.load(Ordering::Acquire) < 0 {}
                    }
                    let data = pop(queue_id).expect(
                        format!(
                            "Failed to pop data in queue {}, worker {}, iter {}",
                            queue_id, worker_id, i
                        )
                        .as_str(),
                    );
                    assert!(data.msg_type == 0);
                }
            });
            handles.push(handle);
        }
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("Test passed!");
    drop(map);
}
