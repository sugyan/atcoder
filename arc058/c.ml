open Base;;

let n, _ =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n k -> (n, k))
in
let d =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let rec f i =
    Int.to_string i |> String.to_list
    |> List.map ~f:Char.get_digit_exn
    |> List.find ~f:(List.mem d ~equal)
    |> function
    | Some _ -> f (i + 1)
    | None -> i
  in
  f n
in
answer |> Int.to_string |> Stdlib.print_endline
