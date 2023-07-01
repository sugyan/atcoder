open Base;;

let s = Caml.read_line () in
let answer =
  String.to_list "abcdefghijklmnopqrstuvwxyz"
  |> List.find ~f:(String.mem s |> Fn.non)
  |> function
  | Some c -> s ^ Char.to_string c
  | _ -> (
      String.to_list s |> List.group ~break:Char.( < ) |> List.rev |> function
      | [ _ ] -> "-1"
      | hd :: _ ->
          let len = 25 - List.length hd in
          List.filter hd ~f:(Char.( < ) s.[len])
          |> List.fold ~init:'z' ~f:Char.min
          |> Char.to_string
          |> ( ^ ) (String.sub s ~pos:0 ~len)
      | _ -> assert false)
in
answer |> Caml.print_endline
