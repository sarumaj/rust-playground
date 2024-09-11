fn main() {
    let addresses = [
        IpAddr::V4(IpV4Addr::from("127.0.0.1")),
        IpAddr::V6(IpV6Addr::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334")),
        IpAddr::V6(IpV6Addr::from("2001:0db8::ff00:0042:8329")),
        IpAddr::V6(IpV6Addr::from("::1")),
        IpAddr::V4(IpV4Addr::from("192.168.1")),
        IpAddr::V4Network(IpV4Network::from("192.168.1.92/24")),
    ];

    for address in addresses.iter() {
        match address {
            IpAddr::V4(ip) => println!("IPv4 address: {:>31}", ip.to_string()),
            IpAddr::V4Network(ip) => println!(
                "IPv4 network: {:>31} ({}, {} - {})",
                ip.to_string(),
                ip.mask().to_string(),
                ip.first_address().to_string(),
                ip.last_address().to_string()
            ),
            IpAddr::V6(ip) => println!(
                "IPv6 address: {:>31} ({})",
                ip.to_string(true),
                ip.to_string(false)
            ),
        }
    }
}

enum IpAddr {
    V4(IpV4Addr),
    V4Network(IpV4Network),
    V6(IpV6Addr),
}

struct IpV4Addr {
    octets: [u8; 4],
}

impl IpV4Addr {
    fn from(s: &str) -> IpV4Addr {
        let mut octets: Vec<u8> = s.split(".").map(|octet| octet.parse().unwrap()).collect();
        while octets.len() != 4 {
            octets.push(0);
        }
        IpV4Addr::new([octets[0], octets[1], octets[2], octets[3]])
    }

    fn new(octets: [u8; 4]) -> IpV4Addr {
        IpV4Addr { octets }
    }

    fn to_string(&self) -> String {
        self.octets
            .iter()
            .map(|octet| octet.to_string()) // Convert each octet to octal
            .collect::<Vec<String>>()
            .join(".")
    }
}

impl Eq for IpV4Addr {}

impl PartialEq for IpV4Addr {
    fn eq(&self, other: &Self) -> bool {
        self.octets == other.octets
    }
}

impl PartialOrd for IpV4Addr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.octets.cmp(&other.octets))
    }
}

impl Ord for IpV4Addr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.octets.cmp(&other.octets)
    }
}

impl std::fmt::Display for IpV4Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

struct IpV4Network {
    address: IpV4Addr,
    prefix: u8,
}

impl IpV4Network {
    fn broadcast_address(&self) -> IpV4Addr {
        let mask = !0 << (32 - self.prefix);
        let network = u32::from_be_bytes(self.address.octets) & mask;
        let broadcast = network | !mask;
        IpV4Addr::new(broadcast.to_be_bytes())
    }

    fn contains(&self, ip: &IpV4Addr) -> bool {
        let mask = !0 << (32 - self.prefix);
        let network = u32::from_be_bytes(self.address.octets) & mask;
        let other = u32::from_be_bytes(ip.octets) & mask;
        network == other
    }

    fn first_address(&self) -> IpV4Addr {
        let network = u32::from_be_bytes(self.network_address().octets);
        let first_usable = network + 1;
        IpV4Addr::new(first_usable.to_be_bytes())
    }

    fn from(s: &str) -> IpV4Network {
        let mut parts: Vec<&str> = s.split('/').collect();
        let ip = IpV4Addr::from(parts[0]);
        if parts.len() != 2 {
            parts.push("32")
        }
        let prefix = parts[1].parse().unwrap();
        IpV4Network::new(ip, prefix)
    }

    fn last_address(&self) -> IpV4Addr {
        let broadcast = u32::from_be_bytes(self.broadcast_address().octets);
        IpV4Addr::new((broadcast - 1).to_be_bytes())
    }

    fn network_address(&self) -> IpV4Addr {
        let mask = !0 << (32 - self.prefix);
        let network = u32::from_be_bytes(self.address.octets) & mask;
        IpV4Addr::new(network.to_be_bytes())
    }

    fn new(address: IpV4Addr, prefix: u8) -> IpV4Network {
        let raw = address.to_string();
        let mut net = IpV4Network { address, prefix };
        net.prefix = if net.prefix > 32 { 32 } else { net.prefix };
        net.address = net.network_address();
        assert!(net.contains(&IpV4Addr::from(&raw)));
        net
    }

