open Base;;

let n = Stdlib.read_int () in
let a = List.range 0 n |> List.map ~f:(fun _ -> Stdlib.read_int ()) in
let answer =
  let a = List.sort a ~compare in
  let s = List.fold ~init:0 ~f:( + ) in
  let f (lo, hi) =
    let lo = List.rev lo in
    (match Int.compare (List.length lo) (List.length hi) with
    | z when z > 0 -> -(List.take lo 2 |> s)
    | z when z < 0 -> List.take hi 2 |> s
    | _ -> List.(hd_exn hi - hd_exn lo))
    |> ( - ) ((s hi - s lo) * 2)
  in
  let sp = List.split_n a in
  (if n % 2 = 0 then [ sp (n / 2) ] else [ sp (n / 2); sp ((n + 1) / 2) ])
  |> List.map ~f |> List.fold ~init:0 ~f:max
in
answer |> Int.to_string |> Stdlib.print_endline
