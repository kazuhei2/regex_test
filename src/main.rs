use regex::Regex;

const TO_SEARCH: &'static str = "
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN qlen 1
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
    inet6 ::1/128 scope host
       valid_lft forever preferred_lft forever
2: ens160: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP qlen 1000
    link/ether 02:01:52:9b:00:09 brd ff:ff:ff:ff:ff:ff
    inet 10.6.0.191/21 brd 10.6.7.255 scope global ens160
       valid_lft forever preferred_lft forever
    inet6 fe80::1:52ff:fe9b:9/64 scope link
       valid_lft forever preferred_lft forever
";

fn main() {
    let re_mac = Regex::new(r"link/\w+ ([[:xdigit:]]{2}):([[:xdigit:]]{2}):([[:xdigit:]]{2}):([[:xdigit:]]{2}):([[:xdigit:]]{2}):([[:xdigit:]]{2})").unwrap();

    for caps in re_mac.captures_iter(TO_SEARCH) {
        println!("mac is {}:{}:{}:{}:{}:{}",
                 caps.get(1).unwrap().as_str(),
                 caps.get(2).unwrap().as_str(),
                 caps.get(3).unwrap().as_str(),
                 caps.get(4).unwrap().as_str(),
                 caps.get(5).unwrap().as_str(),
                 caps.get(6).unwrap().as_str());
    }
    let re_ip_cidr = Regex::new(r"inet (\d{1,3}).(\d{1,3}).(\d{1,3}).(\d{1,3})/(\d{1,2})").unwrap();
    for caps in re_ip_cidr.captures_iter(TO_SEARCH) {
        println!("ipv4 address and cidr is {}.{}.{}.{}/{}",
                 caps.get(1).unwrap().as_str(),
                 caps.get(2).unwrap().as_str(),
                 caps.get(3).unwrap().as_str(),
                 caps.get(4).unwrap().as_str(),
                 caps.get(5).unwrap().as_str());
    }
}
