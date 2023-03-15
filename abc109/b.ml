open Base;;

let n = Caml.read_int () in
let w = List.range 0 n |> List.map ~f:(fun _ -> Caml.read_line ()) in
let answer =
  let rec loop = function
    | [] | [ _ ] -> true
    | w0 :: w1 :: tl -> Char.( = ) (String.rev w0).[0] w1.[0] && loop tl
  in
  loop w && not (List.contains_dup w ~compare:String.compare)
in
answer |> (function true -> "Yes" | false -> "No") |> Caml.print_endline
