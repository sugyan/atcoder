open Base;;

let n = Stdlib.read_int () in
let answer =
  let rec f n =
    if n = 0 then []
    else (if n % 2 = 0 then '0' else '1') :: f ((n - (n % 2)) / -2)
  in
  if n = 0 then "0" else f n |> List.rev |> String.of_char_list
in
answer |> Stdlib.print_endline
