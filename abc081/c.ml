open Base;;

let _, k = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n k -> (n, k)) in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let h = Hashtbl.create (module Int) in
  List.iter a ~f:(Hashtbl.incr h);
  Hashtbl.data h
  |> List.sort ~compare:descending
  |> Fn.flip List.drop k
  |> List.sum (module Int) ~f:Fn.id
in
answer |> Int.to_string |> Stdlib.print_endline
