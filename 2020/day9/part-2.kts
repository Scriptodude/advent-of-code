import java.io.File;
import java.nio.charset.Charset

val lines = File("input.txt").bufferedReader(Charset.defaultCharset(), 25).readLines().map { it.trim().toLong() }
val invalidIndex = 616
val invalidNumber = lines[invalidIndex]

for (i in (0..invalidIndex-1)) {
    for (j in (i+1..invalidIndex - 1)) {
        val li = lines.subList(i, j)
        if (li.sum() == invalidNumber) {
            println(li.maxOrNull()!! + li.minOrNull()!!)
        }
    }
}