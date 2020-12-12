import java.io.File;
var lines = File("input.txt").bufferedReader().readLines();

val allIds = (0..128*8).toMutableSet();

lines.forEach {
    var rowHigh = 128
    var rowLow = 0
    var colHigh = 8
    var colLow = 0

    it.forEach { c ->
        when(c) {
            'F' -> rowHigh = rowLow + (rowHigh - rowLow) / 2
            'B' -> rowLow = rowLow + (rowHigh - rowLow) / 2
            'L' -> colHigh = colLow + (colHigh - colLow) / 2
            'R' -> colLow = colLow + (colHigh - colLow) / 2
        }
    }

    allIds.remove(rowLow * 8 + colLow);
}

println(allIds)