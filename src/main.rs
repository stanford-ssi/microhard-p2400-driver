use core::time::Duration;
use std::str;
// use std::fs;
// extern crate base64;

fn main() {
    // let filename = "/Users/elijah/Documents/SSI/image_serializer/dogeToTheMoon.jpg";
    // let output_file = "test_output.jpg";
    // println!("reading in image {}", filename);

    // let contents = fs::read(filename)
    //     .expect("Something went wrong reading the file");

    // println!("length: {:?}", contents.len());
    // println!("first 10 elements of image bytes vector: {:?}", &contents[0..10]);

    // let content_b64 = base64::encode(&contents);
    // // println!("{:?}", content_b64);
    // let decoded = base64::decode(&content_b64);
    // // println!("{:?}", &decoded[0..10]);

    // fs::write(output_file, contents).expect("could not write file");
    // println!("output file -> {}", output_file);


    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);

        let mut port = serialport::new("/dev/tty.usbmodem144401", 9600)
            .timeout(Duration::from_millis(10))
            .open().expect("Failed to open port");

        let output = "This is a test. This is only a test.".as_bytes();
        port.write(output).expect("Write failed!");

        

        while true {
            let mut serial_buf: Vec<u8> = vec![0; 32];
            port.read(serial_buf.as_mut_slice());
            if serial_buf != vec![0; 32] {
                let as_str = str::from_utf8(&serial_buf).unwrap();
                println!("{}", as_str);
            }
        }
    }
}
