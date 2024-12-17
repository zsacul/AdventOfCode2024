use std::collections::{HashMap, HashSet};
//use crate::dijkstria;
//use super::dijkstria::Edge;

use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Data {
      hash  : HashMap<Vec2,char>,   
      visited : HashMap<(Vec2,char),usize>,
      dx    : usize,
      dy    : usize,
      s     : Vec2,
      e     : Vec2,
}

impl Data {
    fn new(input: &[String]) -> Self {

        let hash = tools::get_hash_table(input);
        let s   : Vec2 = tools::find_in_hash(&hash,'S');
        let e   : Vec2 = tools::find_in_hash(&hash,'E');                          
      
        Data 
        {
            hash,
            visited : HashMap::new(),
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

    fn v(&self,p:Vec2,d:char)->usize
    {
        *self.visited.get(&(p,d)).unwrap_or(&9999999999999999999)
    }

    fn bfs(&mut self,dir:char)->usize
    {
        let mut q = Vec::new();
        q.push((self.s,dir,0));
        let mut best = usize::MAX;

        while !q.is_empty()
        {
            let (p,df,cost) = q.remove(0);
            
            if cost>self.v(p,df)
            {
                continue;
            }
            self.visited.insert((p,df),cost);

            if cost>=best
            {
                continue;
            }

            if p == self.e && cost<best
            {
                best = cost;
                //println!("best: {}",best);
            }

            if self.get(p) != '.' && p != self.s
            {
                continue;
            }

            let dl =        Data::left(df);
            let dr =        Data::right(df);
            let pf = p.addv(Data::get_offset(df));

            if cost+   1<self.v(pf,df) {  q.push((pf,df,cost+1));  }
            if cost+1000<self.v( p,dl) {  q.push((p,dl,cost+1000));  }            
            if cost+1000<self.v( p,dr) {  q.push((p,dr,cost+1000));  }            
        }
        best
    }    

    fn bfs2(&mut self,dir:char,bestv:usize)->usize
    {
        self.visited.clear();
        
        let mut q = Vec::new();
        q.push((self.s,dir,0));
       
        let mut ok = HashSet::new();
        let mut end_dirs = vec![];

        while !q.is_empty()
        {
            let (p,df,cost) = q.remove(0);

            let cc = self.v(p,df);
            
            if cost>cc || cost>bestv
            {
                continue;
            }
            self.visited.insert((p,df),cost);

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

                if cost+   1<self.v(pf,df)  {  q.push((pf,df,cost+1));     }
                if cost+1000<self.v( p,dl)  {    q.push((p,dl,cost+1000));   }            
                if cost+1000<self.v( p,dr)  {    q.push((p,dr,cost+1000));   }
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

                let cf = self.v(pb,d);
                let cl = self.v(p ,dl);
                let cr = self.v(p,dr);

                if cf < cost && (self.get(pb)=='.' || self.get(pb)=='S')
                {
                    q.push((pb,d,cf));  ok.insert(pb);
                }
                if cost>=cl+1000
                {
                    q.push((p,dl,cl));  ok.insert(p);
                }
                if cost>=cr+1000
                {
                    q.push((p,dr,cr));  ok.insert(p);
                }
            }
       }

       //Data::print_hash(&self, ok.clone());
            
       ok.len()
    }

    //todo in future
    /*
    fn get_edges(&self,p:Vec2,d:char)->Vec<Edge>
    {
        let mut edges = vec![];
        edges.push(Edge { node: 2, cost: 10 });
        edges
    }

    fn bfsd(&self,dir:char)->usize
    {
        let mut graph = vec![];
        let s = self.dx*self.dy;

        graph.push(self.get_edges(self.s,dir)); //start
        graph.push(vec![]); //end


        dijkstria::shortest_path(&graph, 0, 1).unwrap()

            // Node 0
            vec![Edge { node: 2, cost: 10 },
                 Edge { node: 1, cost: 1 }],
            // Node 1
            vec![Edge { node: 3, cost: 2 }],
            // Node 2
            vec![Edge { node: 1, cost: 1 },
                 Edge { node: 3, cost: 3 },
                 Edge { node: 4, cost: 1 }],
            // Node 3
            vec![Edge { node: 0, cost: 7 },
                 Edge { node: 4, cost: 2 }],
            // Node 4
            vec![]];
        
        
    }
    */

    fn count1(&mut self)->usize
    {
        self.bfs( '>')        
    }

    fn count2(&mut self)->usize
    {
        let best = self.bfs( '>');
        self.bfs2('>',best)
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
