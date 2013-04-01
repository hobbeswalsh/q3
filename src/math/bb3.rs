/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/bb3.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A 3D bounding box.
*/

use math::vec3::Vec3;
#[path = "./vec3.rs"]
mod vec3;

pub struct BB3 /* TODO: Check math on this shit. */
{
  top_left: Vec3<f32>,
  bottom_right: Vec3<f32>,
}

impl BB3
{
  pub fn new( t_left: Vec3<f32>, b_right: Vec3<f32>) -> BB3
  {
    BB3{  top_left: t_left, bottom_right: b_right }
  }

  pub fn zero() -> BB3
  {
    BB3{  top_left: Vec3::zero::<f32>(), bottom_right: Vec3::zero::<f32>() } 
  }

  pub fn center(&self) -> Vec3<f32>
  { Vec3::new::<f32>( (self.bottom_right.x - self.top_left.x) / 2.0,
                      (self.top_left.y - self.bottom_right.y) / 2.0,
                      (self.top_left.z - self.bottom_right.z) / 2.0 )
  }

  pub fn center_with_offset(&self, offset: Vec3<f32>) -> Vec3<f32>
  { Vec3::new::<f32>( ((self.bottom_right.x - self.top_left.x) / 2.0) + offset.x,
                      ((self.top_left.y - self.bottom_right.y) / 2.0) + offset.y,
                      ((self.top_left.z - self.bottom_right.z) / 2.0) + offset.z )
  }
}
