use std::collections::HashMap;
use std::collections::HashSet;
use super::tools;
use super::vec2::Vec2;

#[derive(Debug)]
struct Data 
{
    hash : HashMap<Vec2,char>,
     pos : Vec2,
     dir : usize,
      dx : usize,
      dy : usize
}

impl Data {
    fn new(input: &[String]) -> Self 
    {
        let hash = tools::get_hash_table(input);
        let pos = *hash.iter().find(|&(_,v)| *v!='.' && *v!='#').unwrap().0;
        let d = *hash.get(&pos).unwrap();
            
        Data 
        {
            hash,
            pos,
            dir : Data::get_dir(d),
            dx  : input[0].len(),
            dy  : input.len()
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
            0 => Vec2::new( 0,-1),
            1 => Vec2::new( 1, 0),
            2 => Vec2::new( 0, 1),
            3 => Vec2::new(-1, 0),
            _ => panic!("unknown dir")
        }
    }

    fn rotate_right(&mut self)
    {
        self.dir = (self.dir+1)%4;        
    }

    fn patrol(&mut self)->(usize,usize)
    {
        let mut visited = HashSet::new();
        let mut count = HashSet::new();
        self.hash.insert(self.pos,'.');
        let mut steps = 0;

        while !visited.contains(&(self.pos,self.dir)) &&
              self.hash.contains_key(&self.pos)
        {
            visited.insert((self.pos,self.dir));
            count.insert(self.pos);
            
            let new_pos = self.pos.addv(Data::get_off(self.dir));           
            let n: char= *self.hash.get(&new_pos).unwrap_or(&'+');

            match n  
            {
                '+' => return (count.len(),steps),
                '.' => self.pos = new_pos,
                '#' => self.rotate_right(),
                 _  => panic!("unknown char")
            }
            
            steps+=1;
        }

        (count.len(),steps)
    }

    fn count1(&mut self) -> usize
    {
        self.patrol().0
    }

    fn count2(&mut self) -> usize
    {
        let org_pos = self.pos;
        let org_dir = self.dir;
        
        tools::get_2d_iter(0,self.dx,0,self.dy)
        .into_iter()
        .filter(|&(x,y)| 
        {
            let p = Vec2::new(x as i64,y as i64);
            let c = *self.hash.get(&p).unwrap_or(&' ');         
            self.hash.insert(p,'#');

            let steps = self.patrol().1;

            self.hash.insert(p,c);
            self.pos = org_pos;
            self.dir = org_dir;

            c!='#' && steps>=self.dx*self.dy
        }        
        ).count()
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
