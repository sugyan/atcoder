open Base;;

let n = Caml.read_int () in
let abcde = List.range 0 5 |> List.map ~f:(fun _ -> Caml.read_int ()) in
let answer = List.fold abcde ~init:n ~f:min |> ( / ) (n - 1) |> ( + ) 5 in
answer |> Int.to_string |> Caml.print_endline
