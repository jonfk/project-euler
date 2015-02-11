import Stream._

// To run
// $ scala one.scala

object one {
  def main(args : Array[String]) {
    val result = from(0).take(1000).foldLeft(0){ (acc, x) => if (x % 3 == 0|| x % 5 == 0) { acc + x } else { acc }}
    println(result)
  }
}
