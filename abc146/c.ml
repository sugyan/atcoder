open Base;;

let a, b, x =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun a b x -> (a, b, x))
in
let answer =
  List.init 10 ~f:(fun i ->
      (x - (b * (i + 1))) / a |> min ((10 ** (i + 1)) - 1))
  |> List.fold ~init:0 ~f:max
  |> min (10 ** 9)
in
answer |> Int.to_string |> Caml.print_endline
