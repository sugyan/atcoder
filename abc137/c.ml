open Base;;

let n = Stdlib.read_int () in
let s = List.range 0 n |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let f s =
    String.to_list s |> List.sort ~compare:Char.compare |> String.of_char_list
  in
  let h = Hashtbl.create (module String) in
  List.map s ~f |> List.iter ~f:(Hashtbl.incr h);
  Hashtbl.data h |> List.sum (module Int) ~f:(fun x -> x * (x - 1) / 2)
in
answer |> Int.to_string |> Stdlib.print_endline
