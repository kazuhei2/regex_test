use std::process::Command;
use std::str;

use regex::Regex;

fn main() {
    let re_mac = Regex::new(r"link/\w+ ([[:xdigit:]]{2}):([[:xdigit:]]{2}):([[:xdigit:]]{2}):([[:xdigit:]]{2}):([[:xdigit:]]{2}):([[:xdigit:]]{2})").unwrap();

    let output = Command::new("ip").arg("a").output().expect("failed to ip command");
    let text = str::from_utf8(&output.stdout).unwrap();

    for caps in re_mac.captures_iter(text) {
        println!("mac is {}:{}:{}:{}:{}:{}",
                 caps.get(1).unwrap().as_str(),
                 caps.get(2).unwrap().as_str(),
                 caps.get(3).unwrap().as_str(),
                 caps.get(4).unwrap().as_str(),
                 caps.get(5).unwrap().as_str(),
                 caps.get(6).unwrap().as_str());
    }
    let re_ip_cidr = Regex::new(r"inet (\d{1,3}).(\d{1,3}).(\d{1,3}).(\d{1,3})/(\d{1,2})").unwrap();
    for caps in re_ip_cidr.captures_iter(text) {
        println!("ipv4 address and cidr is {}.{}.{}.{}/{}",
                 caps.get(1).unwrap().as_str(),
                 caps.get(2).unwrap().as_str(),
                 caps.get(3).unwrap().as_str(),
                 caps.get(4).unwrap().as_str(),
                 caps.get(5).unwrap().as_str());
    }


}
