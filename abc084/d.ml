open Base;;

let q = Stdlib.read_int () in
let f _ =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun l r -> (l, r))
in
let lr = List.range 0 q |> List.map ~f in
let answer =
  let a = Array.init 100_001 ~f:(( < ) 1) in
  for i = 2 to 1000 do
    if a.(i) then
      List.range i ((100_000 / i) + 1)
      |> List.iter ~f:(fun j -> a.(i * j) <- false)
  done;
  let b = Array.create ~len:100_001 0 in
  let f i = b.(i) <- b.(i - 1) + Bool.to_int (a.(i) && a.((i + 1) / 2)) in
  List.range 1 100_001 |> List.iter ~f;
  List.map lr ~f:(fun (l, r) -> b.(r) - b.(l - 1))
in
answer |> List.iter ~f:(Fn.compose Stdlib.print_endline Int.to_string)
