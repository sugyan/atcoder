open Base;;

let s = Stdlib.read_line () in
let answer =
  let rec f s =
    [ "maerd"; "remaerd"; "esare"; "resare" ]
    |> List.find ~f:(fun p -> String.is_prefix s ~prefix:p)
    |> function
    | Some t -> f String.(subo s ~pos:(length t))
    | None -> String.is_empty s
  in
  f (String.rev s)
in
answer |> (function true -> "YES" | false -> "NO") |> Stdlib.print_endline
