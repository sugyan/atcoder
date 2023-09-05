open Base;;

let s = Stdlib.read_line () in
let answer =
  let f l = function
    | 'B' -> if List.is_empty l then l else List.tl_exn l
    | c -> c :: l
  in
  List.fold (String.to_list s) ~init:[] ~f |> List.rev |> String.of_char_list
in
answer |> Stdlib.print_endline
