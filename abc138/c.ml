open Base;;

let _ = Caml.read_int () in
let v =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.sort v ~compare |> List.map ~f:Float.of_int |> function
  | hd :: tl -> List.fold tl ~init:hd ~f:(fun acc x -> (acc +. x) /. 2.)
  | _ -> assert false
in
answer |> Float.to_string |> Caml.print_endline
