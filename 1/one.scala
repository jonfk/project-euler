import Stream._

// To run
// $ scala one.scala

object one {
  def main(args : Array[String]) {
    // val result = from(0).take(1000).foldLeft(0){ (acc, x) => if (x % 3 == 0|| x % 5 == 0) { acc + x } else { acc }}
    val result = (from(0).map(_ * 3).takeWhile(_ < 1000) ++ from(0).map(_ * 5).takeWhile(_ < 1000)).distinct.sum

    println(result)
  }
}
