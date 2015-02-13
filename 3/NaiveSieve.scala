
import Stream._

object NaiveSieve {
  def main(args : Array[String]) {
    val primes = sieve(from(2))
    val p = primes.takeWhile( _ < 10000).toList
    println(p)
  }
  def sieve(s :Stream[Int]): Stream[Int] = s.head #:: sieve(s.tail filter (_ % s.head != 0))
}
