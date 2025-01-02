use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use rand::Rng;

#[derive(Debug,Clone,PartialEq, Eq,PartialOrd, Ord)]
enum Operator
{
    And,
     Or,
    Xor,
}

#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
struct Gate {
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

        let mut ops = [v[0],v[2]];
        ops.sort();

        let a = ops[0];
        let op = v[1];
        let b = ops[1];
        let out = v[4];

//        println!("a:{} op:{} b:{} out:{}",a,op,b,out);
        let oper = match op
        {
            "AND" => Operator::And,
            "OR"  => Operator::Or,
            "XOR" => Operator::Xor,
            _ => panic!("Unknown operator")
        };
        Gate { a:a.to_string(), b:b.to_string(), out:out.to_string(), oper}
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
            }

        }

        None

    }
}

#[derive(Debug,Clone)]
struct Data {
    instr        : HashMap<String,Gate>,    
    vals         : HashMap<String,bool>,
    instr_b      : HashMap<String,Gate>,    
    vals_b       : HashMap<String,bool>,
    possibilites : Vec<Vec<usize>>,
}

impl Data {
    fn new(input: &[String]) -> Self 
    {
        let sections: Vec<&[String]> = input.split(|line| line.is_empty()).collect();
        
        let vals = sections[0].iter()
            .map(|line| {
                let s = line.split(": ").collect::<Vec<&str>>();
                (s[0].to_string(),s[1]=="1")
            })
            .collect();
        
        let instr = sections[1].iter()
            .map(|line| {
                    let s = line.split(" ").collect::<Vec<&str>>();
                    (s[4].to_string(), Gate::new(line))
                } )            
            .collect();

        Data {
            instr,
            vals,
            instr_b : HashMap::new(),
            vals_b  : HashMap::new(),
            possibilites : vec![],
        }
    }

