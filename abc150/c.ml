open Base;;

let n = Stdlib.read_int () in
let p =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let q =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a =
    let rec loop acc i =
      if i > n then acc else loop ((i * List.hd_exn acc) :: acc) (i + 1)
    in
    loop [ 1 ] 1 |> List.rev |> List.to_array
  in
  let rec count = function
    | [] -> 0
    | hd :: tl -> (List.count tl ~f:(( > ) hd) * a.(List.length tl)) + count tl
  in
  count p - count q |> abs
in
answer |> Int.to_string |> Stdlib.print_endline
