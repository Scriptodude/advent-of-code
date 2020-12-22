import java.io.File;
var data = File("input.txt").bufferedReader().readLines()

enum class Side {
    West,
    East,
    North,
    South
}

data class Tile(val id: Int, val image: List<String>) {
    fun getBorder(side: Side): String {
        when(side) {
            Side.West -> return this.image.map { it.first().toString() }.reduce(String::plus)
            Side.East -> return this.image.map { it.last().toString() }.reduce(String::plus)
            Side.North -> return this.image.first()
            Side.South -> return this.image.last()
        }
    }

    fun getAllBorders(): List<String> {
        return listOf(
            this.getBorder(Side.West),
            this.getBorder(Side.East),
            this.getBorder(Side.North),
            this.getBorder(Side.South))
    }
}

val it = data.iterator()
val tiles = mutableListOf<Tile>()

while (it.hasNext()) {
    var current = it.next()
    var id = 0
    var data = mutableListOf<String>()

    if (current.startsWith("Tile")) {
        id = current.dropLast(1).dropWhile { !it.isDigit() }.toInt()
        current = it.next()
    }

    while (!current.isBlank()) {
        data.add(current)

        if (it.hasNext()) {
            current = it.next();
        } else {
            break
        }
    }

    tiles.add(Tile(id, data))
}

var result = 1L
val allBorders = tiles.flatMap { it.getAllBorders() }

tiles.forEach { tile ->
    val borders = tile.getAllBorders()

    val matching = borders.map { border -> allBorders.count { it == border || it.reversed() == border } }.reduce(Int::plus)

    if (matching - 4 == 2) {
        result *= tile.id
    }
}

println(result)