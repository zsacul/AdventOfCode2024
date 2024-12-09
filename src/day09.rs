const DOT : usize = 9999999;

fn check_sum(t:&[usize])->usize
{
    t.iter().enumerate().map(|(id,&n)|       
        if n==DOT {    0 }
             else { id*n }
    ).sum()
}

fn get_blocks(s:&str)->Vec<usize>
{
    let mut blocks = vec![];
    let mut down = true;

    for (id,c) in s.chars().enumerate()
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
                blocks.push(DOT);
            }                
        }
        
        down = !down;
    }

    blocks
}

fn ok(s:&str)->usize
{
    let mut blocks = get_blocks(s);

    let mut l = 0;
    let mut r = blocks.len()-1;

    while l<r
    {
        while blocks[l]!=DOT { l+=1; }
        while blocks[r]==DOT { r-=1; }

        if l>=r { break; }

        if blocks[l]==DOT && blocks[r]!=DOT
        {
            blocks.swap(l, r);
            l+=1;
            r-=1;
        }
    }

    check_sum(&blocks)
}

fn count(t:&[usize],n:usize)->usize
{
    let mut i = n;
    let v = t[i];

    while i < t.len() && t[i]==v
    {
        i+=1;
    }

    i-n    
}

fn ok2(s:&str)->usize
{
    let mut blocks = get_blocks(s);

    for io in (0..=s.len()/2).rev()
    {        
        let f = blocks.iter().enumerate().find(|&(_,e)|*e==io);
        
        if f.is_some()
        {
            let inx = f.unwrap().0;
            let cnt = count(&blocks,inx);

            for x in 0..blocks.len()
            {
                if blocks[x]==DOT && x<inx
                {
                    let dots = count(&blocks, x);
                    
                    if dots>=cnt
                    {
                        for i in 0..cnt
                        {
                            blocks.swap(x  +i, inx+i);
                        }        
                        break;
                    }                    
                }
            }
        }
    }

    check_sum(&blocks)
}


pub fn part1(data:&[String])->usize
{
   data.iter()
       .map(|n| ok(n))
       .sum()
}

pub fn part2(data:&[String])->usize
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
