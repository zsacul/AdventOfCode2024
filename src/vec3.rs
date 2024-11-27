use std::ops::{Add,Sub,Mul};
use super::vec3f::Vec3f;   

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Vec3 
{
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3 {
    pub fn new(x:i64,y:i64,z:i64)->Vec3
    {
        Vec3
        {
            x,y,z
        }
    }

    #[allow(unused)]
    pub const ZERO: Vec3 = Vec3 {
        x: 0,
        y: 0,
        z: 0,
    };

    #[allow(unused)]
    pub const ONE: Vec3 = Vec3 {
        x: 1,
        y: 1,
        z: 1,
    };        
    
    #[allow(unused)]
    pub fn to_vec3f(self)->Vec3f
    {
        Vec3f
        {
            x: self.x as f64,
            y: self.y as f64,
            z: self.z as f64,
        }
    }

    #[allow(unused)]
    pub fn len(&self)->f64
    {
        ((self.x*self.x + self.y*self.y + self.z*self.z) as f64).sqrt()
    }

    #[allow(unused)]
    pub fn normalize(&self)->Self
    {
        let l = self.len();
        if l>0.0
        {
            return Self {
                x : (self.x as f64 / l) as i64,
                y : (self.y as f64 / l) as i64,
                z : (self.z as f64 / l) as i64,    
            }
        }
        *self
    }

    #[allow(unused)]
    pub fn dot(a: &Vec3, b: &Vec3)->i64 
    {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    #[allow(unused)]
    pub fn cross(a: &Vec3, b: &Vec3)->Self
    {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }


    #[allow(unused)]
    pub fn l(&self)->Vec3
    {
        Vec3::new(self.x-1,self.y  ,self.z  )
    }

    #[allow(unused)]
    pub fn r(&self)->Vec3
    {
        Vec3::new(self.x+1,self.y  ,self.z  )
    }

    #[allow(unused)]
    pub fn u(&self)->Vec3
    {
        Vec3::new(self.x  ,self.y-1,self.z  )
    }

    #[allow(unused)]
    pub fn d(&self)->Vec3
    {
        Vec3::new(self.x  ,self.y+1,self.z  )
    }    

    #[allow(unused)]
    pub fn f(&self)->Vec3
    {
        Vec3::new(self.x  ,self.y  ,self.z-1)
    }    

    #[allow(unused)]
    pub fn b(&self)->Vec3
    {
        Vec3::new(self.x  ,self.y  ,self.z+1)
    }    

    #[allow(unused)]
    pub fn around6(&self)->Vec<Vec3>
    {
         vec![
            self.u(),
            self.l(),
            self.b(),
            self.f(),
            self.r(),
            self.d(),
         ]
    }

    #[allow(unused)]
    pub fn around27(&self)->Vec<Vec3>
    {
         vec![           
            Vec3::new(self.x-1,self.y-1,self.z+1), Vec3::new(self.x  ,self.y-1,self.z+1), Vec3::new(self.x+1,self.y-1,self.z+1),
            Vec3::new(self.x-1,self.y  ,self.z+1), Vec3::new(self.x  ,self.y  ,self.z+1), Vec3::new(self.x+1,self.y  ,self.z+1),
            Vec3::new(self.x-1,self.y+1,self.z+1), Vec3::new(self.x  ,self.y+1,self.z+1), Vec3::new(self.x+1,self.y+1,self.z+1),            

            Vec3::new(self.x-1,self.y-1,self.z  ), Vec3::new(self.x  ,self.y-1,self.z  ), Vec3::new(self.x+1,self.y-1,self.z  ),
            Vec3::new(self.x-1,self.y  ,self.z  ), Vec3::new(self.x  ,self.y  ,self.z  ), Vec3::new(self.x+1,self.y  ,self.z  ),
            Vec3::new(self.x-1,self.y+1,self.z  ), Vec3::new(self.x  ,self.y+1,self.z  ), Vec3::new(self.x+1,self.y+1,self.z  ),            

            Vec3::new(self.x-1,self.y-1,self.z-1), Vec3::new(self.x  ,self.y-1,self.z-1), Vec3::new(self.x+1,self.y-1,self.z-1),
            Vec3::new(self.x-1,self.y  ,self.z-1), Vec3::new(self.x  ,self.y  ,self.z-1), Vec3::new(self.x+1,self.y  ,self.z-1),
            Vec3::new(self.x-1,self.y+1,self.z-1), Vec3::new(self.x  ,self.y+1,self.z-1), Vec3::new(self.x+1,self.y+1,self.z-1),
         ]
    }

    #[allow(unused)]
    pub fn around26(&self)->Vec<Vec3>
    {
         vec![           
            Vec3::new(self.x-1,self.y-1,self.z+1), Vec3::new(self.x  ,self.y-1,self.z+1), Vec3::new(self.x+1,self.y-1,self.z+1),
            Vec3::new(self.x-1,self.y  ,self.z+1), Vec3::new(self.x  ,self.y  ,self.z+1), Vec3::new(self.x+1,self.y  ,self.z+1),
            Vec3::new(self.x-1,self.y+1,self.z+1), Vec3::new(self.x  ,self.y+1,self.z+1), Vec3::new(self.x+1,self.y+1,self.z+1),            

            Vec3::new(self.x-1,self.y-1,self.z  ), Vec3::new(self.x  ,self.y-1,self.z  ), Vec3::new(self.x+1,self.y-1,self.z  ),
            Vec3::new(self.x-1,self.y  ,self.z  ),                                        Vec3::new(self.x+1,self.y  ,self.z  ),
            Vec3::new(self.x-1,self.y+1,self.z  ), Vec3::new(self.x  ,self.y+1,self.z  ), Vec3::new(self.x+1,self.y+1,self.z  ),            

            Vec3::new(self.x-1,self.y-1,self.z-1), Vec3::new(self.x  ,self.y-1,self.z-1), Vec3::new(self.x+1,self.y-1,self.z-1),
            Vec3::new(self.x-1,self.y  ,self.z-1), Vec3::new(self.x  ,self.y  ,self.z-1), Vec3::new(self.x+1,self.y  ,self.z-1),
            Vec3::new(self.x-1,self.y+1,self.z-1), Vec3::new(self.x  ,self.y+1,self.z-1), Vec3::new(self.x+1,self.y+1,self.z-1),
         ]
    }

    #[allow(unused)]
    pub fn add(&self,other:&Vec3)->Vec3
    {
        Vec3
        {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    #[allow(unused)]
    pub fn sub(&self,other:&Vec3)->Self
    {
        Vec3
        {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }






}

impl Add for Vec3 
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<i64> for Vec3 
{
    type Output = Self;

    fn mul(self, n: i64) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }
}

