use std::collections::{HashMap,HashSet,VecDeque};
use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Data 
{
    hash    : HashMap<Vec2,u8>,
    fen     : HashMap<Vec2,u8>,
    visited : HashSet<Vec2>,
    dx : usize,
    dy : usize
}

impl Data {
    fn new(input: &[String]) -> Self 
    {
        let hash = tools::get_hash_table(input).iter()
                                                .map(|(k,v)| (*k, *v as u8) )
                                                .collect::<HashMap<Vec2,u8>>();
            
        Data 
        {
            hash,           
            fen     : HashMap::new(),           
            visited : HashSet::new(),
            dx      : input[0].len(),
            dy      : input.len(),
        }        
    }
   
    fn pos(&self,p:Vec2)->bool
    {
        p.x>=0 && p.x<self.dx as i64 && p.y>=0 && p.y<self.dy as i64
    }

    fn fence(&self,p:Vec2)->usize
    {
        let mut c = 0;
        for &n in p.around4().iter()
        {
            if !self.pos(n) || self.hash[&n]!=self.hash[&p] { c+=1; }
        }
        c
    }

    #[allow(clippy::collapsible_else_if)]
    fn fence2(&mut self,p:Vec2)->usize
    {
        let c = 0;
        for &n in p.around4().iter()
        {
            if !self.pos(n) || self.hash[&n]!=self.hash[&p] 
            { 
                if n.y==p.y
                {
                    if n.x<p.x
                    {
                        self.fen.insert( Vec2::new(2*p.x-1, 2*p.y),b'|');
                    }
                    else
                    {
                        self.fen.insert( Vec2::new(2*p.x+1, 2*p.y),b'^');
                    }
                }
                  else
                {
                    if n.y<p.y
                    {
                        self.fen.insert( Vec2::new(2*p.x, 2*p.y-1),b'>');
                    }
                    else
                    {
                        self.fen.insert( Vec2::new(2*p.x, 2*p.y+1),b'-');
                    }
                }              
            }
        }
        c
    }


    fn flood(&mut self,p:Vec2,part_two:bool)->(usize,usize)
    {
        let mut stack = VecDeque::new();
        let mut field = 0;

        if self.visited.contains(&p) { return (0,0); }

        stack.push_back(p);
        let mut fen = 0;

        

        while let Some(p) = stack.pop_front()
        {
            if !self.visited.contains(&p)
            {
                let n = self.hash[&p];
                self.visited.insert(p);

                field+=1;
                
                if part_two
                {
                    fen+=self.fence2(p);
                }
                else 
                {
                    fen+=self.fence(p);                    
                }

                for dir in Vec2::dirs4()
                {
                    let p2 = p.addv(dir);

                    if self.pos(p2) && self.hash[&p2]==n && !self.visited.contains(&p2)
                    {                       
                        stack.push_back(p2);                        
                    }
                }
            }
        }
        (field,fen)
    }

    fn count1(&mut self)->usize
    {
        tools::get_2d_i(self.dx,self.dy).iter().map(|&(x,y)|
            {
                let (field,fence) = self.flood(Vec2::new(x as i64,y as i64),false);                
                field*fence
            }
        ).sum()
    }

    #[allow(unused)]
    fn print_fen(&self)
    {
        for y in -1..=(self.dy as i64*2)+1
        {
            for x in -1..=(self.dx as i64*2)+1
            {   
                let vv = *self.fen.get(&Vec2::new(x,y)).unwrap_or(&b'.');
                print!("{}",vv as char);
            }
            
            println!();
        }             
    }

    fn count_horizontally(&self,c:u8)->usize
    {
        let mut hor =0;
        for y in (-1..=(self.dy as i64*2)+1).step_by(2)
        {
            let mut h=0;
            let mut prev = 0;
            for x in (0..=(self.dx as i64)*2+1).step_by(2)
            {                        
                let p = Vec2::new(x,y);
                if self.fen.contains_key(&p) && self.fen[&p]==c
                {
                    if prev==c { h+=1; }
                          else { if h>0 {hor+=1;} h=1;}                            
                }
                else
                {
                    if h>0 {hor+=1;}
                    h=0;
                }
                prev = *self.fen.get(&p).unwrap_or(&0);
                
            }                   
            if h>0 { hor+=1; }    
        }
        hor
    }

    fn count_vertically(&self,c:u8)->usize
    {
        let mut ver =0;
        for x in (-1..=(self.dx as i64*2)+1).step_by(2)
        {
            let mut v=0;
            let mut prev = 0;
            for y in (0..=(self.dy as i64*2)+1).step_by(2)
            {                        
                let p = Vec2::new(x,y);
                if self.fen.contains_key(&p) && self.fen[&p]==c
                {
                    if prev==c { v+=1; }
                          else { if v>0 {ver+=1;} v=1; } 
                }
                else
                {
                    if v>0 {ver+=1;}
                    v=0;
                }
                prev = *self.fen.get(&p).unwrap_or(&0);
            }               
            if v>0 { ver+=1; }    
        }
        ver
    }


    fn count2(&mut self)->usize
    {
        let mut res = 0;

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {                
                let pp = Vec2::new(x as i64,y as i64);
                if self.visited.contains(&pp) { continue; }

                self.fen.clear();
                let f = self.flood(Vec2::new(x as i64,y as i64),true);
                
                let fence = self.count_horizontally(b'-') + self.count_horizontally(b'>') +
                                   self.count_vertically(b'^')   + self.count_vertically(b'|');

                res+=f.0*fence;
            }            
        }

        res
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
    println!("Day12");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "AAAA".to_string(),
        "BBCD".to_string(),
        "BBCC".to_string(),
        "EEEC".to_string(),
     ];
    assert_eq!(part1(&v),2*4*10+4*8+3*8+1*4);
}

#[test]
fn test2()
{
    let v = vec![
        "AAAA".to_string(),
        "BBCD".to_string(),
        "BBCC".to_string(),
        "EEEC".to_string(),
    ];
    assert_eq!(part2(&v),80);
}

#[test]
fn test3()
{
    let v = vec![
        "EEEEE".to_string(),
        "EXXXX".to_string(),
        "EEEEE".to_string(),
        "EXXXX".to_string(),
        "EEEEE".to_string(),        
    ];
    assert_eq!(part2(&v),236);
}


#[test]
fn test4()
{
    let v = vec![
        "AAAAAA".to_string(),
        "AAABBA".to_string(),
        "AAABBA".to_string(),
        "ABBAAA".to_string(),
        "ABBAAA".to_string(),
        "AAAAAA".to_string(),
    ];
    assert_eq!(part2(&v),368);
}

