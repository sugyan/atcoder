open Base;;

let n = Caml.read_int () in
let s = List.range 0 n |> List.map ~f:(fun _ -> Caml.read_int ()) in
let answer =
  let sum = List.sum (module Int) s ~f:Fn.id in
  if sum % 10 = 0 then
    List.sort s ~compare |> List.find ~f:(fun x -> x % 10 <> 0) |> function
    | Some x -> sum - x
    | None -> 0
  else sum
in
answer |> Int.to_string |> Caml.print_endline
