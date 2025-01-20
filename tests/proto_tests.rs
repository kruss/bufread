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
    use super::proto::Parser;
    use super::proto::Source;
    use bufread::BufReader;
    use rand::prelude::*;

    #[test]
    fn test_fixed() {
        let num_packets = 1000;
        let source = Source::fixed(num_packets);

        let min_size = u16::MAX as usize;
        let max_size = 3 * min_size;
        let reader = BufReader::new(max_size, min_size, source.data());
        let mut parser = Parser::new(reader);

        match Parser::run(&mut parser) {
            Ok(result) => {
                assert_eq!(num_packets, result.0);
                assert_eq!(source.data().len(), result.1);
            }
            Err(error) => {
                panic!("{}", error);
            }
        }
    }

    #[test]
    fn test_random() {
        let num_packets = 1000;
        let source = Source::random(num_packets);

        let min_size = u16::MAX as usize;
        let max_size = rand::thread_rng().gen_range(min_size..(3 * min_size));
        let reader = BufReader::new(max_size, min_size, source.data());
        let mut parser = Parser::new(reader);

        match Parser::run(&mut parser) {
            Ok(result) => {
                assert_eq!(num_packets, result.0);
                assert_eq!(source.data().len(), result.1);
            }
            Err(error) => {
                panic!("{}", error);
            }
        }
    }
}
