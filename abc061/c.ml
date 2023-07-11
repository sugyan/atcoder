open Base;;

let f _ = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun x y -> (x, y)) in
let n, k = f () in
let ab = List.range 0 n |> List.map ~f in
let answer =
  List.sort ab ~compare:Poly.compare
  |> List.fold_until ~init:0
       ~f:(fun acc (a, b) ->
         if k <= acc + b then Stop a else Continue (acc + b))
       ~finish:Fn.id
in
answer |> Int.to_string |> Caml.print_endline
