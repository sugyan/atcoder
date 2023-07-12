open Base;;

let n, m = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n m -> (n, m)) in
let a = List.range 0 m |> List.map ~f:(fun _ -> Caml.read_int ()) in
let answer =
  let h = Hash_set.of_list (module Int) (-1 :: a) in
  let dp = Array.create ~len:(n + 1) 0 in
  dp.(0) <- 1;
  for i = 1 to n do
    let f j = if Hash_set.mem h (i - j) then 0 else dp.(i - j) in
    List.map [ 1; 2 ] ~f
    |> List.fold ~init:dp.(i) ~f:( + )
    |> Fn.flip ( % ) 1_000_000_007
    |> Array.set dp i
  done;
  dp.(n)
in
answer |> Int.to_string |> Caml.print_endline
