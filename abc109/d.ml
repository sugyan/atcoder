open Base;;

let h, w =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun h w -> (h, w))
in
let a =
  let f _ =
    Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
  in
  List.range 0 h |> List.map ~f
in
let answer =
  let m = a |> List.map ~f:List.to_array |> List.to_array in
  let ret = ref [] in
  for i = 0 to h - 1 do
    for j = 0 to w - 2 do
      if m.(i).(j) % 2 = 1 then (
        m.(i).(j + 1) <- m.(i).(j + 1) + 1;
        ret := ((i + 1, j + 1), (i + 1, j + 2)) :: !ret)
    done
  done;
  for i = 0 to h - 2 do
    if m.(i).(w - 1) % 2 = 1 then (
      m.(i + 1).(w - 1) <- m.(i + 1).(w - 1) + 1;
      ret := ((i + 1, w), (i + 2, w)) :: !ret)
  done;
  !ret
in
answer |> List.length |> Int.to_string |> Stdlib.print_endline;
answer
|> List.iter ~f:(fun ((y, x), (y', x')) ->
       Printf.sprintf "%d %d %d %d" y x y' x' |> Stdlib.print_endline)
