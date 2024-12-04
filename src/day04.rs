use super::tools;
use super::vec2::Vec2;
use std::collections::HashMap;

fn count(pos:Vec2,h:&mut HashMap<Vec2,char>,w:String)->usize
{
    pos.around8().iter().filter(|dir2|
        {
            let dir = Vec2::new(dir2.x as i64-pos.x,dir2.y as i64-pos.y);
            
            let mut i = 0;
            let mut t = Vec::new();
            while let Some(c) = h.get(&(pos.add(dir.x*i,dir.y*i))) 
            {
                t.push(*c);
                i+=1;
                if i as usize>=w.len() { break };
            }
            let s1 = t.iter().collect::<String>();
    
            s1==w
        }
    ).count()
}

fn count2(x:i64,y:i64,h:&mut HashMap<Vec2,char>,w:String)->usize
{
    let mut cnt = 0;
    let pos = Vec2::new(x,y);

    //for dir2 in pos.around8()
    {
        let mut t1 = Vec::new();
        let mut t2 = Vec::new();

        let dir1 = Vec2::new(1,1);
        let dir2 = Vec2::new(1,-1);


        
        let mut i = 0;
        for i in -1..=1
        {
            if let Some(c) = h.get(&(pos.add(dir1.x*i,dir1.y*i))) 
            {
                t1.push(*c);
            }
            if let Some(c) = h.get(&(pos.add(dir2.x*i,dir2.y*i))) 
            {
                t2.push(*c);
            }

        }
    
        //convert t to string
        let s1 = t1.iter().collect::<String>();
        let s2 = t2.iter().collect::<String>();
      //  let s2 = t.iter().rev().collect::<String>();

      let rev_w = w.chars().rev().collect::<String>();

       // println!("{}",s1);

        if (s1==w || s1==rev_w) && (s2==w || s2==rev_w) 
        {
           // println!("{} {} = {} ",pos.x,pos.y,s1);
            cnt+=1;
        }
    }

    cnt
}

pub fn part1(data:&[String])->usize
{
    let (dx,dy) = (data[0].len(),data.len());
    let mut h = tools::get_hash_table(data);
    h.retain(|k,v| "XMAS".contains(*v));
    let hh = h.clone();

   
    
    hh.into_iter()
    .map(|(p,_)| count(p.x as i64,p.y as i64,&mut h,"XMAS".to_string())
    )
    .sum()
 
 
    
    //h.filter_map(|(k,v)| if "XMAS".contains(v) { Some(k) } else { None }).count()

/*
       .values()
       .filter(|n| ok(n,false))
       .count()
   data.iter()
       .filter(|n| ok(n,false))
       .count()
        */
}

pub fn part2(data:&[String])->usize
{
    let (dx,dy) = (data[0].len(),data.len());
    let mut h = tools::get_hash_table(data);
    h.retain(|k,v| "MAS".contains(*v));
    let hh = h.clone();

   
    
    hh.into_iter()
    .map(|(p,_)| count2(p.x as i64,p.y as i64,&mut h,"MAS".to_string())
    )
    .sum()   //data.iter()
     //  .filter(|n| ok(n,true))
       //.count()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day4");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
         "MMMSXXMASM".to_string(),
         "MSAMXMSMSA".to_string(),
         "AMXSXMAAMM".to_string(),
         "MSAMASMSMX".to_string(),
         "XMASAMXAMM".to_string(),
         "XXAMMXXAMA".to_string(),
         "SMSMSASXSS".to_string(),
         "SAXAMASAAA".to_string(),
         "MAMMMXMMMM".to_string(),
         "MXMXAXMASX".to_string(),
    ];
    assert_eq!(part1(&v),18);
}

#[test]
fn test2()
{
    let v = vec![
        "..X...".to_string(),
        ".SAMX.".to_string(),
        ".A..A.".to_string(),
        "XMAS.S".to_string(),
        ".X....".to_string(),
    ];
    assert_eq!(part1(&v),4);
}


#[test]
fn test3()
{
    let v = vec![
        ".M.S......".to_string(),
        "..A..MSMS.".to_string(),
        ".M.S.MAA..".to_string(),
        "..A.ASMSM.".to_string(),
        ".M.S.M....".to_string(),
        "..........".to_string(),
        "S.S.S.S.S.".to_string(),
        ".A.A.A.A..".to_string(),
        "M.M.M.M.M.".to_string(),
        "..........".to_string(),
    ];
    assert_eq!(part2(&v),9);
}
