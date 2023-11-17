open Base;;

let _, k =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n k -> (n, k))
in
let s = Stdlib.read_line () in
let answer =
  let a =
    String.to_list s
    |> List.group ~break:Char.( <> )
    |> List.map ~f:List.length
    |> (fun l -> if Char.(s.[0] = '0') then 0 :: l else l)
    |> Fn.flip List.append [ 0 ] |> List.to_array
  in
  let f i (acc, ans) x =
    let acc = acc + x - if i > 2 * k then a.(i - (2 * k) - 1) else 0 in
    (acc, if i % 2 = 0 then max ans acc else ans)
  in
  Array.foldi a ~init:(0, 0) ~f |> snd
in
answer |> Int.to_string |> Stdlib.print_endline
