open Base;;

let n, ab =
  let f _ =
    Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y))
  in
  let n, m = f () in
  let ab = List.range 0 m |> List.map ~f in
  (n, ab)
in
let answer =
  let c = Array.create ~len:n 0 in
  List.iter ab ~f:(fun (a, b) ->
      c.(a - 1) <- c.(a - 1) + 1;
      c.(b - 1) <- c.(b - 1) + 1);
  Array.for_all c ~f:(fun x -> x % 2 = 0)
in
answer |> (function true -> "YES" | false -> "NO") |> Stdlib.print_endline
