fn check_sum(t:&Vec<(u32)>)->u128
{
    let dot = 9999999u32;    

    t.iter().enumerate().map(|(ii,&n)|       
        if n==dot
        {
            0
        }
        else
        {
            (ii as u128)*(n as u128)
        }
    ).sum()
}

fn ok(s:&str)->u128
{
    let mut blocks = vec![];

    let mut id=0u32;

    let mut t : Vec<u32> = vec![];


    let dot = 9999999u32;
    let mut down = true;

    for c in s.chars()
    {
        let n = c.to_digit(10).unwrap();

        for _ in 0..n
        {
            if down
            {
                blocks.push(id/2);
            }
                else
            {
                blocks.push(dot);
            }                
        }
        
        down = !down;
        id+=1;    
    }

    t = blocks.clone();

    //blocks.push((n,prev,id));
    //println!("{:?}",blocks);
    

    let mut l = 0;
    let mut r = t.len()-1;

    while l<r
    {
        while t[l]!=dot {l+=1;}
        while t[r]==dot {r-=1;}

        if l>=r {break;}

        if t[l]==dot && t[r]!=dot
        {
            let tmp = t[l];
            t[l] = t[r];
            t[r] = tmp;

            l+=1;
            r-=1;
        }
    }

    check_sum(&t) as u128    
}

fn count(t:&Vec<u32>,n:usize)->usize
{
    let mut i = n;
    let v = t[i];

    while i < t.len() && t[i]==v
    {
        i+=1;
    }

    i-n    
}

fn ok2(s:&str)->u128
{
    let mut blocks = vec![];
    let mut id=0u32;
    let mut t : Vec<u32> = vec![];

    let dot = 9999999u32;
    let mut down = true;


    for c in s.chars()
    {
        let n = c.to_digit(10).unwrap();

        for x in 0..n
        {
            if down
            {
                blocks.push(id/2);
            }
                else
            {
                blocks.push(dot);
            }                
        }
        
        down = !down;
        id+=1;    
    }

    t = blocks.clone();

    for io in (0..=s.len()/2).rev()
    {        
        let f = t.iter().enumerate().find(|&(_,e)|*e==io as u32);
        
        if f.is_some()
        {
            let inx = f.unwrap().0 as usize;
            let cnt = count(&t,inx);

            for x in 0..t.len()
            {
                if t[x]==dot && x<inx
                {
                    let dots = count(&t, x);
                    //println!("DOTS={}x{}",x,dots);
                    
                    if dots>=cnt
                    {
                        //println!("TB = {:?}",t);
                        for i in 0..cnt
                        {
                            let tmp = t[x  +i];
                            t[x  +i]     = t[inx+i];
                            t[inx+i]     = tmp;
                        }        

                        //println!("swap {}x{} = {}",x,inx,cnt);    
                        //println!("TA = {:?}",t);
                        break;
                    }                    
                }
            }
        }
    }

    check_sum(&t) as u128   
}


pub fn part1(data:&[String])->u128
{
   data.iter()
       .map(|n| ok(n))
       .sum()
}

pub fn part2(data:&[String])->u128
{
    data.iter()
    .map(|n| ok2(n))
    .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day2");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "2333133121414131402".to_string(),
    ];
    assert_eq!(part1(&v),1928);
}

#[test]
fn test2()
{
    let v = vec![
        "2333133121414131402".to_string(),
    ];
    assert_eq!(part2(&v),2858);
}
