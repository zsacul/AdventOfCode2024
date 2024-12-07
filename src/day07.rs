use std::collections::HashMap;
use std::collections::HashSet;
use super::tools;
use super::vec2::Vec2;

fn concatenate(a:u128,b:u128)->u128
{
    let mut a = a;
    let mut c = b;
    while c > 0
    {
        c /= 10;
        a *= 10;
    }
    a+b
}

fn calc1(n:&Vec<usize>,m:usize)->u128
{
    let mut acc = n[0] as u128;

    for i in 1..n.len()
    {
        let n = n[i] as u128;
        if m & (1<<i) != 0
        {
            acc += n;
        }
        else
        {
            acc *= n;
        }
    }

    acc

}

fn calc2(n:&Vec<usize>,m:usize,sum:u128)->u128
{
    let mut acc:u128 = n[0] as u128;

    for i in 1..n.len()
    {
        let b0 = (m & (1<<((2*i)  )))!=0;
        let b1 = (m & (1<<((2*i)+1)))!=0;

        let n  = n[i] as u128;

        match (b0,b1)
        {
            (false,false) => acc  = concatenate(acc,n),
            (false, true) => acc += n,
            ( true,false) => acc *= n,
            _             => acc += n,
        }
        if acc>sum { return 0; }
    }

    acc
}

fn ok(s:&str,second:bool)->u128
{
    let t :Vec<&str>= s.split(": ").collect();
    let sum = t[0].parse::<u128>().unwrap();
    let n = tools::split_to_usize(t[1], " ");

    let m = if second {1<<(2*n.len())} else { 1<<n.len()};
   
    for i in 0..=m
    {
        if !second
        {
            if sum==calc1(&n,i)
            {
                return sum;
            }
        }
          else 
        {
            if sum as u128==calc2(&n,i,sum as u128)
            {
                println!("{:?}",sum);
                return sum;
            }
        }

    }
    return 0;


}

//271692122210925 wrong


pub fn part1(data:&[String])->u128
{
   data.iter()
       .map(|n| ok(n,false))
       .sum()
}

pub fn part2(data:&[String])->u128
{
    data.iter()
        .map(|n| ok(n,true))
        .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day7");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "190: 10 19".to_string(),
        "3267: 81 40 27".to_string(),
        "83: 17 5".to_string(),
        "156: 15 6".to_string(),
        "7290: 6 8 6 15".to_string(),
        "161011: 16 10 13".to_string(),
        "192: 17 8 14".to_string(),
        "21037: 9 7 18 13".to_string(),
        "292: 11 6 16 20".to_string(),
    ];
    assert_eq!(part1(&v),3749);
}

#[test]
fn test2()
{
    let v = vec![
        "190: 10 19".to_string(),
        "3267: 81 40 27".to_string(),
        "83: 17 5".to_string(),
        "156: 15 6".to_string(),
        "7290: 6 8 6 15".to_string(),
        "161011: 16 10 13".to_string(),
        "192: 17 8 14".to_string(),
        "21037: 9 7 18 13".to_string(),
        "292: 11 6 16 20".to_string(),
    ];
    assert_eq!(part2(&v),11387);
}
