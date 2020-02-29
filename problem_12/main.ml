
(* 
Project Euler Problem 12: Highly divisible triangular number

D. C. Groothuizen Dijkema - February, 2020
*)

let triangle n=
  n*(n+1)/2

let num_divisors n=
  let upper=int_of_float (floor(sqrt(float_of_int n))) in
    let rec count_divisors m init=
      if m==1||m==2 then m
      else let rec loop itr=
        if itr>upper then 2
        else if m mod itr==0 then
          let rec divide o cnt=
            if o mod itr==0 then divide (o/itr) (succ cnt)
            else (o,cnt)
          in 
          let (out,cnt)=divide m 1 in
            (count_divisors out (itr+1))*cnt
        else loop (itr+1)
      in loop init
    in count_divisors n 2

let run=
  let rec loop itr=
    let t=triangle itr in
      if itr mod 2==0 then 
        if (num_divisors (itr/2))*(num_divisors (succ itr)) > 500 then t
        else loop (succ itr)
      else
        if (num_divisors itr)*(num_divisors ((succ itr)/2)) > 500 then t
        else loop (succ itr)
  in loop 3

let a=run
let ()=print_endline (string_of_int a)
