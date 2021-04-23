use core::time::Duration;
use xmodem::Xmodem;
use std::fs;

fn main() {
  let filename = "/Users/elijah/Documents/SSI/microhard-p2400-driver/dogeToTheMoon tiny.jpg";
  let file = fs::read(filename)
        .expect("Something went wrong reading the file");

  let mut port = serialport::new("/dev/tty.usbmodem144401", 9600)
  .timeout(Duration::from_millis(1000))
  .open().expect("Failed to open port");

  let mut coms = Xmodem::new();
  coms.send(&mut port, &mut file.as_slice());


  loop {
      let mut serial_buf: Vec<u8> = vec![0; 32];
      port.read(serial_buf.as_mut_slice());
      if serial_buf != vec![0; 32] {
          println!("{:?}", serial_buf);
      }
  }
}