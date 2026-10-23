use super::{run, texts};
use tagging_core::{EntityType, detectors::IpDetector};

fn d() -> IpDetector {
    IpDetector::new().unwrap()
}

#[test]
fn finds_ipv4() {
    assert_eq!(texts(&d(), "Server 192.168.1.20 is down"), ["192.168.1.20"]);
    assert_eq!(texts(&d(), "Login from 10.0.0.1."), ["10.0.0.1"]);
    assert_eq!(run(&d(), "8.8.8.8")[0].kind, EntityType::IpAddress);
}

#[test]
fn finds_ipv6() {
    for ip in [
        "2001:db8::8a2e:370:7334",
        "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
        "fe80::1",
        "::ffff:192.0.2.1",
    ] {
        assert_eq!(texts(&d(), &format!("addr {ip}, ok")), [ip], "{ip}");
    }
}

#[test]
fn rejects_out_of_range_and_short() {
    for t in ["256.1.1.1", "1.2.3", "999.999.999.999", "01.02.03.04"] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}

#[test]
fn rejects_version_numbers() {
    for t in [
        "v1.2.3.4",
        "version 1.2.3.4",
        "Version 10.0.1.2",
        "build 4.3.2.1",
        "1.2.3.4.5",
    ] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}

#[test]
fn rejects_times_and_paths() {
    for t in ["12:30:45", "a::b", "std::io::Error", "Price 1.234.567"] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}
