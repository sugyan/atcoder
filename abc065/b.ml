open Base;;

let n = Caml.read_int () in
let a = List.range 0 n |> List.map ~f:(fun _ -> Caml.read_int ()) in
let answer =
  let a = List.map a ~f:Int.pred |> List.to_array in
  let rec f i c = if i = 1 then c else if c = n then -1 else f a.(i) (c + 1) in
  f 0 0
in
answer |> Int.to_string |> Caml.print_endline
