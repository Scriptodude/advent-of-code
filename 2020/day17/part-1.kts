import java.io.File;
typealias ZSlice = MutableList<MutableList<Boolean>>
typealias Cube = MutableList<ZSlice>

class PocketCube {
    private var cubes: Cube = mutableListOf(mutableListOf(mutableListOf()))

    constructor(initSize: Int, initData: ZSlice) {
        this.cubes = MutableList(initSize) {
            MutableList(initSize) {
                MutableList(initSize) { false }
            }
        }
        this.cubes[0] = initData.toMutableList()
    }

    fun simulate() {
        var newCube = this.cubes.toMutableList()
        val cubeSize = this.cubes.size

        for(z in (-1..cubeSize)) {
            for (y in (-1..cubeSize)) {
                for (x in (-1..cubeSize)) {
                    val neighbors = getNeighbors(x, y, z)
                    val activeNeigh = neighbors.count { it == true }
                    var newVal: Boolean

                    if(isActive(x, y, z)) {
                        newVal = (activeNeigh == 2 || activeNeigh == 3)
                    } else {
                        newVal = (activeNeigh == 3)
                    }

                    if (isOutsideCube(x, y, z)) {
                        newCube = this.stretch(newCube)
                        newCube[Math.max(z, 0)][Math.max(y, 0)][Math.max(x, 0)] = newVal
                    } else {
                        newCube[z][y][x] = newVal
                    }
                }
            }
        }

        this.cubes = newCube
    }

    fun countActives(): Int {
        val size = this.cubes.size - 1
        var count = 0
        for(z in (0..size)) {
            for (y in (0..size)) {
                for (x in (0..size)) {
                    if (this.cubes[z][y][x]) count += 1
                }
            }
        }

        return count
    }

    private fun isOutsideCube(x: Int, y: Int, z: Int): Boolean {
        return ((z < 0 || y < 0 || x < 0) ||
                        (z >= cubes.size) ||
                        (y >= cubes[z].size) ||
                        (x >= cubes[z][y].size))
    }

    private fun isActive(x: Int, y: Int, z: Int): Boolean {
        if (isOutsideCube(x, y, z)) {
            return false
        }

        return cubes[z][y][x]
    }

    private fun getNeighbors(cx: Int, cy: Int, cz: Int): List<Boolean> {
        val neighbors = mutableListOf<Boolean>()

        for(z in (-1..1)) {
            for (y in (-1..1)) {
                for (x in (-1..1)) {
                    if (x == y && y == z && x == 0) continue

                    val xidx = cx + x
                    val yidx = cy + y
                    val zidx = cz + z
                    if (isOutsideCube(xidx, yidx, zidx)) {
                        neighbors.add(false)
                        continue
                    }

                    neighbors.add(this.cubes[zidx][yidx][xidx])
                }
            }
        }

        return neighbors
    }

    private fun stretch(cube: Cube): Cube {
        val size = this.cubes.size

        val grownCube: Cube = MutableList(size + 1) {
            MutableList(size + 1) {
                MutableList(size + 1) { false }
            }
        }

        for (z in (1..size)) {
            for (y in (1..size)) {
                for (x in (1..size)) {
                    grownCube[z][y][x] = cube[z-1][y-1][x-1]
                }
            }
        }

        return grownCube
    }
}

val data = File("input-test-1.txt").bufferedReader().readLines().map { it.map { it == '#' }.toMutableList() }.toMutableList()
val cube = PocketCube(data.size, data)

(0..5).forEach {
    cube.simulate()
}

println(cube.countActives())