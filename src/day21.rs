use std::collections::HashMap;
use super::vec2::Vec2;

struct AI
{
    depth    :usize,
    small    :HashMap<(char,char),Vec<String>>,
    positions:HashMap<char,Vec2>,
}

impl AI 
{
    fn new(depth:usize)->AI
    {
        AI
        {
            depth,
            positions: HashMap::new(),
            small    : HashMap::new(),
        }
    }

    fn finish(&mut self)
    {
        self.add_positions();
        self.small = self.short();
    }

    fn possiblex(p:Vec2,dx:i64,dy:i64,small:bool)->bool
    {
        let mut p = p;
        let wrong = if small {Vec2::new(0, 0)} else {Vec2::new(0, 3)};

        for _ in 0..dx.abs()
        {
            p.x+=dx.signum();
            if p==wrong { return false }
        }
        for _ in 0..dy.abs()
        {
            p.y+=dy.signum();
            if p==wrong { return false }
        }
        true
    }

    fn possibley(p:Vec2,dx:i64,dy:i64,small:bool)->bool
    {
        let mut p = p;
        let wrong = if small {Vec2::new(0, 0)} else {Vec2::new(0, 3)};

        for _ in 0..dy.abs()
        {
            p.y+=dy.signum();
            if p==wrong { return false }
        }
        for _ in 0..dx.abs()
        {
            p.x+=dx.signum();
            if p==wrong { return false }
        }
        true
    }
    
    fn add_pos(keys:String,small:bool,pos:&mut HashMap<char,Vec2>,map:&mut HashMap<(char,char),Vec<String>>)
    {
        for a in 0..keys.len()
        {
            let ac = keys.chars().nth(a).unwrap();

            for b in 0..keys.len()
            {
                if a!=b
                {
                    let bc = keys.chars().nth(b).unwrap();
                    let posa = *pos.get(&ac).unwrap();
                    let posb = *pos.get(&bc).unwrap();
                    let del  = posb.subv(posa);

                    let h = (if del.x>0 {">"} else {"<"}).repeat(del.x.abs() as usize);
                    let v = (if del.y>0 {"v"} else {"^"}).repeat(del.y.abs() as usize);

                    let mut moves = vec![];

                    if AI::possibley(posa,del.x,del.y,small)
                    {                        
                            moves.push([v.clone(),h.clone(),"A".to_string()].join(""));                                                    
                    }

                    if AI::possiblex(posa,del.x,del.y,small)
                    {
                        if v!=h && del.y!=0 && del.x!=0
                        {
                            moves.push([h.clone(),v.clone(),"A".to_string()].join(""));
                        }
                    }

                    map.insert((ac,bc), moves);
                }
            }

            map.insert((ac,ac), vec!["A".to_string()]);
        }

    }

    fn add_positions(&mut self)
    {        
        self.positions.insert('^',Vec2::new( 1, 0));
        self.positions.insert('A',Vec2::new( 2, 0));

        self.positions.insert('<',Vec2::new( 0, 1));
        self.positions.insert('v',Vec2::new( 1, 1));
        self.positions.insert('>',Vec2::new( 2, 1));

        self.positions.insert('7',Vec2::new( 0, 0));
        self.positions.insert('8',Vec2::new( 1, 0));
        self.positions.insert('9',Vec2::new( 2, 0));

        self.positions.insert('4',Vec2::new( 0, 1));
        self.positions.insert('5',Vec2::new( 1, 1));
        self.positions.insert('6',Vec2::new( 2, 1));

        self.positions.insert('1',Vec2::new( 0, 2));
        self.positions.insert('2',Vec2::new( 1, 2));
        self.positions.insert('3',Vec2::new( 2, 2));

        self.positions.insert('0',Vec2::new( 1, 3));
        self.positions.insert('*',Vec2::new( 2, 3));
    } 

    fn short(&mut self)->HashMap<(char,char),Vec<String>>
    {
        let mut map = HashMap::new();

        AI::add_pos(      "A^<v>".to_string(),true , &mut self.positions,&mut map);
        AI::add_pos("0123456789*".to_string(),false, &mut self.positions,&mut map);

        map
    }

    fn bfs(&mut self,memo:&mut HashMap<(char,String,usize),String>,des:String,old_c:char,level:usize)->String
    {
        let key = (old_c,des.clone(),level);

        if memo.contains_key(&key) && level>0
        {
            return memo.get(&key).unwrap().to_string();
        }
            
        if level==self.depth
        {
            return des;
        }
    
        let  pieces = des.clone()
                                      .split("")
                                      .filter(|a|!a.is_empty())
                                      .map(|a|a.to_string()).collect::<Vec<String>>();
    
        let mut last_c = old_c;

        let strs = 
        pieces.iter().map(|el|
        {
                let c = el.chars().nth(0 as usize).unwrap();

                let moves = self.small.get(&(last_c,c)).unwrap().clone();
                let mut res = "".to_string();
                let mut min_l = usize::MAX;

                for m in moves.iter()
                {
                    let v = self.bfs(memo,m.clone(),'A',level+1);
                    let v_len = v.len();
        
                    if v_len<min_l
                    {
                        min_l = v_len;
                        res   = v.clone();
                    }
                }          

                last_c = c;

                res
            }
    
        ).collect::<Vec<String>>();
        
        let result = strs.join("").clone();
        memo.insert(key, result.clone());
        result
    }

