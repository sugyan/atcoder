open Base;;

let f _ = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y)) in
let n, q = f () in
let s = Stdlib.read_line () in
let lr = List.range 0 q |> List.map ~f in
let answer =
  let a = Array.create ~len:(n + 1) 0 in
  for i = 1 to n - 1 do
    a.(i + 1) <- (a.(i) + if Char.(s.[i - 1] = 'A' && s.[i] = 'C') then 1 else 0)
  done;
  List.map lr ~f:(fun (l, r) -> a.(r) - a.(l))
in
answer |> List.iter ~f:(Fn.compose Stdlib.print_endline Int.to_string)
