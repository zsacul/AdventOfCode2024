use std::collections::HashMap;
use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Data {
      game: Vec<(Vec2,Vec2,Vec2)>,
    
}

impl Data {
    fn new(input: &[String]) -> Self {
        let sections: Vec<&[String]> = input.split(|line| line.is_empty()).collect();
        
        let mut game = Vec::new();
        //let mut nums = vec![];

        let game = sections.iter()
            .map(|line| {

                let aa = line[0].split(" Y").collect::<Vec<&str>>();                
                let button_ax = tools::i64_get_between(aa[0],"X+",",");
                let button_ay = tools::i64_get_between(aa[1],"+","");

                let bb = line[1].split(" Y").collect::<Vec<&str>>();                
                let button_bx = tools::i64_get_between(bb[0],"X+",",");
                let button_by = tools::i64_get_between(bb[1],"+","");
                
                let prize_x = tools::i64_get_between(&line[2],"X=",", ");
                let prize_y = tools::i64_get_between(&line[2],"Y=","");

            
                let va = Vec2::new(button_ax,button_ay);
                let vb = Vec2::new(button_bx,button_by);
                let vp = Vec2::new(prize_x,prize_y);
                let k = (va,vb,vp);
      
                game.push(k);
                k
      
            })
            .collect();
        


        Data {
            game,
        }
    }



    fn ok1(&self,a:Vec2,b:Vec2,p:Vec2) -> usize
    {
        //(a.x*A + a.y*A) + (b.x*B + b.y*B) == (c.x*C + c.y*C) ,minimize 3*A+B
        let mut res = vec![];

        for A in 0..=100
        {
            if A*a.x > p.x { break; }
            if A*a.y > p.y { break; }
            
            let pb = p.x-A*a.x;
            if (p.x-A*a.x)%b.x!=0 { continue; }
            if (p.y-A*a.y)%b.y!=0 { continue; }

            let Bx = (p.x-A*a.x)/b.x;
            let By = (p.y-A*a.y)/b.y;

            if Bx==By
            {
                res.push(3*A+Bx);
            }
        }

        if res.len()==0 { return 0; }
        *res.iter().min().unwrap() as usize
    }

    fn gdc(a:usize,b:usize)->usize
    {
        if b==0 { return a; }
        Data::gdc(b,a%b)
    }

    
 
    fn intersect(&self,a1:Vec2,a2:Vec2,b1:Vec2,b2:Vec2)->(f64,f64)
    {
        let s1_x = a2.x as f64 - a1.x as f64;
        let s1_y = a2.y as f64 - a1.y as f64;
        let s2_x = b2.x as f64 - b1.x as f64;
        let s2_y = b2.y as f64 - b1.y as f64;

        let s = (-s1_y * (a1.x as f64 - b1.x as f64) + s1_x * (a1.y as f64 - b1.y as f64)) / (-s2_x * s1_y + s1_x * s2_y);
        let t = ( s2_x * (a1.y as f64 - b1.y as f64) - s2_y * (a1.x as f64 - b1.x as f64)) / (-s2_x * s1_y + s1_x * s2_y);

        if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&s)
        {
            let i_x = a1.x as f64 + (t * s1_x);
            let i_y = a1.y as f64 + (t * s1_y);

            return (i_x,i_y);
        }

        (-1.0,-1.0)
    }

    fn ok2(&self,a:Vec2,b:Vec2,p:Vec2) -> usize
    {               
        let p = Vec2::new(p.x + 10000000000000,p.y + 10000000000000);
        
        let ax = a.x as usize;
        let ay = a.y as usize;

        let bx = b.x as usize;
        let by = b.y as usize;

        let px = p.x as usize;
        let py = p.y as usize;

        let v1a = Vec2::new(0,0);
        let v1b = Vec2::new(p.x*a.x,p.x*a.y);

        let v2a = p.subv(Vec2::new(p.x*b.x,p.x*b.y));
        let v2b = p.clone();

        let (i_x,i_y) = self.intersect(v1a,v1b,v2a,v2b);
        
        let mut res = vec![];

        let Aa = ((i_x)/(ax as f64)) as usize;
        
        for A in (Aa-1..=Aa+1)
        {
            if A*ax > px { break; }
            if A*ay > py { break; }
          
            if (px-A*ax)%bx!=0 { continue; }
            if (py-A*ay)%by!=0 { continue; }

            let Bx = (px-A*ax)/bx;
            let By = (py-A*ay)/by;

            if Bx==By
            {
                res.push(3*A+Bx);
            }
        }

        if res.len()==0 { return 0; }

        *res.iter().min().unwrap() as usize
    }

    fn count1(&self)->usize
    {
        self.game.iter()
            .map(|n| self.ok1(n.0,n.1,n.2) )
            .sum()
    
    }

    fn count2(&self)->usize
    {
        self.game.iter()
            .map(|n| self.ok2(n.0,n.1,n.2) )
            .sum()    
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
    println!("Day13");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "Button A: X+94, Y+34".to_string(),
        "Button B: X+22, Y+67".to_string(),
        "Prize: X=8400, Y=5400".to_string(),
        "".to_string(),
        "Button A: X+26, Y+66".to_string(),
        "Button B: X+67, Y+21".to_string(),
        "Prize: X=12748, Y=12176".to_string(),
        "".to_string(),
        "Button A: X+17, Y+86".to_string(),
        "Button B: X+84, Y+37".to_string(),
        "Prize: X=7870, Y=6450".to_string(),
        "".to_string(),
        "Button A: X+69, Y+23".to_string(),
        "Button B: X+27, Y+71".to_string(),
        "Prize: X=18641, Y=10279".to_string(),
    ];
    assert_eq!(part1(&v),480);
}

#[test]
fn test2()
{
    let v = vec![
        "Button A: X+94, Y+34".to_string(),
        "Button B: X+22, Y+67".to_string(),
        "Prize: X=10000000008400, Y=10000000005400".to_string(),
        "".to_string(),
        "Button A: X+26, Y+66".to_string(),
        "Button B: X+67, Y+21".to_string(),
        "Prize: X=10000000012748, Y=10000000012176".to_string(),
        "".to_string(),
        "Button A: X+17, Y+86".to_string(),
        "Button B: X+84, Y+37".to_string(),
        "Prize: X=10000000007870, Y=10000000006450".to_string(),
        "".to_string(),
        "Button A: X+69, Y+23".to_string(),
        "Button B: X+27, Y+71".to_string(),
        "Prize: X=10000000018641, Y=10000000010279".to_string(),
    ];
    assert_eq!(part2(&v),123);
}
