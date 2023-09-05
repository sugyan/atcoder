open Base;;

let s = Stdlib.read_line () in
let t = Stdlib.read_line () in
let answer =
  let f u =
    let a = Array.create ~len:128 [] in
    String.to_list u |> List.map ~f:Char.to_int
    |> List.iteri ~f:(fun i j -> a.(j) <- i :: a.(j));
    Array.to_list a |> List.sort ~compare:Poly.compare
  in
  Poly.equal (f s) (f t)
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
