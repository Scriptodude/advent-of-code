import java.io.File;
val lines = File("input.txt").bufferedReader().readLines().toMutableList()
var lastState = lines.toList()
var newState = lines.toMutableList()

fun getAdjacent(x: Int, y: Int): String {
    val lowY = Math.max(y - 1, 0)
    val highY = Math.min(y + 2, lines.size)
    val lowX = Math.max(x - 1, 0)
    val highX = Math.min(x + 2, lines[y].length)
    var char = lastState[y][x].toString()

    return lastState.subList(lowY, highY)
            .map { it.substring(lowX, highX) }
            .joinToString("")
            .replace(".", "")
            .replaceFirst(char, "")
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
                    if (adj.count { it == '#' } >= 4) {
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

println(newState)
println(newState.map { it.count { c -> c == '#' }}.sum())