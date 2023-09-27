open Base;;

let k = Stdlib.read_int () in
let answer =
  let q = List.range 1 10 |> Queue.of_list in
  let rec f k =
    let n = Queue.dequeue_exn q in
    if k = 1 then n
    else (
      [ -1; 0; 1 ]
      |> List.map ~f:(( + ) (n % 10))
      |> List.filter ~f:(List.mem (List.range 0 10) ~equal)
      |> List.map ~f:(( + ) (n * 10))
      |> List.iter ~f:(Queue.enqueue q);
      f (k - 1))
  in
  f k
in
answer |> Int.to_string |> Stdlib.print_endline
