use std::collections::HashMap;
use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Data {
      hash  : HashMap<Vec2,char>,   
      dx    : usize,
      dy    : usize,
      s     : Vec2,
      e     : Vec2,
      pos   : Vec<Vec2>,
}

impl Data {
    fn new(input: &[String],dx:usize,dy:usize,fall:usize) -> Self 
    {
        let mut hash = HashMap::new();
        let mut pos = vec![];

        for line in input.iter()
        {
            let tab = tools::split_to_usize(line, ",");
            pos.push(Vec2::new(tab[0] as i64,tab[1] as i64));
        }

        for &p in pos.iter().take(fall)
        {            
            hash.insert(p,'#');
        }

        Data 
        {
            hash,
            dx,
            dy,
            s : Vec2::new(0,0),
            e : Vec2::new((dx-1) as i64,(dy-1) as i64),
            pos
        }
    }


    #[allow(unused)]
    fn print_hash(&self)
    {
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let p = Vec2::new(x as i64,y as i64);                        
                print!("{}", *self.hash.get(&p).unwrap_or(&'.'));
            }
            println!();
        }
        println!();
    }

    fn get(&self,p:Vec2)->char
    {
        *self.hash.get(&p).unwrap_or(&'.')
    }

    fn set(&mut self,p:Vec2,c:char)
    {
        self.hash.insert(p,c);
    }

    fn count(&mut self)->usize
    {
        let mut stack = vec![];
        let mut steps = 0usize;

        stack.push(self.s);
        self.set(self.s,'O');

        while !stack.is_empty() 
        {
            let list = stack.clone();
            stack.clear();
    
            for p in list
            {              
                if p==self.e
                {
                    return steps;
                }

                for n in p.around4()
                {                   
                    if n.x>=0 && n.y>=0 && n.x<self.dx as i64 && n.y<self.dy as i64 && self.get(n) == '.'
                    {
                        self.set(n,'O');
                        stack.push(n);
                    }
                }
            }
            steps+=1;
        }
        0
    }

}

pub fn part1(data:&[String],dx:usize,dy:usize,fall:usize)->usize
{
    Data::new(data,dx,dy,fall).count()
}

pub fn part2(data:&[String],dx:usize,dy:usize)->String
{
    let mut steps = 1;

    loop
    {
        let mut data = Data::new(data,dx,dy,steps);

        if data.count()==0
        {
            let pos= data.pos[steps-1];
            return format!("{},{}",pos.x,pos.y);
        }
        steps+=1;
    }
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day18");
    println!("part1: {}",part1(data,71,71,1024));
    println!("part2: {}",part2(data,71,71));
}

#[test]
fn test1()
{
    let v = vec![
        "5,4".to_string(),
        "4,2".to_string(),
        "4,5".to_string(),
        "3,0".to_string(),
        "2,1".to_string(),
        "6,3".to_string(),
        "2,4".to_string(),
        "1,5".to_string(),
        "0,6".to_string(),
        "3,3".to_string(),
        "2,6".to_string(),
        "5,1".to_string(),
        "1,2".to_string(),
        "5,5".to_string(),
        "2,5".to_string(),
        "6,5".to_string(),
        "1,4".to_string(),
        "0,4".to_string(),
        "6,4".to_string(),
        "1,1".to_string(),
        "6,1".to_string(),
        "1,0".to_string(),
        "0,5".to_string(),
        "1,6".to_string(),
        "2,0".to_string(),
     ];
    assert_eq!(part1(&v,7,7,12),22);
}

#[test]
fn test2()
{
    let v = vec![
        "5,4".to_string(),
        "4,2".to_string(),
        "4,5".to_string(),
        "3,0".to_string(),
        "2,1".to_string(),
        "6,3".to_string(),
        "2,4".to_string(),
        "1,5".to_string(),
        "0,6".to_string(),
        "3,3".to_string(),
        "2,6".to_string(),
        "5,1".to_string(),
        "1,2".to_string(),
        "5,5".to_string(),
        "2,5".to_string(),
        "6,5".to_string(),
        "1,4".to_string(),
        "0,4".to_string(),
        "6,4".to_string(),
        "1,1".to_string(),
        "6,1".to_string(),
        "1,0".to_string(),
        "0,5".to_string(),
        "1,6".to_string(),
        "2,0".to_string(),
     ];
    assert_eq!(part2(&v,7,7),"6,1");
}
