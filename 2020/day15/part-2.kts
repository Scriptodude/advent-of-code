import java.io.File;
val data = File("input.txt").bufferedReader().readLine().split(",").map { it.toInt() }
val seen = mutableMapOf<Int, Int>()

var previousOne = data.first()
seen.put(previousOne, 0)

(1..30_000_000).forEach {
    if (it <= data.size) {
        seen.put(previousOne, it)
        previousOne = data[it - 1]
    } else {

        val tmp = previousOne

        if (seen.containsKey(previousOne)) {
            previousOne = it - seen.get(previousOne)!!
        } else {
            previousOne = 0
        }

        seen.put(tmp, it)
    }
}

println(previousOne)