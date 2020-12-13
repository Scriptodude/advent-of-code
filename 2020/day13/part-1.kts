import java.io.File;

val value = File("input.txt").bufferedReader().readLine().toInt()
val arr = File("input.txt").bufferedReader().readLines().drop(1)
    .flatMap { it.split(",") }
    .filter { it != "x" }
    .map { it.toInt() }
    .filter { it > 0 }
    .map {
        val high = Math.ceil(value.toDouble() / it) * it
        val diff = high - value

        Pair<Double, Int>(diff, it)
    }
    .sortedBy { it.first }

println(arr.first().first * arr.first().second)