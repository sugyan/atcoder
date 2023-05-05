open Base;;

let n, k = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n k -> (n, k)) in
let p =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let p = List.to_array p in
  List.range 0 n
  |> List.fold ~init:(0, 0) ~f:(fun acc i ->
         let sum = fst acc + p.(i) - if i >= k then p.(i - k) else 0 in
         (sum, max (snd acc) sum))
  |> snd |> ( + ) k |> Int.to_float |> Fn.flip ( /. ) 2.0
in
answer |> Float.to_string |> Caml.print_endline
