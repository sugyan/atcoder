open Base;;

let n, _ = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n _ -> (n, ())) in
let s = List.range 0 n |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer = List.sort s ~compare:String.compare |> String.concat ~sep:"" in
answer |> Stdlib.print_endline
