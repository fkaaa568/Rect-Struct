#[derive(Debug)]
struct length{
      length_of_rectangle:i32,
      width_of_rectangle:i32,
}

impl length{
      fn calculate_length(self) -> i32{
      let area:i32 = (self.length_of_rectangle)*(self.width_of_rectangle);
      area
}
}

fn main(){
      let length01 = length{length_of_rectangle:3,width_of_rectangle:8};
      println!("The length of rectangle {} ",length01.calculate_length());
}
