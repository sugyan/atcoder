open Base;;

let n = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let p = 1_000_000_007 in
  List.init 60 ~f:(fun i -> List.count a ~f:(fun x -> x land (1 lsl i) <> 0))
  |> List.foldi ~init:0 ~f:(fun i acc x ->
         (acc + ((1 lsl i) % p * x % p * (n - x))) % p)
in
answer |> Int.to_string |> Stdlib.print_endline
