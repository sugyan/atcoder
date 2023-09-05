open Base;;

let s = Stdlib.read_line () in
let answer =
  let n = String.length s in
  List.init n ~f:(fun i ->
      s.[i] |> function 'U' -> i | 'D' -> n - i - 1 | _ -> assert false)
  |> List.fold ~init:(n * (n - 1)) ~f:( + )
in
answer |> Int.to_string |> Stdlib.print_endline
