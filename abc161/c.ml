open Base;;

let n, k = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" @@ fun n k -> (n, k) in
let answer =
  let m = Int.(rem) n k in
  min m (k - m)
in
answer |> Int.to_string |> Stdio.print_endline
