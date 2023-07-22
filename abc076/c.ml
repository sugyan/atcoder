open Base;;

let s = Caml.read_line () in
let t = Caml.read_line () in
let answer =
  let sl, tl = String.(length s, length t) in
  let ok a =
    String.to_list s
    |> List.for_alli ~f:(fun i c -> Char.(c = '?' || c = a.[i]))
  in
  let f i =
    String.(sub s ~pos:0 ~len:i ^ t ^ sub s ~pos:(i + tl) ~len:(sl - i - tl))
    |> String.map ~f:(function '?' -> 'a' | c -> c)
    |> Option.some |> Option.filter ~f:ok
  in
  List.range 0 (sl - tl + 1)
  |> List.filter_map ~f
  |> List.min_elt ~compare:String.compare
  |> function
  | Some s -> s
  | None -> "UNRESTORABLE"
in
answer |> Caml.print_endline
