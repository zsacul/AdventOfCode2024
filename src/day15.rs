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


    fn print_hash(&self)
    {
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let p = Vec2::new(x as i64,y as i64);                        
                print!("{}", *self.hash.get(&p).unwrap_or(&'.'));
            }
            println!();
        }
        println!();
    }

    fn get(&self,p:Vec2)->char
    {
        *self.hash.get(&p).unwrap_or(&'#')
    }

    fn set(&mut self,p:Vec2,c:char)
    {
        self.hash.insert(p,c);
    }

    fn count1(&mut self)->usize
    {
        for m in self.moves.chars()
        {
            let offset = Data::get_offset(m);
            let new_pos = self.pos.addv(offset);
            let new_pos_char = self.get(new_pos);

            if new_pos_char == '#'
            {
                continue;
            }

            
            let mut last_pos= new_pos;
            let mut move_ok = false;

            if  self.get(last_pos) == '.'
            {             
                move_ok = true;
            }

            while self.get(last_pos) == 'O'
            {
                last_pos = last_pos.addv(offset);
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
        }

        self.hash.iter()
                 .map(|(p,c)| if c==&'O' { p.x + p.y*100 } else { 0 })
                 .sum::<i64>() as usize
    }

    fn transform(&self)->HashMap<Vec2,char>
    {
        let mut hash = HashMap::new();

        for (p,c) in self.hash.iter()
        {
            let p1 = Vec2::new(p.x*2  ,p.y);
            let p2 = Vec2::new(p.x*2+1,p.y);

            match c
            {
                '#' => { hash.insert(p1,'#'); hash.insert(p2,'#'); },
                '.' => { hash.insert(p1,'.'); hash.insert(p2,'.'); },
                'O' => { hash.insert(p1,'['); hash.insert(p2,']'); },
                '@' => { hash.insert(p1,'@'); hash.insert(p2,'.'); },
                _   => panic!("transform")
                
            }
        }
        
        hash        
    }

    fn is_box(&self,p:Vec2)->bool
    {
        self.get(p) == '[' || self.get(p) == ']'
    }

    fn move_ok(&self,p:Vec2,dir:char)->bool
    {
        match dir
        {
            '^' => self.get(p.add( 0,-1)) == '.' && self.get(p.add(1,-1)) == '.',
            'v' => self.get(p.add( 0, 1)) == '.' && self.get(p.add(1, 1)) == '.',
            '<' => self.get(p.add(-1, 0)) == '.',
            '>' => self.get(p.add( 2, 0)) == '.',
            _   => false
            
        }
    }

    fn move_do(&mut self,p:Vec2,dir:char)
    {
            match dir
            {
                '^' => {
                    self.set(p.add(0,-1),'['); 
                    self.set(p.add(1,-1),']');
                    self.set(p.add(0, 0),'.'); 
                    self.set(p.add(1, 0),'.');
                },
                'v' =>
                {
                    self.set(p.add(0, 1),'['); 
                    self.set(p.add(1, 1),']');
                    self.set(p.add(0, 0),'.'); 
                    self.set(p.add(1, 0),'.');
                },
                '<' => 
                {
                    self.set(p.add(-1, 0),'['); 
                    self.set(p.add( 0, 0),']');
                    self.set(p.add( 1, 0),'.');                 
                },
                '>' => 
                {
                    self.set(p.add( 2, 0),']'); 
                    self.set(p.add( 1, 0),'[');
                    self.set(p.add( 0, 0),'.');                 
                },                       
                _   => panic!("move_do")
            }
            
    }
    

    fn push(&mut self,moved:&Vec<Vec2>,dir:char,offs:Vec2)->(bool,Vec<Vec2>)
    {       
        if moved.iter().all(|f|self.move_ok(*f,dir)) && !moved.is_empty()
        {
            return (true,moved.clone());
        }

        let mut new_moved = vec![]; 
        
        for m in moved.iter()
        {
            let np =  m.addv(offs);
            let npr = np.addv(Vec2::new( 1,0));
            let npl = np.addv(Vec2::new(-1,0));

            if self.get(np) == '#' || self.get(npr) == '#'
            {
                return (false,new_moved.clone());
            }

            if dir=='>'
            {
                if self.get(npr) == '['
                {
                    new_moved.push(npr);
                }
            }
            else if dir=='<'
            {
                if self.get(np) == ']'
                {
                    new_moved.push(npl);
                }
            }
            else 
            {
                if self.get(np) == '[' 
                {
                    new_moved.push(np);
                }
                if self.get(np) == ']'
                {                    
                    new_moved.push(npl);
                }
                if self.get(npr) == '[' 
                {
                    new_moved.push(npr);
                }
                if self.get(npr) == ']'
                {                    
                    new_moved.push(np);
                }

            }
        }
        new_moved.sort();
        new_moved.dedup();

        let np = self.push( &new_moved, dir, offs);

        if np.0
        {
            let mut res_moved = vec![];
            for m in np.1.iter()
            {
                res_moved.push(*m);
            }
            for m in moved.iter()
            {
                res_moved.push(*m);
            }

            return (true,res_moved);
        }

        (false,moved.clone())
    }
    

    fn count2(&mut self)->usize
    {
        self.hash = self.transform();
        self.pos = self.pos.mulv(Vec2::new(2,1));
        self.dx*=2;
 
        let mut step=0;

        let mm = self.moves.clone();

        for m in mm.chars()
        {
            let offset = Data::get_offset(m);
            let new_pos = self.pos.addv(offset);
            let new_pos_char = self.get(new_pos);            

            if new_pos_char == '#'
            {
                continue;
            }

            let mut last_pos = new_pos;
            let mut move_ok = false;

            if  self.get(last_pos) == '.'
            {             
                move_ok = true;
            }

            if self.is_box(last_pos)
            {                
                if self.get(last_pos)==']'
                {
                    last_pos = last_pos.addv(Vec2::new(-1,0));
                }
                let mm = vec![last_pos];
                let r = self.push(&mm,m,offset);

                if r.0
                {
                    for b in r.1.iter()
                    {
                        self.move_do(*b, m);
                        move_ok = true;
                    }
                }
            }
            
            if move_ok
            {
                self.hash.insert(self.pos,'.');
                self.hash.insert(new_pos,'@');
                self.pos = new_pos;                            
            }

            step+=1;
        }

        self.hash.iter()
                 .map(|(p,c)| if c==&'[' { p.x + p.y*100 } else { 0 })
                 .sum::<i64>() as usize
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
    assert_eq!(part1(&v),2028);
}



#[test]
fn test0()
{
    let v = vec![
        "##########".to_string(),
        "#..O..O.O#".to_string(),
        "#......O.#".to_string(),
        "#.OO..O.O#".to_string(),
        "#..O@..O.#".to_string(),
        "#O#..O...#".to_string(),
        "#O..O..O.#".to_string(),
        "#.OO.O.OO#".to_string(),
        "#....O...#".to_string(),
        "##########".to_string(),
        "".to_string(),
        "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^".to_string(),
        "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v".to_string(),
        "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<".to_string(),
        "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^".to_string(),
        "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><".to_string(),
        "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^".to_string(),
        ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^".to_string(),
        "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>".to_string(),
        "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>".to_string(),
        "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^".to_string(),
    ];
    assert_eq!(part1(&v),10092);
}


#[test]
fn test2()
{
    let v = vec![
        "#######".to_string(),
        "#...#.#".to_string(),
        "#.....#".to_string(),
        "#..OO@#".to_string(),
        "#..O..#".to_string(),
        "#.....#".to_string(),
        "#######".to_string(),
        "".to_string(),
        "<vv<<^^<<^^".to_string(),
    ];
    assert_eq!(part2(&v),618);
}


#[test]
fn test3()
{
    let v = vec![
        "##########".to_string(),
        "#..O..O.O#".to_string(),
        "#......O.#".to_string(),
        "#.OO..O.O#".to_string(),
        "#..O@..O.#".to_string(),
        "#O#..O...#".to_string(),
        "#O..O..O.#".to_string(),
        "#.OO.O.OO#".to_string(),
        "#....O...#".to_string(),
        "##########".to_string(),
        "".to_string(),
        "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^".to_string(),
        "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v".to_string(),
        "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<".to_string(),
        "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^".to_string(),
        "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><".to_string(),
        "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^".to_string(),
        ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^".to_string(),
        "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>".to_string(),
        "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>".to_string(),
        "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^".to_string(),
    ];
    assert_eq!(part2(&v),9021);
}



#[test]
fn test4()
{
    let v = vec![
        "##########".to_string(),
        "#......O.#".to_string(),
        "#...OO..O#".to_string(),
        "#.@.OO.O.#".to_string(),
        "#O#.OO...#".to_string(),
        "#O..O..O.#".to_string(),
        "##########".to_string(),
        "".to_string(),
        ">>>>>>>>".to_string(),
    ];
    assert_eq!(part2(&v),4434);
}


