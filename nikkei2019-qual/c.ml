open Base;;

let n = Stdlib.read_int () in
let ab =
  let f _ =
    Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b))
  in
  List.range 0 n |> List.map ~f
in
let answer =
  let f i acc (a, b) = if i % 2 = 0 then acc + a else acc - b in
  List.sort ab ~compare:(fun (a0, b0) (a1, b1) -> compare (a1 + b1) (a0 + b0))
  |> List.foldi ~init:0 ~f
in
answer |> Int.to_string |> Stdlib.print_endline
