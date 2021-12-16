#![allow(unused_imports)]
use utils::{Input, Solution};

trait ToDecimal {
    fn to_decimal(&self) -> usize;
}

impl ToDecimal for [usize] {
    fn to_decimal(&self) -> usize {
        self.iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + x * 2_usize.pow(i as u32))
    }
}

impl ToDecimal for Vec<usize> {
    fn to_decimal(&self) -> usize {
        self.as_slice().to_decimal()
    }
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: usize,
    id: usize,
    size: usize,
    data: PacketData,
}

#[derive(Debug, PartialEq)]
enum PacketData {
    Literal { value: usize },
    Op { 
        op: usize,
        subpackets: Vec<Packet>,
    },
}

impl Packet {
    pub fn new(data: &[usize]) -> Packet {
        let version = data[0..3].to_decimal();
        let id = data[3..6].to_decimal();

        if id == 4 {
            let mut indicator = 6;
            let mut value = Vec::new();

            let mut last = false;
            while !last {
                last = data[indicator] == 0;

                value.extend_from_slice(&data[indicator + 1..indicator + 5]);

                indicator += 5;
            }

            let packet_end = indicator;
            return Packet {
                version,
                id,
                size: packet_end,
                data: PacketData::Literal {
                    value: value.to_decimal(),
                },
            };
        }

        // Otherwise, it's an Op packet
        let length_type_id = data[6];
        if length_type_id == 0 {
            let subpacket_length = data[7..7 + 15].to_decimal();
            let total_packet_length = 3 + 3 + 1 + 15 + subpacket_length;

            let mut subpackets = Vec::new();

            let mut subpacket_start = 7 + 15;
            while subpacket_start != total_packet_length {
                let subpacket = Packet::new(&data[subpacket_start..]);

                subpacket_start += subpacket.size;

                subpackets.push(subpacket);
            }

            Packet {
                version,
                id,
                size: total_packet_length,
                data: PacketData::Op { op: id, subpackets },
            }
        } else {
            assert!(length_type_id == 1);

            let num_subpackets = data[7 .. 7 + 11].to_decimal();

            let mut subpackets = Vec::new();
            let mut subpacket_start = 7 + 11;
            for _ in 0..num_subpackets {
                let subpacket = Packet::new(&data[subpacket_start..]);

                subpacket_start += subpacket.size;

                subpackets.push(subpacket);
            }

            Packet {
                version,
                id,
                size: subpacket_start,
                data: PacketData::Op { op: id, subpackets },
            }
        }
    }

    pub fn sum_versions(&self) -> usize {
        match &self.data {
            PacketData::Literal { .. }  => self.version,
            PacketData::Op { subpackets, .. } => self.version + subpackets.iter().map(|p| p.sum_versions()).sum::<usize>(),
        }
    }

    pub fn value(&self) -> usize {
        match &self.data {
            PacketData::Literal { value } => *value,
            PacketData::Op { op, subpackets } => {
                let mut values = subpackets.iter().map(|p| p.value());
                match op {
                    0 => values.sum(),
                    1 => values.product(),
                    2 => values.min().unwrap(),
                    3 => values.max().unwrap(),

                    5 => (values.next().unwrap() >  values.next().unwrap()) as usize,
                    6 => (values.next().unwrap() <  values.next().unwrap()) as usize,
                    7 => (values.next().unwrap() == values.next().unwrap()) as usize,

                    _ => unreachable!(),
                }
            }
        }
    }
}

#[cfg(test)]
mod test_packet {
    use super::*;

    #[test]
    fn test_value_packet_example_1() {
        // Add trailing chars to test that length is calculated correctly
        for s in ["D2FE28", "D2FE28123", "D2FE28F"] {
            let packet = Packet::new(&hex_str_to_bit(s));
            assert_eq!(packet.version, 6);
            assert_eq!(packet.id, 4);
            assert_eq!(packet.size, 21);
            assert_eq!(packet.data, PacketData::Literal { value: 2021 });
        }
    }

    #[test]
    fn test_op_subpacket_length_type_id_0() {
        let packet = Packet::new(&hex_str_to_bit("38006F45291200"));
        assert_eq!(packet.version, 1);
        assert_eq!(packet.id, 6);
        assert_eq!(packet.size, 3 + 3 + 1 + 15 + 27);
        assert_eq!(
            packet.data,
            PacketData::Op {
                op: 6,
                subpackets: vec![
                    Packet {
                        version: 6,
                        id: 4,
                        size: 11,
                        data: PacketData::Literal { value: 10 },
                    },
                    Packet {
                        version: 2,
                        id: 4,
                        size: 16,
                        data: PacketData::Literal { value: 20 },
                    },
                ]
            }
        )
    }

    #[test]
    fn test_op_subpacket_length_type_id_1() {
        let packet = Packet::new(&hex_str_to_bit("EE00D40C823060"));
        assert_eq!(packet.version, 7);
        assert_eq!(packet.id, 3);
        assert_eq!(packet.size, 3 + 3 + 1 + 11 + 11 + 11 + 11);
        assert_eq!(
            packet.data,
            PacketData::Op {
                op: 3,
                subpackets: vec![
                    Packet {
                        version: 2,
                        id: 4,
                        size: 11,
                        data: PacketData::Literal { value: 1 },
                    },
                    Packet {
                        version: 4,
                        id: 4,
                        size: 11,
                        data: PacketData::Literal { value: 2 },
                    },
                    Packet {
                        version: 1,
                        id: 4,
                        size: 11,
                        data: PacketData::Literal { value: 3 },
                    },
                ]
            }
        )
    }

    #[test]
    fn version_sum() {
        assert_eq!(Packet::new(&hex_str_to_bit("8A004A801A8002F478")).sum_versions(), 16);
        assert_eq!(Packet::new(&hex_str_to_bit("620080001611562C8802118E34")).sum_versions(), 12);
        assert_eq!(Packet::new(&hex_str_to_bit("C0015000016115A2E0802F182340")).sum_versions(), 23);
        assert_eq!(Packet::new(&hex_str_to_bit("A0016C880162017C3686B18A3D4780")).sum_versions(), 31);
    }
}

fn hex_str_to_bit(s: &str) -> Vec<usize> {
    s.replace("\n", "")
        .chars()
        .map(|c| match c {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => unreachable!("Got unexpected chat {}", c),
        })
        .flatten()
        .collect()
}
//}}}

pub fn solve(input: Input) -> Solution {
    let bits = hex_str_to_bit(&input.raw);

    let packet = Packet::new(&bits);

    Solution::new(packet.sum_versions(), packet.value())
}
