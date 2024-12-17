use super::vec2::Vec2;
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
    Combo(u8),
}

#[derive(Debug,Clone)]
struct Data {
      reg_A: u64,  
      reg_B: u64,  
      reg_C: u64,  
      prog : Vec<Code>,
      i : usize,
      res : Vec<u8>,
}

impl Data {
    fn new(input: &[String]) -> Self {
        let sections: Vec<&[String]> = input.split(|line| line.is_empty()).collect();

        let reg_A = tools::i64_get_between(&sections[0][0],"Register A: ","");
        let reg_B = tools::i64_get_between(&sections[0][1],"Register B: ","");
        let reg_C = tools::i64_get_between(&sections[0][2],"Register C: ","");
        let prog  = tools::get_between(&sections[1][0],"Program: ","");

        let num = prog.split(",").map(|n| n.parse().unwrap()).collect::<Vec<u64>>();


        let mut program = vec![];

        for i in (0..num.len()).step_by(2)
        {
            let co = num[i  ];
            let op  = num[i+1] as u8;
            let code = match co
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
            };
            program.push(code);
        }
    //).collect();      
        
        Data {
            reg_A : reg_A as u64,
            reg_B : reg_B as u64,
            reg_C : reg_C as u64,
            prog: program,
            i:0,
            res: vec![]
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

    fn run(&mut self)
    {
        let code = self.prog[self.i];

        match code
        {
            Code::Adv(n) => {
                {
                    println!("bef Adv {} {}",self.reg_A,n);
                    if self.reg_A!=0 {
                        
                        let m = self.combo(n);
                        let a = self.reg_A; // as f64;
                        let b = 2u64.pow(m as u32); // 1<<m;//if m==0 {1} else {2<<m};
                        self.reg_A = ((a as f64)/(b as f64)) as u64;
                    }
                    println!("Adv {} {}",self.reg_A,n);
                }
            },
            Code::Bxl(n) => {
                {
                    self.reg_B ^= n as u64;
                }
            },
            Code::Bst(n) => {                    
                {
                    self.reg_B = self.combo(n) % 8;
                }
            },
            Code::Jnz(n) => {
                if self.reg_A != 0
                {
                    self.i = n as usize;
                    self.i-=1;
                }
            },
            Code::Bxc(n) => {
                {
                    self.reg_B = self.reg_B^self.reg_C;
                }
            },
            Code::Out(n) => {
                {
                    println!("{:?} {}",self.combo(n),n);

                    let c = (self.combo(n)%8) as u8;
                    self.res.push(c);
                }
            },
            Code::Bdv(n) => {
                if n == 0
                {
                    self.reg_B = self.reg_A/(1<<self.combo(n));                    
                }
            },
            Code::Cdv(n) => {
                if n == 0
                {
                    self.reg_C = self.reg_A/(1<<self.combo(n));
                }
            },
            _ => panic!("Unknown code"),
        }
        


    }

    fn ok1(&mut self) -> Vec<u8>
    {
        
        let len = self.prog.len();

        println!("{:?}",self.prog);
        println!("{:?}",self.reg_A);
        println!("{:?}",self.reg_B);
        println!("{:?}",self.reg_C);

        while self.i < len && self.i>=0
        {
            self.run();
            self.i+=1;
        }
 
        self.res.clone()
    }

    fn ok2(&self,a:Vec2,b:Vec2,p:Vec2) -> usize
    {               
        0
       }

    fn count1(&mut self)->String
    {        
        self.ok1().iter().map(|a| a.to_string()).collect::<Vec<String>>().join(",")
    }

    fn count2(&self)->usize
    {
        0        
    }

}

pub fn part1(data:&[String])->String
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
    let mut d = Data::new(&v);
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

//00011101
//    0111
//   11010 
/*
+If register C contains 9, the program 2,6 would set register B to 1.
+If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
 If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
 +If register B contains 29, the program 1,7 would set register B to 26.
 +If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
 */


 //3,1,6,4,1,3,7,3,1