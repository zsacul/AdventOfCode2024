use std::char;
use std::collections::HashMap;
use std::collections::HashSet;
use super::tools;
use super::vec2::Vec2;

#[derive(Debug)]
struct Data {
       hash: HashMap<Vec2,char>,
        pos: Vec2,
        dir: usize,
        dx:usize,
        dy:usize
}

impl Data {
    fn new(input: &[String]) -> Self {
        let  hash = tools::get_hash_table(input);
        

        let pos = *hash.iter().find(|&(k,v)| *v!='.' && *v!='#').unwrap().0;
        let d = *hash.clone().get(&pos).unwrap();
            
        Data {
            hash:hash ,
            pos:pos,
            dir:Data::get_dir(d),
            dx:input[0].len(),
            dy:input.len()
        }
    }


    fn get_dir(c:char)->usize
    {
        match c
        {
            '^' => 0,
            '>' => 1,
            'v' => 2,
            '<' => 3,
            _ => 0
        }
    }

    fn get_off(dir:usize)->Vec2
    {
        match dir
        {
            0 => Vec2::new( 0,-1 ),
            1 => Vec2::new( 1, 0),
            2 => Vec2::new( 0, 1),
            3 => Vec2::new(-1, 0),
            _ => Vec2::new( 0, 0)
        }
    }

    fn right(&mut self)
    {
        self.dir = (self.dir+1)%4;        
    }

    fn get_pos(line:&[usize])->HashMap<usize,usize>
    {
        line.iter()
            .enumerate()
            .map(|(id,v)| (*v,id))
            .collect()
    }

    fn print_hash(&self,hash:&HashMap<Vec2,char>)
    {
        println!();
        //println!("dx = {}, dy = {}",self.dx,self.dy);
        let s = 32;

        for yy in 0..10
        {
            for xx in 0..10
            {
                let x = xx; //s + xx-yy;
                let y = yy; //xx + yy;
                let p = Vec2::new(x as i64,y as i64);
                let c = hash.get(&p).unwrap_or(&' ');
                print!("{c}");               
            }
            println!();
        }
    }

    fn print_hash2(&self,hash:&HashSet<Vec2>)
    {
        println!();
        //println!("dx = {}, dy = {}",self.dx,self.dy);
        let s = 32;

        for yy in 0..10
        {
            for xx in 0..10
            {
                let x = xx; //s + xx-yy;
                let y = yy; //xx + yy;
                let p = Vec2::new(x as i64,y as i64);
                let c = if hash.contains(&p) {'X'} else {' '};
                print!("{c}");               
            }
            println!();
        }
    }


    fn patrol(&mut self)->usize
    {
        let mut len = 0;

        self.hash.insert(self.pos,'.');

        let mut visited = HashSet::new();

        while self.hash.get(&self.pos).is_some() && self.hash.get(&self.pos).unwrap_or(&'+')!=&'+'
        {
            //self.hash.insert(self.pos,'X');
            visited.insert(self.pos);
            
            let nm = self.pos.addv(Data::get_off(self.dir));           
            let n= *self.hash.get(&nm).unwrap_or(&'+');
            
            //println!("{:?}->{:?} = {} {}",self.pos,nm,n,self.dir);


            if n=='+'
            {
                return visited.len();
            }
            if n=='.'
            {
                self.pos = nm;
            }
            if n=='#'
            {
                self.right();
            }
            if n=='.'
            {
                self.pos = nm;
            }
            //println!("{:?}",self.pos);
            //self.print_hash2(&visited);
            
            len+=1;
        }

        visited.len()
    }


    fn patrol2(&mut self)->(usize,usize)
    {
        let mut len = 0;

        self.hash.insert(self.pos,'.');

        let mut visited = HashSet::new();

        while len<132*132 && self.hash.get(&self.pos).is_some() && self.hash.get(&self.pos).unwrap_or(&'+')!=&'+'
        {
            //self.hash.insert(self.pos,'X');
            visited.insert(self.pos);
            
            let nm = self.pos.addv(Data::get_off(self.dir));           
            let n= *self.hash.get(&nm).unwrap_or(&'+');
            
            //println!("{:?}->{:?} = {} {}",self.pos,nm,n,self.dir);


            if n=='+'
            {
                return (visited.len(),len);
            }
            if n=='.'
            {
                self.pos = nm;
            }
            if n=='#'
            {
                self.right();
            }
            if n=='.'
            {
                self.pos = nm;
            }
            //println!("{:?}",self.pos);
            //self.print_hash2(&visited);
            
            len+=1;
        }

        (visited.len(),len)
    }



    fn count1(&mut self) -> usize
    {
        self.patrol()
        //self.hash.values().filter(|&v| *v=='X').count()
    }


    fn count2(&mut self) -> usize
    {
        let org_pos = self.pos;
        let org_dir = self.dir;
        let mut count = 0;
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                let p = Vec2::new(x as i64,y as i64);
                let c = *self.hash.get(&p).unwrap_or(&' ');
                
                if c=='#'
                {
                    continue;
                }
                self.hash.insert(p,'#');
                let p2 = self.patrol2();

                println!("{:?} ",p2);

                if p2.1>130*130
                {
                    count+=1;
                }
                self.hash.insert(p,c);
                self.pos = org_pos;
                self.dir = org_dir;
            }
        }
        
        count
        
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
    println!("Day6");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "....#.....".to_string(),
        ".........#".to_string(),
        "..........".to_string(),
        "..#.......".to_string(),
        ".......#..".to_string(),
        "..........".to_string(),
        ".#..^.....".to_string(),
        "........#.".to_string(),
        "#.........".to_string(),
        "......#...".to_string(),
    ];
    assert_eq!(part1(&v),41);
}

#[test]
fn test2()
{
    let v = vec![
        "....#.....".to_string(),
        ".........#".to_string(),
        "..........".to_string(),
        "..#.......".to_string(),
        ".......#..".to_string(),
        "..........".to_string(),
        ".#..^.....".to_string(),
        "........#.".to_string(),
        "#.........".to_string(),
        "......#...".to_string(),
    ];
    assert_eq!(part2(&v),6);
}
