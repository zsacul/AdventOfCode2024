use std::fmt::Debug;
use std::collections::HashMap;

static mut ID: i32 = 0;

pub struct CyclicList
{
    head   : i32,
    data   : HashMap<i32,Node>,   
    lookup : HashMap<i128,i32>,
}

#[derive(Debug)]
struct Node
{
    elem : i128,
    next : i32,
    prev : i32,
}

impl Node {
    fn new(elem: i128) -> Node {
        Node {
            elem,
            prev: 0,
            next: 0,
        }
    }
}

#[allow(unused)]
impl CyclicList
{
    pub fn new()->Self
    {
        CyclicList 
        { 
            head: 0,
            data: HashMap::new(),            
            lookup: HashMap::new(),
        }
    }

    fn next_id()->i32 {
        unsafe {
            ID+=1;
            ID
        }                
    }

    #[allow(unused)]
    pub fn len(&self)->usize {
        self.data.len()
    }

    #[allow(unused)]
    pub fn right(&mut self)
    {        
        if self.len()==0 { return; }
        self.head = self.data.get(&self.head).unwrap().next;
    }

    #[allow(unused)]
    pub fn left(&mut self)
    {
        if self.len()==0 { return; }
        self.head = self.data.get(&self.head).unwrap().prev;
    }

    #[allow(unused)]
    pub fn move_right_till_value(&mut self,value:i128)->usize
    {
        let mut res = 0;
        self.head = *self.lookup.get(&value).unwrap();

        while self.peek().unwrap()!=value {
            self.right();
            res+=1;
        }
        res
    }

    #[allow(unused)]
    pub fn move_left_till_value(&mut self,value:i128)->usize
    {
        let mut res = 0;
        self.head = *self.lookup.get(&value).unwrap();
        
        while self.peek().unwrap()!=value {
            self.left();
            res+=1;
        }
        res
    }

    pub fn peek(&self)->Option<i128>
    {
        if self.data.is_empty() 
        {
            None
        }
          else
        {
            Some(self.data.get(&self.head).unwrap().elem)
        }
    }

    #[allow(unused)]
    pub fn print(&self)
    {
        println!("  print: {}, head={}",self.data.len(),self.head);    
        let h = self.head;
        let mut ptr = self.head;

        if self.data.is_empty() 
        {
            println!("empty");
            return;
        }

        for i in 0..self.data.len() 
        {
            if ptr==h 
            {
                println!("{}=({}), ",i,self.data.get(&ptr).unwrap().elem);
            }
                else
            {
                println!("{}={}, ",i,self.data.get(&ptr).unwrap().elem);
            }
            ptr = self.data.get(&ptr).unwrap().next;
        }
    }

    pub fn push_right(&mut self, elem: i128) {
        let mut new_elem = Node::new(elem);
        let next_id = CyclicList::next_id();

        if !self.data.is_empty()
        {
            if self.data.get(&self.head).unwrap().next!=self.head
            {
                let n = self.data.get(&self.head).unwrap().next;
                self.data.get_mut(&self.head).unwrap().next = next_id;
                self.data.get_mut(&n        ).unwrap().prev = next_id;
                new_elem.prev = self.head;
                new_elem.next = n;
                self.data.insert(next_id, new_elem);
            }
                else
            {
                new_elem.prev = self.head;
                new_elem.next = self.head;
                self.data.get_mut(&self.head).unwrap().next = next_id;
                self.data.get_mut(&self.head).unwrap().prev = next_id;
                self.data.insert(next_id, new_elem);
            }
        } 
          else 
        {
            self.head = next_id;
            new_elem.prev = next_id;
            new_elem.next = next_id;
            self.data.insert(self.head, new_elem);
        }

        self.lookup.insert(elem, next_id);
        self.right();
    }    

    #[allow(unused)]
    pub fn push_left(&mut self, elem: i128) {
        self.left();
        self.push_right(elem);
    }

    pub fn pop(&mut self) -> Option<i128> {
      
        if self.data.is_empty()
        {
            return None;
        }

        let h   = self.head;
        let res = self.data.get(&h).unwrap().elem;
        let n = self.data.get_mut(&h).unwrap().next;
        let p = self.data.get_mut(&h).unwrap().prev;
        
        self.lookup.remove(&res);

        self.data.get_mut(&n).unwrap().prev = p;
        self.data.get_mut(&p).unwrap().next = n;

        self.data.remove(&h);
        self.head = p;
        
        Some(res)
    }    
}

