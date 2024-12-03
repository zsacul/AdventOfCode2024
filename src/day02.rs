fn valid(t:Vec<i32>)->bool
{
    let mut same = None;

    t.windows(2)
     .all(|t|
        {
            let dist = t[1] - t[0];
            if same.is_none() { same = Some(dist.signum()); }
            (1..=3).contains(&dist.abs()) && Some(dist.signum())==same         
        }
    )
}

fn ok(s:&str,second:bool)->bool
{
    let tab = s.split(" ").map(|a| a.parse().unwrap()).collect::<Vec<i32>>();

    if second
    {
        (0..tab.len()).any(|i|
            valid(tab[..i].iter().chain(tab[i+1..].iter()).copied().collect())
        )
    }    
      else 
    {
        valid(tab)
    }
}

pub fn part1(data:&[String])->usize
{
   data.iter()
       .filter(|n| ok(n,false))
       .count()
}

pub fn part2(data:&[String])->usize
{
   data.iter()
       .filter(|n| ok(n,true))
       .count()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day2");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "7 6 4 2 1".to_string(),
        "1 2 7 8 9".to_string(),
        "9 7 6 2 1".to_string(),
        "1 3 2 4 5".to_string(),
        "8 6 4 4 1".to_string(),
        "1 3 6 7 9".to_string(),        
    ];
    assert_eq!(part1(&v),2);
}

#[test]
fn test2()
{
    let v = vec![
        "7 6 4 2 1".to_string(),
        "1 2 7 8 9".to_string(),
        "9 7 6 2 1".to_string(),
        "1 3 2 4 5".to_string(),
        "8 6 4 4 1".to_string(),
        "1 3 6 7 9".to_string(),        
    ];
    assert_eq!(part2(&v),4);
}
