use std::collections::HashMap;
use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Robot {
    p: Vec2, 
    v: Vec2, 
}

impl Robot
{
    fn new(line: String) -> Self 
    {
        let px = tools::i64_get_between(line.as_str(),"p=",",");
        let py = tools::i64_get_between(line.as_str(),","," v");
        let vx = tools::i64_get_between(line.as_str(),"v=",",");
        let vy = line.split(",").last().unwrap().parse::<i64>().unwrap();                
        
        Robot {
            p:Vec2::new(px,py),
            v:Vec2::new(vx,vy),
        }
    }

    fn go(&mut self,dx:usize,dy:usize,n:usize)
    {
        self.p   = self.p.add(self.v.x*n as i64,self.v.y*n as i64);
        self.p.x = (n as i64*dx as i64 + self.p.x)%dx as i64;
        self.p.y = (n as i64*dy as i64 + self.p.y)%dy as i64;
    }
}

#[derive(Debug)]
struct Data 
{
      game: Vec<Robot>,   
        dx: usize,
        dy: usize,
}

impl Data {
    fn new(input: &[String],dx:usize,dy:usize) -> Self 
    {
        Data {
            game: input.iter()
                       .map(|line| Robot::new(line.clone()))
                       .collect(),
                       dx,
                       dy,
        }
    }

    fn get_hash(&self)->HashMap<Vec2,usize>
    {
        let mut hash: HashMap<Vec2,usize> = HashMap::new();
        
        for r in self.game.iter()
        {         
            *hash.entry(r.p).or_insert(0) += 1;
        }        
        
        hash
    }

    fn print_hash(&self)
    {
        let hash = self.get_hash();

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let p = Vec2::new(x as i64,y as i64);                        
                print!("{}",if *hash.get(&p).unwrap_or(&0)>0 {"X"} else {"."});
            }
            println!();
        }
        println!();
    }

    fn count1(&mut self,n:usize)->usize
    {
        self.game.iter_mut().for_each(|r| r.go(self.dx,self.dy,n));

        let mx = (self.dx as i64-1)/2;
        let my = (self.dy as i64-1)/2;
        
        let mut hash : HashMap<usize,usize> = HashMap::new();
                
        self.game
            .iter()
            .filter(|&r| r.p.x!=mx && r.p.y!=my)
            .for_each(|r| *hash.entry((r.p.y<my) as usize*2 + (r.p.x<mx) as usize).or_insert(0) += 1 );
                
        hash.values().product()        
    }

    fn count2(&mut self,n:usize)->usize
    {
        for i in 0..n
        {
            self.game.iter_mut().for_each(|r| r.go(self.dx,self.dy,1));
            let hash: HashMap<Vec2,usize> = self.get_hash();

            for y in 0..self.dy
            {
                let mut cnt=0;

                for x in 0..self.dx
                {
                    let v = Vec2::new(x as i64,y as i64);

                    if *hash.get(&v).unwrap_or(&0)==1
                    {
                        cnt+=1;
                        if cnt>10 
                        {
                            self.print_hash();
                            return i+1;
                        }
                    }
                      else
                    {
                        cnt=0;
                    }
                }
            } 
        }
        0
    }


}

pub fn part1(data:&[String],dx:usize,dy:usize,n:usize)->usize
{
    Data::new(data,dx,dy).count1(n)
}

pub fn part2(data:&[String],dx:usize,dy:usize,n:usize)->usize
{
    Data::new(data,dx,dy).count2(n)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day14");
    println!("part1: {}",part1(data,101,103,100));
    println!("part2: {}",part2(data,101,103,100_000));
}

#[test]
fn test1()
{
    let v = vec![
        "p=0,4 v=3,-3".to_string(),
        "p=6,3 v=-1,-3".to_string(),
        "p=10,3 v=-1,2".to_string(),
        "p=2,0 v=2,-1".to_string(),
        "p=0,0 v=1,3".to_string(),
        "p=3,0 v=-2,-2".to_string(),
        "p=7,6 v=-1,-3".to_string(),
        "p=3,0 v=-1,-2".to_string(),
        "p=9,3 v=2,3".to_string(),
        "p=7,3 v=-1,2".to_string(),
        "p=2,4 v=2,-3".to_string(),
        "p=9,5 v=-3,-3".to_string(),
    ];
    assert_eq!(part1(&v,11,7,100),12);
}

#[test]
fn test2()
{
    let v = vec![
        "p=2,4 v=2,-3".to_string(),
    ];
    assert_eq!(part1(&v,5,11,7),1);
}