    fn gen_possibilities(&mut self)
    {
        let v = vec![0usize,1,2,3,4,5,6,7];
        let all = v.clone().into_iter().permutations(8).count();
        println!("all:{}",all);

        let mut used = HashSet::new();
        
        v.into_iter().permutations(8).for_each(|per|
            {
                let mut p :Vec<usize> = per.into_iter().collect();
                if p[0]>p[1] { let t = p[0]; p[0] = p[1]; p[1] = t; }
                if p[2]>p[3] { let t = p[2]; p[2] = p[3]; p[3] = t; }
                if p[4]>p[5] { let t = p[4]; p[4] = p[5]; p[5] = t; }
                if p[6]>p[7] { let t = p[6]; p[6] = p[7]; p[7] = t; }

                for a in 0..4
                {
                    for b in a+1..4
                    {
                        if p[2*a]>p[2*b]
                        {
                            let t = p[2*a  ]; p[2*a  ] = p[2*b  ]; p[2*b  ] = t;
                            let t = p[2*a+1]; p[2*a+1] = p[2*b+1]; p[2*b+1] = t;
                        }
                    }

                    if !used.contains(&p)
                    {
                        used.insert(p.clone());
                        self.possibilites.push(p.clone());
                        //println!("{:?}",p);
                    }
                }
            }
        );  

        println!("good:{}",self.possibilites.len());
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

    fn number(&self,c:char)->usize
    {
         self.vals
             .iter()
             .filter(|(k,&v)| k.starts_with(c) && v)
             .map(|z| 1<<z.0[1..].parse::<usize>().unwrap())
             .sum()
    }

    fn set_number(&mut self,c:char,v:usize)
    {
        for i in 0..=44
        {
            let s = format!("{}{:02}",c,i);
            self.vals.insert(s, (v&(1<<i))!=0);
        }

         //self.vals
         //    .iter()
         //    .filter(|(k,&v)| k.starts_with(c) && v)
         //    .map(|z| 1<<z.0[1..].parse::<usize>().unwrap())
         //    .sum()
    }

    fn perms(&self,s:Vec<String>)->Vec<Vec<String>>
    {
        let mut res = vec![];
        for p in self.possibilites.iter()
        {
            let  r = p.iter()            
                                   .map(|&v| s[v].to_string())
                                   .collect::<Vec<String>>();
            res.push(r);           
        }
        res
    }


    fn count1(&mut self)->usize
    {
        let mut done = false;

        while !done
        {
            done = true;
            let mut update = vec![];

            for (_,v) in self.instr.iter()
            {
                if self.get_v(v.out.clone()).is_none()
                {
                    let va = self.get_v(v.a.clone());
                    let vb = self.get_v(v.b.clone());
                    let res= v.evaluate(va,vb);

                    if res.is_some()
                    {
                        update.push((v.out.clone(),res));
                        done = false;
                    }
                }   
            }

            for (v,g) in update.iter()
            {
                self.set_v(v.clone(),g.unwrap());
            }
        }

        self.number('z')
    }

    fn swap(&mut self,a:String,b:String)
    {     
        let                       t = self.instr.get(&a).unwrap().out.clone();
        self.instr.get_mut(&a).unwrap().out = self.instr.get(&b).unwrap().out.clone();
        self.instr.get_mut(&b).unwrap().out = t;
    }

    fn is(&self,s:String,z:char)->bool
    {
        s.starts_with(z)
    }

    fn try_that(&mut self,bx:usize,by:usize)->bool
    {
        self.vals  = self.vals_b.clone();
        self.set_number('x',bx);
        self.set_number('y',by);

        bx+by==self.count1()
    }

    fn check(&mut self,s:Vec<String>)->bool
    {
        //self.instr = self.instr_b.clone();
        self.vals  = self.vals_b.clone();
        
        self.swap(s[0].clone(),s[1].clone());
        self.swap(s[2].clone(),s[3].clone());
        self.swap(s[4].clone(),s[5].clone());
        self.swap(s[6].clone(),s[7].clone());
        
        let ox = self.number('x');
        let oy = self.number('y');

        let c = self.count1();
        
        if ox+oy!=c
        {
            self.swap(s[0].clone(),s[1].clone());
            self.swap(s[2].clone(),s[3].clone());
            self.swap(s[4].clone(),s[5].clone());
            self.swap(s[6].clone(),s[7].clone());
            return false;
        }
//1111111111111111111111111111111111111111111
//self.instr = self.instr_b.clone();
        
//11111111111111111111111111111111111111111111

        if !self.try_that(0b111111111111111111111111111111111111111111111, 
                          0b111111111111111111111111111111111111111111111) ||
           !self.try_that(0b101110101111011011111101111101110111111111, 
                          0b111101111101011101110111101111011111111111) ||
           !self.try_that(0b10111110110111101010110111110111011111111110, 
                          0b11110111111011101111010100111101111111111110) ||
                  !self.try_that(0b0, 
                           0b0)  
        {

            self.set_number('x',ox);
            self.set_number('y',oy);
            self.swap(s[0].clone(),s[1].clone());
            self.swap(s[2].clone(),s[3].clone());
            self.swap(s[4].clone(),s[5].clone());
            self.swap(s[6].clone(),s[7].clone());
            return false;
        }

        let rr = &mut rand::thread_rng();

        for a in 0..1000
        {
            let ax = rr.gen_range(0..=0b111111111111111111111111111111111111111111111);
            let bx = rr.gen_range(0..=0b111111111111111111111111111111111111111111111);

            if !self.try_that(ax,bx)  
                {
                self.set_number('x',ox);
                self.set_number('y',oy);
                self.swap(s[0].clone(),s[1].clone());
                self.swap(s[2].clone(),s[3].clone());
                self.swap(s[4].clone(),s[5].clone());
                self.swap(s[6].clone(),s[7].clone());
                return false;
            }

        }
        //        rr.gen_range(0..ss.len());
        

        /*
        self.vals  = self.vals_b.clone();

        let bx = 0b11111111111111111111111111111111111111;
        let by = 0b11111111111111111111111111111111111111;

        self.set_number('x',bx);
        self.set_number('y',by);

        let c = self.count1();

        if bx+by!=c
        {
            self.set_number('x',ox);
            self.set_number('y',oy);
            self.swap(s[0].clone(),s[1].clone());
            self.swap(s[2].clone(),s[3].clone());
            self.swap(s[4].clone(),s[5].clone());
            self.swap(s[6].clone(),s[7].clone());
            return false;
        }
         */

         //qa: 43 ..........................................dnt,z05,gdf,mcm,gwc,z30,jst,z15
         //part2: dnt,gdf,gwc,jst,mcm,z05,z15,z30

         //qa: 43 .....................dnt,z15,gdf,hrq,gwc,z30,jst,z05
         //part2: dnt,gdf,gwc,hrq,jst,z05,z15,z30         
         //Elapsed: 6.5490003 secs        

        let res = s.join(",");
        println!("{}",res);
        
        true
    }

    fn count2(&mut self)->String
    {
        let n1 = self.number('x');
        let n2 = self.number('y');

        let mut ss : Vec<String> = self.instr.iter()
                                             .filter(|(_,g)| g.out!="z45" && self.is(g.out.clone(),'z') && g.oper != Operator::Xor)
                                             .map(|(k,g)| 
                                                //format!("{} {} {} => {}",g.a.clone(),self.name(g.oper.clone()),g.b.clone(),g.out.clone())
                                                g.out.clone()
                                            ).collect::<Vec<String>>();
        ss.sort();
        ss.dedup();

        println!("ss:{:#?}",ss);
        println!("len:{}",ss.len());


        let mut qq : Vec<String> = self.instr.iter()
                                             .filter(|(_,g)| 
                                             !self.is(g.out.clone(),'z') && 
                                             !self.is(g.a.clone(),'x') && 
                                             !self.is(g.a.clone(),'y') &&                                              
                                             !self.is(g.b.clone(),'x') && 
                                             !self.is(g.b.clone(),'y') &&                                              
                                             g.oper == Operator::Xor)
                                             .map(|(k,g)| 
                                             //format!("{} {} {} => {}",g.a.clone(),self.name(g.oper.clone()),g.b.clone(),g.out.clone())
                                             g.out.clone()
                                        ).collect::<Vec<String>>();

        qq.sort();
        qq.dedup();

        qq.append(&mut ss);

        println!("qq:{:#?}",qq);
        println!("len:{}",qq.len());

        let mut outs = self.instr.iter().map(|(k,g)| g.out.clone() ).collect::<Vec<String>>();
        outs.sort();
        let mut id=0;

        self.instr_b = self.instr.clone();
        self.vals_b  = self.vals.clone();

        for a in 43..outs.len()
        {
            if !qq.contains(&outs[a])
            {
                eprint!("qa: {} ",a);
                for b in a+1..outs.len()
                {
                    if !qq.contains(&outs[b])
                    {
                        if a!=b
                        {
                            let mut add = vec![outs[a].clone(),outs[b].clone()];
                            let mut qa = qq.clone();
                            qa.append(&mut add);
                            qa.sort();

                            let mut u=0;

                            //qa.iter().permutations(8)
                            
                            for e in self.perms(qa.clone())
                            {
                                if self.check(e)
                                {
                                    return qa.clone().join( ",");
                                    //return e.clone().join(",");                                  
                                }
                                u+=1;
                            }
                            

                            //return "".to_string();
                            
                            id+=1;
                        }                        
                        eprint!(".");                        
                    }
                }
                println!();
             
            }
        }

        return "".to_string();

/*

        return "".to_string();

        let mut v: Vec<_> = self.instr.iter().filter(|a| a.1.out.starts_with("z") ).collect();

        let mut oo = vec![];
        for t in v
        {
            let gg = t.1.clone();
            oo.push((gg.out.clone(),self.name(gg.oper)));
        }

        oo.sort();
        println!("v:{:#?}",oo);
        
        //use rand to fill num as 8 distinct random numbers in range 0..ss.len()
       
        let rr = &mut rand::thread_rng();
//        rr.gen_range(0..ss.len());

        let mut num = vec![ 151,206,33,71,74,78,88,93];
        //vec![157, 43, 141, 206, 43, 9, 77, 70];
        // vec![165, 66, 137, 110, 214, 14, 150, 8];
        
        //vec![1,2,3,4,5,6,7,8];

        let mut wrong = usize::MAX;
        let mut best = num;

        for i in 0..=44         
        {
            //2 trailing zeros
            let xx = format!("x{:02}",i);
            self.vals.insert(xx, !true);
            let yy = format!("y{:02}",i);
            self.vals.insert(yy, !true);

            //format!("{:b}",i);
        }
        //return "".to_string();

        let n1 = self.number('x');
        let n2 = self.number('y');
        let des = n1+n2;

        println!("n1:{} n2:{} des:{}",n1,n2,des);
        */

        let vals_bkp = self.vals.clone();
        let inst_bkp = self.instr.clone();

        //vec![56, 206, 101, 191, 126, 66, 136, 25];
        //vec![199, 8, 47, 87, 107, 23, 113, 218];
        //vec![164, 192, 187, 87, 107, 23, 47, 191];
        //vec![199, 8, 47, 87, 107, 23, 113, 218];

        //let lucky = vec![56, 206, 101, 191, 126, 66, 136, 25];

        //return 0;
        
        //bhb,ffj,gvj,kgr,nsc,tdw,wff,z30
        //cfp,fhn,gdf,gdr,jjk,kff,tdw,z30
        //bjc,bjf,bkg,bwq,hfq,kmr,ngc,vmd
        //dvj,gvj,hrq,ngc,rkg,sgt,z15,z30
        //dvj,gvj,hrq,ngc,rkg,sgt,z15,z30
        //dvj,gvj,hrq,ngc,rkg,sgt,z15,z30
        //gvj,z30,ngc,z15,rkg,hrq,sgt,dvj

/*
        loop {
            
            self.vals  = vals_bkp.clone();
            self.instr = inst_bkp.clone();
            
            num = best.to_vec();

            let count = rr.gen_range(1..=4)*2;

                for i in 0..count
                {
                    let r = rr.gen_range(0..ss.len());
                    let id = rr.gen_range(0..8);
                    num[id] = r;
                }

             //   println!("num:{:?}",num);

             self.swap(ss[num[0]].clone(),ss[num[1]].clone());
             self.swap(ss[num[2]].clone(),ss[num[3]].clone());
             self.swap(ss[num[4]].clone(),ss[num[5]].clone());
             self.swap(ss[num[6]].clone(),ss[num[7]].clone());


//           if self.instr.contains_key(&ss[d])
//           self.instr.get_mut(&ss[d]).unwrap().swap();


                let c = self.count1();

                //println!("a:{} b:{} c:{} d:{} res:{}",a,b,c,d,c);
                if c == des 
                {

                    //println!("yes!");
                    self.vals  = vals_bkp.clone();

                    for i in 0..=44         
                    {
                        //2 trailing zeros
                        let xx = format!("x{:02}",i);
                        self.vals.insert(xx, true);
                        let yy = format!("y{:02}",i);
                        self.vals.insert(yy, true);
            
                        //format!("{:b}",i);
                    }
                    self.instr = inst_bkp.clone();
                    self.swap(ss[num[0]].clone(),ss[num[1]].clone());
                    self.swap(ss[num[2]].clone(),ss[num[3]].clone());
                    self.swap(ss[num[4]].clone(),ss[num[5]].clone());
                    self.swap(ss[num[6]].clone(),ss[num[7]].clone());
    

                    let n1 = self.number('x');
                    let n2 = self.number('y');
                    let des = n1+n2;

                    if self.count1()==des
                    {
                        let mut outs = num.iter().map(|a| ss[*a as usize].to_string() ).collect::<Vec<String>>();
                        outs.sort();
                        let res = outs.join(",");
                        println!("{}",res);
                        return res;
                    }

                    

            
                }

                let x = c^des;
                let cc = x.count_ones() as usize;

                if cc<=wrong
                {
                    wrong = cc;
                    best = num.to_vec();

                    
                    println!("des:{:#048b}",des);
                    println!("cnt:{:#048b}",c);
                    println!("xor:{:#048b}",x);
                    println!("ok :{}",cc);
                    println!("num:{:?}",num);                    
                }



        }
*/

        "none".to_string()
    }

}

pub fn part1(data:&[String])->usize
{
    Data::new(data).count1()
}

pub fn part2(data:&[String])->String
{
    let mut data = Data::new(data);
    data.gen_possibilities();
    data.count2()
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
    

#[test]
fn test3()
{
    let v = vec![
        "x00: 0".to_string(),
        "x01: 1".to_string(),
        "x02: 0".to_string(),
        "x03: 1".to_string(),
        "x04: 0".to_string(),
        "x05: 1".to_string(),
        "y00: 0".to_string(),
        "y01: 0".to_string(),
        "y02: 1".to_string(),
        "y03: 1".to_string(),
        "y04: 0".to_string(),
        "y05: 1".to_string(),
        "".to_string(),
        "x00 AND y00 -> z05".to_string(),
        "x01 AND y01 -> z02".to_string(),
        "x02 AND y02 -> z01".to_string(),
        "x03 AND y03 -> z03".to_string(),
        "x04 AND y04 -> z04".to_string(),
        "x05 AND y05 -> z00".to_string(),
        ];
        assert_eq!(part2(&v),"aaa,aoc,bbb,ccc,eee,ooo,z24,z99".to_string());
}
    

//dnt,gdf,gwc,jst,z05,z10,z15,z30 - wrong
//dnt,gdf,gwc,jst,tdw,z05,z15, - wrong (6,61)
//dnt,gdf,gwc,hrq,jst,z05,z15, - wrong
//dnt,gdf,gwc,jst,mcm,z05,z15,z30

/*
qa: 25/81 
dnt,hrq,gwc,z05,jst,z15,z11,z30
part2: dnt,gwc,hrq,jst,z05,z11,z15,z30


qa: 42 ..............................................................................................................................................................................
qa: 43 .....................dnt,hrq,gdf,z05,gwc,z15,jst,z30
part2: dnt,gdf,gwc,hrq,jst,z05,z15,z30

qa: 43 ..........................................dnt,z15,gdf,mcm,gwc,z30,jst,z05
part2: dnt,gdf,gwc,jst,mcm,z05,z15,z30

qa: 43 ..........................................dnt,z15,gdf,mcm,gwc,z30,jst,z05
part2: dnt,gdf,gwc,jst,mcm,z05,z15,z30
 */
