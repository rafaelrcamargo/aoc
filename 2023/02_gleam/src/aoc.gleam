import simplifile.{read}
import gleam/string
import gleam/list
import gleam/int
import gleam/io

pub fn p1(rounds) {
  let rounds =
    rounds
    |> list.map(fn(hands) {
      hands
      |> list.map(fn(hand) {
        case hand {
          [cubes, color] ->
            case int.parse(cubes) {
              Ok(amount) ->
                case color {
                  "red" if amount <= 12 -> True
                  "green" if amount <= 13 -> True
                  "blue" if amount <= 14 -> True
                  _ -> False
                }
              _ -> panic
            }
        }
      })
    })

  let hands =
    rounds
    |> list.map(fn(round) {
      list.any(round, fn(revealed) { revealed == False })
    })
    |> list.index_map(fn(i, revealed) {
      case revealed {
        False -> i + 1
        _ -> 0
      }
    })

  hands
  |> int.sum()
  |> int.to_string()
}

pub fn p2(rounds) {
  let rounds = {
    let hands =
      rounds
      |> list.map(fn(hand) {
        hand
        |> list.fold(
          [0, 0, 0],
          fn(rgb, hand) {
            let [r, g, b] = rgb

            case hand {
              [amount, color] ->
                case int.parse(amount) {
                  Ok(a) ->
                    case color {
                      "red" if a > r -> [a, g, b]
                      "green" if a > g -> [r, a, b]
                      "blue" if a > b -> [r, g, a]
                      _ -> [r, g, b]
                    }
                  _ -> panic
                }
            }
          },
        )
      })

    hands
    |> list.map(fn(hand) {
      let [r, g, b] = hand
      r * g * b
    })
  }

  rounds
  |> int.sum
  |> int.to_string
}

pub fn main() {
  let assert Ok(input) = read(from: "./assets/input.txt")

  let rounds = {
    let lines = string.split(input, on: "\n")

    lines
    |> list.map(fn(line) {
      let assert Ok(chunks) =
        line
        |> string.split(on: "Game ")
        |> list.at(1)

      let assert Ok(rounds) =
        chunks
        |> string.split(on: ": ")
        |> list.at(1)

      rounds
      |> string.split(on: ": ")
      |> list.flat_map(fn(hand) {
        hand
        |> string.split(", ")
        |> list.map(fn(hand) {
          hand
          |> string.split("; ")
        })
      })
      |> list.flat_map(fn(hand) {
        hand
        |> list.map(fn(hand) {
          hand
          |> string.split(" ")
        })
      })
    })
  }

  io.println("Part 1: " <> p1(rounds))
  io.println("Part 2: " <> p2(rounds))
}
