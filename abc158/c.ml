open Base;;

let a, b = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b)) in
let answer =
  List.range 0 1010 |> List.find ~f:(fun x -> x * 8 / 100 = a && x / 10 = b)
  |> function
  | Some x -> x
  | None -> -1
in
answer |> Int.to_string |> Stdlib.print_endline
