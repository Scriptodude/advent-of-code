import java.io.File;
var lines = File("input.txt").bufferedReader().readLines();

var max = 0

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

    println("$rowLow, $rowHigh, $colLow, $colHigh")
    max = Math.max(max, rowLow * 8 + colLow)
}

println(max)