open Base;;

let f _ = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y)) in
let n, m = f () in
let sc = List.range 0 m |> List.map ~f in
let answer =
  List.range 0 1000 |> List.map ~f:Int.to_string
  |> List.filter ~f:(fun s -> String.length s = n)
  |> List.find ~f:(fun s ->
         List.for_all sc ~f:(fun (i, c) ->
             Char.(s.[i - 1] = of_int_exn (c + to_int '0'))))
  |> function
  | Some s -> Int.of_string s
  | None -> -1
in
answer |> Int.to_string |> Stdlib.print_endline
