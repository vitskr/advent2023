pub fn run() -> usize {
    let b = include_str!("../../inputs/day2.txt");
    for r in b.split('\n') {
        let parts: Vec<&str> = r.split(':').collect();
    }

    0
}

fn count_max(s: &str) -> (u32, u32, u32) {    

    let mut result = (0, 0, 0);
    let games: Vec<&str> = s.split(';').collect();

    for game in games {
        let parsed = parse_game(game);

        if parsed.0 > result.0 {
            result.0 = parsed.0;
        }

        if parsed.1 > result.1 {
            result.1 = parsed.1;
        }

        if parsed.2 > result.2 {
            result.2 = parsed.2;
        }

    }
    
    result
}

fn parse_game(s: &str) -> (u32, u32, u32) {

    let mut result = (0, 0, 0);

    for sub in s.split(',') {
        let p: Vec<&str> = sub.trim().split(' ').collect();
        let v = p.first().unwrap().parse::<u32>().unwrap();

        match p.iter().nth(1) {
            Some(&"red") => result.0 = v,
            Some(&"green") => result.1 = v,
            Some(&"blue") => result.2 = v,
            _ => ()
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        assert_eq!(parse_game("1 red, 2 green, 6 blue"), (1, 2, 6));
    }

    #[test]    
    fn it_works() {                
        assert_eq!(count_max(" 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), (4, 2, 6));
    }
}
