use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn merge_ts(id_range: usize){
    let mut buffer = Vec::new();
    for id in 0..id_range{
        let mut f = File::open(format!("./{}.ts", id)).unwrap();
        f.read_to_end(&mut buffer).unwrap();
        std::fs::remove_file(format!("./{}.ts", id)).unwrap();
    }
    let mut f = File::create("./output.ts").unwrap();
    f.write_all(&buffer).unwrap();
}