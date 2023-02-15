open Core;;

In_channel.input_lines In_channel.stdin
|> List.map ~f:(String.split ~on:' ')
|> List.map ~f:(List.map ~f:Int.of_string)
|> function
| [ _; _; c ] :: b :: a ->
    let f x =
      List.map2_exn x b ~f:( * ) |> List.sum (module Int) ~f:Fn.id > -c
    in
    List.count a ~f |> Int.to_string |> print_endline
| _ -> assert false
