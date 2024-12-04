use super::tools;
use super::vec2::Vec2;
use std::collections::HashMap;

fn count(pos:Vec2,h:&HashMap<Vec2,char>,w:String)->usize
{
    Vec2::dirs8().iter().filter(|dir|
        {
            w.chars()
             .enumerate()
             .map(|(i,_)| h.get(&(pos.add(dir.x*i as i64,dir.y*i as i64))).unwrap_or(&'.') )
             .collect::<String>()==w
        }
    ).count()
}

fn count2(pos:Vec2,h:&HashMap<Vec2,char>,w:String)->bool
{    
    let mut t1 = Vec::new();
    let mut t2 = Vec::new();
    
    for i in -1..=1
    {
        if let Some(c) = h.get(&(pos.add(i, i))) { t1.push(*c); }
        if let Some(c) = h.get(&(pos.add(i,-i))) { t2.push(*c); }
    }

    let s1 = t1.iter().collect::<String>();
    let s2 = t2.iter().collect::<String>();
    let rev_w = w.chars().rev().collect::<String>();

    (s1==w || s1==rev_w) && (s2==w || s2==rev_w) 
}

fn clean(data:&[String],word:&str)->HashMap<Vec2,char>
{
    let mut h = tools::get_hash_table(data);
    h.retain(|_,v| word.contains(*v));
    h
}

pub fn part1(data:&[String])->usize
{    
    let hash = clean(data,"XMAS");
   
    hash.keys()
        .map(|p| count(*p,&hash,"XMAS".to_string()))
        .sum()   
}

pub fn part2(data:&[String])->usize
{
    let hash = clean(data,"MAS");
    
    hash.iter()
        .filter(|(p,_)| count2(**p,&hash,"MAS".to_string()))
        .count()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day4");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
         "MMMSXXMASM".to_string(),
         "MSAMXMSMSA".to_string(),
         "AMXSXMAAMM".to_string(),
         "MSAMASMSMX".to_string(),
         "XMASAMXAMM".to_string(),
         "XXAMMXXAMA".to_string(),
         "SMSMSASXSS".to_string(),
         "SAXAMASAAA".to_string(),
         "MAMMMXMMMM".to_string(),
         "MXMXAXMASX".to_string(),
    ];
    assert_eq!(part1(&v),18);
}

#[test]
fn test2()
{
    let v = vec![
        "..X...".to_string(),
        ".SAMX.".to_string(),
        ".A..A.".to_string(),
        "XMAS.S".to_string(),
        ".X....".to_string(),
    ];
    assert_eq!(part1(&v),4);
}


#[test]
fn test3()
{
    let v = vec![
        ".M.S......".to_string(),
        "..A..MSMS.".to_string(),
        ".M.S.MAA..".to_string(),
        "..A.ASMSM.".to_string(),
        ".M.S.M....".to_string(),
        "..........".to_string(),
        "S.S.S.S.S.".to_string(),
        ".A.A.A.A..".to_string(),
        "M.M.M.M.M.".to_string(),
        "..........".to_string(),
    ];
    assert_eq!(part2(&v),9);
}
