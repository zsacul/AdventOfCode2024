fn count(s:String)->u32
{
    let v:Vec<_> = s.chars()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| c.to_digit(10).unwrap())
                    .collect();

    10*v.first().unwrap() + v.last().unwrap()
}

fn count2(s:String)->usize
{
    0
}

pub fn part1(data:&[String])->u32
{
    data.iter()
        .map(|s| count(s.to_string()))
        .sum() 
}

pub fn part2(data:&[String])->usize
{
    data.iter()
        .map(|s| count2(s.to_string()))
        .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day1");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string(),
    ];
    assert_eq!(part1(&v),142);
}

#[test]
fn test2()
{
    let v = vec![
        "two1nine".to_string(),
        "eightwothree".to_string(),
        "abcone2threexyz".to_string(),
        "xtwone3four".to_string(),
        "4nineeightseven2".to_string(),
        "zoneight234".to_string(),
        "7pqrstsixteen".to_string(),        
    ];
    assert_eq!(part2(&v),281);
}

#[test]
fn test3()
{
    let v = vec![
        "eighthree".to_string(),
        "sevenine".to_string(),
    ];
    assert_eq!(part2(&v),83+79);
}
