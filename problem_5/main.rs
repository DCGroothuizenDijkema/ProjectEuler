
//
// Project Euler Problem 5: Smallest multiple
//
// D. C. Groothuizen Dijkema - December, 2019
//

fn run()
{
  let mut itr: i32=2520;
  let divs: [i32;7]=[5,3_i32.pow(2),11,13,2_i32.pow(4),17,19];

  let val: i32=loop
  {
    let mut lcm: bool=true;
    for div in divs.iter()
    {
      if itr%div!=0
      {
        lcm=false;
        break
      }
    }
    if lcm
    {
      break itr;
    }
    itr+=2520;
  };
  println!("The lcm of all numbers from 1 to 20 is {}",val);
}

fn main()
{
  let now: std::time::Instant=std::time::Instant::now();
  run();
  println!("Elapsed time: {}ns", now.elapsed().as_nanos());
  
  let now: std::time::Instant=std::time::Instant::now();
  let val: i32=5*9*11*13*16*17*19;
  println!("The lcm of all numbers from 1 to 20 is {}",val);
  println!("Elapsed time: {}ns", now.elapsed().as_nanos());
}
