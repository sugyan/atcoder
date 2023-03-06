open Base;;

let n, k, q =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun n k q -> (n, k, q))
in
let a = List.init q ~f:(fun _ -> Caml.read_int ()) in
let answer =
  let b = Array.init n ~f:(Fn.const (k - q)) in
  List.iter a ~f:(fun i -> b.(i - 1) <- b.(i - 1) + 1);
  Array.to_list b |> List.map ~f:(( < ) 0)
in
answer
|> List.map ~f:(function true -> "Yes" | false -> "No")
|> List.iter ~f:Caml.print_endline
