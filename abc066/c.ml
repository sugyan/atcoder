open Base;;

let n = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let b = Array.create ~len:n 0 in
  List.iteri a ~f:(fun i x ->
      b.((n / 2) + if i % 2 = 0 then i / 2 else (-i / 2) - 1) <- x);
  Array.to_list b |> if n % 2 = 0 then Fn.id else List.rev
in
answer |> List.map ~f:Int.to_string |> String.concat ~sep:" "
|> Caml.print_endline
