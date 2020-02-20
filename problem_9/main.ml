
(* 
Project Euler Problem 9: Special Pythagorean triplet

D. C. Groothuizen Dijkema - February, 2020
*)

let rec run a b=
  let c= 1000-b-a in
    if a*a+b*b==c*c then a*b*c
    else if b<(1000-a)/2 then run a (b+1)
    else if a<333 then run (a+1) (a+2)
    else failwith "They lied to us."

let prod=run 1 2 
let ()=print_endline (string_of_int prod)
