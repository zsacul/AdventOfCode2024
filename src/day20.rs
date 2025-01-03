use std::collections::{HashMap, HashSet, VecDeque};
use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Data 
{
    hash    : HashMap<Vec2,char>,   
    visited : HashSet<Vec2>,
    dx      : usize,
    dy      : usize,
    s       : Vec2,
    e       : Vec2,
}

impl Data {
    fn new(input: &[String]) -> Self 
    {
        let mut hash = tools::get_hash_table(input);
        let s : Vec2 = tools::find_in_hash(&hash,'S');
        let e : Vec2 = tools::find_in_hash(&hash,'E');        

        hash.insert(s,'.');
        hash.insert(e,'.');
      
        Data 
        {
            hash,
            visited : HashSet::new(),
            dx : input[0].len(),
            dy : input.len(),
            s,
            e
        }
    }

    #[allow(unused)]
    fn print_hash(&self,vis:HashSet<Vec2>)
    {
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let p = Vec2::new(x as i64,y as i64);                        
                let c = *self.hash.get(&p).unwrap_or(&'.');
               
                if vis.contains(&p)
                {
                    print!("O");
                }
                    else 
                {
                    print!("{}", c);
                }
            }
            println!();
        }
        println!();
    }

    fn get(&self,p:Vec2)->char
    {
        *self.hash.get(&p).unwrap_or(&'#')
    }

    fn v(&self,p:Vec2)->bool
    {
        self.visited.contains(&p)
    }

    fn bfs(&mut self)->usize
    {
        self.visited.clear();
        //let mut q = vec![(self.s,0)];       
        let mut q = VecDeque::new();
        q.push_back((self.s,0));

        let mut best = usize::MAX;

        while !q.is_empty()
        {
            let (p,cost) = q.pop_front().unwrap();
            
            if p == self.e 
            {
                best = cost;
                return best;
            }
            
            if self.get(p) != '.' && p != self.s 
            {
                continue;
            }
            
            if self.v(p)
            {
                continue;
            }

            self.visited.insert(p);

            for np in p.around4()
            {
                q.push_back((np,cost+1));                
            }
        }

        best
    }    

    fn count2(&mut self,step:i64,lim:i64)->usize
    {
        let orgs = self.s;
        let orge = self.e;
        let mut costs_to = HashMap::new();

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let pos = Vec2::new(x as i64,y as i64);

                if self.get(pos)=='.'
                {
                    self.e = Vec2::new(x as i64,y as i64);
                    costs_to.insert(pos, self.bfs());
                }
            }
        }

        self.e = orge;
        let mut costs = HashMap::new();

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let pos = Vec2::new(x as i64,y as i64);


                if self.get(pos)=='.'
                {
                    self.s = Vec2::new(x as i64,y as i64);
                    costs.insert(pos, self.bfs());
                }
            }
        }

        self.s = orgs;

        let mut res = 99999999;
        
        let mut count=0;
        let mut amount = HashMap::new();

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let pos = Vec2::new(x as i64,y as i64);
                if !costs_to.contains_key(&pos) { continue; }                            

                for ny in pos.y-step..=pos.y+step
                {                    
                    for nx in pos.x-step..=pos.x+step
                    {
                        let cos = (pos.x-nx).abs() + (pos.y-ny).abs();
                        let moves = cos;

                        if  moves>=2 && moves<= step
                        {
                            let np = Vec2::new(nx,ny);


                            if self.get(np)=='.' && costs.contains_key(&np) && costs_to.contains_key(&pos) 
                            {
                                let cost_t   = *costs_to.get(&pos).unwrap();  
                                let cost_f   = *costs.get(&np).unwrap();
                                let ncost    =  cost_t + cost_f + cos as usize;   
                                let cost_org = *costs.get(&self.s).unwrap();

                                let save = (cost_org as i64) - (ncost as i64);
                                res = res.min(ncost);                                

                                if save>=lim
                                {
                                    count+=1;

                                    let a = amount.get(&save).unwrap_or(&0);
                                    amount.insert(save, a+1);                                  
                                }
                            }
                        }
                    }
                }
            }
        }
        
        count

    }

}

pub fn part1(data:&[String])->usize
{    
    Data::new(data).count2(2,100)
}

pub fn part2(data:&[String],step:i64,lim:i64)->usize
{
    Data::new(data).count2(step,lim)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day20");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data,20,100))
}

#[test]
fn test1()
{
    let v = vec![
        "###############".to_string(),
        "#...#...#.....#".to_string(),
        "#.#.#.#.#.###.#".to_string(),
        "#S#...#.#.#...#".to_string(),
        "#######.#.#.###".to_string(),
        "#######.#.#...#".to_string(),
        "#######.#.###.#".to_string(),
        "###..E#...#...#".to_string(),
        "###.#######.###".to_string(),
        "#...###...#...#".to_string(),
        "#.#####.#.###.#".to_string(),
        "#.#...#.#.#...#".to_string(),
        "#.#.#.#.#.#.###".to_string(),
        "#...#...#...###".to_string(),
        "###############".to_string(),
    ];
    
    assert_eq!(part2(&v,2,38),3);
}

#[test]
fn test2()
{
    let v = vec![
        "###############".to_string(),
        "#...#...#.....#".to_string(),
        "#.#.#.#.#.###.#".to_string(),
        "#S#...#.#.#...#".to_string(),
        "#######.#.#.###".to_string(),
        "#######.#.#...#".to_string(),
        "#######.#.###.#".to_string(),
        "###..E#...#...#".to_string(),
        "###.#######.###".to_string(),
        "#...###...#...#".to_string(),
        "#.#####.#.###.#".to_string(),
        "#.#...#.#.#...#".to_string(),
        "#.#.#.#.#.#.###".to_string(),
        "#...#...#...###".to_string(),
        "###############".to_string(),
    ];

    assert_eq!(part2(&v,20,50),285);
}
