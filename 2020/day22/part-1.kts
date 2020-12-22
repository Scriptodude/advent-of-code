import java.io.File;
var data = File("input.txt").bufferedReader().readLines()

val player1 = mutableListOf<Int>()
val player2 = mutableListOf<Int>()
val dataIt = data.iterator()

while (dataIt.hasNext()) {
    val line = dataIt.next()

    if (line.startsWith("Player 1")) {

        do {
            val value = dataIt.next()
            if (value.isBlank()) break;
            player1.add(value.toInt())
        } while (true)
    }
    if (line.startsWith("Player 2")) {

        do {
            val value = dataIt.next()
            if (value.isBlank()) break;
            player2.add(value.toInt())
        } while (dataIt.hasNext())
    }
}

do {
    val p1Card = player1.first();
    player1.removeFirst();
    val p2Card = player2.first();
    player2.removeFirst();

    if (p1Card > p2Card) {
        player1.addAll(listOf(p1Card, p2Card))
    } else {
        player2.addAll(listOf(p2Card, p1Card))
    }

} while (player1.isNotEmpty() && player2.isNotEmpty())

if (player1.isEmpty()) {
    println(player2.reversed().mapIndexed {i, v -> (i+1) * v}.sum())
} else {
    println(player1.reversed().mapIndexed {i, v -> (i+1) * v}.sum())
}