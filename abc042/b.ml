open Base;;

let n, _ = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n _ -> (n, ())) in
let s = List.range 0 n |> List.map ~f:(fun _ -> Caml.read_line ()) in
let answer = List.sort s ~compare:String.compare |> String.concat ~sep:"" in
answer |> Caml.print_endline
