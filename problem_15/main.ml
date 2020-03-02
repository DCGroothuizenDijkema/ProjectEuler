
(* 
Project Euler Problem 15: Lattice paths

D. C. Groothuizen Dijkema - March, 2020
*)

let binomial n k=
  let rec loop itr c=
    if itr==(k+1) then c
    else loop (succ itr) (c*(n+1-itr)/itr)
  in loop 1 1

let run n=
  binomial (2*n) n

let solution=run 20
let ()=print_endline (string_of_int solution)
