open Base;;

let n, x, y =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun n x y -> (n, x, y))
in
let answer =
  let a = Array.create ~len:n 0 in
  for i = 1 to n - 1 do
    for j = i + 1 to n do
      min (j - i) (abs (x - i) + 1 + abs (y - j)) |> fun d -> a.(d) <- a.(d) + 1
    done
  done;
  Array.to_list a |> List.tl_exn
in
answer |> List.iter ~f:(Fn.compose Caml.print_endline Int.to_string)
