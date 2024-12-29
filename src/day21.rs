use std::collections::HashMap;
use super::vec2::Vec2;

#[derive(Debug,Clone)]
struct Robot
{
    pos     : Vec2,        
    buttons : HashMap<Vec2,char>,
}

impl Robot {

    fn new(fin:bool,code:String )->Robot
    {
        let mut robot = Robot{
            pos :  if fin {Vec2::new(2,3)} else {Vec2::new(2,0)},
            buttons : HashMap::new()
        };
        robot.add_buttons(fin);

        robot
    }

    fn add_buttons(&mut self,fin:bool)
    {
        if fin
        {
            self.buttons.insert(Vec2::new( 0, 0),'7');
            self.buttons.insert(Vec2::new( 1, 0),'8');
            self.buttons.insert(Vec2::new( 2, 0),'9');
            self.buttons.insert(Vec2::new( 0, 1),'4');
            self.buttons.insert(Vec2::new( 1, 1),'5');
            self.buttons.insert(Vec2::new( 2, 1),'6');
            self.buttons.insert(Vec2::new( 0, 2),'1');
            self.buttons.insert(Vec2::new( 1, 2),'2');
            self.buttons.insert(Vec2::new( 2, 2),'3');
            
            self.buttons.insert(Vec2::new( 1, 3),'0');
            self.buttons.insert(Vec2::new( 2, 3),'*');
        }
          else
        {
            self.buttons.insert(Vec2::new( 1, 0),'^');
            self.buttons.insert(Vec2::new( 2, 0),'A');
            self.buttons.insert(Vec2::new( 0, 1),'<');
            self.buttons.insert(Vec2::new( 1, 1),'v');
            self.buttons.insert(Vec2::new( 2, 1),'>');
        }
    }

    fn valid(&self,pos:Vec2)->bool
    {
        self.buttons.contains_key(&pos)
    }

    fn valid_at_pos(&self)->bool
    {
        self.valid(self.pos)        
    }

    fn valid_moves(&self)->Vec<Vec2>
    {
        let mut res = Vec::new();
        for v in self.pos.around4()
        {        
            if self.valid(v)
            {
                res.push(v);
            }
        }
        res
    }

    fn get_key(&self)->char
    {
        *self.buttons.get(&self.pos).unwrap()
    }

    fn get_offset(&self,c:char)->Vec2
    {
        match c
        {
            '^' => Vec2::new( 0,-1),
            'v' => Vec2::new( 0, 1),
            '<' => Vec2::new(-1, 0),
            '>' => Vec2::new( 1, 0),
            //'A' => Vec2::new( 1, 1),
            _   => panic!("get_offset")
        }
    }

    /*
    fn press(&mut self)->Vec2
    {     
        if self.buttons.contains_key(&self.pos)
        {
            let k = *self.buttons.get(&self.pos).unwrap();

            return self.get_offset(k);
            //if k == 'A'
            //{
            //    return true
            //}
            //else
            //{
            //    self.pos = self.pos.addv();
            //}
        }
        panic!("unable to press the key")
        //false
    }
     */

    fn press_key(&mut self,key:char)->(bool,char)
    {           
        let was_action = key=='A';

        if !was_action
        {
            let off = self.get_offset(key);
            self.pos = self.pos.addv(off);
        }

        if self.valid_at_pos()
        {
            (was_action,self.get_key())
        }
            else
        {
            (was_action,'*')
        }
    }

}

struct AI
{
    robots:Vec<Robot>,
    code:Vec<char>,
    des:Vec<char>,
    depth:u8,
    big:HashMap<(char,char),Vec<String>>,
    small:HashMap<(char,char),Vec<String>>,
}

impl AI {

