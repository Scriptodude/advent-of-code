import java.io.File;

val TILE_SIZE_X = 10
val TILE_SIZE_Y = 10

var data = File("input-test-1.txt").bufferedReader().readLines()

enum class Side {
    West,
    North,
    East,
    South;

    fun opposite(): Side {
        return when (this) {
            West -> East
            East -> West
            North -> South
            South -> North
        }
    }
}

data class Tile(val id: Int, var image: MutableList<String>) {
    fun getBorder(side: Side): String {
        when (side) {
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
            this.getBorder(Side.South)
        )
    }

    fun flipAccordingTo(tile: Tile, side: Side): Tile {
        if (this.getBorder(side).reversed() == tile.getBorder(side.opposite())) {
            when (side) {
                Side.North -> this.flipVerti()
                Side.South -> this.flipVerti()
                Side.West -> this.flipHoriz()
                Side.East -> this.flipHoriz()
            }
        }

        return this
    }

    private fun getMatchingSideP(tile: Tile): Side? {
        return this.getAllBorders().mapIndexed { i, b ->
            if (tile.getAllBorders().any { it == b }) i
            else -1
        }
            .filter { it != -1 }
            .map {
                when (it) {
                    0 -> Side.West
                    1 -> Side.East
                    2 -> Side.North
                    3 -> Side.South
                    else -> null
                }
            }.firstOrNull()
    }

    private fun flipHoriz() {
        this.image = this.image.map { it.reversed() }.toMutableList()
    }

    private fun flipVerti() {
        this.image = this.image.reversed().toMutableList()
    }

    override fun toString(): String {
        return "$id -> \n" + this.image.joinToString("\n")
    }
}

fun getDegreesBetween(side1: Side, side2: Side): Int {
return Math.abs(side1.ordinal - side2.ordinal) * 90
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

val finalImageSize = Math.sqrt(tiles.size.toDouble()).toInt()
val finalImage = mutableListOf<String>("")
val allBorders = tiles.flatMap { it.getAllBorders() }

val borders = tiles.filter { tile ->
val matching = tile.getAllBorders().map { border -> allBorders.count { it == border || it.reversed() == border } }.reduce(Int::plus)

matching - 4 == 2
}.toList()

fun findNeighborMatching(tile: Tile, side: Side): Tile? {
val neigh = tiles
.filter { it.id != tile.id }
.filter { t -> t.getAllBorders().any { it == tile.getBorder(side) || it.reversed() == tile.getBorder(side) }}
.firstOrNull()

return neigh
}

var first = borders.filter {
val north = findNeighborMatching(it, Side.North)
val west = findNeighborMatching(it, Side.West)
val east = findNeighborMatching(it, Side.East)
val south = findNeighborMatching(it, Side.South)

println("${north?.id}, ${west?.id}, ${east?.id}, ${south?.id}")

north == null && west == null && east != null && south != null
}.firstOrNull()!!
var currentLine = mutableListOf<Tile>()
println(first.id)

while (true) {

tiles.remove(first)
currentLine.clear()
currentLine.add(first)
var next = findNeighborMatching(first, Side.East)
do {
print("${next?.id} ")
tiles.remove(next!!)
currentLine.add(next!!)
next = findNeighborMatching(next!!, Side.East)?.flipAccordingTo(next!!, Side.East)
} while (next != null)

(0..9).forEach { line ->
finalImage.add(currentLine.map { it.image[line] }.joinToString(" "))
}

var down = findNeighborMatching(first, Side.South)?.flipAccordingTo(first, Side.South)
if (down == null) break

first = down
}

println(finalImage.joinToString("\n"))