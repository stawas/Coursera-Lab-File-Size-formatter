use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl FileSize {
    fn to_byte(self) -> u64 {
        match self {
            FileSize::Bytes(bytes) => bytes,
            FileSize::Kilobytes(kb) => kb * 1_000,
            FileSize::Megabytes(mb) => mb * 1_000_000,
            FileSize::Gigabytes(gb) => gb * 1_000_000_000,
            FileSize::Terabytes(tb) => tb * 1_000_000_000_000,
        }
    }

    fn get_sizes(bytes_size: u64) -> Sizes {
        let sizes = Sizes {
            bytes: format!("{} bytes", bytes_size),
            kilobytes: format!("{} kilobytes", bytes_size / 1_000),
            megabytes: format!("{} megabytes", bytes_size / 1_000_000),
            gigabytes: format!("{} gigabytes", bytes_size / 1_000_000_000),
        };
        sizes
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let size_with_unit:Vec<&str> = args[1].split_whitespace().collect();
    let size: u64 = size_with_unit[0].parse().unwrap();

    let filesize = match size_with_unit[1].to_lowercase().trim() {
        "kb" => FileSize::Kilobytes(size),
        "mb" => FileSize::Megabytes(size),
        "gb" => FileSize::Gigabytes(size),
        _=> panic!("Unit not supported.")
    };

    println!("{:?}", FileSize::get_sizes(filesize.to_byte()));
}
