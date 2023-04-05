open Base;;

let _ = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let sum = List.sum (module Int) a ~f:Fn.id in
  List.folding_map a ~init:0 ~f:(fun acc x ->
      let y = acc + x in
      (y, (y * 2) - sum |> abs))
  |> List.fold ~init:sum ~f:min
in
answer |> Int.to_string |> Caml.print_endline
