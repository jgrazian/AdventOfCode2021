use crate::prelude::*;

pub struct Day16 {}

fn hex_to_bits(input: &str) -> Vec<bool> {
    input
        .trim()
        .chars()
        .flat_map(|c| {
            let v = c.to_digit(16).unwrap();
            [
                v & (1 << 3) == 8,
                v & (1 << 2) == 4,
                v & (1 << 1) == 2,
                v & (1 << 0) == 1,
            ]
        })
        .collect()
}

fn bits_to_u64(bits: &[bool]) -> u64 {
    bits.iter().rev().enumerate().fold(
        0,
        |acc, (i, b)| if *b { acc + 2u64.pow(i as u32) } else { acc },
    )
}

fn _print_bits(bits: &[bool]) {
    println!(
        "{}",
        bits.iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>()
    )
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    id: u8,
    contents: PacketType,
}

#[derive(Debug, PartialEq)]
enum PacketType {
    Literal(u64),
    Operator(Vec<Packet>),
}

impl Packet {
    fn parse(bits: &[bool]) -> (&[bool], Option<Self>) {
        if bits.len() < 7 {
            return (bits, None);
        }

        let version = bits_to_u64(&bits[0..3]) as u8;
        let id = bits_to_u64(&bits[3..6]) as u8;

        let (remain, contents) = if id == 4 {
            let mut value_bits = Vec::new();
            let mut bits_seen = 0;
            for chunk in bits[6..].chunks_exact(5) {
                value_bits.extend_from_slice(&chunk[1..]);
                bits_seen += 5;
                if !chunk[0] {
                    break;
                }
            }
            (
                &bits[6 + bits_seen..],
                PacketType::Literal(bits_to_u64(&value_bits)),
            )
        } else {
            let op_flag = &bits[6];
            let (rem, sub_contents) = if !*op_flag {
                let content_length = bits_to_u64(&bits[7..22]);
                let mut sub_contents = Vec::new();
                let mut to_parse = &bits[22..];
                let mut parsed_len = 0;
                while parsed_len < content_length {
                    let (rem, packet) = Self::parse(to_parse);
                    sub_contents.push(packet.unwrap());
                    parsed_len += to_parse.len() as u64 - rem.len() as u64;
                    to_parse = rem;
                }
                (to_parse, sub_contents)
            } else {
                let num_packets = bits_to_u64(&bits[7..18]);
                let mut sub_contents = Vec::new();
                let mut to_parse = &bits[18..];
                for _ in 0..num_packets {
                    let (rem, packet) = Self::parse(to_parse);
                    sub_contents.push(packet.unwrap());
                    to_parse = rem;
                }
                (to_parse, sub_contents)
            };

            (rem, PacketType::Operator(sub_contents))
        };

        (
            remain,
            Some(Packet {
                version,
                id,
                contents,
            }),
        )
    }
}

impl From<&[bool]> for Packet {
    fn from(bits: &[bool]) -> Self {
        Self::parse(&bits).1.unwrap()
    }
}

impl Solution for Day16 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let parsed = hex_to_bits(input);
        let packet = Packet::from(parsed.as_slice());

        fn get_version_sum(packet: &Packet) -> i64 {
            let mut sum = packet.version as i64;
            match &packet.contents {
                PacketType::Operator(children) => {
                    children.iter().for_each(|c| sum += get_version_sum(c))
                }
                _ => (),
            }
            sum
        }

        Box::new(get_version_sum(&packet))
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let parsed = hex_to_bits(input);
        let packet = Packet::from(parsed.as_slice());

        fn get_value(packet: &Packet) -> u64 {
            match (packet.id, &packet.contents) {
                (_, PacketType::Literal(v)) => *v,
                (id, PacketType::Operator(sub_packets)) => {
                    let mut values = sub_packets.iter().map(|p| get_value(p));
                    match id {
                        0 => values.sum(),
                        1 => values.product(),
                        2 => values.min().unwrap(),
                        3 => values.max().unwrap(),
                        5 => {
                            if values.next() > values.next() {
                                1
                            } else {
                                0
                            }
                        }
                        6 => {
                            if values.next() < values.next() {
                                1
                            } else {
                                0
                            }
                        }
                        7 => {
                            if values.next() == values.next() {
                                1
                            } else {
                                0
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        Box::new(get_value(&packet))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal() {
        let bits = hex_to_bits("D2FE28");
        let (rem, packet) = Packet::parse(&bits);
        assert_eq!(rem, &[false; 3]);
        assert_eq!(
            packet.unwrap(),
            Packet {
                version: 6,
                id: 4,
                contents: PacketType::Literal(2021)
            }
        );
    }

    #[test]
    fn test_parse_operator_0() {
        let bits = hex_to_bits("38006F45291200");
        let (rem, packet) = Packet::parse(&bits);
        assert_eq!(rem, &[false; 7]);
        assert_eq!(
            packet.unwrap(),
            Packet {
                version: 1,
                id: 6,
                contents: PacketType::Operator(vec![
                    Packet {
                        version: 6,
                        id: 4,
                        contents: PacketType::Literal(10)
                    },
                    Packet {
                        version: 2,
                        id: 4,
                        contents: PacketType::Literal(20)
                    }
                ])
            }
        );
    }

    #[test]
    fn test_parse_operator_1() {
        let bits = hex_to_bits("EE00D40C823060");
        let (rem, packet) = Packet::parse(&bits);
        assert_eq!(rem, &[false; 5]);
        assert_eq!(
            packet.unwrap(),
            Packet {
                version: 7,
                id: 3,
                contents: PacketType::Operator(vec![
                    Packet {
                        version: 2,
                        id: 4,
                        contents: PacketType::Literal(1)
                    },
                    Packet {
                        version: 4,
                        id: 4,
                        contents: PacketType::Literal(2)
                    },
                    Packet {
                        version: 1,
                        id: 4,
                        contents: PacketType::Literal(3)
                    }
                ])
            }
        );
    }

    #[test]
    fn test_part1() {
        let day = Day16 {};
        assert_eq!(day.part1("8A004A801A8002F478").to_string(), "16");
        assert_eq!(day.part1("620080001611562C8802118E34").to_string(), "12");
        assert_eq!(day.part1("C0015000016115A2E0802F182340").to_string(), "23");
    }

    #[test]
    fn test_part2() {
        let day = Day16 {};
        assert_eq!(day.part2("C200B40A82").to_string(), "3");
        assert_eq!(day.part2("04005AC33890").to_string(), "54");
        assert_eq!(day.part2("880086C3E88112").to_string(), "7");
        assert_eq!(day.part2("CE00C43D881120").to_string(), "9");
        assert_eq!(day.part2("D8005AC2A8F0").to_string(), "1");
        assert_eq!(day.part2("F600BC2D8F").to_string(), "0");
        assert_eq!(day.part2("9C005AC2F8F0").to_string(), "0");
        assert_eq!(day.part2("9C0141080250320F1802104A08").to_string(), "1");
    }
}
