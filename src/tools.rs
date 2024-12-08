use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use super::vec2::Vec2;

#[allow(unused)]
pub fn read_1d_i32(path:&str)->Vec<i32>
{
    let mut res:Vec<i32> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok)
        {
            res.push(line.parse::<i32>().unwrap());
        }
    }
    res
}

#[allow(unused)]
pub fn read_1d_i64(path:&str)->Vec<i64>
{
    let mut res:Vec<i64> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok)
        {
            res.push(line.parse::<i64>().unwrap());
        }
    }
    res
}

#[allow(unused)]
pub fn read_1d_string(path:&str)->Vec<String>
{
    let mut res:Vec<String> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok)
        {
            res.push(line);
        }
    }
    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(unused)]
pub fn str_get_between<'a>(str:&'a str,from:&'a str,to:&'a str)->&'a str
{
    if from.is_empty()
    {
        let e =          str.find(to).unwrap();
        return &str[..e];
    }

    if to.is_empty()
    {
        let s =          str.find(from).unwrap();
        return &str[s+from.len()..];
    }

        let s =          str.find(from).unwrap() + from.len();
        let e = s + str[s..].find(to  ).unwrap();
        
        &str[s..e]
}

#[allow(unused)]
pub fn get_between(str:&str,from:&str,to:&str)->String
{
    str_get_between(str,from,to).to_string()
}

#[allow(unused)]
pub fn i32_get_between(str:&str,from:&str,to:&str)->i32
{
    get_between(str, from, to).parse::<i32>().unwrap()
}

#[allow(unused)]
pub fn i64_get_between(str:&str,from:&str,to:&str)->i64
{
    get_between(str, from, to).parse::<i64>().unwrap()
}

#[allow(unused)]
pub fn f32_get_between(str:&str,from:&str,to:&str)->f32
{
    get_between(str, from, to).parse::<f32>().unwrap()
}

#[allow(unused)]
pub fn usize_get_between(str:&str,from:&str,to:&str)->usize
{
    get_between(str, from, to).parse::<usize>().unwrap()
}

#[allow(unused)]
pub fn split_to_usize(str:&str,sep:&str)->Vec<usize>
{
    str.split(sep)
       .collect::<Vec<&str>>()
       .iter()
       .map( |e| e.trim().parse::<usize>().unwrap() )
       .collect::<Vec<usize>>()
}


#[allow(unused)]
pub fn get_2d_iter(x_from:usize,x_to:usize,y_from:usize,y_to:usize)->Vec<(usize,usize)>
{
    (y_from..y_to).flat_map(|y|
        (x_from..x_to).map(move |x|
            {
                (y,x)
            }
        )
    ).collect::<Vec<(usize,usize)>>()
}

#[allow(unused)]
pub fn get_2d_i(x_to:usize,y_to:usize)->Vec<(usize,usize)>
{
    get_2d_iter(0,x_to,0,y_to)
}

#[allow(unused)]
pub fn get_hash_table(data:&[String])->HashMap<Vec2,char>
{
    let mut hash = HashMap::new();

    for (y,x) in get_2d_i(data[0].len(),data.len())
    {
        hash.insert(Vec2::new(x as i64,y as i64),data[y].chars().nth(x).unwrap());
    }

    hash
}

#[allow(unused)]
pub fn print_hash_set(data:&HashSet<Vec2>,dx:usize,dy:usize)
{
    //let mut hash = HashMap::new();

    for y in 0..dy
    {
        for x in 0..dx
        {
            if data.contains(&Vec2::new(x as i64,y as i64))
            {
                print!("#");
            }
            else
            {
                print!(".");
            }
        }
        println!();
    }

    //hash
}