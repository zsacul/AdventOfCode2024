use std::collections::HashMap;
use super::tools;

#[derive(Debug)]
struct Data {
      pairs: Vec<(usize,usize)>,
    numbers: Vec<Vec<usize>>
}

impl Data {
    fn new(input: &[String]) -> Self {
        let sections: Vec<&[String]> = input.split(|line| line.is_empty()).collect();
        
        let pairs = sections[0].iter()
            .map(|line| {
                let nums = tools::split_to_usize(line, "|");
                (nums[0], nums[1])
            })
            .collect();
        
        let numbers: Vec<Vec<usize>> = sections[1].iter()
            .map(|line| tools::split_to_usize(line, ",") )
            .collect();

        Data {
            pairs,
            numbers
        }
    }

    fn get_pos(line:&[usize])->HashMap<usize,usize>
    {
        line.iter()
            .enumerate()
            .map(|(id,v)| (*v,id))
            .collect()
    }

    fn ok1(&self,line:&[usize]) -> usize
    {
        let pos = Data::get_pos(line);

        for (a,b) in &self.pairs
        {            
            if pos.contains_key(a) && pos.contains_key(b) && pos[a] > pos[b]
            {
                return 0;
            }
        }
        line[line.len()/2]
    }

    fn compare(&self,a:usize,b:usize)->std::cmp::Ordering
    {
        for (a1,b1) in &self.pairs
        {
            if a==*a1 && b==*b1 { return std::cmp::Ordering::Less;    }
            if a==*b1 && b==*a1 { return std::cmp::Ordering::Greater; }
        }
        std::cmp::Ordering::Equal
    }

    fn ok2(&self,line:&[usize]) -> usize
    {
        if self.ok1(line)!=0 { return 0; }

        let mut posok = line.to_owned();
        posok.sort_by(|a,b| self.compare(*a,*b));
        posok[posok.len()/2]
    }

    fn count1(&self)->usize
    {
        self.numbers.iter()
            .map(|n| self.ok1(n) )
            .sum()
    
    }

    fn count2(&self)->usize
    {
        self.numbers.iter()
            .map(|n| self.ok2(n))
            .sum()    
    }

}

pub fn part1(data:&[String])->usize
{
    Data::new(data).count1()
}

pub fn part2(data:&[String])->usize
{
    Data::new(data).count2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day5");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "47|53".to_string(),
        "97|13".to_string(),
        "97|61".to_string(),
        "97|47".to_string(),
        "75|29".to_string(),
        "61|13".to_string(),
        "75|53".to_string(),
        "29|13".to_string(),
        "97|29".to_string(),
        "53|29".to_string(),
        "61|53".to_string(),
        "97|53".to_string(),
        "61|29".to_string(),
        "47|13".to_string(),
        "75|47".to_string(),
        "97|75".to_string(),
        "47|61".to_string(),
        "75|61".to_string(),
        "47|29".to_string(),
        "75|13".to_string(),
        "53|13".to_string(),
        "".to_string(),
        "75,47,61,53,29".to_string(),
        "97,61,53,29,13".to_string(),
        "75,29,13".to_string(),
        "75,97,47,61,53".to_string(),
        "61,13,29".to_string(),
        "97,13,75,29,47".to_string(),
    ];
    assert_eq!(part1(&v),143);
}

#[test]
fn test2()
{
    let v = vec![
        "47|53".to_string(),
        "97|13".to_string(),
        "97|61".to_string(),
        "97|47".to_string(),
        "75|29".to_string(),
        "61|13".to_string(),
        "75|53".to_string(),
        "29|13".to_string(),
        "97|29".to_string(),
        "53|29".to_string(),
        "61|53".to_string(),
        "97|53".to_string(),
        "61|29".to_string(),
        "47|13".to_string(),
        "75|47".to_string(),
        "97|75".to_string(),
        "47|61".to_string(),
        "75|61".to_string(),
        "47|29".to_string(),
        "75|13".to_string(),
        "53|13".to_string(),
        "".to_string(),
        "75,47,61,53,29".to_string(),
        "97,61,53,29,13".to_string(),
        "75,29,13".to_string(),
        "75,97,47,61,53".to_string(),
        "61,13,29".to_string(),
        "97,13,75,29,47".to_string(),
    ];
    assert_eq!(part2(&v),123);
}
