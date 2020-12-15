import java.io.File;

val data = File("input-test-1.txt").bufferedReader().readLines()
val available = data.drop(1)
    .flatMap { it.trim().split(',') }
    .mapIndexed { i, v ->
        if (v != "x") {
            Pair(i, v.toInt())
        } else {
            null
        }
    }
    .filterNotNull()
    .sortedBy { it.first }

var step = available.first().second
var start = 0
for (ids in available) {
    val delta = ids.first.toInt()
    val id = ids.second
    for (i in (start..step*id).step(step)) {
        if (((i + delta) % id) != 0) {
            step = step * id
            start = i
        }
    }
}

println(start)