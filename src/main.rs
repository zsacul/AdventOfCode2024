use timer::Timer;
use aoc_cache::get;
// my.cookie is a file containing the cookie string.
const MY_COOKIE: &str = include_str!("my.cookie");

mod tools;
mod dijkstria;
//use std::thread;
//mod cycliclist;
//mod cyclic2;
mod timer;
mod vec2;
mod vec3;
mod vec3f;
mod day01;
//mod day02;
//mod day03;
//mod day04;
//mod day05;
//mod day06;
//mod day07;
//mod day08;
//mod day09;
//mod day10;
//mod day11;
//mod day12;
//mod day13;
//mod day14;
//mod day15;
//mod day16;
//mod day17;
//mod day18;
//mod day19;
//mod day20;
//mod day21;
//mod day22;
//mod day23;
//mod day24;
//mod day25;

fn get_from_web(day:i32)->String
{
    // my.cookie is a file containing the cookie string.
    const MY_COOKIE: &str = include_str!("my.cookie");
    let input: Result<String, aoc_cache::Error> = // Grabs from web if
    get(              // it's the first run
        "https://adventofcode.com/2022/day/1/input", MY_COOKIE);

}

fn get_data(day:i32)->String
{
    let path = format!("https://adventofcode.com/2022/day/{day}/input");
    let file = format!("data/day{:02}.txt",day);
    dbg!(&path);
    dbg!(&file);
    
    //if file exists return it
    if !std::path::Path::new(&file).exists() {
        return file;
    }

    tools::read_1d_string(file)
}

fn load_data()
{
    let day1_data = get(MY_COOKIE, get_data(1));
    day01::solve(&day1_data);
}

fn main() {
    {
        let _timer = Timer::new();
        let day1_data = get_data(1);
        day01::solve(&day1_data);
    }
/*
    {
        let _timer = Timer::new();
        let day2_data  = tools::read_1d_string("data/day02.txt");
        day02::solve(&day2_data);
    }

    {
        let _timer = Timer::new();
        let day3_data  = tools::read_1d_string("data/day03.txt");
        day03::solve(&day3_data);
    }
    
    {
        let _timer = Timer::new();
        let day4_data  = tools::read_1d_string("data/day04.txt");
        day04::solve(&day4_data);
    }
    
    {
        let _timer = Timer::new();
        let day5_data  = tools::read_1d_string("data/day05.txt");
        day05::solve(&day5_data);
    }
    
    {
        let _timer = Timer::new();
        let day6_data  = tools::read_1d_string("data/day06.txt");
        day06::solve(&day6_data);
    }
    
    {
        let _timer = Timer::new();
        let day7_data  = tools::read_1d_string("data/day07.txt");
        day07::solve(&day7_data);
    }
    
    {
        let _timer = Timer::new();
        let day8_data  = tools::read_1d_string("data/day08.txt");
        day08::solve(&day8_data);
    }
    
    {
        let _timer = Timer::new();
        let day9_data  = tools::read_1d_string("data/day09.txt");
        day09::solve(&day9_data);
    }

    {
        let _timer = Timer::new();
        let day10_data  = tools::read_1d_string("data/day10.txt");
        day10::solve(&day10_data);
    }

    {
        let _timer = Timer::new();
        let day11_data  = tools::read_1d_string("data/day11.txt");
        day11::solve(&day11_data);
    }
    
    {
        let _timer = Timer::new();
        let day12_data  = tools::read_1d_string("data/day12.txt");
        day12::solve(&day12_data);
    }
    
    {
        let _timer = Timer::new();
        let day13_data  = tools::read_1d_string("data/day13.txt");
        day13::solve(&day13_data);
    }

    {
        let _timer = Timer::new();
        let day14_data  = tools::read_1d_string("data/day14.txt");
        day14::solve(&day14_data);
    }
    
    {
        let _timer = Timer::new();
        let day15_data  = tools::read_1d_string("data/day15.txt");
        day15::solve(&day15_data);
    }
    
    {
        let _timer = Timer::new();
        let day16_data  = tools::read_1d_string("data/day16.txt");
        day16::solve(&day16_data); 
    }
    
    {
        let _timer = Timer::new();
        let day17_data  = tools::read_1d_string("data/day17.txt");
        day17::solve(&day17_data);
    }
    {
        let _timer = Timer::new();
        let day18_data  = tools::read_1d_string("data/day18.txt");
        day18::solve(&day18_data);
    }
    
    {
        let _timer = Timer::new();
        let day19_data  = tools::read_1d_string("data/day19.txt");
        day19::solve(&day19_data);
    }
    
    {
        let _timer = Timer::new();
        let day20_data  = tools::read_1d_string("data/day20.txt");
        day20::solve(&day20_data);
    }
    
    {
        //let _timer = Timer::new();
        //let day21_data  = tools::read_1d_string("data/day21.txt");
        //day21::solve(&day21_data);
    }

    {
        //let _timer = Timer::new();
        //let day22_data  = tools::read_1d_string("data/day22.txt");       
        //day22::solve(&day22_data);
    }
    
    {
        //let child = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || { 
        //    let _timer = Timer::new();
        //    let day23_data  = tools::read_1d_string("data/day23.txt");
        //    day23::solve(&day23_data);       
        //}).unwrap(); 
        //child.join().unwrap();
    }
    
    {
        let _timer = Timer::new();
        let day24_data  = tools::read_1d_string("data/day24.txt");
        day24::solve(&day24_data);       
    } 
    
    {
        let _timer = Timer::new();
        let day25_data  = tools::read_1d_string("data/day25.txt");
        day25::solve(&day25_data);
    }
*/
}