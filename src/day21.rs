use std::collections::HashMap;
use super::vec2::Vec2;


struct AI
{
    depth:usize,
    small:HashMap<(char,char),Vec<String>>,
}

impl AI {

    fn new(depth:usize)->AI
    {
        AI
        {
            depth,
            small : AI::short()
        }
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
            for b in 0..keys.len()
            {
                if a!=b
                {
                    let ac = keys.chars().nth(a).unwrap();
                    let bc = keys.chars().nth(b).unwrap();
                    let posa = *pos.get(&ac).unwrap();
                    let posb = *pos.get(&bc).unwrap();
                    let del  = posb.subv(posa);

                    let h = (if del.x>0 {">"} else {"<"}).repeat(del.x.abs() as usize);
                    let v = (if del.y>0 {"v"} else {"^"}).repeat(del.y.abs() as usize);

                    let mut moves = vec![];
                    if AI::possiblex(posa,del.x,del.y,small)
                    {
                        moves.push([h.clone(),v.clone(),"A".to_string()].join(""));
                    }
                    if AI::possibley(posa,del.y,del.x,small)
                    {
                        if v!=h
                        {
                            let sec = [v.clone(),h.clone(),"A".to_string()].join("");
                            //if  moves[0]!=sec
                            {
                                moves.push(sec);
                            }
                        }
                    }
                    map.insert((ac,bc), moves);
                }
            }
        }

    }
    

    fn short()->HashMap<(char,char),Vec<String>>
    {
        let mut map = HashMap::new();

        //let mut keys = "".to_string();
        let mut pos = HashMap::new();

        //if small
        {
            //keys = "^A<v>".to_string();
            pos.insert('^',Vec2::new( 1, 0));
            pos.insert('A',Vec2::new( 2, 0));

            pos.insert('<',Vec2::new( 0, 1));
            pos.insert('v',Vec2::new( 1, 1));
            pos.insert('>',Vec2::new( 2, 1));
        }
//          else 
        {
            //keys = "0123456789A^<v>*".to_string();
            pos.insert('7',Vec2::new( 0, 0));
            pos.insert('8',Vec2::new( 1, 0));
            pos.insert('9',Vec2::new( 2, 0));

            pos.insert('4',Vec2::new( 0, 1));
            pos.insert('5',Vec2::new( 1, 1));
            pos.insert('6',Vec2::new( 2, 1));

            pos.insert('1',Vec2::new( 0, 2));
            pos.insert('2',Vec2::new( 1, 2));
            pos.insert('3',Vec2::new( 2, 2));

            pos.insert('0',Vec2::new( 1, 3));
            pos.insert('*',Vec2::new( 2, 3));
        }

        AI::add_pos(      "A^<v>".to_string(),true ,&mut pos,&mut map);
        AI::add_pos("0123456789*".to_string(),false,&mut pos,&mut map);


        map.insert(('*','*'), vec!["A".to_string()]);
        map.insert(('A','A'), vec!["A".to_string()]);
        map.insert(('^','^'), vec!["A".to_string()]);
        map.insert(('<','<'), vec!["A".to_string()]);
        map.insert(('v','v'), vec!["A".to_string()]);
        map.insert(('>','>'), vec!["A".to_string()]);

        //println!("{:?}",map);

        map
    }

    //"029A".to_string(),
    //<A^A^^>AvvvA
    
    //<A<A^A<A^A>^^A<A^A>^^AvvvA
    //v<<A (<) >>^A (A) <A (^) >A (A) vA (>) <^A (^) A (^) >A (A) <vAAA>^A
    // 029A
    // <A^A>^^AvvvA

  
    fn bfs(&mut self,memo:&mut HashMap<(char,String,usize),String>,des:String,old_c:char,level:usize)->String
    {
        let key = (old_c,des.clone(),level);//if level>5 {5} else {level});

        if memo.contains_key(&key) && level>0
        {
            return memo.get(&key).unwrap().to_string();
        }
            
        if level==self.depth
        {
            //  println!("finito {}: {} -> {}",level,des,pref);
            return des;//pref.clone();
        }
    
        let  pieces = des.clone()
                                      .split("")
                                      .filter(|a|!a.is_empty())
                                      .map(|a|a.to_string()).collect::<Vec<String>>();
    
    //  println!("l={} pieces: {:?}   pref={:?}",level,pieces,pref);
        let mut last_c = old_c;//'A';

        //if "0123456789*".contains(last_c) && level>0
        //{
         //   last_c = 'A';
        //}

        let strs = 
        pieces.iter().map(|el|
        {
                let c = el.chars().nth(0 as usize).unwrap();

                if level<=1
                {
          //          println!("l=[{}] el=[{}] des=[{}] -> (from {} to {})",level,el,des,last_c,c);
                }

                //let cc = e.chars().next().unwrap();                
                let moves = self.small.get(&(last_c,c)).unwrap().clone();
                let mut res = "".to_string();
                let mut min_l = usize::MAX;

                //let m = moves[0].to_string();
                for m in moves.iter()
                {
                    //let p = m.clone();//format!("{}A",m);
                    // let v = format!("{}{}",pref.clone(), 
                    //let v = self.bfs(memo,m.clone(),last_c,level+1);
                    let v = self.bfs(memo,m.clone(),'A',level+1);
                    // println!("[{},{}] v={}",level,id,v);
                    let v_len = v.len();
        
                    if v_len<min_l
                    {
                        min_l = v_len;
                        res   = v.clone();//format!("{}{}",p,v);
                    }
                }          

                last_c = c;//el.chars().last().unwrap_or('A');
                if last_c=='*'
                {
                    //last_c = 'A';
                }

                res
            }
    
        ).collect::<Vec<String>>();
        
        /*
        {
            let mut p = p.to_string();

            if p.len()>0
            {
                p.push('A');
                let v = self.bfs(memo,c,id+1,level);
                if v.len()<min_l
                {
                    min_l = v.len();
                    res   = v.clone();
                }
            }
        }
*/


//golden:

//<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
//28x A
//let moves = self.small.get(&(old_c,c)).unwrap().clone();
        //println!("moves: {}->{} = [{:?}]",old_c,c,moves);
        //let p = self.pref.clone();
        let result = strs.join("").clone();
        //if level==0
        {
            //println!("lev={} res={}",level,result);
            //println!("{} -> {}",pref,result);
        }
        memo.insert(key, result.clone());
        //}
        result
    }


      
    fn bfsc(&mut self,memo:&mut HashMap<(String,usize),usize>,des:String,old_c:char,level:usize)->usize
    {
        let key = (des.clone(),level);//if level>5 {5} else {level});

        if memo.contains_key(&key) && level>0
        {
            return *memo.get(&key).unwrap();
        }
            
        if level==self.depth
        {
            //  println!("finito {}: {} -> {}",level,des,pref);
            return des.len();//pref.clone();
        }
        
        let  pieces = des.clone()
                         .split("")
                         .filter(|a|!a.is_empty())
                         .map(|a|a.to_string()).collect::<Vec<String>>();
    
   
    //  println!("l={} pieces: {:?}   pref={:?}",level,pieces,pref);
        let mut last_c = old_c;//'A';

        //if "0123456789*".contains(last_c) && level>0
        //{
         //   last_c = 'A';
        //}

        let strs = 
        pieces.iter().map(|el|
        {
                let c = el.chars().nth(0 as usize).unwrap();

                if level<=1
                {
          //          println!("l=[{}] el=[{}] des=[{}] -> (from {} to {})",level,el,des,last_c,c);
                }

                //let cc = e.chars().next().unwrap();                
                let moves = self.small.get(&(last_c,c)).unwrap().clone();
                let mut res = 0usize;
                let mut min_l = usize::MAX;

                //let m = moves[0].to_string();
                for m in moves.iter()
                {
                    let v = self.bfsc(memo,m.clone(),'A',level+1);
                    let v_len = v;
        
                    if v_len<min_l
                    {
                        min_l = v_len;
                        res   = v;//.len();//format!("{}{}",p,v);
                    }
                }          

                last_c = c;//el.chars().last().unwrap_or('A');
                if last_c=='*'
                {
                    //last_c = 'A';
                }

                res
            }
    
        ).collect::<Vec<usize>>();
        

        let result = strs.iter().sum();
        //if level==0
        {
            //println!("lev={} res={}",level,result);
            //println!("{} -> {}",pref,result);
        }
        memo.insert(key, result);
        //}
        result
    }


   
}




