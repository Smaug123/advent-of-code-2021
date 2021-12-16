pub mod day_16 {

    pub(crate) fn parse(s: &str) -> Vec<u8> {
        let s = s.trim_end();
        let mut answer = Vec::with_capacity(s.len() * 4);

        for c in s.chars() {
            let number = if ('0'..='9').contains(&c) {
                c as u8 - b'0'
            } else {
                (c as u8 - b'A') + 10
            };
            answer.push(number / 8);
            answer.push((number / 4) % 2);
            answer.push((number / 2) % 2);
            answer.push(number % 2);
        }

        answer
    }

    struct OperatorPacket {
        version: u8,
        type_id: u8,
        sub_packets: Vec<Packet>,
    }

    struct LiteralPacket {
        version: u8,
        value: u64,
    }

    enum Packet {
        Literal(LiteralPacket),
        Operator(OperatorPacket),
    }

    fn consume_literal(bits: &[u8], places: u8) -> u32 {
        let mut answer = 0;
        for &bit in bits.iter().take(places as usize) {
            answer *= 2;
            answer += bit as u32;
        }
        answer
    }

    fn chomp_literal(bits: &[u8]) -> (u8, &[u8], bool) {
        (
            consume_literal(&bits[1..=4], 4) as u8,
            &bits[5..],
            bits[0] == 1,
        )
    }

    fn parse_packet(bits: &[u8]) -> (Packet, &[u8]) {
        let version = 4 * bits[0] + 2 * bits[1] + bits[2];
        let type_id = 4 * bits[3] + 2 * bits[4] + bits[5];
        match type_id {
            4 => {
                let mut value = 0;
                let mut should_continue = true;
                let mut bits = &bits[6..];
                while should_continue {
                    let (byte, bits_2, should_continue_2) = chomp_literal(bits);
                    should_continue &= should_continue_2;
                    bits = bits_2;
                    value = value * 16 + byte as u64;
                }
                (Packet::Literal(LiteralPacket { version, value }), bits)
            }
            _ => match bits[6] {
                0 => {
                    let length = consume_literal(&bits[7..], 15);
                    let unparsed = &bits[7 + 15 + (length as usize)..];
                    let to_parse = &bits[7 + 15..7 + 15 + length as usize];
                    let sub_packets = parse_packets(to_parse);
                    (
                        Packet::Operator(OperatorPacket {
                            version,
                            type_id,
                            sub_packets,
                        }),
                        unparsed,
                    )
                }
                1 => {
                    let packets = consume_literal(&bits[7..], 11);
                    let mut sub_packets = Vec::with_capacity(packets as usize);
                    let mut bits = &bits[7 + 11..];
                    for _ in 0..packets {
                        let (packet, next_bits) = parse_packet(bits);
                        bits = next_bits;
                        sub_packets.push(packet);
                    }
                    (
                        Packet::Operator(OperatorPacket {
                            version,
                            type_id,
                            sub_packets,
                        }),
                        bits,
                    )
                }
                c => panic!("Expected a bit, got {}", c),
            },
        }
    }

    fn parse_packets(bits: &[u8]) -> Vec<Packet> {
        let mut bits = bits;
        let mut answer = Vec::new();
        while bits.contains(&1) {
            let (packet, unparsed) = parse_packet(bits);
            answer.push(packet);
            bits = unparsed;
        }

        answer
    }

    fn sum_versions(p: &Packet) -> u32 {
        match p {
            Packet::Literal(p) => p.version as u32,
            Packet::Operator(p) => {
                p.sub_packets.iter().map(sum_versions).sum::<u32>() + p.version as u32
            }
        }
    }

    fn evaluate(p: &Packet) -> u64 {
        match p {
            Packet::Literal(p) => p.value,
            Packet::Operator(p) => match p.type_id {
                0 => p.sub_packets.iter().map(evaluate).sum(),
                1 => p.sub_packets.iter().map(evaluate).product(),
                2 => p.sub_packets.iter().map(evaluate).min().unwrap() as u64,
                3 => p.sub_packets.iter().map(evaluate).max().unwrap() as u64,
                5 => {
                    let v1 = evaluate(&p.sub_packets[0]);
                    let v2 = evaluate(&p.sub_packets[1]);
                    if v1 > v2 {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let v1 = evaluate(&p.sub_packets[0]);
                    let v2 = evaluate(&p.sub_packets[1]);
                    if v1 < v2 {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let v1 = evaluate(&p.sub_packets[0]);
                    let v2 = evaluate(&p.sub_packets[1]);
                    if v1 == v2 {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("Unexpected type"),
            },
        }
    }

    pub fn input() -> Vec<u8> {
        parse(include_str!("../input.txt"))
    }

    pub fn part_1(data: &[u8]) -> u32 {
        sum_versions(&parse_packets(data)[0])
    }

    pub fn part_2(data: &[u8]) -> u64 {
        evaluate(&parse_packets(data)[0])
    }
}

#[cfg(test)]
mod tests {
    use super::day_16::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("D2FE28"),
            [1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        );
    }

    #[test]
    fn part1_known() {
        // Version sums
        let data = parse("8A004A801A8002F478");
        assert_eq!(part_1(&data), 16);
        let data = parse("620080001611562C8802118E34");
        assert_eq!(part_1(&data), 12);
        let data = parse("C0015000016115A2E0802F182340");
        assert_eq!(part_1(&data), 23);
        let data = parse("A0016C880162017C3686B18A3D4780");
        assert_eq!(part_1(&data), 31);
    }

    #[test]
    fn part2_known() {
        let data = parse("D2FE28");
        assert_eq!(part_2(&data), 2021);
        let data = parse("38006F45291200");
        assert_eq!(part_2(&data), 1);
        let data = parse("EE00D40C823060");
        assert_eq!(part_2(&data), 3);
        let data = parse("C200B40A82");
        assert_eq!(part_2(&data), 3);
        let data = parse("04005AC33890");
        assert_eq!(part_2(&data), 54);
        let data = parse("880086C3E88112");
        assert_eq!(part_2(&data), 7);
        let data = parse("CE00C43D881120");
        assert_eq!(part_2(&data), 9);
        let data = parse("D8005AC2A8F0");
        assert_eq!(part_2(&data), 1);
        let data = parse("F600BC2D8F");
        assert_eq!(part_2(&data), 0);
        let data = parse("9C005AC2F8F0");
        assert_eq!(part_2(&data), 0);
        let data = parse("9C0141080250320F1802104A08");
        assert_eq!(part_2(&data), 1);
    }

    #[test]
    fn test_day_16() {
        let input = input();
        assert_eq!(part_1(&input), 923);
        assert_eq!(part_2(&input), 258888628940);
    }
}