    fn bfsc(&mut self,memo:&mut HashMap<(String,char,usize),usize>,des:String,old_c:char,level:usize)->usize
    {
        let key = (des.clone(),old_c,level);

        if memo.contains_key(&key) && level>0
        {
            return *memo.get(&key).unwrap();
        }
            
        if level==self.depth
        {
            return des.len();
        }
        
        let  pieces = des.clone()
                         .split("")
                         .filter(|a|!a.is_empty())
                         .map(|a|a.to_string()).collect::<Vec<String>>();
   

        let mut last_c = old_c;

        let strs = 
        pieces.iter().map(|el|
        {
                let c = el.chars().nth(0 as usize).unwrap();
                let moves = self.small.get(&(last_c,c)).unwrap().clone();
                let mut res = 0usize;                

                if moves.len()==1
                {
                    let m = moves[0].to_string();
                    res = self.bfsc(memo,m.clone(),'A',level+1);
                }
                  else
                {
                    let f = *self.positions.get(&last_c).unwrap();
                    let t = *self.positions.get(     &c).unwrap();

                    let v = t.subv(f);
                    let dx = v.x.signum();
                    let dy = v.y.signum();

                    let rr = if dx>0 {">".repeat(v.x as usize)} else {"<".repeat(-v.x as usize)};
                    let uu = if dy>0 {"v".repeat(v.y as usize)} else {"^".repeat(-v.y as usize)};

                    let small = level>0;

                    let nmove = 
                    match (dx,dy)
                    {
                        (-1,-1)=>if AI::possiblex(f,v.x,v.y,small) { format!("{rr}{uu}A") } else { format!("{uu}{rr}A") },
                        (-1, 1)=>if AI::possiblex(f,v.x,v.y,small) { format!("{rr}{uu}A") } else { format!("{uu}{rr}A") },
                        ( 1, 1)=>if AI::possibley(f,v.x,v.y,small) { format!("{uu}{rr}A") } else { format!("{rr}{uu}A") },
                        ( 1,-1)=>if AI::possibley(f,v.x,v.y,small) { format!("{uu}{rr}A") } else { format!("{rr}{uu}A") },
                        ( _, 0)=>{format!("{rr}A")},
                        ( 0, _)=>{format!("{uu}A")},
                        _      => panic!("mov[{:?}] {} {}",moves,dx,dy),
                    };

                    res = self.bfsc(memo,nmove.clone(),'A',level+1);
                }

                last_c = c;
                res
            }
    
        ).collect::<Vec<usize>>();
        
        let result = strs.iter().sum();
        memo.insert(key, result);

        result
    } 
}


fn ok1(s:&str,depth:usize)->usize
{
    let s = s.replace("A", "*");
    let mut ai = AI::new(depth+1);
    ai.finish();
    let mut memo = HashMap::new();

    let code = ai.bfs(&mut memo,s.to_string(),'*',0);  

//  println!("{} -> {}",s,code.len());
    points(s,code.len())
}

fn ok2(s:&str,depth:usize)->usize
{
    let s = s.replace("A", "*");
    let mut ai = AI::new( depth+1);
    ai.finish();
    let mut memo = HashMap::new();

    let code = ai.bfsc(&mut memo,s.to_string(),'*',0);

//  println!("{} -> {}",s,code);
    points(s,code)
}

fn points(s:String,res:usize)->usize
{
   s[..s.len()-1].parse::<usize>().unwrap()*res
}

pub fn part1(data:&[String])->usize
{
   data.iter()
       .map(|n| ok2(n,2))
       .sum()
}

pub fn part2(data:&[String])->usize
{
   data.iter()
       .map(|n| ok2(n,25))
       .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day21");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test0()
{
    let v = vec![
        "029A".to_string(),
    ];
    assert_eq!(part1(&v),68*29);
}

#[test]
fn test1()
{
    let v = vec![
        "029A".to_string(),
        "980A".to_string(),
        "179A".to_string(),
        "456A".to_string(),
        "379A".to_string(),
    ];
    assert_eq!(part1(&v),126384);
}

#[test]
fn test2()
{
    let v = vec![
        "805A".to_string(),
        "964A".to_string(),
        "459A".to_string(),
        "968A".to_string(),
        "671A".to_string(),
    ];
    assert_eq!(part1(&v),278748);
}
