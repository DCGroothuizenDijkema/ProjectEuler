
let rec collatz n itr=
  if n==1 then itr
  else if n mod 2==0 then collatz (n/2) (succ itr)
  else collatz (3*n+1) (succ itr)

let rec loop itr=
  if itr=25 then ()
  else let num=(collatz itr 1) in
    print_endline (string_of_int num)

(* let ()=print_endline (string_of_int (collatz 13 1)) *)

let ()=loop 1