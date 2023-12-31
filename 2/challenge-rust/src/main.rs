use std::env;
use std::fs::read_to_string;
use std::cmp::max;

struct Cubeset {
    red: i32,
    green: i32,
    blue: i32    
}

impl Cubeset {
    fn new() -> Cubeset {
        let c = Cubeset{
            red: 0,
            green: 0,
            blue: 0
        };
        c
    }

    fn new_with(red: i32, green: i32, blue: i32) -> Cubeset {
        Cubeset { red: red, green: green, blue: blue }
    }

    fn new_from_play(raw_play: &str) -> Cubeset {
        let item_i = raw_play.split(", ");
        let mut retval = Cubeset::new();
        for i in item_i {
            let mut cubecount_s = i.split(" ");
            let count = cubecount_s.next().unwrap().parse()
                .expect("Expected a number for the count");
            let color = cubecount_s.next().unwrap();
            match color {
                "red" => { retval.red = count},
                "green" => { retval.green = count},
                "blue" => { retval.blue = count},
                _ => { panic!("unexpected color {}", color)}
            };
        };
        retval
    }

    fn passes(&self, filter: &Cubeset) -> bool {
        let val = self.blue <= filter.blue &&
        self.red <= filter.red &&
        self.green <= filter.green;
        val

    }

    fn max(&self, other: &Cubeset) -> Cubeset {
        let mut result = Cubeset::new();
        result.red = max(self.red, other.red);
        result.green = max(self.green, other.green);
        result.blue = max(self.blue, other.blue);
        result
    }
}

struct Game {
    num: i32,
    plays: Vec<Cubeset>
}

impl Game {
    fn new(raw_line: &str) -> Game {
        let line = String::from(raw_line.trim());
        let mut prefix_plus_playlist = line.split(": ");
        let prefix = prefix_plus_playlist.next().unwrap();
        let playlist_raw = prefix_plus_playlist.next().unwrap();
        //println!("{}", playlist_raw);
        let mut prefix_i = prefix.split(" ");
        prefix_i.next();
        let num = prefix_i.next().unwrap().parse().expect("Game number not valid");
        let play_set = playlist_raw.split("; ")
            .map(Cubeset::new_from_play).collect();
        let game = Game { num: num, plays: play_set};
        //println!("Game num is {}", num);

        game
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{}", args[1]);
    let file_name = &args[1];
    let _lines = read_to_string(file_name)
        .unwrap();  // panic on possible file-reading errors
    let lines = _lines;
    let games: Vec<_> = lines.lines()  // split the string into an iterator of string slices
        .map(Game::new).collect();
    println!("Part 1\n======");
    let filter = Cubeset::new_with(12, 13,14);
    let sum = games.iter().filter(|g| {
        g.plays.iter().all(|cs| { cs.passes(&filter)})
    }).map(|g| g.num)
        .fold(0, |total, sum| {let new_total = total + sum;
     new_total});
    println!("{}", sum);
    println!("Part 2\n======");
    let power_sum = games.iter().map(|g| {
        g.plays.iter().fold(Cubeset::new(), |total, val| {
            total.max(val)
        })
    }).map(|cs| { cs.red * cs.green * cs.blue}).fold(0, |a, b| a + b);
    println!("{}", power_sum);
}

#[test]
fn test1() {
    let test = Cubeset::new_with(3,4,5);
    let filter = Cubeset::new_with(7,6,5);
    assert!(test.passes(&filter));
}

#[test]
fn test2() {
    let game = Game::new("Game 1: 3 blue, 4 red");
    assert!(game.num == 1);
    assert!(game.plays[0].red == 4);
}

#[test]
fn test_cubeset_constructor() {
    let c = Cubeset::new_from_play("3 blue, 4 red, 15 green, 2 red");
    assert!(c.green == 15);
    assert!(c.red == 2);
}