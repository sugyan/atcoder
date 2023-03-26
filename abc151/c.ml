open Base;;

let n, m = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n m -> (n, m)) in
let ps =
  List.range 0 m
  |> List.map ~f:(fun _ ->
         Caml.Scanf.sscanf (Caml.read_line ()) "%d %s" (fun p s -> (p, s)))
in
let answer =
  let a = Array.create ~len:n (false, 0) in
  List.iter ps ~f:(fun (p, s) ->
      let ac, wa = a.(p - 1) in
      if String.(s = "AC") then a.(p - 1) <- (true, wa)
      else if not ac then a.(p - 1) <- (false, wa + 1)
      else ());
  ( Array.count a ~f:fst,
    Array.fold a ~init:0 ~f:(fun acc (ac, wa) -> acc + if ac then wa else 0) )
in
answer
|> (fun a -> Printf.sprintf "%d %d" (fst a) (snd a))
|> Caml.print_endline
