use std::process::Command;
use std::str;

use regex::Regex;

fn main() {
    let output = Command::new("ip").arg("a").output().expect("failed at ip command");
    let text = str::from_utf8(&output.stdout).unwrap();
    let re_mac = Regex::new(r"link/\w+ (?P<mac>[[:xdigit:]]{2}:[[:xdigit:]]{2}:[[:xdigit:]]{2}:[[:xdigit:]]{2}:[[:xdigit:]]{2}:[[:xdigit:]]{2})").unwrap();
    for caps in re_mac.captures_iter(text) {
        println!("mac is {}", &caps["mac"]);
    }
    let re_ip_cidr = Regex::new(r"inet (?P<ip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3})/(?P<cidr>\d{1,2})").unwrap();
    for caps in re_ip_cidr.captures_iter(text) {
        println!("ipv4 address and cidr is {}/{}", &caps["ip"], &caps["cidr"]);
    }


}
