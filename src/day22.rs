use std::collections::{HashMap, HashSet};

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
    sec = prune(mix(sec*64  ,sec));
    sec = prune(mix(sec/32  ,sec));
    sec = prune(mix(sec*2048,sec));
    sec
}

fn count(s:i64,n:usize)->i64
{
    let mut s = s;
    for _ in 0..n
    {
        s = evolve(s);
    }
    s
}

fn count2(s:i64,n:usize)->(Vec<(i64,i64)>,HashMap<(i64,i64,i64,i64),i64>)
{
    let mut map = HashMap::new();
    let mut res = Vec::new();

    let mut s = s;
    let mut prev = s%10;

    for i in 0..n
    {
        s = evolve(s);
        res.push((s%10,s%10-prev));
        prev = s%10;

        if i>3
        {
            let code = (res[i-4].1,res[i-3].1,res[i-2].1,res[i-1].1);
            if !map.contains_key(&code)
            {
                map.insert(code,res[i-1].0);
            }
        }
    }
    (res,map)
}

pub fn part1(data:&[String])->usize
{
    data.iter().map(|s| count(s.parse::<i64>().unwrap(),2000) ).sum::<i64>() as usize
}

fn summ(res:&Vec<HashMap<(i64,i64,i64,i64),i64>>,p:&(i64,i64,i64,i64))->usize
{
    res.iter().map(|m| 
        *m.get(p).unwrap_or(&0) as usize
    ).sum()
}

pub fn part2(data:&[String])->usize
{
    let mut res = vec![];
    let mut pos = HashSet::new();

    for s in data
    {
        let n= s.parse::<i64>().unwrap();
        let (vec,map) = count2(n,2000);

        vec.windows(4).for_each(|w| {
            pos.insert((w[0].1,w[1].1,w[2].1,w[3].1));
        });

        res.push(map);
    }

    pos.iter().map(|p| summ(&res,p) ).max().unwrap()
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
