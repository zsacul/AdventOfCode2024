use std::collections::{HashMap, HashSet};
use super::tools;

fn mix(n:i64,sec:i64)->i64
{
    n^sec
}

fn prune(n:i64)->i64
{
     n%16777216
}

fn evolve(sec:i64)->i64
{
    let mut sec = sec;    
    sec = prune(mix(sec*64,sec));
    sec = prune(mix(sec/32,sec));
    sec = prune(mix(sec*2048,sec));
    sec
}

fn count(s:i64,n:usize)->i64
{
    let mut s = s;
    for i in 0..n
    {
        s = evolve(s);
    }
    s
}

fn count2(s:i64,n:usize)->Vec<(i64,i64)>
{
    let mut res = Vec::new();
    let mut s = s;
    let mut prev = s%10;
    for i in 0..n
    {
        s = evolve(s);
        res.push((s%10,s%10-prev));
        prev = s%10;
    }
    res
}

pub fn part1(data:&[String])->usize
{
    let mut res = 0;
    for s in data
    {
        let n= s.parse::<i64>().unwrap();
        res+=count(n,2000);
    }

    res as usize
}

fn bananas(v:Vec<(i64,i64)>,p:&(i64,i64,i64,i64))->usize
{
    for i in 0..v.len()-4
    {
        if v[i].1==p.0 && v[i+1].1==p.1 && v[i+2].1==p.2 && v[i+3].1==p.3
        {
            //println!("{}: ",v[i+3].0);
            return v[i+3].0 as usize;
        }
    }
    0
}

fn summ(res:&Vec<Vec<(i64,i64)>>,p:&(i64,i64,i64,i64))->usize
{
    res.iter().map(|v| bananas(v.clone(),p)).sum()
}

pub fn part2(data:&[String])->usize
{
    let mut res = vec![];
    for s in data
    {
        let n= s.parse::<i64>().unwrap();
        res.push(count2(n,2000));
    }

    let mut pos = HashSet::new();
    for seq in res.clone().iter()
    {
        seq.windows(4).for_each(|w| {
            pos.insert((w[0].1,w[1].1,w[2].1,w[3].1));
        });
    }
    let mut cnt=0;

    let mut gg=0;
    for p in pos.iter()
    {      
        cnt = cnt.max(summ(&res,p));
    }


    cnt
}



#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day22");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test0()
{
    assert_eq!(mix(15,42),37);
    assert_eq!(prune(100000000),16113920);    
}

#[test]
fn test1()
{
    let v = vec![
        "1".to_string(),
        "10".to_string(),
        "100".to_string(),
        "2024".to_string(),
    ];
    assert_eq!(part1(&v),37327623);
}

#[test]
fn test2()
{
    let v = vec![
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "2024".to_string(),
    ];
    assert_eq!(part2(&v),23);
}
