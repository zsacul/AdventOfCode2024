use std::collections::HashMap;

fn calc(s:usize,i:usize,n:usize,hash:&mut HashMap<(usize,usize),usize>)->usize
{
    if hash.contains_key(&(s,i)) { return hash[&(s,i)]; }

    let mut stones = vec![];

    if s==0
    {
        stones.push(1);
    }
    else if (s.to_string().len()%2)==0
    {
        let str = s.to_string();
        stones.push(str[           ..str.len()/2].parse::<usize>().unwrap());
        stones.push(str[str.len()/2..           ].parse::<usize>().unwrap());
    }
    else 
    {
        stones.push(s*2024);
    }

    if i==n-1
    {
        hash.insert((s,i),stones.len());
        stones.len()
    }
      else
    {
        let cnt = stones.iter().map(|&f| calc(f,i+1,n,hash)).sum();
        hash.insert((s,i),cnt);
        cnt
    }

}

pub fn compute(data:&[String],n:usize)->usize
{
    let mut hash = HashMap::new(); 

    data[0].split_whitespace()
           .map(|a| calc(a.parse::<usize>().unwrap(),0,n,&mut hash) )
           .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day11");
    println!("part1: {}",compute(data,25));
    println!("part2: {}",compute(data,75));
}

#[test]
fn test0()
{
    let v = vec![
        "125 17".to_string(),
    ];
    assert_eq!(compute(&v,6),22);
}

#[test]
fn test1()
{
    let v = vec![
        "0 1 10 99 999".to_string(),
    ];
    assert_eq!(compute(&v,1),7);
}