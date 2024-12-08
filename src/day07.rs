use std::collections::HashSet;
use super::tools;

fn concatenate(a:usize,b:usize)->usize
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

fn possible(n:&[usize],m:usize,sum:usize)->HashSet<usize>
{    
    let mut hash = HashSet::new();
    let mut hash_new = HashSet::new();

    hash.insert(n[0]);
    
    for &it in n.iter().take(m).skip(1)
    {
        for &v in hash.iter()
        {
            hash_new.insert(v+it);
            hash_new.insert(v*it);
            hash_new.insert(concatenate(v,it));
        }
        hash_new.retain(|&v| v<=sum);

        hash = hash_new.clone();
        hash_new.clear();
    }

    hash
}

fn calc1(n:&[usize],m:usize)->usize
{
    let mut acc = n[0];

    for (i, ni) in n.iter().enumerate().skip(1)
    {
        if m & (1<<i) != 0 { acc += ni; }
                      else { acc *= ni; }
    }

    acc
}



fn get_data(s:&str)->(usize,Vec<usize>)
{
    let tab : Vec<&str>= s.split(": ").collect();
    ( 
      tab[0].parse::<usize>().unwrap(),
      tools::split_to_usize(tab[1], " ") 
    )
}

fn ok1(s:&str)->usize
{
    let (sum,n) = get_data(s);
       
    if (0..=1<<n.len()).any(|i| sum==calc1(&n,i))
    {
       sum
    }
      else 
    {
        0
    }
}

fn ok2(s:&str)->usize
{
    let (sum,n) = get_data(s);
  
    *possible(&n,n.len(),sum)
     .iter()
     .find(|&&v| v==sum)
     .unwrap_or(&0)
}


pub fn part1(data:&[String])->usize
{
   data.iter()
       .map(|n| ok1(n))
       .sum()
}

pub fn part2(data:&[String])->usize
{
    data.iter()
        .map(|n| ok2(n))
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
