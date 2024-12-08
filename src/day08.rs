use std::collections::hash_set;
use std::collections::HashMap;
use std::collections::HashSet;
use super::tools;
use super::vec2::Vec2;

#[derive(Debug)]
struct Data 
{
    hash : HashMap<Vec2,char>,
      dx : usize,
      dy : usize
}

impl Data {
    fn new(input: &[String]) -> Self 
    {
        let hash = tools::get_hash_table(input);
            
        Data 
        {
            hash,           
            dx  : input[0].len(),
            dy  : input.len()
        }
    }
   
    fn pos(&self,p:Vec2)->bool
    {
        p.x>=0 && p.x<self.dx as i64 && p.y>=0 && p.y<self.dy as i64
    }

    fn count1(&mut self)->usize
    {
        let mut anti : HashSet<Vec2> = hash_set::HashSet::new();
        let mut freq : HashSet<char> = HashSet::new();
        
        tools::get_2d_iter(0,self.dx,0,self.dy)
        .into_iter()
        .for_each(|(x,y)|         
            {
                let p = Vec2::new(x as i64,y as i64);
                let c = *self.hash.get(&p).unwrap_or(&' ');         

                if c!='.'
                {
                    freq.insert(c);
                }
            }
        );

        for &f in freq.iter()
        {
            let freqs : HashSet<Vec2> = self.hash.iter()
            .filter(|&(v,c)| *c==f )
            .map(|(v,_)| *v)
            .collect();

            for &p1 in freqs.iter()
            {
                for &p2 in freqs.iter()
                {
                    if p1!=p2
                    {
                        let delta = p2.subv(p1);
                        let p = p2.addv(delta);

                        if self.pos(p)
                        {
                            anti.insert(p);
                        }
                    }
                }
            }
        }

        anti.len()
    }

    fn count2(&mut self)->usize
    {
        let mut anti : HashSet<Vec2> = hash_set::HashSet::new();
        let mut freq : HashSet<char> = HashSet::new();
        
        tools::get_2d_iter(0,self.dx,0,self.dy)
        .into_iter()
        .for_each(|(x,y)|         
            {
                let p = Vec2::new(x as i64,y as i64);
                let c = *self.hash.get(&p).unwrap_or(&' ');         

                if c!='.'
                {
                    anti.insert(p);
                    freq.insert(c);
                }
            }
        );

        for &f in freq.iter()
        {
            let freqs : HashSet<Vec2> = self.hash.iter()
                                                 .filter(|&(v,c)| *c==f )
                                                 .map(|(v,_)| *v)
                                                 .collect();            

            for &p1 in freqs.iter()
            {
                for &p2 in freqs.iter()
                {
                    if p1!=p2
                    {
                        let diff = p2.subv(p1);
                        let mut p = p2.addv(diff);

                        while self.pos(p)  
                        {
                            anti.insert(p);
                            p = p.addv(diff);
                        }
                    }
                }
            }               
        }

        anti.len()
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
    println!("Day8");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "............".to_string(),
        "........0...".to_string(),
        ".....0......".to_string(),
        ".......0....".to_string(),
        "....0.......".to_string(),
        "......A.....".to_string(),
        "............".to_string(),
        "............".to_string(),
        "........A...".to_string(),
        ".........A..".to_string(),
        "............".to_string(),
        "............".to_string(),
    ];
    assert_eq!(part1(&v),14);
}

#[test]
fn test2()
{
    let v = vec![
        "............".to_string(),
        "........0...".to_string(),
        ".....0......".to_string(),
        ".......0....".to_string(),
        "....0.......".to_string(),
        "......A.....".to_string(),
        "............".to_string(),
        "............".to_string(),
        "........A...".to_string(),
        ".........A..".to_string(),
        "............".to_string(),
        "............".to_string(),
    ];
    assert_eq!(part2(&v),34);
}
