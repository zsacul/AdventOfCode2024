use std::collections::HashMap;

fn calc(s:String,i:usize,n:usize,hash:&mut HashMap<(String,usize),usize>)->usize
{
    if hash.contains_key(&(s.clone(),i))
    {
        return hash[&(s,i)];
    }
    
    let st :Vec<usize> = s.split_whitespace().map(|a|a.parse().unwrap()).collect();
    let mut st2 = vec![];

    for &s in st.iter()
    {
        if s==0
        {
            st2.push(1);
        }
        else if (s.to_string().len()%2)==0
        {
            let ss = s.to_string();
            let s1 = ss[          ..ss.len()/2].parse::<usize>().unwrap();
            let s2 = ss[ss.len()/2..          ].parse::<usize>().unwrap();
            st2.push(s1);
            st2.push(s2);
        }
        else {
            st2.push(s*2024);
        }
    }

    if i==n-1
    {
        hash.insert((s,i),st2.len());
        st2.len()
    }
    else
    {
        let mut cnt = 0;
        for &s in st2.iter()
        {
            cnt+=calc(s.to_string(),i+1,n,hash)
        }
        hash.insert((s,i),cnt);
        cnt
    }

}

pub fn part1(data:&[String],n:usize)->usize
{
    let mut hash = HashMap::new(); 
    calc(data[0].to_string(),0,n,&mut hash)
}

pub fn part2(data:&[String],n:usize)->usize
{   
    let mut hash  = HashMap::new(); 
    calc(data[0].to_string(),0,n,&mut hash)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day11");
    println!("part1: {}",part1(data,25));
    println!("part2: {}",part2(data,75));
}

#[test]
fn test0()
{
    let v = vec![
        "125 17".to_string(),
    ];
    assert_eq!(part2(&v,6),22);
}

#[test]
fn test1()
{
    let v = vec![
        "0 1 10 99 999".to_string(),
    ];
    assert_eq!(part1(&v,1),7);
}

