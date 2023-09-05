open Base;;

let n, p = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n p -> (n, p)) in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  if List.exists a ~f:(fun x -> x % 2 = 1) then 2 ** (n - 1)
  else if p = 0 then 2 ** n
  else 0
in
answer |> Int.to_string |> Stdlib.print_endline
