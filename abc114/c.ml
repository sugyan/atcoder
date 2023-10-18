open Base;;

let n = Stdlib.read_int () in
let answer =
  let is753 m =
    Int.to_string m |> String.to_list
    |> List.dedup_and_sort ~compare:Char.compare
    |> List.length = 3
  in
  let rec f m =
    if m > n then 0
    else
      List.sum (module Int) [ 3; 5; 7 ] ~f:(fun x -> f ((m * 10) + x))
      |> ( + ) (if is753 m then 1 else 0)
  in
  f 0
in
answer |> Int.to_string |> Stdlib.print_endline
