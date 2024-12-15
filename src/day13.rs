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
        (0..=100).filter_map(|button_a|       
            if button_a*a.x < p.x &&
               button_a*a.y < p.y &&                    
               (p.x - button_a*a.x)%b.x==0 &&
               (p.y - button_a*a.y)%b.y==0 &&
               (p.x - button_a*a.x)/b.x==(p.y - button_a*a.y)/b.y
               {
                    let button_b = (p.x - button_a*a.x)/b.x;
                    Some(3*button_a + button_b)
               }
                 else
               {
                    None
               }
            ).min()
            .unwrap_or(0) as usize
    }

    fn ok2(&self,a:Vec2,b:Vec2,p:Vec2) -> usize
    {               
        let p = p.addv(Vec2::new(10000000000000, 10000000000000));

        let v1a = Vec2::new(0,0);
        let v1b = Vec2::new(p.x*a.x,p.x*a.y);

        let v2a = p.subv(Vec2::new(p.x*b.x,p.x*b.y));
        let v2b = p;

        let (i_x,_i_y) = Vec2::intersect(v1a,v1b,v2a,v2b);
        let a_around = (i_x/(a.x as f64)) as i64;
                
        (a_around-1..=a_around+1).filter_map(|button_a|            
             if button_a*a.x < p.x && 
                button_a*a.y < p.y &&
                (p.x - button_a*a.x)%b.x==0 &&
                (p.y - button_a*a.y)%b.y==0 &&     
                (p.x - button_a*a.x)/b.x==(p.y - button_a*a.y)/b.y
                {
                    let button_b = (p.x - button_a*a.x)/b.x;
                    Some(3*button_a + button_b)
                } else {
                    None
                }
        ).min()
         .unwrap_or(0) as usize
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
