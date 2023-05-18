open Base;;

let s = Caml.read_line () in
let answer =
  let rec loop c t =
    let f i = if Char.(t.[i] = c || t.[i + 1] = c) then c else t.[i] in
    if String.for_all t ~f:(Char.( = ) c) then 0
    else String.length t - 1 |> String.init ~f |> loop c |> ( + ) 1
  in
  String.to_list s
  |> List.dedup_and_sort ~compare:Char.compare
  |> List.map ~f:(Fn.flip loop s)
  |> List.fold ~init:(String.length s) ~f:min
in
answer |> Int.to_string |> Caml.print_endline
