pub fn run() -> usize {
    let bytes = include_bytes!("../../../inputs/day1.txt");
    let lines = bytes.split(|b| b == &b'\n');

    lines.map(get).sum::<usize>()
}

fn get(bytes: &[u8]) -> usize {

    let first = bytes.iter().find(|b| b.is_ascii_digit()).unwrap() - 48;
    let last = bytes.iter().rfind(|b| b.is_ascii_digit()).unwrap() - 48;

    (first * 10 + last) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(get("1abc2".as_bytes()), 12);
        assert_eq!(get("pqr3stu8vwx".as_bytes()), 38);
        assert_eq!(get("a1b2c3d4e5f".as_bytes()), 15);
        assert_eq!(get("treb7uchet".as_bytes()), 77);
    }
}