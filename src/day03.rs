fn token(s:&str,i:usize,token:&str)->usize
{
    if i<s.len() && s[i..].starts_with(token)
    {
        i + token.len()
    }
      else 
    {
        i
    }
}

fn number(s:&str,i:usize,max_size:usize)->usize
{
    let mut i2 = i;

    while i2-i<=max_size && i2<s.len() && s.chars().nth(i2).unwrap_or(' ').is_digit(10)
    {
        i2+=1;
    }

    i2
}

fn compute(line:String,part_two:bool)->usize
{
    let s = line.as_str();
    let mut i=0;
    
    let mut enabled = true;
    let mut sum = 0;

    while i<s.len() 
    {        
        if part_two
        {
            let itrue = token(s,i,"do()");
            if itrue>i  { enabled = true;  i = itrue-1;  }
            
            let ifalse = token(s,i,"don't()");        
            if ifalse>i { enabled = false; i = ifalse-1; }
        }
        
        
        let i2 = token(s,i,"mul(");
        let i3 = number(s,i2,3);
        let i4 = token(s,i3,",");
        let i5 = number(s,i4,3);
        let i6 = token(s,i5,")");

        if i2>i && i3>i2 && i4>i3 && i5>i4 && i6>i5
        {
            let n1 = s[i2..i3].parse::<usize>().unwrap_or(0);
            let n2 = s[i4..i5].parse::<usize>().unwrap_or(0);

            if n1>0 && n2>0 && enabled
            {
                sum+=n1*n2;
                i = i6-1;
            }
        }
        
        i+=1;
    }
    sum
}

pub fn part1(data:&[String])->usize
{        
    compute(data.join(""), false)
}

pub fn part2(data:&[String])->usize
{
    compute(data.join(""), true)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day3");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
    ];
    assert_eq!(part1(&v),161);
}

#[test]
fn test2()
{
    let v = vec![
        "mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 )".to_string(),
    ];
    assert_eq!(part1(&v),0);
}

#[test]
fn test3()
{
    let v = vec![
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string(),
    ];
    assert_eq!(part2(&v),48);
}
