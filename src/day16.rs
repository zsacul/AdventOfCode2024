use std::collections::{HashMap, HashSet};
use std::usize;
use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Data {
      hash  : HashMap<Vec2,char>,   
      dx    : usize,
      dy    : usize,
      s     : Vec2,
      e     : Vec2,
}

impl Data {
    fn new(input: &[String]) -> Self {

        let hash = tools::get_hash_table(input);


        let s   : Vec2 = *hash.clone()
                              .iter()
                              .find(|c|c.1==&'S')
                              .unwrap().0;
        let e   : Vec2 = *hash.clone()
                            .iter()
                            .find(|c|c.1==&'E')
                            .unwrap().0;
      
        Data 
        {
            hash,
            dx : input[0].len(),
            dy : input.len(),
            s,
            e
        }
    }

    fn get_offset(c:char)->Vec2
    {
        match c
        {
            '^' => Vec2::new( 0,-1),
            'v' => Vec2::new( 0, 1),
            '<' => Vec2::new(-1, 0),
            '>' => Vec2::new( 1, 0),
            _   => panic!("get_offset")
        }
    }

    fn get_offset_back(c:char)->Vec2
    {
        Data::get_offset(c).mulv(Vec2::new(-1,-1))
    }


    fn left(c:char)->char
    {
        match c
        {
            '^' => '<',
            '<' => 'v',
            'v' => '>',
            '>' => '^',
            _   => panic!("left")
        }
    }

