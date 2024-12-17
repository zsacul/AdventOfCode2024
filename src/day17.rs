use super::tools;

#[derive(Debug,Clone, Copy)]
enum Code
{
    Adv(u8),//0
    Bxl(u8),//1
    Bst(u8),//2
    Jnz(u8),//3
    Bxc(u8),//4
    Out(u8),//5
    Bdv(u8),//6
    Cdv(u8),//7
 }

#[derive(Debug,Clone)]
struct Data {
      reg_A: u64,  
      reg_B: u64,  
      reg_C: u64,  
      prog : Vec<Code>,
      i    : usize,
      res  : Vec<u8>,
      golden : Vec<u8>,
      part2  : bool,
}

impl Data {
    fn new(input: &[String]) -> Self {
        let sections: Vec<&[String]> = input.split(|line| line.is_empty()).collect();

        let reg_A = tools::usize_get_between(&sections[0][0],"Register A: ","") as u64;
        let reg_B = tools::usize_get_between(&sections[0][1],"Register B: ","") as u64;
        let reg_C = tools::usize_get_between(&sections[0][2],"Register C: ","") as u64;
        let prog  = tools::get_between(&sections[1][0],"Program: ","");

        let num = prog.split(",").map(|n| n.parse().unwrap()).collect::<Vec<u64>>();

        let prog : Vec<Code> = 

        (0..num.len()).step_by(2)
            .map(|i|
            {
                let op  = num[i+1] as u8;
                
                match num[i]
                {
                    0 => Code::Adv(op),
                    1 => Code::Bxl(op),
                    2 => Code::Bst(op),
                    3 => Code::Jnz(op),
                    4 => Code::Bxc(op),
                    5 => Code::Out(op),
                    6 => Code::Bdv(op),
                    7 => Code::Cdv(op),
                    _ => panic!("Unknown code"),
                }            
            }
        ).collect();

        let golden = num.iter().map(|a| *a as u8).collect::<Vec<u8>>();
             
        Data {
            reg_A,
            reg_B,
            reg_C,
            prog,
            i:0,
            res: vec![],
            golden,
            part2:false,
        }
    }

    fn combo(&self,n:u8)->u64
    {
        match n
        {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_A,
            5 => self.reg_B,
            6 => self.reg_C,
            7 => panic!("Invalid program"),
            _ => panic!("Unknown code"),
            
        }
    }

    fn run(&mut self)->bool
    {
        let code = self.prog[self.i];

        match code
        {
            Code::Adv(n) => self.reg_A = self.reg_A/(1<<self.combo(n)),
            Code::Bxl(n) => self.reg_B ^= n as u64,
            Code::Bst(n) => self.reg_B = self.combo(n) % 8,
            Code::Jnz(n) => {
                                    if self.reg_A != 0
                                    {
                                        self.i = (n/2) as usize;
                                        self.i-=1;
                                    }
                                },
            Code::Bxc(n) => self.reg_B = self.reg_B^self.reg_C,
            Code::Out(n) => {
                                    let c = (self.combo(n)%8) as u8;
                                    if self.part2 && self.golden[self.res.len()]!=c { return false; }
                                    self.res.push(c);
                                },
            Code::Bdv(n) => self.reg_B = self.reg_A/(1<<self.combo(n)),
            Code::Cdv(n) => self.reg_C = self.reg_A/(1<<self.combo(n)),
            _                => panic!("Unknown code"),
        }
        self.i+=1;
        true
    }

    fn ok1(&mut self) -> Vec<u8>
    {
        while self.i < self.prog.len() { self.run(); }
        self.res.clone()
    }

    fn ok2(&mut self)
    {
        self.part2 = true;
        while self.i < self.prog.len() && self.run() {};
    }

    fn count1(&mut self)->String
    {        
        self.ok1().iter().map(|a| a.to_string()).collect::<Vec<String>>().join(",")
    }


}

pub fn part1(data:&[String])->String
{
    Data::new(data).count1()
}

//105_706_278_758_810 too high
//105_706_277_661_082
  
