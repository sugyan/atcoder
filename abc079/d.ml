open Base;;

let h, _ =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun h w -> (h, w))
in
let f _ =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let c = List.range 0 10 |> List.map ~f in
let a = List.range 0 h |> List.map ~f in
let answer =
  let c = List.(map c ~f:to_array |> to_array) in
  let m = Array.copy_matrix c in
  for k = 0 to 9 do
    for i = 0 to 9 do
      for j = 0 to 9 do
        m.(i).(j) <- min m.(i).(j) (m.(i).(k) + m.(k).(j))
      done
    done
  done;
  List.concat a
  |> List.sum (module Int) ~f:(fun x -> if x = -1 then 0 else m.(x).(1))
in
answer |> Int.to_string |> Stdlib.print_endline
