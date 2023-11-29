open Base;;

let _ = Stdlib.read_int () in
let s1 = Stdlib.read_line () in
let s2 = Stdlib.read_line () in
let answer =
  let div = 1_000_000_007 in
  let f (acc, p) b =
    ((if p then acc * 2 else acc * if b then 1 else 3) % div, b)
  in
  List.zip_exn (String.to_list s1) (String.to_list s2)
  |> List.remove_consecutive_duplicates ~equal:Poly.equal
  |> List.map ~f:(fun (c1, c2) -> Char.equal c1 c2)
  |> fun l ->
  List.(hd_exn l, tl_exn l) |> fun (hd, tl) ->
  List.fold tl ~init:((if hd then 3 else 6), hd) ~f |> fst
in
answer |> Int.to_string |> Stdlib.print_endline
