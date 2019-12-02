use std::str::Chars;

fn main() {
    let input = include_str!("../input");

    first(&input);
    second(&input);
}

fn char_to_bin(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn bin_str_to_int(s: &str) -> u64 {
    u64::from_str_radix(s, 2).unwrap()
}

fn decode(s: &str) -> String {
    s.chars().map(char_to_bin).collect()
}

#[derive(Debug)]
enum PacketValue {
    Lit(u64),
    Op(PacketType, Vec<Packet>),
}

#[derive(Debug)]
enum PacketType {
    Prod,
    Min,
    Max,
    Gt,
    Lt,
    Sum,
    Eq,
}

#[derive(Debug)]
struct Packet {
    version: u64,
    value: PacketValue,
}

fn get_type(n: u32) -> PacketType {
    match n {
        0 => PacketType::Sum,
        1 => PacketType::Prod,
        2 => PacketType::Min,
        3 => PacketType::Max,
        5 => PacketType::Gt,
        6 => PacketType::Lt,
        7 => PacketType::Eq,
        _ => panic!("unknown type id {}", n),
    }
}

fn parse(chars: &mut Chars) -> Option<Packet> {
    if chars.clone().collect::<String>().len() < 10 {
        return None;
    }

    let version: String = chars.by_ref().take(3).collect();
    let type_id_num: u64 = bin_str_to_int(&chars.by_ref().take(3).collect::<String>());

    let val: PacketValue = match type_id_num {
        4 => {
            //4
            let mut s = String::new();

            loop {
                let mut c = chars.by_ref().take(5);
                let is_last = c.by_ref().next().unwrap() == '0';
                s.push_str(&c.collect::<String>());
                if is_last {
                    break;
                }
            }
            PacketValue::Lit(bin_str_to_int(&s) as u64)
        }
        _ => {
            let len = chars.next().unwrap();

            let sub_packets: Vec<Packet> = match len {
                '0' => {
                    let amt = bin_str_to_int(&chars.by_ref().take(15).collect::<String>());
                    let new_chars = chars.take(amt as usize).collect::<String>();
                    let mut new_chars = new_chars.chars();

                    let mut ps = Vec::new();

                    loop {
                        let p = parse(&mut new_chars);

                        if p.is_none() {
                            break;
                        }
                        ps.push(p.unwrap());
                    }

                    ps
                }
                '1' => {
                    let amt = bin_str_to_int(&chars.by_ref().take(11).collect::<String>());

                    let mut ps = Vec::new();

                    for _ in 0..amt {
                        let p = parse(chars);
                        if p.is_none() {
                            break;
                        }
                        ps.push(p.unwrap());
                    }
                    ps
                }
                _ => panic!("unknown len {}", len),
            };

            PacketValue::Op(get_type(type_id_num as u32), sub_packets)
        }
    };

    return Some(Packet {
        // p_type: p_type,
        version: bin_str_to_int(&version),
        value: val,
    });
    // rrrest,
}

fn sum_versions(p: Packet) -> u64 {
    return match p.value {
        PacketValue::Lit(_) => p.version as u64,
        PacketValue::Op(_, packets) => {
            let mut sum = p.version;

            for p in packets {
                sum += sum_versions(p);
            }

            return sum;
        }
    };
}

fn eval_packet(p: &Packet) -> u64 {
    // println!("{:?}", p);
    match &p.value {
        PacketValue::Lit(v) => *v,
        PacketValue::Op(op, packets) => match op {
            PacketType::Sum => {
                let mut sum = 0;

                for p in packets {
                    sum += eval_packet(&p);
                }

                return sum;
            }
            PacketType::Prod => {
                let mut prod = 1;

                for p in packets {
                    prod *= eval_packet(&p);
                }

                return prod;
            }
            PacketType::Min => {
                let mut vals = Vec::new();

                for p in packets {
                    vals.push(eval_packet(&p));
                }

                return *vals.iter().min().unwrap();
            }
            PacketType::Max => {
                let mut vals = Vec::new();

                for p in packets {
                    vals.push(eval_packet(&p));
                }

                return *vals.iter().max().unwrap();
            }
            PacketType::Gt => {
                let p1 = eval_packet(&packets[0]);
                let p2 = eval_packet(&packets[1]);

                return if p1 > p2 { 1 } else { 0 };
            }
            PacketType::Lt => {
                let p1 = eval_packet(&packets[0]);
                let p2 = eval_packet(&packets[1]);

                return if p1 < p2 { 1 } else { 0 };
            }
            PacketType::Eq => {
                let p1 = eval_packet(&packets[0]);
                let p2 = eval_packet(&packets[1]);

                return if p1 == p2 { 1 } else { 0 };
            }
        },
    }
}

fn first(p: &str) -> u64 {
    let decoded = decode(p);
    let mut chars = decoded.chars();
    let mut version_sum = 0;

    let mut packets: Vec<Packet> = Vec::new();

    let packet = parse(&mut chars);

    packets.push(packet.unwrap());

    for p in packets {
        version_sum += sum_versions(p);
    }

    println!("{}", version_sum);

    return version_sum;
}

fn second(p: &str) -> u64 {
    let decoded = decode(p);
    let mut chars = decoded.chars();
    let mut version_sum = 0;

    let mut packets: Vec<Packet> = Vec::new();

    let packet = parse(&mut chars);

    packets.push(packet.unwrap());

    for p in packets {
        version_sum += eval_packet(&p);
    }

    println!("{}", version_sum);

    return version_sum;
}

#[cfg(test)]
mod tests {
    use crate::{first, second};

    #[test]
    fn test() {
        assert_eq!(first(&"8A004A801A8002F478"), 16);
        assert_eq!(first(&"620080001611562C8802118E34"), 12);
        assert_eq!(first(&"C0015000016115A2E0802F182340"), 23);
        assert_eq!(first(&"A0016C880162017C3686B18A3D4780"), 31);

        assert_eq!(second(&"C200B40A82"), 3);
        assert_eq!(second(&"04005AC33890"), 54);
        assert_eq!(second(&"880086C3E88112"), 7);
        assert_eq!(second(&"CE00C43D881120"), 9);
        assert_eq!(second(&"D8005AC2A8F0"), 1);
        assert_eq!(second(&"F600BC2D8F"), 0);
        assert_eq!(second(&"9C005AC2F8F0"), 0);
        assert_eq!(second(&"9C0141080250320F1802104A08"), 1);
    }
}
