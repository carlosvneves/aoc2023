use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let _filename = "input.txt";

    let file = File::open(_filename).expect("Error opening file");

    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

   let mut games: HashMap<u32, Vec<Vec<&str>>> = HashMap::new();

    for line in lines.into_iter() {

        let game = line.split(":").collect::<Vec<&str>>();
        let game_num = game[0].split(";").collect::<Vec<&str>>()[0]
            .split(' ')
            .collect::<Vec<&str>>()[1]
            .trim()
            .parse::<u32>()
            .unwrap();
        

        let game_sets:Vec<Vec<&str>> = IntoIterator::into_iter(game[1]
            .split(";")
            .clone())
            .map(|x: &str| x.split(",").collect::<Vec<&str>>())
            .collect();

        games.insert(game_num, game_sets.clone());
        // for game_set in game_sets {
        //     let balls = game_set.trim().split(",");
        //     println!()
        // }
        // println!("{:?}", game);
        println!("{:?}", game_num);
        //println!("{:?}", game_sets);
        println!("{:?}", game_sets);
    }
    // let mut game_count = 1;

    // for line in  {
    //     let line = line.unwrap();
    //     let mut l = <str as AsRef<str>>::as_ref(&line.to_string())
    //         .as_ref()
    //         .split(";")
    //         .collect::<Vec<&str>>();
    //
    //     games.insert(game_count, l);
    //     game_count += 1;
    //     //println!("{}", line?);
    //
    // }
    //

    //let lines = fs::read("input.txt").unwrap();

    // "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    //     .to_string()
    //     .split(';');
    //
    //println!("{:?}", &lines);

    // for line in games{
    //     println!(line.to_string())
    //
    // }
    //

    // games.insert(1.to_string(), "");
    // colors.insert("red", vec![]);
    // colors.insert("green", vec![]);
    // colors.insert("blue", vec![]);
    //
    // println!("{:?}", games);
    Ok(())
}
