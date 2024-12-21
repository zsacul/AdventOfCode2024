use std::collections::{HashMap, HashSet};

use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Data {
      hash  : HashMap<Vec2,char>,   
    visited : HashSet<Vec2>,
      dx    : usize,
      dy    : usize,
      s     : Vec2,
      e     : Vec2,
      cost  : HashMap<Vec2,i64>,   
}

impl Data {
    fn new(input: &[String]) -> Self {

        let mut hash = tools::get_hash_table(input);
        let s   : Vec2 = tools::find_in_hash(&hash,'S');
        let e   : Vec2 = tools::find_in_hash(&hash,'E');        

        hash.insert(s,'.');
        hash.insert(e,'.');
      
        Data 
        {
            hash,
            visited : HashSet::new(),
            dx : input[0].len(),
            dy : input.len(),
            s,
            e,
            cost : HashMap::new(),
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
        let mut q = Vec::new();
        q.push((self.s,0));
        let mut best = usize::MAX;

        while !q.is_empty()
        {
            let (p,cost) = q.remove(0);
            
            
            if self.get(p) != '.' && p != self.s 
            {
                continue;
            }
            
            let a = *self.cost.get(&p).unwrap_or(&9999999);
            self.cost.insert(p,a.min(cost));

            if p == self.e 
            {
                best = best.min(cost as usize);
                //return best;
                //println!("best: {}",best);
            }
            
            if self.v(p)
            {
                continue;
            }
            else
            {
                self.visited.insert(p);
            }
            
            self.visited.insert(p);

            for np in p.around4()
            {
                q.push((np,cost+1));                
            }
        }

        best
    }    

    fn bfs2(&mut self,dir:char,bestv:usize)->usize
    {
        0
    }

    fn count1(&mut self,step:i64)->usize
    {
        let orgs = self.s;
        let orge = self.e;
        let mut costs_to = HashMap::new();

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let pos = Vec2::new(x as i64,y as i64);

                if *self.hash.get(&pos).unwrap_or(&'#')=='.'
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

                if *self.hash.get(&pos).unwrap_or(&'#')=='.'
                {
                    self.s = Vec2::new(x as i64,y as i64);
                    costs.insert(pos, self.bfs());
                }
            }
        }

        self.s = orgs;

        let mut res = 99999999;
        let mut count=0;

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let pos = Vec2::new(x as i64,y as i64);

                if !costs_to.contains_key(&pos) {continue;}

                for d in Vec2::dirs4()
                {
                    let np = pos.addv(d.mulv(Vec2::new(step ,step)));
                    let ast = self.get(np);

                    if (ast=='.' || ast=='E') && costs.contains_key(&np) && costs_to.contains_key(&pos) 
                    {
                         let cost_t   = *costs_to.get(&pos).unwrap();  
                         let cost_f   = *costs.get(&np).unwrap();
                         let ncost    =  cost_t + cost_f + 2;   
                         let cost_org = *costs.get(&self.s).unwrap();

                         let save = (cost_org as i64) - (ncost as i64);
                         res = res.min(ncost);

                         if save>=100
                         //if save>=1
                         {
                            count+=1;
                            println!("{:?} {:?} {} {} {} = {}",pos,np,ncost,res,cost_org,save);
                         }
                    }
                }
            }
        }

        count

    }

    fn count2(&mut self,step:i64,lim:i64)->usize
    {
        let orgs = self.s;
        let orge = self.e;
        let mut costs_to = HashMap::new();

        self.s = orge;
        self.e = orgs;

        let total_cost = self.bfs() as i64;
        self.cost.clear();

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let pos = Vec2::new(x as i64,y as i64);

                if self.get(pos)=='.'
                {
                    self.e = Vec2::new(x as i64,y as i64);
                    let cc = *self.cost.get(&self.e).unwrap_or(&9999999);
                    if cc!=9999999  
                    {                        
                        costs_to.insert(pos, cc);
                    }
                }
            }
        }

        self.e = orge;
        self.s = orgs;

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

        
        let mut count=0;
        let mut amount = HashMap::new();

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let pos = Vec2::new(x as i64,y as i64);

                if !costs_to.contains_key(&pos) { continue; }

                let ss = step;

                for ny in pos.y-ss..=pos.y+ss
                {
                    for nx in pos.x-ss..=pos.x+ss
                    {
                        let cos = (pos.x-nx).abs() + (pos.y-ny).abs();
                        let moves = cos;

                        if  moves>=2 && moves<= step
                        {
                            let np = Vec2::new(nx,ny);

                            if self.get(np)=='.' && costs.contains_key(&np) && costs_to.contains_key(&pos) 
                            {
                                let cost_t   = *costs_to.get(&pos).unwrap() as usize;  
                                let cost_f   = *costs.get(&np).unwrap();
                                let ncost    =  cost_t + cost_f + cos as usize;   
                                let cost_org = *costs.get(&self.s).unwrap();

                                let save = (cost_org as i64) - (ncost as i64);

                                if save>=lim
                                {
                                    count+=1;
                                    let a = amount.get(&save).unwrap_or(&0);
                                    amount.insert(save, a+1);
                                    
                                   // println!("{:?} {:?} {} {} {} = {}",pos,np,ncost,res,cost_org,save);
                                }
                            }
                        }
                    }
                }
            }
        }
        

        let mut v = vec![];
        for (s,a) in amount
        {
            v.push((s,a));
        }
        v.sort();
        println!("{:?}",v);

        count

    }

}




pub fn part1(data:&[String],lim:i64)->usize
{
    Data::new(data).count2(2,lim)
}

pub fn part2(data:&[String],step:i64,lim:i64)->usize
{
    Data::new(data).count2(step,lim)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day20");
    println!("part1: {}",part1(data,100));
    //println!("part2: {}",part2(data,20,100));
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
    assert_eq!(part1(&v,100),5);
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
    assert_eq!(part2(&v,20,74),7);
}