pub fn part2(data:&[String])->usize
{
    let mut d = Data::new(data);
    let b = d.reg_B;
    let c = d.reg_C;

    println!("{:?}",d.golden);

    let mut re = 0u64;//281_474_900_000_000;
    let mut prev = 0;

    //d.golden = vec![2,4,1,5,7,5,1,6,0,3,4,3,5,5,3,0];
    //d.golden = vec![0,3,5,4,3,0];

    let mut rng = rand::thread_rng();
    //3110000000
    loop 
    {
//        let res = re;//(re << 19) + 0b0100010100110011010;
          let res = (re << 14) + 0b10100110011010;
//
//
//10100110011010
        //
//"00100010100110011010"
//100100010100110011010
        d.reg_A = res as u64;
        d.reg_B = b;
        d.reg_C = c;
        d.i = 0;
        d.res = vec![];
        //d.run();

        //>64K
        //let tab = 
        d.ok2();

        //hard test up to:
        //78_000_000_000
        
        //2,4,1,5,7,5,1,6,0,3,4,3,5,5,3,0
        if d.res.len()==d.golden.len()
        //-10
        {
            let del = res-prev;
            prev = res;
            //let re = tab.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(",");
           // println!("{:#064b} {:?} = {:?} del={}",res,res,d.res,del);
            
            if d.res == d.golden
            {
                return res as usize;            
            }   
        }


        if re%1000_000_000==0
        {
            println!("{}",res);
        }
        re+=1;
    }
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day17");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "Register A: 729".to_string(),
        "Register B: 0".to_string(),
        "Register C: 0".to_string(),
        "".to_string(),
        "Program: 0,1,5,4,3,0".to_string(),
    ];
    assert_eq!(part1(&v),"4,6,3,5,6,3,5,2,1,0");
}


#[test]
fn test2()
{
    let v = vec![
        "Register A: 729".to_string(),
        "Register B: 0".to_string(),
        "Register C: 9".to_string(),
        "".to_string(),
        "Program: 2,6".to_string(),
    ];
    let mut d = Data::new(&v);
    d.reg_C = 9;
    d.run();

    assert_eq!(d.reg_B,1);
}

#[test]
fn test3()
{
    let v = vec![
        "Register A: 10".to_string(),
        "Register B: 0".to_string(),
        "Register C: 0".to_string(),
        "".to_string(),
        "Program: 5,0,5,1,5,4".to_string(),
    ];
    //let mut d = Data::new(&v);
    assert_eq!(part1(&v),"0,1,2");
}

#[test]
fn test4()
{
    let v = vec![
        "Register A: 2024".to_string(),
        "Register B: 0".to_string(),
        "Register C: 0".to_string(),
        "".to_string(),
        "Program: 0,1,5,4,3,0".to_string(),
    ];
    println!("tesd1");
    assert_eq!(part1(&v),"4,2,5,6,7,7,7,7,3,1,0");
    
    println!("tesd2");
    let mut d = Data::new(&v);
    let c = d.count1();
    //d.run();
    assert_eq!(d.reg_A,0);
}

#[test]
fn test5()
{
    let v = vec![
        "Register A: 2024".to_string(),
        "Register B: 29".to_string(),
        "Register C: 0".to_string(),
        "".to_string(),
        "Program: 1,7".to_string(),
    ];

    let mut d = Data::new(&v);
    d.run();
    assert_eq!(d.reg_B,26);   
}

#[test]
fn test6()
{
    let v = vec![
        "Register A: 2024".to_string(),
        "Register B: 2024".to_string(),
        "Register C: 43690".to_string(),
        "".to_string(),
        "Program: 4,0".to_string(),
    ];

    let mut d = Data::new(&v);
    //let c = d.count1();
    d.run();
    assert_eq!(d.reg_B,44354);   
}

#[ignore]
#[test]
fn test7()
{
    let v = vec![
        "Register A: 2024".to_string(),
        "Register B: 0".to_string(),
        "Register C: 0".to_string(),
        "".to_string(),
        "Program: 0,3,5,4,3,0".to_string(),
    ];

    assert_eq!(part2(&v),117440);
}

#[test]
fn test8()
{
    let v = vec![
        "Register A: 117440".to_string(),
        "Register B: 0".to_string(),
        "Register C: 0".to_string(),
        "".to_string(),
        "Program: 0,3,5,4,3,0".to_string(),
    ];
    assert_eq!(part1(&v),"0,3,5,4,3,0");
}
