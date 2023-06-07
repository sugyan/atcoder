open Base;;

let s = Caml.read_line () in
let q = Caml.read_int () in
let qs =
  List.range 0 q
  |> List.map ~f:(fun _ -> Caml.read_line () |> String.split ~on:' ')
in
let answer =
  let f (s, t) = function
    | [ "1" ] -> (t, s)
    | [ "2"; "1"; c ] -> (c :: s, t)
    | [ "2"; "2"; c ] -> (s, c :: t)
    | _ -> assert false
  in
  List.fold qs ~init:(String.to_list s |> List.map ~f:String.of_char, []) ~f
  |> fun (s, t) -> String.concat s ^ String.concat (List.rev t)
in
answer |> Caml.print_endline
