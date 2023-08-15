use rand::random;

//3-2
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: vec![],
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut file = File::new(name);
        file.data = data.clone();
        file
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        read_length
    }
}

fn file_open(f: &File) -> Result<&File, String> {
    if random() {
        return Err(String::from("Open Error"));
    } else {
        return Ok(&f);
    }
}

fn file_close(f: &File) -> Result<&File, String> {
    if random() {
        return Err(String::from("Close Error"));
    } else {
        return Ok(&f);
    }
}

fn main() {
    let f3_data = vec![114, 117, 115, 116, 33];
    let f3 = File::new_with_data("f2.txt", &f3_data);

    let mut buffer: Vec<u8> = vec![];

    file_open(&f3).unwrap();
    let f3_length = f3.read(&mut buffer);
    file_close(&f3).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, f3_length);
    println!("{}", text);
}
