open Base;;

let x = Caml.read_int () in
let answer =
  let is_prime n =
    let rec loop i =
      if i * i > n then true else if n % i = 0 then false else loop (i + 1)
    in
    loop 2
  in
  let rec loop i = if is_prime i then i else loop (i + 1) in
  loop x
in
answer |> Int.to_string |> Stdio.print_endline
