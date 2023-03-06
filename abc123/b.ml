open Base;;

let abcde = List.init 5 ~f:(fun _ -> Caml.read_int ()) in
let answer =
  List.sum (module Int) abcde ~f:(fun m -> m + (-m % 10))
  - List.fold abcde ~init:0 ~f:(fun acc m -> max acc (-m % 10))
in
answer |> Int.to_string |> Caml.print_endline
