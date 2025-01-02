use std::collections::HashSet;
use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Data {
      keys: HashSet<(bool,Vec<i32>)>,    
}

impl Data {
    fn new(input: &[String]) -> Self {
        let sections: Vec<&[String]> = input.split(|line| line.is_empty()).collect();

        let mut keys = HashSet::new();
        
        for s in &sections
        {
            let h = tools::get_hash_table(s);

            let mut key = vec![];

            let is_key : bool = *h.get(&Vec2::new(0,0)).unwrap()=='.';

            for x in 0..5
            {
                let mut cnt = 0;
                if is_key
                {
                    for y in (0..=6).rev()
                    {
                        if *h.get(&Vec2::new(x,y)).unwrap()=='#'
                        {
                            cnt+=1;
                        }                        
                    }
                }
                else
                {
                    for y in 0..=6
                    {
                        if *h.get(&Vec2::new(x,y)).unwrap()=='#'
                        {
                            cnt+=1;
                        }                        
                    }
                }

                key.push(cnt);

                
                
            }
            keys.insert((is_key,key));

            //println!("{} {:?}",is_key,key);
        }

        Data {
            keys
        }
    }

    fn fit(key:&(bool,Vec<i32>),lock:&(bool,Vec<i32>))->bool
    {
        let mut res = true;
        for i in 0..5
        {
            if key.1[i]>7-lock.1[i]
            {
                res = false;
                break;
            }
        }
        res
    }

    fn count1(&self)->usize
    {
        let keys : Vec<(bool,Vec<i32>)> = 
        self.keys
            .iter()
            .filter(|(is_key,key)| *is_key )
            .map(|f|f.clone())   
            .collect();

        let locks : Vec<(bool,Vec<i32>)> =  
        self.keys
            .iter()
            .filter(|(is_key,key)| !(*is_key) )
            .map(|f|f.clone())   
            .collect();

            let mut res = 0;
            for k in keys
            {
                for l in locks.iter()
                {
                    if Data::fit(&k,l)
                    {
                        res+=1;
                    }
                }
            }

            res
            
            
    }

}

pub fn part1(data:&[String])->usize
{
    Data::new(data).count1()
}


#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day25");
    println!("part1: {}",part1(data));
}

#[test]
fn test1()
{
    let v = vec![
        "#####".to_string(),
        ".####".to_string(),
        ".####".to_string(),
        ".####".to_string(),
        ".#.#.".to_string(),
        ".#...".to_string(),
        ".....".to_string(),
        "".to_string(),
        "#####".to_string(),
        "##.##".to_string(),
        ".#.##".to_string(),
        "...##".to_string(),
        "...#.".to_string(),
        "...#.".to_string(),
        ".....".to_string(),
        "".to_string(),
        ".....".to_string(),
        "#....".to_string(),
        "#....".to_string(),
        "#...#".to_string(),
        "#.#.#".to_string(),
        "#.###".to_string(),
        "#####".to_string(),
        "".to_string(),
        ".....".to_string(),
        ".....".to_string(),
        "#.#..".to_string(),
        "###..".to_string(),
        "###.#".to_string(),
        "###.#".to_string(),
        "#####".to_string(),
        "".to_string(),
        ".....".to_string(),
        ".....".to_string(),
        ".....".to_string(),
        "#....".to_string(),
        "#.#..".to_string(),
        "#.#.#".to_string(),
        "#####".to_string(),
    ];
    assert_eq!(part1(&v),3);
}
