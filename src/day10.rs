use std::collections::HashMap;
use super::tools;
use super::vec2::Vec2;

#[derive(Debug)]
struct Data 
{
    hash : HashMap<Vec2,i8>,
      dx : usize,
      dy : usize
}

impl Data {
    fn new(input: &[String]) -> Self 
    {
        let hash = tools::get_hash_table(input).iter()
                                                .map(|(k,v)| (*k,if *v=='.' {-1i8} else {v.to_digit(10).unwrap() as i8}) )
                                                .collect::<HashMap<Vec2,i8>>();
            
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

    fn count(&mut self,part_two:bool)->usize
    {
        let start : Vec<Vec2> = self.hash.clone()
                                         .into_iter()
                                         .filter(|&(_,v)| v==0)
                                         .map(|(k,_)| k)
                                         .collect();
                                    
        let mut count = 0;
        
        for s in start
        {
            let mut stack = vec![s];
            let mut tab   = vec![vec![0;self.dx];self.dy];

            while let Some(p) = stack.pop()
            {
                let n = tab[p.y as usize][p.x as usize];

                for dir in Vec2::dirs4()
                {
                    let p2 = p.addv(dir);

                    if self.pos(p2) && self.hash[&p2]==n+1 && (part_two || tab[p2.y as usize][p2.x as usize]==0)
                    {
                        tab[p2.y as usize][p2.x as usize] = n+1;
                    
                        if n+1==9 { count+=1; }
                        stack.push(p2);    
                    }
                }
            }
        }
        count
    }
}

pub fn part1(data:&[String])->usize
{
    Data::new(data).count(false)
}

pub fn part2(data:&[String])->usize
{
    Data::new(data).count(true)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day10");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "...0...".to_string(),
        "...1...".to_string(),
        "...2...".to_string(),
        "6543456".to_string(),
        "7.....7".to_string(),
        "8.....8".to_string(),
        "9.....9".to_string(),
    ];
    assert_eq!(part1(&v),2);
}



#[test]
fn test2()
{
    let v = vec![
        "..90..9".to_string(),
        "...1.98".to_string(),
        "...2..7".to_string(),
        "6543456".to_string(),
        "765.987".to_string(),
        "876....".to_string(),
        "987....".to_string(),
    ];
    assert_eq!(part1(&v),4);
}


#[test]
fn test3()
{
    let v = vec![
        "10..9..".to_string(),
        "2...8..".to_string(),
        "3...7..".to_string(),
        "4567654".to_string(),
        "...8..3".to_string(),
        "...9..2".to_string(),
        ".....01".to_string(),
    ];
    assert_eq!(part1(&v),3);
}


#[test]
fn test4()
{
    let v = vec![
        "89010123".to_string(),
        "78121874".to_string(),
        "87430965".to_string(),
        "96549874".to_string(),
        "45678903".to_string(),
        "32019012".to_string(),
        "01329801".to_string(),
        "10456732".to_string(),
    ];
    assert_eq!(part1(&v),36);
}

#[test]
fn test5()
{
    let v = vec![
        ".....0.".to_string(),
        "..4321.".to_string(),
        "..5..2.".to_string(),
        "..6543.".to_string(),
        "..7..4.".to_string(),
        "..8765.".to_string(),
        "..9....".to_string(),        
    ];
    assert_eq!(part2(&v),3);
}

#[test]
fn test6()
{
    let v = vec![
        "..90..9".to_string(),
        "...1.98".to_string(),
        "...2..7".to_string(),
        "6543456".to_string(),
        "765.987".to_string(),
        "876....".to_string(),
        "987....".to_string(),
    ];
    assert_eq!(part2(&v),13);
}


#[test]
fn test7()
{
    let v = vec![
        "012345".to_string(),
        "123456".to_string(),
        "234567".to_string(),
        "345678".to_string(),
        "4.6789".to_string(),
        "56789.".to_string(),
    ];
    assert_eq!(part2(&v),227);
}


#[test]
fn test8()
{
    let v = vec![
        "89010123".to_string(),
        "78121874".to_string(),
        "87430965".to_string(),
        "96549874".to_string(),
        "45678903".to_string(),
        "32019012".to_string(),
        "01329801".to_string(),
        "10456732".to_string(),
    ];
    assert_eq!(part2(&v),81);
}