    fn mask(&self) -> IpV4Addr {
        let mask: u32 = !0 << (32 - self.prefix);
        IpV4Addr::new(mask.to_be_bytes())
    }

    fn to_string(&self) -> String {
        format!("{}/{}", self.address.to_string(), self.prefix)
    }
}

impl Eq for IpV4Network {}

impl PartialEq for IpV4Network {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.prefix == other.prefix
    }
}

impl PartialOrd for IpV4Network {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.prefix.cmp(&other.prefix) {
            std::cmp::Ordering::Equal => Some(self.address.cmp(&other.address)),
            ordering => Some(ordering),
        }
    }
}

impl Ord for IpV4Network {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.prefix.cmp(&other.prefix) {
            std::cmp::Ordering::Equal => self.address.cmp(&other.address),
            ordering => ordering,
        }
    }
}

impl std::fmt::Display for IpV4Network {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

struct IpV6Addr {
    hextets: [u16; 8],
}

impl IpV6Addr {
    fn from(s: &str) -> IpV6Addr {
        let mut hextets: Vec<u16> = Vec::new();

        // Check if the input contains '::' (shorthand notation)
        if s.contains("::") {
            // Split the address by '::'
            let parts: Vec<&str> = s.split("::").collect();

            let left_part = parts[0];
            let right_part = if parts.len() > 1 { parts[1] } else { "" };

            // Parse the left part
            if !left_part.is_empty() {
                hextets.extend(
                    left_part
                        .split(':')
                        .filter(|h| !h.is_empty())
                        .map(|h| u16::from_str_radix(h, 16).unwrap()),
                );
            }

            // Calculate the number of zero hextets to insert
            let zero_fill_count =
                8 - hextets.len() - right_part.split(':').filter(|h| !h.is_empty()).count();

            // Fill with zeros
            hextets.extend(vec![0; zero_fill_count]);

            // Parse the right part
            if !right_part.is_empty() {
                hextets.extend(
                    right_part
                        .split(':')
                        .filter(|h| !h.is_empty())
                        .map(|h| u16::from_str_radix(h, 16).unwrap()),
                );
            }
        } else {
            // No shorthand notation, simply parse the address normally
            hextets = s
                .split(':')
                .map(|h| u16::from_str_radix(h, 16).unwrap())
                .collect();
        }

        // Ensure we have exactly 8 hextets
        while hextets.len() != 8 {
            hextets.push(0);
        }

        // Convert to array and create a new IpV6Addr
        IpV6Addr::new([
            hextets[0], hextets[1], hextets[2], hextets[3], hextets[4], hextets[5], hextets[6],
            hextets[7],
        ])
    }

    fn new(hextets: [u16; 8]) -> IpV6Addr {
        IpV6Addr { hextets }
    }

    fn to_string(&self, compress: bool) -> String {
        let mut full_representation = self
            .hextets
            .iter()
            .map(|hextet| format!("{:04x}", hextet)) // Convert each hextet to hexadecimal
            .collect::<Vec<String>>()
            .join(":");

        if !compress {
            return full_representation;
        }

        // Regex to match sequences of ":0+:0+:0+:..." for compression
        let regex = regex::Regex::new(r"(?:^|:)(0+(:0+)*)(?::|$)").unwrap();

        // Find the longest match
        if let Some(longest_match) = regex
            .find_iter(&full_representation)
            .max_by_key(|m| m.end() - m.start())
        {
            full_representation.replace_range(longest_match.start()..longest_match.end(), "::");
        }

        // Handle leading zeros
        let regex = regex::Regex::new(r":0+").unwrap();

        // Remove leading zeros
        full_representation = regex.replace_all(&full_representation, ":").to_string();

        // Ensure we don't replace everything with just "::"
        if full_representation == "::" {
            full_representation.push_str(":0");
        }

        full_representation
    }
}

impl Eq for IpV6Addr {}

impl PartialEq for IpV6Addr {
    fn eq(&self, other: &Self) -> bool {
        self.hextets == other.hextets
    }
}

impl PartialOrd for IpV6Addr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.hextets.cmp(&other.hextets))
    }
}

impl Ord for IpV6Addr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hextets.cmp(&other.hextets)
    }
}

impl std::fmt::Display for IpV6Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string(true))
    }
}
