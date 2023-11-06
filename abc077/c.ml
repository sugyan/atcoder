open Base;;

let n = Stdlib.read_int () in
let f _ =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let a = f () in
let b = f () in
let c = f () in
let answer =
  let a = List.sort a ~compare |> List.to_array in
  let c = List.sort c ~compare |> List.to_array in
  let f x =
    let s = Array.binary_search ~compare in
    (s a `Last_strictly_less_than x |> function Some x -> x + 1 | None -> 0)
    * (s c `First_strictly_greater_than x |> Option.value ~default:n |> ( - ) n)
  in
  List.sum (module Int) b ~f
in
answer |> Int.to_string |> Stdlib.print_endline
