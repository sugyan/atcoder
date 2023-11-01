open Base;;

let n = Stdlib.read_int () in
let answer =
  let rec dfs s i =
    if String.length s = n then [ s ]
    else
      List.range 0 (i + 1)
      |> List.map ~f:(fun j -> s ^ (Char.of_int_exn (j + 97) |> String.of_char))
      |> List.concat_mapi ~f:(fun j t -> dfs t (if i = j then i + 1 else i))
  in
  dfs "a" 1
in
answer |> List.iter ~f:Stdlib.print_endline
