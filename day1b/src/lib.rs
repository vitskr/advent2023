const NUMS: [&[u8]; 10] = [
    b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn run() -> usize {
    let bytes = include_bytes!("../../inputs/day1.txt");

    bytes.split(|b| b == &b'\n').map(get).sum()
}

fn get(bytes: &[u8]) -> usize {
    let converted = convert(bytes);

    let first = (*converted.first().unwrap() - b'0') as usize;
    let last = (*converted.last().unwrap() - b'0') as usize;

    first * 10 + last
}

fn convert(line: &[u8]) -> Vec<u8> {
    let mut v = Vec::<u8>::new();

    for i in 0..line.len() {
        let a = &line[i..];

        if a[0].is_ascii_digit() {
            v.push(a[0]);
        } else {
            for (i, n) in NUMS.iter().enumerate() {
                if a.starts_with(n) {
                    v.push(i as u8 + b'0');
                }
            }
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(convert("one1two".as_bytes()), "112".as_bytes());
        assert_eq!(get("two1two".as_bytes()), 22, "get should work");
    }
}
