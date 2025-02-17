#[cfg(test)]
mod tests {
    use bufread::DeqBuffer;
    use proptest::prelude::*;
    use proptest::test_runner::FileFailurePersistence;
    use std::{cmp::min, collections::HashMap};

    proptest! {
        // Proptest for the [bufread::DeqBuffer] with random API calls.
        #![proptest_config(ProptestConfig::with_failure_persistence(FileFailurePersistence::Off))]
        #[test]
        fn buffer_proptest(
            total_size in 100usize..1000usize,
            chunk_size in 1usize..100usize,
            rand1 in 1usize..100usize,
            rand2 in 1usize..100usize,
        ) {
            let mut stat: HashMap<usize, (usize, usize)> = HashMap::new();
            for call in 0..4 {
                stat.insert(call, (0, 0));
            }

            let mut buffer = DeqBuffer::new(total_size);
            let mut storage = Vec::with_capacity(total_size);

            for i in 0..rand1 {
                for j in 0..rand2 {
                    let step = i + j;
                    let call = (i * j) % 4;
                    match call {
                        0 => { /* WRITE */
                            let size = min(buffer.write_available(), chunk_size);
                            let mut chunk: Vec<u8> = vec![step as u8; size];
                            assert_eq!(size, buffer.write_from(&chunk));
                            storage.append(&mut chunk);
                            if let Some(item) = stat.get_mut(&0) {
                                item.0 += 1;
                                item.1 += size;
                            }
                        },
                        1 => { /* READ */
                            let size = min(buffer.read_available(), chunk_size);
                            let mut chunk: Vec<u8> = vec![0u8; size];
                            assert_eq!(size, buffer.read_to(&mut chunk));
                            let stored: Vec<_> = storage.drain(..size).collect();
                            assert_eq!(stored, chunk);
                            if let Some(item) = stat.get_mut(&1) {
                                item.0 += 1;
                                item.1 += size;
                            }
                        },
                        2 => { /* FLUSH */
                            let size = total_size - (buffer.read_available() + buffer.write_available());
                            assert_eq!(size, buffer.flush());
                            if let Some(item) = stat.get_mut(&2) {
                                item.0 += 1;
                                item.1 += size;
                            }
                        },
                        3 => { /* CLEAR */
                            let size = buffer.read_available();
                            assert_eq!(size, buffer.clear());
                            storage.clear();
                            if let Some(item) = stat.get_mut(&3) {
                                item.0 += 1;
                                item.1 += size;
                            }
                        },
                        _ => { panic!("unexpected") }
                    }
                }
            }

            println!("proptest");
            for call in 0..4 {
                if let Some(item) = stat.get(&call) {
                    println!("- call {} => times: {}, bytes: {}", call, item.0, item.1)
                }
            }
        }
    }
}
