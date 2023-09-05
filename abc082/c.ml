open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  Hashtbl.(
    let h = create (module Int) in
    List.iter a ~f:(incr h);
    to_alist h)
  |> List.sum (module Int) ~f:(fun (k, v) -> if k > v then v else v - k)
in
answer |> Int.to_string |> Stdlib.print_endline
