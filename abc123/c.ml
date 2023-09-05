open Base;;

let n = Stdlib.read_int () in
let abcde = List.range 0 5 |> List.map ~f:(fun _ -> Stdlib.read_int ()) in
let answer = List.fold abcde ~init:n ~f:min |> ( / ) (n - 1) |> ( + ) 5 in
answer |> Int.to_string |> Stdlib.print_endline
