use std::collections::HashMap;
use std::collections::HashSet;
use super::vec2::Vec2;
use super::tools;

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
            self.buttons.insert(Vec2::new( 2, 3),'A');
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
    depth:usize
}

impl AI {

    fn new(code:String,depth:usize)->AI
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
            depth
        }
    }

    fn possiblex(&self,p:Vec2,dx:i64,dy:i64)->bool
    {
        let mut p = p;
        for _ in 0..dx.abs()
        {
            p.x+=dx.signum();
            if p==Vec2::new(0,4) { return false }
        }
        for _ in 0..dy.abs()
        {
            p.y+=dy.signum();
            if p==Vec2::new(0,4) { return false }
        }
        true
    }

    fn possibley(&self,p:Vec2,dx:i64,dy:i64)->bool
    {
        let mut p = p;
        for _ in 0..dy.abs()
        {
            p.y+=dy.signum();
            if p==Vec2::new(0,4) { return false }
        }
        for _ in 0..dx.abs()
        {
            p.x+=dx.signum();
            if p==Vec2::new(0,4) { return false }
        }
        true
    }

    fn short(&self)->HashMap<(char,char),Vec<String>>
    {
        let mut map = HashMap::new();

        let keys = "A0123456789";
        let mut pos = HashMap::new();

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
        pos.insert('A',Vec2::new( 2, 3));

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
                    if self.possiblex(posa,del.x,del.y)
                    {
                        moves.push([h.clone(),v.clone()].join(""));
                    }
                    if self.possibley(posa,del.y,del.x)
                    {
                        if v!=h
                        {
                            let sec = [v.clone(),h.clone()].join("");
                            if moves[0]!=sec
                            {
                                moves.push(sec);
                            }
                        }
                    }
                    map.insert((ac,bc), moves);
                }
            }
        }

        println!("{:?}",map);

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

    fn bfs(&mut self,des_code:String)->String
    {
        let mut q = Vec::new();

        let state = self.get_state();

  //      println!("state {:?}",state);
//        return "".to_string();

        let mut visited = HashMap::new();

        
        let k2 = ('<',"".to_string(),"".to_string(),0,state.clone());
        q.push(k2);
        
        let k3 = ('v',"".to_string(),"".to_string(),0,state.clone());
        q.push(k3);

        let k4 = ('>',"".to_string(),"".to_string(),0,state.clone());
        q.push(k4);

        let k0 = ('A',"".to_string(),"".to_string(),0,state.clone());
        q.push(k0);

        let k1 = ('^',"".to_string(),"".to_string(),0,state.clone());
        q.push(k1);
        

        let mut best = usize::MAX;
        let mut best_code = "".to_string();

        let mut count=0;

        while !q.is_empty()
        {
            let (dir,code,keys,cost,states) = q.remove(0);

            let hkey = (dir,code.clone(),states.clone());

            //println!("{} {:?}",count,hkey);
            //count+=1;


            if cost>=*visited.get(&hkey).unwrap_or(&88888888)
            {
                continue;
            }

            //warning
            if cost>22
            {
                continue;
            }
            self.set_state(&states);


            if !self.robots.clone().iter().all(|r| r.valid_at_pos())
            {
                //println!("keys {:?} dir: {} {:?}",keys,dir,states);
                continue;
            }

            if code.len()>0 && !des_code.starts_with(&code)
            {
                //println!("not start {:?}",code);
                continue;
            }

            if cost>=best
            {
                println!("greater cost");
                continue;
            }            

            if des_code == code && cost<best
            {                
                best      = cost;
                best_code = keys.clone();
                continue;
            }

            if code.len()>=des_code.len()
            {
                println!("greater len");
                continue;
            }

            let k = self.do_key(dir);

            if k.1=='*'
            {
                continue;
            }
            
            let nstate : Vec<Vec2> = self.get_state();
            let mut ncode= code.clone();
            
            if k.0 && k.1!='*'
            {
                ncode += &k.1.to_string();

                if k.1!='A'
                {
                    println!("append {} adding from state {} [{}] keys=[{}] {} = {:?}",k.1,dir,ncode.clone(),keys,cost,nstate.clone());
                }                
                
            }
            
            visited.insert((dir,ncode.clone(),nstate.clone()),cost+1);

            //println!("adding from state {} {} keys=[{}] {} = {:?}",dir,code,keys,cost,state);


            if dir!='v'
            {
                let k2 = ('^',ncode.clone(),keys.clone()+"^",cost+1,nstate.clone());            
                q.push(k2);
            }

            if dir!='>'
            {
                let k3 = ('<',ncode.clone(),keys.clone()+"<",cost+1,nstate.clone());
                q.push(k3);
            }

            if dir!='^'
            {
                let k4 = ('v',ncode.clone(),keys.clone()+"v",cost+1,nstate.clone());
                q.push(k4);            
            }

            if dir!='<'
            {
                let k5 = ('>',ncode.clone(),keys.clone()+">",cost+1,nstate.clone());            
                q.push(k5);            
            }

            let k1 = ('A',ncode.clone(),keys.clone()+"A",cost+1,nstate.clone());            
            q.push(k1);


        }

        best_code
    }

    
}

fn ok(s:&str,second:bool)->usize
{
    //let tab = s.split(" ").map(|a| a.parse().unwrap()).collect::<Vec<i32>>();
    //let tab: Vec<String> = s.split(", ").map(|s| s.to_string()).collect();

    if second
    {
        0
        //(0..tab.len()).any(|i|
          //  valid(tab[..i].iter().chain(tab[i+1..].iter()).copied().collect())
        //)
    }    
      else 
    {
        let mut ai = AI::new(s.to_string(),3);

        ai.short();

        let ss = "".to_string();
        //ai.bfs(s.to_string());
        let sc  = s[..s.len()-1].parse::<usize>().unwrap();
        sc*ss.len()
    }
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
    assert_eq!(part1(&v),99999999);
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
    ];
    assert_eq!(part2(&v),0);
}
