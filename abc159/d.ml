open Base;;

let n = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let c = Array.create ~len:(n + 1) 0 in
  List.iter a ~f:(fun i -> c.(i) <- c.(i) + 1);
  let sum = Array.sum (module Int) c ~f:(fun x -> x * (x - 1) / 2) in
  List.map a ~f:(fun i -> sum - c.(i) + 1)
in
answer |> List.map ~f:Int.to_string |> List.iter ~f:Stdlib.print_endline
