open Base;;

let n, k = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n k -> (n, k)) in
let h = List.range 0 n |> List.map ~f:(fun _ -> Caml.read_int ()) in
let answer =
  let a = List.sort h ~compare |> List.to_array in
  List.init (n - k + 1) ~f:(fun i -> a.(i + k - 1) - a.(i))
  |> List.fold ~init:Int.max_value ~f:min
in
answer |> Int.to_string |> Caml.print_endline
