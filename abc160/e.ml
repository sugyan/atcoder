open Base;;

let x, y, _, _, _ =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d %d %d" (fun x y a b c ->
      (x, y, a, b, c))
in
let f _ =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let p = f () in
let q = f () in
let r = f () in
let answer =
  [
    List.take (List.sort p ~compare:descending) x;
    List.take (List.sort q ~compare:descending) y;
    r;
  ]
  |> List.concat
  |> List.sort ~compare:descending
  |> Fn.flip List.take (x + y)
  |> List.fold ~init:0 ~f:( + )
in
answer |> Int.to_string |> Stdlib.print_endline
