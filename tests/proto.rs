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

use bufread::BufReader;
use rand::prelude::*;
use std::{
    io::{BufRead, Result},
    mem::size_of,
};

/// A source for pseudo packages consisting of:
/// - A u16 header containing the total packet_len
/// - A u8 byte payload of packet_len - size_of(u16) bytes
/// And packages having a length-range from size_of(u16) to u16::MAX.
pub struct Source {
    data: Vec<u8>,
}

impl Source {
    /// Creates a new source with fixed packet lengths.
    pub fn fixed(num_packets: usize) -> Self {
        let mut data = Vec::new();

        for i in 0..num_packets {
            let packet_len: u16 = (size_of::<u16>() + i) as u16;
            data.append(&mut Self::create_packet(packet_len));
        }

        Source { data }
    }

    /// Creates a new source with random packet lengths.
    pub fn random(num_packets: usize) -> Self {
        let mut data = Vec::new();

        for _ in 0..num_packets {
            let packet_len: u16 = rand::thread_rng().gen_range(size_of::<u16>() as u16..u16::MAX);
            data.append(&mut Self::create_packet(packet_len));
        }

        Source { data }
    }

    fn create_packet(packet_len: u16) -> Vec<u8> {
        assert!(packet_len >= size_of::<u16>() as u16);

        let mut packet: Vec<u8> = Vec::with_capacity(packet_len as usize);
        unsafe {
            packet.set_len(packet_len as usize);
        }

        let header = packet_len.to_be_bytes().to_vec();
        packet[0] = header[0];
        packet[1] = header[1];
        rand::thread_rng().fill_bytes(&mut packet[size_of::<u16>()..]);

        packet
    }

    /// Returns the contained data slice.
    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    /// Returns the length of the contained data.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub struct Parser<'a> {
    reader: BufReader<&'a [u8]>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser for the given source.
    pub fn new(reader: BufReader<&'a [u8]>) -> Self {
        Parser { reader }
    }

    /// Parses the next package from the source, if available.
    /// Returns the length of the package being parsed, or
    /// a zero-length if at EOF.
    pub fn next(&mut self) -> Result<usize> {
        let buffer = self.reader.fill_buf()?;
        if buffer.is_empty() {
            return Ok(0);
        }

        let mut header = [0; size_of::<u16>() as usize];
        header[0] = buffer[0];
        header[1] = buffer[1];
        let packet_len = u16::from_be_bytes(header);
        self.reader.consume(packet_len as usize);

        Ok(packet_len as usize)
    }

    /// Runs a parser and returns the total number of packets and bytes being read.
    pub fn run(parser: &mut Parser) -> Result<(usize, usize)> {
        let mut result: (usize, usize) = (0, 0);

        loop {
            let size = parser.next()?;
            if size == 0 {
                break;
            }

            result.0 += 1;
            result.1 += size;
        }

        Ok(result)
    }
}
