open Base;;

let h, w =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun h w -> (h, w))
in
let a = List.range 0 h |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let c =
    a |> List.map ~f:String.to_list |> List.concat
    |> List.sort_and_group ~compare:Char.compare
    |> List.map ~f:List.length
  in
  let f c =
    List.sort c ~compare:descending |> function
    | [] -> []
    | hd :: tl -> (hd - 4) :: tl
  in
  Fn.apply_n_times ~n:(h / 2 * (w / 2)) f c |> fun c ->
  List.(for_all c ~f:(( <= ) 0) && count c ~f:(fun x -> x % 2 = 1) < 2)
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
