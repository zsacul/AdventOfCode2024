#[derive(Hash, Eq, PartialEq,  Debug, Copy, Clone,PartialOrd, Ord)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    #[allow(unused)]
    pub fn new(x:i64,y:i64)->Self 
    {
        Self { x      , y      } 
    }

    #[allow(unused)]
    pub fn newu(x:usize,y:usize)->Self 
    {
        Self { x: x as i64, y: y as i64  } 
    }

    #[allow(unused)]
    pub fn newv(v:&Vec2)->Self 
    { 
        Self { x: v.x , y: v.y } 
    }

    #[allow(unused)]
    pub fn zero()->Self 
    { 
        Self { x:   0 , y:0    } 
    }

    #[allow(unused)]
    pub fn north()->Self 
    { 
        Self { x:   0 , y:-1    } 
    }
    
    #[allow(unused)]
    pub fn west()->Self 
    { 
        Self { x:  -1 , y: 0    } 
    }

    #[allow(unused)]
    pub fn south()->Self 
    { 
        Self { x:   0 , y: 1    } 
    }
    
    #[allow(unused)]
    pub fn east()->Self 
    { 
        Self { x:   1 , y: 0    } 
    }

    #[allow(unused)]
    pub fn offset(&mut self,v:&Vec2)
    {
        self.x+=v.x;
        self.y+=v.y;
    }

    #[allow(unused)]
    pub fn offset2(&mut self,x:i64,y:i64)
    {
        self.x+=x;
        self.y+=y;
    }

    #[allow(unused)]
    pub fn around4(&self)->Vec<Vec2>
    {
         vec![
            Vec2::new(self.x+1,self.y  ),
            Vec2::new(self.x  ,self.y+1),
            Vec2::new(self.x-1,self.y  ),
            Vec2::new(self.x  ,self.y-1)
         ]
    }

    #[allow(unused)]
    pub fn around8(&self)->Vec<Vec2>
    {
         vec![
            Vec2::new(self.x-1,self.y-1), Vec2::new(self.x  ,self.y-1), Vec2::new(self.x+1,self.y-1),
            Vec2::new(self.x-1,self.y  ),                               Vec2::new(self.x+1,self.y  ),
            Vec2::new(self.x-1,self.y+1), Vec2::new(self.x  ,self.y+1), Vec2::new(self.x+1,self.y+1),            
         ]
    }

    #[allow(unused)]
    pub fn around9(&self)->Vec<Vec2>
    {
         vec![
            Vec2::new(self.x-1,self.y-1), Vec2::new(self.x  ,self.y-1), Vec2::new(self.x+1,self.y-1),
            Vec2::new(self.x-1,self.y  ), Vec2::new(self.x  ,self.y  ), Vec2::new(self.x+1,self.y  ),
            Vec2::new(self.x-1,self.y+1), Vec2::new(self.x  ,self.y+1), Vec2::new(self.x+1,self.y+1),            
         ]
    }

    #[allow(unused)]
    pub fn r(&self)->Vec2
    {
        Vec2::new(self.x+1,self.y  )
    }

    #[allow(unused)]
    pub fn l(&self)->Vec2
    {
        Vec2::new(self.x-1,self.y  )
    }

    #[allow(unused)]
    pub fn u(&self)->Vec2
    {
        Vec2::new(self.x  ,self.y-1)
    }

    #[allow(unused)]
    pub fn d(&self)->Vec2
    {
        Vec2::new(self.x  ,self.y+1)
    }

    #[allow(unused)]
    pub fn distance2(&self,x:i64,y:i64)->i64
    {
        (self.x-x).abs() +
        (self.y-y).abs()
    }

    #[allow(unused)]
    pub fn distance2v(&self,p:&Vec2)->i64
    {
        (self.x-p.x).abs() +
        (self.y-p.y).abs()
    }

    #[allow(unused)]
    pub fn add(&self,x:i64,y:i64)->Vec2
    {
        Vec2
        {
            x : self.x + x,
            y : self.y + y
        }
    }

    #[allow(unused)]
    pub fn addv(&self,p:Vec2)->Vec2
    {
        Vec2
        {
            x : self.x + p.x,
            y : self.y + p.y
        }
    }

    #[allow(unused)]
    pub fn sub(&self,x:i64,y:i64)->Vec2
    {
        Vec2
        {
            x : self.x - x,
            y : self.y - y
        }
    }

    #[allow(unused)]
    pub fn subv(&self,p:Vec2)->Vec2
    {
        Vec2
        {
            x : self.x - p.x,
            y : self.y - p.y
        }
    }

    #[allow(unused)]
    pub fn signum(&self)->Vec2
    {
        Vec2
        {
            x : self.x.signum(),
            y : self.y.signum()
        }
    }

    #[allow(unused)]
	fn cross(&self,o : Vec2)->i64
	{
		self.x*o.y - self.y*o.x
	}

    #[allow(unused)]
	fn is_cross(&self,o : Vec2)->bool
	{
		self.x * o.y >= self.y*o.x
	}

    #[allow(unused)]
	fn dot(&self,o : Vec2)->i64
	{
		self.x*o.x + self.y*o.y
	}

}

