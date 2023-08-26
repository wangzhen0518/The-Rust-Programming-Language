#include <boost/timer.hpp>
#include <iostream>

int main() {
  auto t = boost::timer();
  printf("elapsed: %f\n", t.elapsed());
}
