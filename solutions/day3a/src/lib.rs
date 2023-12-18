pub fn run() -> usize {
    let input = include_str!("../../../inputs/day3.txt");
    solve(input)
}

fn solve(input: &str) -> usize {
    use atoi::*;

    let mut result: usize = 0;
    let m: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    for y in 0..m.len() {
        let mut x = 0;
        while x <= m[y].len() {
            if let (Some(a), size) = usize::from_radix_10_checked(&m[y][x..]) {
                if size > 0 {
                    print!("Candidate {} ", a);
                    if check(&m, (x, y), size) {
                        println!("Found {} at location ({},{})\n", a, x, y);
                        result += a;
                    } else {
                        println!("Not found\n");
                    }
                    x += size;
                } else {
                    x += 1;
                }
            }
        }
    }

    result
}

fn check(v: &Vec<Vec<u8>>, (x, y): (usize, usize), size: usize) -> bool {
    let px = x as i32;
    let py = y as i32;
    let psize = size as i32;

    let sy = v.len() as i32;
    let sx = v[0].len() as i32;

    println!(
        "Checking starting at ({},{}) size:{}. px: {}, py: {}, sx: {}, sy: {}",
        x, y, size, px, py, sx, sy
    );

    fn is_symbol(v: &Vec<Vec<u8>>, (x, y): (usize, usize)) -> bool {
        print!(" Checking symbol at {},{}", x, y);

        let c = &v[y][x];
        if !c.is_ascii_alphanumeric() && c != &b'.' {
            return true;
        }

        false
    }

    if px > 0 && is_symbol(v, ((px - 1) as usize, y)) {
        println!("1");
        return true;
    }

    if px + (size as i32) < sx - 1 && is_symbol(v, ((px + size as i32) as usize, y)) {
        println!("2");
        return true;
    }

    let s = i32::max(0, px - 1);
    let e = i32::min(sx - 1, px + psize);

    println!("\tRange s: {}, e: {}", s, e);

    for dx in s..=e {
        print!("\t\tdx: {}", dx);

        if py < sy - 1 {
            let ly = y + 1;
            let lx = dx as usize;

            if is_symbol(v, (lx, ly)) {
                println!("\n\t\tFound symbol at ({lx}, {ly})");
                return true;
            }
        }

        if py > 0 {
            let ly = y - 1;
            let lx = dx as usize;

            if is_symbol(v, (lx, ly)) {
                println!("\n\t\tFound symbol at ({lx}, {ly})");
                return true;
            }
        }

        println!("\n");
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("test.txt");
        assert_eq!(solve(input), 4361);
    }
}