    fn right(c:char)->char
    {
        match c
        {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _   => panic!("right")
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


    fn dfs(&mut self,memo:&mut HashMap<(Vec2,char),usize>,mini:&mut usize,p:Vec2,dir:char,cost:usize)->usize
    {
        if memo.contains_key(&(p,dir))
        {
         //   return *memo.get(&(p,dir)).unwrap();
        }
                
        if cost > *mini
        {
            memo.insert((p,dir),9999999999999999999);
            return 9999999999999999999;
        }
        if p == self.e
        {
            return cost;
        }

        if self.get(p) != '.' && p != self.s
        {
            memo.insert((p,dir),9999999999999999999);
            return 9999999999999999999;
        }

        let df = Data::get_offset(dir);
        let dl = Data::get_offset(Data::left(dir));
        let dr = Data::get_offset(Data::right(dir));

        let f = self.dfs(memo,mini,p.addv(df),                  dir,cost+    1);
         *mini = f.min(*mini);
        let l = self.dfs(memo,mini,p.addv(dl),Data::left(  dir),cost+1001);
        *mini = l.min(*mini);
        let r = self.dfs(memo,mini,p.addv(dr),Data::right( dir),cost+1001);
        *mini = r.min(*mini);

        //if *mini<122_000
        {
         //   println!("{} ",*mini);
        }

        let res = f.min(l.min(r));
        memo.insert((p,dir),res);
        res
    }

    fn bfs(&mut self,dir:char)->usize
    {
        let mut q = Vec::new();
        q.push((self.s,dir,0));
        let mut visited = HashMap::new();
       
        let mut best = usize::MAX;

        while q.len() > 0
        {
            let (p,df,cost) = q.remove(0);
            
            if cost>*visited.get(&(p,df)).unwrap_or(&9999999999999999999)
            {
                continue;
            }
            visited.insert((p,df),cost);

            if cost>=best
            {
                continue;
            }

            if p == self.e
            {
                if cost<best
                {
                    best = cost;
                    println!("best: {}",best);
                }
                //return cost;
            }

            if self.get(p) != '.' && p != self.s
            {
                continue;
            }

            let dl =        Data::left(df);
            let dr =        Data::right(df);
            let pf = p.addv(Data::get_offset(df));

            if cost+1<*visited.get(&(pf,df)).unwrap_or(&9999999999999999999)
            {                    
                q.push((pf,df,cost+1));
            }
            if cost+1000<*visited.get(&(p,dl)).unwrap_or(&9999999999999999999)
            {
                q.push((p,dl,cost+1000));
            }            
            if cost+1000<*visited.get(&(p,dr)).unwrap_or(&9999999999999999999)
            {
                q.push((p,dr,cost+1000));
            }            
        }
        best
    }    

    fn bfs2(&mut self,dir:char,bestv:usize)->usize
    {
        let mut visited = HashMap::new();
        let mut q = Vec::new();
        q.push((self.s,dir,0));
       
        let mut ok = HashSet::new();
        let mut end_dirs = vec![];

        while !q.is_empty()
        {
            let (p,df,cost) = q.remove(0);

            let cc = *visited.get(&(p,df)).unwrap_or(&9999999999999999999);
            
            if cost>cc || cost>bestv
            {
                continue;
            }
            visited.insert((p,df),cost);

            if p == self.e
            {
                if cost<=bestv
                {
                    end_dirs.push(df);
                }
            }
            else
            {
                if self.get(p) != '.' && p != self.s
                {
                    continue;
                }

                let dl =        Data::left(df);
                let dr =        Data::right(df);
                let pf = p.addv(Data::get_offset(df));
                //let pl = p.addv(Data::get_offset(dl));
                //let pr = p.addv(Data::get_offset(dr));

                if cost+1<*visited.get(&(pf,df)).unwrap_or(&9999999999999999999)
                {                    
                    q.push((pf,df,cost+1));
                }
                if cost+1000<*visited.get(&(p,dl)).unwrap_or(&9999999999999999999)
                {
                    q.push((p,dl,cost+1000));
                }            
                if cost+1000<*visited.get(&(p,dr)).unwrap_or(&9999999999999999999)
                {
                    q.push((p,dr,cost+1000));
                }
            }            
       }


       for dir in end_dirs
       {
            q.push((self.e,dir,bestv));
            ok.insert(self.e);

            while !q.is_empty()
            {
                let (p,d,cost) = q.remove(0);

                let dl = Data::right(d);
                let dr = Data::left(d);

                let pb = p.addv(Data::get_offset_back(d));

                let cf = *visited.get(&(pb,d )).unwrap_or(&9999999999999999999);
                let cl = *visited.get(&(p ,dl)).unwrap_or(&9999999999999999999);
                let cr = *visited.get(&(p ,dr)).unwrap_or(&9999999999999999999);

                if cost>=cf+1 && (self.get(pb)=='.' || self.get(pb)=='S')
                {
                    q.push((pb,d,cf));            ok.insert(pb);
                }
                if cost>=cl+1000
                {
                    q.push((p,dl,cl));            ok.insert(p);
                }
                if cost>=cr+1000
                {
                    q.push((p,dr,cr));            ok.insert(p);
                }
            }
       }

       Data::print_hash(&self, ok.clone());
            
        ok.len()
    }

    //4831 around
    //< 4830
    //< 4828
    //not 4820


    fn count1(&mut self)->usize
    {
        //let mut memo = HashMap::new();

        //let mut mini = 87380;
        //self.dfs(&mut memo,&mut mini,self.s, '>',0)
        self.bfs( '>')        
    }

    fn count2(&mut self)->usize
    {
        let best = self.bfs( '>');
        self.bfs2('>',best)
        /* 
        let ss = self.s;
        let ee = self.e;

        self.s = ss;
        self.e = ee;
        let b1 = self.bfs( '>');

        self.s = ee;
        self.e = ss;
        let b2 = self.bfs( '>');
        b2-b1
        */
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
    println!("Day16");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}



#[test]
fn test1()
{
    let v = vec![
        "###############".to_string(),
        "#.......#....E#".to_string(),
        "#.#.###.#.###.#".to_string(),
        "#.....#.#...#.#".to_string(),
        "#.###.#####.#.#".to_string(),
        "#.#.#.......#.#".to_string(),
        "#.#.#####.###.#".to_string(),
        "#...........#.#".to_string(),
        "###.#.#####.#.#".to_string(),
        "#...#.....#.#.#".to_string(),
        "#.#.#.###.#.#.#".to_string(),
        "#.....#...#.#.#".to_string(),
        "#.###.#.#.#.#.#".to_string(),
        "#S..#.....#...#".to_string(),
        "###############".to_string(),
    ];
    assert_eq!(part1(&v),7036);
}


#[test]
fn test2()
{
    let v = vec![
        "#################".to_string(),
        "#...#...#...#..E#".to_string(),
        "#.#.#.#.#.#.#.#.#".to_string(),
        "#.#.#.#...#...#.#".to_string(),
        "#.#.#.#.###.#.#.#".to_string(),
        "#...#.#.#.....#.#".to_string(),
        "#.#.#.#.#.#####.#".to_string(),
        "#.#...#.#.#.....#".to_string(),
        "#.#.#####.#.###.#".to_string(),
        "#.#.#.......#...#".to_string(),
        "#.#.###.#####.###".to_string(),
        "#.#.#...#.....#.#".to_string(),
        "#.#.#.#####.###.#".to_string(),
        "#.#.#.........#.#".to_string(),
        "#.#.#.#########.#".to_string(),
        "#S#.............#".to_string(),
        "#################".to_string(),        
    ];
    assert_eq!(part1(&v),11048);
}



#[test]
fn test3()
{
    let v = vec![
        "###############".to_string(),
        "#.......#....E#".to_string(),
        "#.#.###.#.###.#".to_string(),
        "#.....#.#...#.#".to_string(),
        "#.###.#####.#.#".to_string(),
        "#.#.#.......#.#".to_string(),
        "#.#.#####.###.#".to_string(),
        "#...........#.#".to_string(),
        "###.#.#####.#.#".to_string(),
        "#...#.....#.#.#".to_string(),
        "#.#.#.###.#.#.#".to_string(),
        "#.....#...#.#.#".to_string(),
        "#.###.#.#.#.#.#".to_string(),
        "#S..#.....#...#".to_string(),
        "###############".to_string(),
    ];
    assert_eq!(part2(&v),45);
}



#[test]
fn test4()
{
    let v = vec![
        "#################".to_string(),
        "#...#...#...#..E#".to_string(),
        "#.#.#.#.#.#.#.#.#".to_string(),
        "#.#.#.#...#...#.#".to_string(),
        "#.#.#.#.###.#.#.#".to_string(),
        "#...#.#.#.....#.#".to_string(),
        "#.#.#.#.#.#####.#".to_string(),
        "#.#...#.#.#.....#".to_string(),
        "#.#.#####.#.###.#".to_string(),
        "#.#.#.......#...#".to_string(),
        "#.#.###.#####.###".to_string(),
        "#.#.#...#.....#.#".to_string(),
        "#.#.#.#####.###.#".to_string(),
        "#.#.#.........#.#".to_string(),
        "#.#.#.#########.#".to_string(),
        "#S#.............#".to_string(),
        "#################".to_string(),        
    ];
    assert_eq!(part2(&v),64);
}

#[test]
fn test5()
{
    let v = vec![
        "#####".to_string(),
        "#..E#".to_string(),
        "#.#.#".to_string(),
        "#...#".to_string(),
        "#.###".to_string(),
        "#.#.#".to_string(),
        "#S#.#".to_string(),
        "#####".to_string(),
    ];
    assert_eq!(part2(&v),8);
}
