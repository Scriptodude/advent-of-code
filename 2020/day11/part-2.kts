import java.io.File;
val lines = File("input.txt").bufferedReader().readLines().toMutableList()
var lastState = lines.toList()
var newState = lines.toMutableList()

fun getSeatInDirection(x: Int, xMod: Int, y: Int, yMod: Int): String {
    var i = x + xMod
    var j = y + yMod

    while ((i < lastState[0].length && i >= 0) && (j < lastState.size && j >= 0)) {
        if (lastState[j][i] == 'L' || lastState[j][i] == '#') {
            return lastState[j][i].toString()
        }

        i += xMod
        j += yMod
    }

    return ""
}

fun getAdjacent(x: Int, y: Int): String {
    return listOf(
        getSeatInDirection(x, -1, y, -1),
        getSeatInDirection(x, -1, y, 0),
        getSeatInDirection(x, -1, y, 1),
        getSeatInDirection(x, 0, y, -1),
        getSeatInDirection(x, 0, y, 1),
        getSeatInDirection(x, 1, y, -1),
        getSeatInDirection(x, 1, y, 0),
        getSeatInDirection(x, 1, y, 1)).joinToString("")
}

fun simulate() {
    (0..lines.size-1).forEach {y ->
        (0..lines[y].length-1).forEach {x ->
            val cur = lastState[y][x].toChar()
            val adj = getAdjacent(x, y)

            when(cur) {
                'L' -> {
                    if (adj.count { it == '#' } == 0) {
                        newState[y] = newState[y].replaceRange(x, x+1, "#")
                    }
                }
                '#' -> {
                    if (adj.count { it == '#' } >= 5) {
                        newState[y] = newState[y].replaceRange(x, x+1, "L")
                    }
                }
                else -> {}
            }
        }
    }
}

do {
    lastState = newState.toList()
    simulate()
} while(lastState != newState)

println(newState.map { it.count { c -> c == '#' }}.sum())