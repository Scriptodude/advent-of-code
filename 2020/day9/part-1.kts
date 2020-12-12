import java.io.File;
import java.nio.charset.Charset

val lines = File("input.txt").bufferedReader(Charset.defaultCharset(), 25).readLines().map { it.trim().toLong() }

for (n in (26..lines.size-1)) {
    val currentNumber = lines[n]
    var isValid = false

    for (i in (n-25..n)) {
        for (j in (n-25..n)) {
            if (i != j) {
                if (lines[i] + lines[j] == currentNumber) {
                    isValid = true
                    break
                }
            }
        }
        if (isValid) break
    }

    if (!isValid) println("$currentNumber, $n")
}