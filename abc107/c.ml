open Base;;

let n, k = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n k -> (n, k)) in
let x =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = List.to_array x in
  let f i =
    let h, t = (a.(i), a.(i + k - 1)) in
    min (abs h + abs (h - t)) (abs t + abs (h - t))
  in
  List.init (n - k + 1) ~f |> List.fold ~init:Int.max_value ~f:min
in
answer |> Int.to_string |> Stdlib.print_endline
