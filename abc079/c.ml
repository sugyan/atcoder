open Base;;

let abcd = Stdlib.read_line () |> String.to_list in
let answer =
  let a =
    List.map abcd ~f:(fun c -> Char.(to_int c - to_int '0', to_string c))
  in
  List.fold (List.tl_exn a)
    ~init:[ List.hd_exn a ]
    ~f:(fun acc (d, c) ->
      List.map acc ~f:(fun (n, s) ->
          [ (n + d, s ^ "+" ^ c); (n - d, s ^ "-" ^ c) ])
      |> List.concat)
  |> List.find_exn ~f:(fun (x, _) -> x = 7)
  |> snd |> Fn.flip ( ^ ) "=7"
in
answer |> Stdlib.print_endline