    fn new(code:String,depth:u8)->AI
    {
        let mut robots = Vec::new();
        for i in 0..depth
        {
            robots.push(Robot::new(i == depth-1,code.clone()));
        }
        
        AI
        {
            robots,
            code:Vec::new(),
            des:code.chars().collect(),
            depth,
            big   : AI::short(false),
            small : AI::short(true)
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

    //285840 too high

    
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
                        moves.push([h.clone(),v.clone()].join(""));
                    }
                    if AI::possibley(posa,del.y,del.x,small)
                    {
                        if v!=h
                        {
                            let sec = [v.clone(),h.clone()].join("");
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
    

    fn short(small:bool)->HashMap<(char,char),Vec<String>>
    {
        let mut map = HashMap::new();

        let mut keys = "".to_string();
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


        map.insert(('*','*'), vec!["".to_string()]);
        map.insert(('A','A'), vec!["".to_string()]);
        map.insert(('^','^'), vec!["".to_string()]);
        map.insert(('<','<'), vec!["".to_string()]);
        map.insert(('v','v'), vec!["".to_string()]);
        map.insert(('>','>'), vec!["".to_string()]);

        //println!("{:?}",map);

        map
    }

    fn do_key(&mut self,key:char)->(bool,char)
    {
        let mut id=0;
        let mut action = true;

        let mut key = key;

        while action && id<self.robots.len()
        {            
            let (was_action,nkey) =  self.robots[id].press_key(key);

            action = was_action;
            key = nkey;

            id+=1;
        }

        if id==self.robots.len()
        {
            return (true,key);//self.robots.last().unwrap().get_key());
        }

        (false,key)
    }

    fn get_state(&mut self)->Vec<Vec2>
    {
        self.robots.iter().map(|r| r.pos).collect()
    }

    fn set_state(&mut self,state:&Vec<Vec2>)
    {
        for i in 0..self.robots.len()
        {
            self.robots[i].pos = state[i];
        }
    }

    //"029A".to_string(),
    //<A^A^^>AvvvA
    
    //<A<A^A<A^A>^^A<A^A>^^AvvvA
    //v<<A (<) >>^A (A) <A (^) >A (A) vA (>) <^A (^) A (^) >A (A) <vAAA>^A
    // 029A
    // <A^A>^^AvvvA

    fn bfs(&mut self,memo:&mut HashMap<(char,String,u8),String>,pref:String,old_c:char,des_code:String,id:u16,level:u8)->String
    {
//      let key = (pref.clone(),des_code.clone());
        let key = (old_c,des_code.clone(),if level>5 {5} else {level});      

        if memo.contains_key(&key)
        {
            return memo.get(&key).unwrap().to_string();
        }

        //if level==self.depth
        //{
            //println!("final pref={} old_c={} des={} id={} lvl={}",pref,old_c,des_code,id,level);
            //memo.insert(key, des_code.to_string());
          //  return des_code.to_string();
        //}

        //println!("idL:{}/{}",id,des_code.len());
        if id==des_code.len() as u16
        {
            if level+1<self.depth
            {
                return self.bfs(memo,"".to_string(),'A',  pref, 0,level+1);
            }
            return pref;
        }

        //println!("{}/{}",level,self.depth);

        let mut res = "".to_string();
        let mut min_l = usize::MAX;
        let c = des_code.chars().nth(id as usize).unwrap();               

        let moves = self.small.get(&(old_c,c)).unwrap().clone();
        //println!("moves: {}->{} = [{:?}]",old_c,c,moves);

        for m in moves.iter()
        //let m = moves[0].to_string();
        {
            let pref = format!("{}{}A",pref,m);
            //let v = format!("{}{}",pref.clone(), 
            let v = self.bfs(memo,pref,c,des_code.to_string(),id+1,level);

           // println!("[{},{}] v={}",level,id,v);

            if v.len()<min_l
            {
                min_l = v.len();
                res   = v.clone();
            }
        }          

        //if level<5
        //{
            memo.insert(key, res.clone());
        //}
        res
    }

   
}

//138460,
//344540,
//851690,
//2128420


//


fn ok(s:&str,second:bool)->usize
{
    let s = s.replace("A", "*");
    let mut ai = AI::new(s.to_string(),if second {24} else {3});       
    let mut memo = HashMap::new();
    let code = ai.bfs(&mut memo,"".to_string(),'*',s.clone(),0,0);

    println!("{} -> {}",s,code);
    points(s,code)
}

fn points(s:String,res:String)->usize
{
   s[..s.len()-1].parse::<usize>().unwrap()*res.len()
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