fn ok(s:&str,second:bool)->usize
{
    let s = s.replace("A", "*");
    let mut ai = AI::new(if second {26} else {3});       
    let mut memo = HashMap::new();

    //let cc = s.chars()
    //          .map(|f| f.to_string())   
    //          .collect::<Vec<String>>()
    //          .join("");

    //println!("cc: {}",cc);
    //ai.des_code = s.chars().collect();
    //ai.pref = "".to_string();
    //let code = ai.bfs(&mut memo,cc.clone(),'*',0);
    let code = ai.bfsc(&mut memo,s.to_string(),'*',0);

    println!("{} -> {}",s,code);
    points(s,code)
}

fn points(s:String,res:usize)->usize
{
   s[..s.len()-1].parse::<usize>().unwrap()*res
}

pub fn part1(data:&[String])->usize
{
   data.iter()
       .map(|n| ok(n,false))
       .sum()
}

pub fn part2(data:&[String])->usize
{
   data.iter()
       .map(|n| ok(n,true))
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

#[test]
fn test3()
{
    let v = vec![
        "805A".to_string(),
    ];
    assert_eq!(part2(&v),0);
}

//029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
//029*: <vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A
//
//980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
//980*: v<<A>>^AAAvA^A<vA<AA>>^AvAA<^A>Av<<A>A>^AAAvA<^A>A<vA>^A<A>A
//
//179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//179*: <vA<AA>>^AAvA<^A>AvA^Av<<A>>^AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A
//
//456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
//456*: <vA<AA>>^AAvA<^A>AAvA^A<vA>^A<A>A<vA>^A<A>Av<<A>A>^AAvA<^A>A
//
//379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//379*: v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A

//278748
//47719830362864 too low
//117842718368480 too low
//291009083500026 wrong
  