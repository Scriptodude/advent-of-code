import java.io.File;
val data = File("input.txt").bufferedReader().readLines()
val ranges = mutableListOf<Pair<Int, Int>>()
val iterator = data.iterator()
val yourTickets = mutableListOf<Int>()
val nearbyTickets = mutableListOf<Long>()

while(iterator.hasNext()) {
    var line = iterator.next()
    if (line.isBlank()) {
        continue
    }

    if (line.startsWith("your ticket:")) {
        do {
            line = iterator.next()

            if (line.isBlank()) break
            yourTickets.addAll(line.split(",").map { it.trim().toInt() })
        } while (!line.isBlank())
    } else if (line.startsWith("nearby tickets:")) {
        do {
            line = iterator.next()

            if (line.isBlank()) break
            val nearTickets =
                    line
                            .split(",")
                            .mapIndexed { i, v -> Pair<Int, Long>(i, v.toLong()) }
                            .filter { v -> v.first < ranges.size && yourTickets.contains(v.second.toInt()) }
                            .filter { v -> ranges[v.first].first <= v.second && v.second <= ranges[v.first].second}
                            .map { it.second }

            nearbyTickets.addAll(nearTickets)
        } while (!line.isBlank() && iterator.hasNext())
    } else {
        val cur = line.dropWhile { v -> v != ':' }.drop(1)
        val rang = cur.split(" or ")

        if (line.startsWith("departure")) {
            rang.forEach { r ->
                val range = r.split("-").map { it.trim().toInt() }
                ranges.add(Pair(range[0], range[1]))
            }
        }
    }
}

println(nearbyTickets.reduce(Long::times))