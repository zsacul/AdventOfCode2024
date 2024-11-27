use std::ops::{Add,Sub,Mul,Div,Neg};
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3f {
    pub fn new(x:f64,y:f64,z:f64)->Self
    {
        Self
        {
            x,y,z
        }
    }

    pub const ZERO: Vec3f = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Vec3f = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };        
    
    #[allow(unused)]
    pub fn len(&self)->f64
    {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn length(v:&Vec3f)->f64
    {
        (v.x*v.x + v.y*v.y + v.z*v.z).sqrt()
    }

    #[allow(unused)]
    pub fn normalize(&self)->Self
    {
        let l = self.len();
        if l>0.0
        {
            return Self {
                x : self.x/l,
                y : self.y/l,
                z : self.z/l, 
            }
        }
        *self
    }

    #[allow(unused)]
    pub fn dot(a: &Vec3f, b: &Vec3f)->f64 
    {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    #[allow(unused)]
    pub fn cross(a: &Vec3f, b: &Vec3f)->Self
    {
        Vec3f {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    #[allow(unused)]
    pub fn add(&self,other:&Vec3f)->Self
    {
        Self
        {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    #[allow(unused)]
    pub fn sub(&self,other:&Vec3f)->Self
    {
        Self
        {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    #[allow(unused)]
    fn tangents(n:Vec3f)->(Vec3f,Vec3f)
    {	
        let mut up = Vec3f::new(0.0,1.0,0.0);
        let mut n = n.normalize();
          
        let mut r  = Self::cross(&up,&n);
        r  = r.normalize();
    
        if (Self::length(&r)-1.0).abs()<0.0001
        {
          up = Self::cross(&n,&r);
          up = up.normalize();
        }
          else
        {
           up = Vec3f::new( 0.0, 0.0, if n.y < 0.0 {1.0} else {-1.0} );
           r  = Self::cross(&up,&n);
           r  = r.normalize();
        }

        (up,r)
    }

    #[allow(unused)]
    fn project_to_plane(point:Vec3f,n:Vec3f)->Vec3f
    {
        let res = point -n*Self::dot(&n,&point);
    
        //Vec3f u,v;
        let (u,v) = Self::tangents(n);
        Vec3f::new( Self::dot(&n,&res), Self::dot(&u,&res), Self::dot(&v,&res) )
        //return ToLocal(res,n,u,v);
    }
        
    #[allow(unused)]
    fn closest_point_on_line(a:Vec3f,b:Vec3f,v_point:Vec3f)->Vec3f
    {
      //First, we create a vector from our end point vA to our point vPoint
      let vector1 = v_point - a;
    
      //Now we create a normalized direction vector from end point vA to end point vB
      let vector2 = b - a;

      //Now we use the distance formula to find the distance of the line segment
      let d : f64 = Self::length(&vector2);
      let vector2 = Self::normalize(&vector2);

      //Using the dot product, we project the vVector1 onto the vector vVector2. This essentially
      //gives us the distance of our projected vector from vA
      let t = Self::dot(&vector2, &vector1);
    
      //If our projected distance from vA, "t",  is greater than or equal to 0, it must be closest to the end point
      //vA.  So we return this end point.
      if t<=0.0 { return a;}
    
      //If our projected distance from vA, "t", is greater than or equal to the magnitude or distance of the line
      //segment, it must be closest to the end point vB, so we return vB.
      if t >=d { return b; }
    
      //Here we create a vector that is of length T and in the direction of vVector2
      //To find the closest point on the line, we just add vVector3 to the original end point vA
      a + vector2*t
    }

    pub fn plane_line_intersection(plane_p:Vec3f, plane_n:Vec3f, pos:Vec3f, dir:Vec3f)->(Vec3f,f64)
    {
        let a = Self::dot(&(plane_p-pos),&plane_n);
        let b = Self::dot(          &dir,&plane_n);
        let t = a/b;
        let p = pos + dir*t;
        (p,t)
    }

    pub fn plane_line_intersection2(plane_p:Vec3f, plane_n:Vec3f, line_start:Vec3f, line_end:Vec3f)->(Vec3f,f64)
    {
        let plane_n =  plane_n.normalize();
        let plane_d = - plane_n.x*plane_p.x - plane_n.y*plane_p.y - plane_n.z*plane_p.z;
        let ad = line_start.x*plane_n.x + line_start.y*plane_n.y + line_start.z*plane_n.z;
        let bd =   line_end.x*plane_n.x +   line_end.y*plane_n.y +   line_end.z*plane_n.z;
        let t = (-plane_d - ad) / (bd - ad);
        let line_start_to_end = line_end - line_start;
        let line_to_intersect = line_start_to_end * t;
        (line_start + line_to_intersect,t)
    }

    pub fn plane_from_three_points(p1:Vec3f,p2:Vec3f,p3:Vec3f)->(Vec3f,Vec3f)
    {
        let v1 = p2 - p1;
        let v2 = p3 - p1;
        let cp = Vec3f::cross(&v1,&v2);
        let a = cp.x;
        let b = cp.y;
        let c = cp.z;
        //let d = -(cp.x*p3.x + cp.y*p3.y + cp.z*p3.z);
        let plane = Vec3f::new(a,b,c);
        let line = Vec3f::new(p1.x,p1.y,p1.z);
        (plane,line)
    }
}

impl Add for Vec3f
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

impl Sub for Vec3f
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

impl Mul<f64> for Vec3f
{
    type Output = Self;

    fn mul(self, n: f64) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }
}

impl Div<f64> for Vec3f
{
    type Output = Self;

    fn div(self, n: f64) -> Self {
        Self {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }
}

// It also implements unary operators like - a where a is of
// type Vec3 or &Vec3.
macro_rules! impl_unary_operations {
    // $VectorType is something like `Vec3`
    // $Operation is something like `Neg`
    // $op_fn is something like `neg`
    // $op_symbol is something like `-`
    ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt) => {
  
      // Implement the unary operator for references
      impl<'a> $Operation for &'a $VectorType {
        type Output = $VectorType;
  
        fn $op_fn(self) -> Vec3f {
          $VectorType {
            x: $op_symbol self.x,
            y: $op_symbol self.y,
            z: $op_symbol self.z,
          }
        }
      }
  
      // Have the operator on values forward through to the implementation
      // above
      impl $Operation for $VectorType {
        type Output = $VectorType;
  
        #[inline]
        fn $op_fn(self) -> Vec3f {
          $op_symbol &self
        }
      }
    };
  }

impl_unary_operations!(Vec3f Neg neg -);


#[test]
fn plane_test()
{
/*
    x1 = -1 y1 =  w z1 = 1 
    x2 =  0 y2 = -3 z2 = 2 
    x3 =  1 y3 =  1 z3 = -4 
    Output: equation of plane is 26 x + 7 y + 9 z + 3 = 0.
    Input: 
    x1 = 2, y1 =  1, z1 = -1, 1 
    x2 = 0, y2 = -2, z2 =  0 
    x3 = 1, y3 = -1, z3 =  2 
    Output: equation of plane is -7 x + 5 y + 1 z + 10 = 0.
 */
}

