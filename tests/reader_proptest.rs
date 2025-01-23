#![allow(dead_code)]

// Copyright (c) 2025 ESR Labs GmbH. All rights reserved.
//
// NOTICE:  All information contained herein is, and remains
// the property of E.S.R.Labs and its suppliers, if any.
// The intellectual and technical concepts contained herein are
// proprietary to E.S.R.Labs and its suppliers and may be covered
// by German and Foreign Patents, patents in process, and are protected
// by trade secret or copyright law.
// Dissemination of this information or reproduction of this material
// is strictly forbidden unless prior written permission is obtained
// from E.S.R.Labs.

#[path = "./proto.rs"]
mod proto;

#[cfg(test)]
mod tests {
    use super::proto::{Parser, Source};
    use bufread::BufReader;
    use proptest::prelude::*;
    use proptest::test_runner::FileFailurePersistence;

    proptest! {
        #![proptest_config(ProptestConfig::with_failure_persistence(FileFailurePersistence::Off))]
        #[test]
        #[ignore = "this is long running task"]
        fn reader_proptest(
            num_packets in 1usize..100,
            max_size in u16::MAX as usize..(3 * u16::MAX as usize)
        ) {
            let source = Source::random(num_packets);
            let min_size = u16::MAX as usize;
            let reader = BufReader::new(max_size, min_size, source.data());
            let mut parser = Parser::new(reader);

            match Parser::run(&mut parser) {
                Ok(result) => {
                    if num_packets != result.0 {
                        panic!("num packets does not match: {} != {}", num_packets, result.0);
                    }
                    if source.len() != result.1 {
                        panic!("source len does not match: {} != {}", source.len(), result.1);
                    }
                }
                Err(error) => {
                    panic!("{}", error);
                }
            }
        }
    }
}
