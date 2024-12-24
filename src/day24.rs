use std::collections::HashMap;
use std::collections::HashSet;
use super::tools;

#[derive(Debug,Clone)]
enum Operator
{
    And,
     Or,
    Xor,
}

#[derive(Debug,Clone)]
struct Gate {
      op : String,
       a : String,
       b : String,
     out : String,
    oper : Operator,
}

impl Gate 
{
    fn new(s:&str)->Self
    {
        let v = s.split(" ").collect::<Vec<&str>>();
        let a = v[0];
        let op = v[1];
        let b = v[2];
        let out = v[4];

//        println!("a:{} op:{} b:{} out:{}",a,op,b,out);
        let oper = match op
        {
            "AND" => Operator::And,
            "OR"  => Operator::Or,
            "XOR" => Operator::Xor,
            _ => panic!("Unknown operator")
        };
        Gate { op:op.to_string(), a:a.to_string(), b:b.to_string(), out:out.to_string(), oper}
    }


    fn evaluate(&self,va:Option<bool>,vb:Option<bool>)->Option<bool>
    {
        match self.oper {
            Operator::And =>
            { 
                if va.is_some() && vb.is_some()
                {
                    return Some(va.unwrap() && vb.unwrap());
                }
                else {
                    if va.is_some() && vb.is_none()
                    {
                        if !va.unwrap()
                        {
                            return Some(false);
                        }
                    }
                    if vb.is_some() && va.is_none()
                    {
                        if !vb.unwrap()
                        {
                            return Some(false);
                        }
                    }
                }
            },
            Operator::Or => 
            {
                if va.is_some() && vb.is_some()
                {
                    return Some(va.unwrap() || vb.unwrap());
                }
                else {
                    if va.is_some() && vb.is_none()
                    {
                        if va.unwrap()
                        {
                            return Some(true);
                        }
                    }
                    if vb.is_some() && va.is_none()
                    {
                        if vb.unwrap()
                        {
                            return Some(true);
                        }
                    }
                }
            },
            Operator::Xor => 
            {
                if va.is_some() && vb.is_some()
                {
                    return Some(va.unwrap() ^ vb.unwrap());
                }
            },
            _ => panic!("Unknown operator")

        }

        None

    }
}

#[derive(Debug)]
struct Data {
    instr: HashMap<String,Gate>,    
    vals : HashMap<String,bool>,
}

impl Data {
    fn new(input: &[String]) -> Self {
        let sections: Vec<&[String]> = input.split(|line| line.is_empty()).collect();
        
        let vals = sections[0].iter()
            .map(|line| {
                let s = line.split(": ").collect::<Vec<&str>>();
                (s[0].to_string(),if s[1]=="1" {true} else {false})
            })
            .collect();
        
        let instr  = sections[1].iter()
            .map(|line| {
                    let s = line.split(" ").collect::<Vec<&str>>();
                    (s[4].to_string(), Gate::new(line))
                } )            
            .collect();

        Data {
            instr,
            vals
        }
    }

    fn get_v(&self,n:String)->Option<bool>
    {
        if self.vals.contains_key(&n)
        {
            return Some(*self.vals.get(&n).unwrap());
        }
        None
    }

    fn set_v(&mut self,n:String,v:bool)
    {
        self.vals.insert(n,v);
    }

    fn ok1(&self,line:&[usize]) -> usize
    {
        
        0
    }

    fn count1(&mut self)->usize
    {
        loop 
        {
            let mut done = true;
            let mut update = vec![];

            for (k,v) in self.instr.iter()
            {
                if self.get_v(v.out.clone()).is_none()
                {
                    let va = self.get_v(v.a.clone());
                    let vb = self.get_v(v.b.clone());
                    let res = v.evaluate(va,vb);

                    if res.is_some()
                    {
                        //self.set_v(v.out.clone(),res.unwrap());
                        //if v.vout.is_some()
                        {
                            //self.vals.insert(v.out.clone(),v.vout.unwrap());
                            update.push((v.out.clone(),res));
                        }
                        done = false;
                    }
                }   
            }

            for (v,g) in update.iter()
            {
                //if g.vout.is_some()
                //{
                    self.set_v(v.clone(),g.unwrap());
                //}
            }
            if done
            {
                break;
            }
        }

        let zet:Vec<_> = self.vals
                             .iter()
                             .filter(|(k,v)| k.starts_with("z"))
                             .map(|z| if *z.1 {1<<z.0[1..].parse::<usize>().unwrap()} else {0})
                             .collect();

        let res = zet.iter().sum();


        println!("ins: {:?}",res);

        res
    }

    fn count2(&self)->usize
    {
        0
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
    println!("Day24");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "x00: 1".to_string(),
        "x01: 0".to_string(),
        "x02: 1".to_string(),
        "x03: 1".to_string(),
        "x04: 0".to_string(),
        "y00: 1".to_string(),
        "y01: 1".to_string(),
        "y02: 1".to_string(),
        "y03: 1".to_string(),
        "y04: 1".to_string(),
        "".to_string(),
        "ntg XOR fgs -> mjb".to_string(),
        "y02 OR x01 -> tnw".to_string(),
        "kwq OR kpj -> z05".to_string(),
        "x00 OR x03 -> fst".to_string(),
        "tgd XOR rvg -> z01".to_string(),
        "vdt OR tnw -> bfw".to_string(),
        "bfw AND frj -> z10".to_string(),
        "ffh OR nrd -> bqk".to_string(),
        "y00 AND y03 -> djm".to_string(),
        "y03 OR y00 -> psh".to_string(),
        "bqk OR frj -> z08".to_string(),
        "tnw OR fst -> frj".to_string(),
        "gnj AND tgd -> z11".to_string(),
        "bfw XOR mjb -> z00".to_string(),
        "x03 OR x00 -> vdt".to_string(),
        "gnj AND wpb -> z02".to_string(),
        "x04 AND y00 -> kjc".to_string(),
        "djm OR pbm -> qhw".to_string(),
        "nrd AND vdt -> hwm".to_string(),
        "kjc AND fst -> rvg".to_string(),
        "y04 OR y02 -> fgs".to_string(),
        "y01 AND x02 -> pbm".to_string(),
        "ntg OR kjc -> kwq".to_string(),
        "psh XOR fgs -> tgd".to_string(),
        "qhw XOR tgd -> z09".to_string(),
        "pbm OR djm -> kpj".to_string(),
        "x03 XOR y03 -> ffh".to_string(),
        "x00 XOR y04 -> ntg".to_string(),
        "bfw OR bqk -> z06".to_string(),
        "nrd XOR fgs -> wpb".to_string(),
        "frj XOR qhw -> z04".to_string(),
        "bqk OR frj -> z07".to_string(),
        "y03 OR x01 -> nrd".to_string(),
        "hwm AND bqk -> z03".to_string(),
        "tgd XOR rvg -> z12".to_string(),
        "tnw OR pbm -> gnj".to_string(),
    ];
    assert_eq!(part1(&v),2024);
}

#[test]
fn test2()
{
    let v = vec![
        "x00: 1".to_string(),
        "x01: 1".to_string(),
        "x02: 1".to_string(),
        "y00: 0".to_string(),
        "y01: 1".to_string(),
        "y02: 0".to_string(),
        "".to_string(),
        "x00 AND y00 -> z00".to_string(),
        "x01 XOR y01 -> z01".to_string(),
        "x02 OR y02 -> z02".to_string(),
        ];
        assert_eq!(part1(&v),4);
}
    