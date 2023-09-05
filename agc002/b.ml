open Base;;

let f _ = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b)) in
let n, m = f () in
let xy = List.range 0 m |> List.map ~f in
let answer =
  let a = Array.create ~len:n 1 in
  let b = Array.init n ~f:(( = ) 0) in
  List.iter xy ~f:(fun (x, y) ->
      a.(x - 1) <- a.(x - 1) - 1;
      a.(y - 1) <- a.(y - 1) + 1;
      if b.(x - 1) then b.(y - 1) <- true;
      if a.(x - 1) = 0 then b.(x - 1) <- false);
  Array.count b ~f:Fn.id
in
answer |> Int.to_string |> Stdlib.print_endline
