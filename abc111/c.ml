open Base;;

let n = Stdlib.read_int () in
let v =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let f m =
    List.filteri v ~f:(fun i _ -> i % 2 = m)
    |> List.sort ~compare |> List.group ~break:( <> )
    |> List.map ~f:(fun g -> (List.length g, List.hd_exn g))
    |> List.sort ~compare:Poly.descending
  in
  let e, o = (f 0, f 1) in
  match (List.hd_exn e, List.hd_exn o) with
  | (ec, ev), (oc, ov) when ev = ov ->
      let f l = List.nth l 1 |> Option.map ~f:fst |> Option.value ~default:0 in
      max (ec + f o) (oc + f e) |> ( - ) n
  | (ec, _), (oc, _) -> n - ec - oc
in
answer |> Int.to_string |> Stdlib.print_endline
