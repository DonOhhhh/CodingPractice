#include <cassert>
#include <iostream>

const int MAX_SIZE = 1000;

template <typename T> class Stack {
private:
  T data[MAX_SIZE];
  int topIndex;

public:
  Stack() : topIndex(-1) {}

  void push(T item) {
    if (topIndex >= MAX_SIZE - 1) {
      throw std::overflow_error("Stack Overflow");
    }
    data[++topIndex] = item;
  }

  T pop() {
    if (isEmpty()) {
      throw std::underflow_error("Stack Underflow");
    }
    return data[topIndex--];
  }

  T peek() const {
    if (isEmpty()) {
      throw std::underflow_error("Stack is empty");
    }
    return data[topIndex];
  }

  bool isEmpty() const { return topIndex == -1; }

  int size() const { return topIndex + 1; }

  void clear() { topIndex = -1; }
};

int main() {
  Stack<int> s;

  // Test isEmpty and size on empty stack
  assert(s.isEmpty());
  assert(s.size() == 0);

  // Test push
  s.push(10);
  s.push(20);
  s.push(30);

  assert(!s.isEmpty());
  assert(s.size() == 3);
  assert(s.peek() == 30);

  // Test pop
  assert(s.pop() == 30);
  assert(s.peek() == 20);
  assert(s.size() == 2);

  // Test clear
  s.clear();
  assert(s.isEmpty());
  assert(s.size() == 0);

  // Test generic
  Stack<double> sd;
  sd.push(1.1);
  assert(sd.peek() == 1.1);

  std::cout << "All C++ Array-Stack tests passed!" << std::endl;

  return 0;
}
