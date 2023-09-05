open Base;;

let n, x = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n x -> (n, x)) in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let b, c =
    List.sort a ~compare
    |> List.fold_map ~init:x ~f:(fun acc a -> (acc - a, acc >= a))
  in
  if b > 0 then n - 1 else List.count c ~f:Fn.id
in
answer |> Int.to_string |> Stdlib.print_endline
