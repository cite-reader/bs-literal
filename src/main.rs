use std::ascii;
use std::env::args_os;
use std::fs::File;
use std::io::{stdout, BufReader, Read, Write};

fn main() {
    let mut args = args_os();
    args.next();

    let filename = args.next().unwrap();
    let input = BufReader::new(File::open(filename).unwrap());

    {
        let out = stdout();
        let mut writer = out.lock();

        for byte in input
            .bytes()
            .flat_map(|b| ascii::escape_default(b.unwrap())) {
                writer.write(&[byte]).unwrap();
            }

        writer.write_all(b"\n").unwrap();
    }
}
