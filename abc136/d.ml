open Base;;

let s = Caml.read_line () in
let answer =
  let a = Array.create ~len:(String.length s) 0 in
  let f i (acc, p) x =
    if i % 2 = 1 then (
      a.(acc - 1) <- (x / 2) + (p / 2) + (p % 2);
      a.(acc) <- (x / 2) + (p / 2) + (x % 2));
    (acc + x, x)
  in
  String.to_list s
  |> List.group ~break:Char.( <> )
  |> List.map ~f:List.length
  |> List.foldi ~init:(0, 0) ~f
  |> ignore;
  Array.to_list a
in
answer |> List.map ~f:Int.to_string |> String.concat ~sep:" "
|> Caml.print_endline
