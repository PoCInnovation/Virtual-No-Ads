use pcap::{Capture, Device};

fn main() {
  let device = Device::lookup()
        .expect("Device lookup failed!")
        .expect("No device available!");
  println!("Using device {}!", device.name);

  let mut capture = Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

  while let Ok(packet) = capture.next_packet() {
    println!("{:?}", packet);
  }
}