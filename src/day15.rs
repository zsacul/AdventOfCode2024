use std::collections::HashMap;
use std::collections::HashSet;
use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Data {
      hash: HashMap<Vec2,char>,   
      moves: String,
      dx:usize,
      dy:usize,
      pos:Vec2,
}

impl Data {
    fn new(input: &[String]) -> Self {
        let sections: Vec<&[String]> = input.split(|line| line.is_empty()).collect();          

        let hash = tools::get_hash_table(sections[0]);
        let moves = sections[1].join("");

        let pos : Vec2 = *hash.clone()
                        .iter()
                        .find(|c|c.1==&'@')
                        .unwrap().0;
        
        Data {
            hash,
            moves,
            dx : sections[0][0].len(),
            dy : sections[0].len(),
            pos
        }
    }

    fn get_offset(c:char)->Vec2
    {
        match c
        {
            '^' => Vec2::new(0,-1),
            'v' => Vec2::new(0,1),
            '<' => Vec2::new(-1,0),
            '>' => Vec2::new(1,0),
            _   => Vec2::new(0,0)
        }
    }

    fn get_hash(&self)->HashMap<Vec2,char>
    {
        self.hash.clone()
        //let mut hash: HashMap<Vec2,usize> = HashMap::new();
        
        //for r in self.hash.iter()
        //{         
          //  *hash.entry(r.p).or_insert(0) += 1;
        //}        
        
        
    }

    fn print_hash(&self)
    {
        let hash = self.get_hash();

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let p = Vec2::new(x as i64,y as i64);                        
                print!("{}", *hash.get(&p).unwrap_or(&'.'));
            }
            println!();
        }
        println!();
    }


    fn ok1(&self,a:Vec2,b:Vec2,p:Vec2) -> usize
    {
        0
    }

    fn ok2(&self,a:Vec2,b:Vec2,p:Vec2) -> usize
    {               
        0
    }


    fn get(&self,p:Vec2)->char
    {
        *self.hash.get(&p).unwrap_or(&'#')
    }

    fn count1(&mut self)->usize
    {
      //  println!(
      //      "dx:{} dy:{}",
      //      self.dx,
      //      self.dy
      //  );
      //  println!("pos: {:?}",self.pos);
      //  self.print_hash();

        let mut step=0;

        for m in self.moves.chars()
        {
            let offset = Data::get_offset(m);
            let new_pos = self.pos.addv(offset);
            let new_pos_char = self.get(new_pos);

            if new_pos_char == '#'
            {
                continue;
            }

            let mut moves = 0;
            let mut last_pos= new_pos;
            let mut move_ok = false;

            if  self.get(last_pos) == '.'
            {             
                move_ok = true;
            }

            while self.get(last_pos) == 'O'
            {
                last_pos = last_pos.addv(offset);
                moves+=1;
            }

            if  self.get(last_pos) == '.'
            {
                self.hash.insert(last_pos,'O');
                move_ok = true;
            }
            
            if move_ok
            {
                self.hash.insert(self.pos,'.');
                self.hash.insert(new_pos,'@');
                self.pos = new_pos;                            
            }

            //self.print_hash();
            step+=1;
            //println!("step: {}",step);
        }

//        println!("moves: {}",self.moves);

        self.get_hash().iter()
                 .map(|(p,c)| if c==&'O' { p.x + p.y*100 } else { 0 })
                 .sum::<i64>() as usize
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
    println!("Day15");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "########".to_string(),
        "#..O.O.#".to_string(),
        "##@.O..#".to_string(),
        "#...O..#".to_string(),
        "#.#.O..#".to_string(),
        "#...O..#".to_string(),
        "#......#".to_string(),
        "########".to_string(),
        "".to_string(),
        "<^^>>>vv<v>>v<<".to_string(),
    ];
    assert_eq!(part1(&v),10092);
}
