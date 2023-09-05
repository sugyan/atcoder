open Base;;

let n = Stdlib.read_int () in
let b =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let rec f l a =
    List.filteri l ~f:(fun i x -> i + 1 = x) |> List.last |> function
    | Some i ->
        List.split_n l (i - 1) |> fun (l0, l1) ->
        f (l0 @ List.tl_exn l1) (List.hd_exn l1 :: a)
    | None -> if List.length a = n then a else [ -1 ]
  in
  f b []
in
answer |> List.iter ~f:(Fn.compose Stdlib.print_endline Int.to_string)
