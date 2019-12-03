module Day2

let setRegister input pos value=
    input
    |> List.mapi (fun i v -> if i = pos then value else v)

type Op =
    | Add of int * int * int
    | Mul of int * int * int
    | Halt

let parseOpAtPosition (input:list<int>) position =
    match input.[position] with
    | 1 -> Ok (Add (input.[input.[position + 1]], input.[input.[position + 2]], input.[position + 3]))
    | 2 -> Ok (Mul (input.[input.[position + 1]], input.[input.[position + 2]], input.[position + 3]))
    | 99 -> Ok Halt
    | _ -> Error (sprintf "Invalid op code: %i" input.[position])

let processOp input op =
    match op with
    | Add (x, y, pos) -> setRegister input pos (x + y)
    | Mul (x, y, pos) -> setRegister input pos (x * y)
    | Halt -> input

let processProgram noun verb =
    let mutable registers = 
        System.IO.File.ReadAllText("Day2/input").Split([|','|])
        |> Seq.map int
        |> Seq.toList

    let mutable currentPosition = 0

    registers <- setRegister registers 1 noun
    registers <- setRegister registers 2 verb

    while parseOpAtPosition registers currentPosition <> Ok Halt do
        match parseOpAtPosition registers currentPosition with
        | Ok op -> 
            registers <- processOp registers op
            currentPosition <- currentPosition + 4
        | Error e -> failwith e
    
    registers.[0]

let Run = 
    let part1 = processProgram 12 2

    sprintf "%A" part1


// Part 1: 7594646
// Part 2: 