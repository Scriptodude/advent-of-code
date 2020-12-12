import java.io.File;
val lines = File("input.txt").bufferedReader().readLines().map { it.trim().toInt() }.sorted().reversed().toMutableList()
lines.add(0, lines.first() + 3)
lines.add(0)

val seen = mutableMapOf<Int, Long>()

fun count(idx: Int): Long {
    if (idx == lines.size - 1) {
        seen.put(lines[idx], 1)
        return 1L
    }

    var total = 0L
    for (i in (idx + 1..Math.min(idx + 4, lines.size - 1))) {
        if (Math.abs(lines[i] - lines[idx]) > 3) continue

        val cur = lines[i]
        if (seen.containsKey(cur)) {
            total += seen.getOrDefault(cur, 0L)
        } else {
            total += count(i)
        }
    }

    seen.put(lines[idx], total)
    return total
}

println(count(0))