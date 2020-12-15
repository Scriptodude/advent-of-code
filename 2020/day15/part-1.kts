import java.io.File;
val data = File("input.txt").bufferedReader().readLine().split(",").map { it.toInt() }
val values = Array<Int>(2020, () -> 0)

var lastOne = values.last()
data.dropLast(1).forEachIndexed { index, i -> values.set(index, i) }

fun contains(v: Int, from: Int): Int {
    for (i in from downTo 0) {
        if (values[i] == v) {
            return i
        }
    }

    return -1
}

(data.size-1..20).forEach {

    val lastSeen = contains(lastOne, it)
    if (lastSeen == -1) {
        lastOne = 0
    } else {
        lastOne = it - lastSeen
    }

    println("setting $lastOne at $index")
    values.set(it, lastSeen)
}

println(lastOne)