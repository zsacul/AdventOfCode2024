use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug,Clone)]
struct Data {
    tokens: HashSet<String>,  
    tok: HashSet<String>,  
    asked: Vec<String>,
}

impl Data {
    fn new(input: &[String]) -> Self {
        let sections: Vec<&[String]> = input.split(|line| line.is_empty()).collect();

        let lang = sections[0][0].to_string();
        let hash: HashSet<String> = lang.split(", ").map(|s| s.to_string()).collect();

        let prog : Vec<String> =  sections[1].iter()
                                             .map(|i| { i.to_string() } )
                                             .collect();

        Data {
            tokens: hash,
            tok: HashSet::new(),
            asked: prog,
        }
    }

    fn possible(&mut self,ask:String,memo:&mut HashMap<String,bool>) -> bool
    {
        if memo.contains_key(&ask) { return memo[&ask] }

        if self.tok.contains(&ask.clone())
        {
            memo.insert(ask.clone(),true);
            return true;
        }

        for i in 1..ask.len()
        {
            if self.tok.contains(&ask[..i]) && self.possible(ask[i..].to_string().clone(),memo) 
            {
                memo.insert(ask.clone(),true);
                return true;
            }            
        }

        memo.insert(ask.clone(),false);
        false
    }

    fn possible2(&mut self,ask:String,memo:&mut HashMap<String,usize>) -> usize
    {        
        if memo.contains_key(&ask) {return memo[&ask]}
        let mut res = 0;

        if self.tok.contains(&ask.clone()) { res+=1; }

        for i in 1..ask.len()
        {
            if self.tok.contains(&ask[..i])
            {
                res+=self.possible2(ask[i..].to_string().clone(),memo);
            }
        }

        memo.insert(ask.clone(),res);
        res
    }

    fn limit_tokens(&mut self,ask:String)
    {
        let tok : Vec<String> = self.tokens.iter()
                                           .filter( |t| ask.contains(*t) )
                                           .map(|f| f.to_string())
                                           .collect::<Vec<String>>();
        self.tok.clear();
        for t in tok.iter()
        {
            self.tok.insert(t.to_string());
        }
    }
    
    fn ok1(&mut self,ask:String) -> bool
    {
        let mut memo = HashMap::new();
        self.limit_tokens(ask.clone());
        self.possible(ask,&mut memo)        
    }

    fn ok2(&mut self,ask:String) -> usize
    {
        let mut memo = HashMap::new();
        self.limit_tokens(ask.clone());
        self.possible2(ask,&mut memo)
    }
    

    fn count1(&mut self)->usize
    {        
        self.asked
            .clone()
            .iter()
            .filter(|a| self.ok1(a.to_string()))
            .count()
    }

    fn count2(&mut self)->usize
    {        
        let filtered: Vec<String> = self.asked.clone().iter()
                                    .filter(|a| self.ok1(a.to_string()))
                                    .cloned()
                                    .collect();

        filtered.iter()
                .map(|b| self.ok2(b.to_string()))
                .sum()
    }
}

pub fn part1(data:&[String])->usize
{
    let mut d = Data::new(data);
    d.count1()
}
  
pub fn part2(data:&[String])->usize
{
    let mut d = Data::new(data);
    d.count2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day19");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "r, wr, b, g, bwu, rb, gb, br".to_string(),
        "".to_string(),
        "brwrr".to_string(),
        "bggr".to_string(),
        "gbbr".to_string(),
        "rrbgbr".to_string(),
        "ubwu".to_string(),
        "bwurrg".to_string(),
        "brgr".to_string(),
        "bbrgwb".to_string(),
    ];
    assert_eq!(part1(&v),6);
}

#[test]
fn test2()
{
    let v = vec![
        "r, wr, b, g, bwu, rb, gb, br".to_string(),
        "".to_string(),
        "brwrr".to_string(),
        "bggr".to_string(),
        "gbbr".to_string(),
        "rrbgbr".to_string(),
        "ubwu".to_string(),
        "bwurrg".to_string(),
        "brgr".to_string(),
        "bbrgwb".to_string(),
    ];
    assert_eq!(part2(&v),16);
}
