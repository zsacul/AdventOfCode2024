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
            .map(|line| {
                line.split(',')
                    .flat_map(|num| num.trim().parse::<usize>())
                    .collect()
            })
            .collect();

        Data {
            pairs,
            numbers
        }
    }

    fn print(&self)
    {
        println!("{:?}",self.pairs);
        println!("{:?}",self.numbers);
    }

    fn ok(&self,line:&Vec<usize>) -> bool
    {
        let mut pos = HashMap::new();

        let mut id=0;
        for n in line
        {
            pos.insert(n,id);
            id+=1;
        }

        for (a,b) in &self.pairs
        {            
            if pos.contains_key(a) && pos.contains_key(b)
            {
                if pos[a] > pos[b] { return false; }
            }
        }
        true
    }

    fn compare(&self,a:usize,b:usize)->std::cmp::Ordering
    {
        let mut res = std::cmp::Ordering::Equal;
        for (a1,b1) in &self.pairs
        {
            if a == *a1 as usize && b == *b1 as usize
            {
                res = std::cmp::Ordering::Less;
                break;
            }
            if a == *b1 as usize && b == *a1 as usize
            {
                res = std::cmp::Ordering::Greater;
                break;
            }
        }
        res
    }

    fn ok2(&self,line:&Vec<usize>) -> usize
    {
        let mut pos = HashMap::new();

        let mut id=0usize;
        for n in line
        {
            pos.insert(*n as usize,id);
            id+=1;
        }
        let mut res = false;

        for (a,b) in &self.pairs
        {            
            if pos.contains_key(a) && pos.contains_key(b)
            {
                if pos[a] > pos[b]
                {
                    res = true;
                    break;
                }                
            }
        }

        if res
        {
            let mut posok = line.clone();
            posok.sort_by(|a,b| self.compare(*a as usize,*b as usize));          
            posok[posok.len()/2] as usize
        }
        else
        {
            0
        }

    }


    fn count1(&self,second:bool)->usize
    {
        self.numbers.iter()
            .map(|n| if self.ok(n) {n[n.len()/2] as usize} else {0})
            .sum()
    
    }

    fn count2(&self,second:bool)->usize
    {
        self.numbers.iter()
            .map(|n| self.ok2(n))
            .sum()
    
    }

}






















pub fn part1(data:&[String])->usize
{
    let d = Data::new(data);
    //d.print();
    d.count1(false)

}

pub fn part2(data:&[String])->usize
{
    let d = Data::new(data);
    //d.print();
    d.count2(true)
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
