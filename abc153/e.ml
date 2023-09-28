open Base;;

let f _ =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y))
in
let h, n = f () in
let ab = List.range 0 n |> List.map ~f in
let answer =
  let dp = Array.create ~len:(h + 1) 0 in
  for i = 1 to h do
    let f (a, b) = b + if i <= a then 0 else dp.(i - a) in
    dp.(i) <- List.map ab ~f |> List.fold ~init:Int.max_value ~f:min
  done;
  dp.(h)
in
answer |> Int.to_string |> Stdlib.print_endline
