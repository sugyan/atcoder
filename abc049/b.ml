open Base;;

let h, _ = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun h w -> (h, w)) in
let c = List.range 0 h |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer = List.map c ~f:(fun s -> [ s; s ]) |> List.concat in
answer |> List.iter ~f:Stdlib.print_endline
