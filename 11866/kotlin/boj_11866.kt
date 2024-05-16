fun main() {
    val (n, k) = readln().split(" ").map { it.toInt() }
    val josephus_list = mutableListOf<Int>()
    val range = MutableList(n) { it + 1 }

    var idx = 0
    while (range.isNotEmpty()) {
        idx = (idx + k - 1) % range.size
        josephus_list.add(range[idx])
        range.removeAt(idx)
    }
    println("<${josephus_list.joinToString(", ")}>")
}
