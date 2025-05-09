use std::collections::HashMap;
use super::tools;

pub fn part1(data:&[String])->usize
{
    let mut v1 = vec![];
    let mut v2 = vec![];

    data.iter()
        .for_each(|s| 
            {
                let n = tools::split_to_usize(s,"   ");
                v1.push(n[0]);
                v2.push(n[1]);
            }
        );
        
    v1.sort();
    v2.sort();

    v1.iter()
      .enumerate()
      .map(|(i,_)| v1[i].abs_diff(v2[i]))
      .sum::<usize>()
}

pub fn part2(data:&[String])->usize
{
    let mut v = vec![];
    let mut hash = HashMap::new();

    data.iter()
        .for_each(|s| 
            {
                let n= tools::split_to_usize(s,"   ");
                v.push(n[0]);
                hash.entry(n[1]).and_modify(|e| *e+=1).or_insert(1);
            }
        );

    v.iter()
     .map(|n| (n*hash.get(n).unwrap_or(&0)))
     .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day1");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "3   4".to_string(),
        "4   3".to_string(),
        "2   5".to_string(),
        "1   3".to_string(),
        "3   9".to_string(),
        "3   3".to_string(),
    ];
    assert_eq!(part1(&v),11);
}

#[test]
fn test2()
{
    let v = vec![
        "3   4".to_string(),
        "4   3".to_string(),
        "2   5".to_string(),
        "1   3".to_string(),
        "3   9".to_string(),
        "3   3".to_string(),
    ];
    assert_eq!(part2(&v),31);
}
