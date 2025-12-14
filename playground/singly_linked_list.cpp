#include <iostream>

struct Node {
  int data;
  Node *next;

  Node(int val) : data(val), next(nullptr) {}
};

class LinkedList {
private:
  Node *head;

public:
  LinkedList() : head(nullptr) {}

  // Destructor to clean up memory
  ~LinkedList() {
    Node *current = head;
    while (current != nullptr) {
      Node *next = current->next;
      delete current;
      current = next;
    }
  }

  // Insert at the beginning
  void push_front(int val) {
    Node *newNode = new Node(val);
    newNode->next = head;
    head = newNode;
  }

  // Insert at the end
  void push_back(int val) {
    Node *newNode = new Node(val);
    if (head == nullptr) {
      head = newNode;
      return;
    }
    Node *temp = head;
    while (temp->next != nullptr) {
      temp = temp->next;
    }
    temp->next = newNode;
  }

  // Get the size of the list
  int size() const {
    int count = 0;
    Node *temp = head;
    while (temp != nullptr) {
      count++;
      temp = temp->next;
    }
    return count;
  }

  // Find element at index (0-based)
  // Returns -1 if index is out of bounds (assuming positive integers for data
  // for simplicity, or standard error handling could be used)
  int find(int index) const {
    Node *temp = head;
    int count = 0;
    while (temp != nullptr) {
      if (count == index) {
        return temp->data;
      }
      count++;
      temp = temp->next;
    }
    std::cerr << "Index out of bounds" << std::endl;
    return -1;
  }

  // Update element at index
  void update(int index, int val) {
    Node *temp = head;
    int count = 0;
    while (temp != nullptr) {
      if (count == index) {
        temp->data = val;
        return;
      }
      count++;
      temp = temp->next;
    }
    std::cerr << "Index out of bounds" << std::endl;
  }

  // Insert at arbitrary index
  void insert(int index, int val) {
    if (index < 0) {
      std::cerr << "Index cannot be negative" << std::endl;
      return;
    }

    if (index == 0) {
      push_front(val);
      return;
    }

    Node *newNode = new Node(val);
    Node *temp = head;
    int count = 0;

    while (temp != nullptr && count < index - 1) {
      temp = temp->next;
      count++;
    }

    if (temp == nullptr) {
      std::cerr << "Index out of bounds" << std::endl;
      delete newNode;
      return;
    }

    newNode->next = temp->next;
    temp->next = newNode;
  }

  // Remove at arbitrary index
  void remove(int index) {
    if (head == nullptr)
      return;

    if (index == 0) {
      Node *temp = head;
      head = head->next;
      delete temp;
      return;
    }

    Node *temp = head;
    int count = 0;
    while (temp->next != nullptr && count < index - 1) {
      temp = temp->next;
      count++;
    }

    if (temp->next == nullptr) {
      std::cerr << "Index out of bounds" << std::endl;
      return;
    }

    Node *nodeToDelete = temp->next;
    temp->next = nodeToDelete->next;
    delete nodeToDelete;
  }

  // Display the list
  void display() const {
    Node *temp = head;
    while (temp != nullptr) {
      std::cout << temp->data << " -> ";
      temp = temp->next;
    }
    std::cout << "nullptr" << std::endl;
  }
};

int main() {
  LinkedList list;

  std::cout << "Pushing: 1, 2, 3" << std::endl;
  list.push_back(1);
  list.push_back(2);
  list.push_back(3);
  list.display(); // 1 -> 2 -> 3 -> nullptr

  std::cout << "Size: " << list.size() << std::endl; // 3

  std::cout << "Find at index 1: " << list.find(1) << std::endl; // 2

  std::cout << "Update index 1 to 5" << std::endl;
  list.update(1, 5);
  list.display(); // 1 -> 5 -> 3 -> nullptr

  std::cout << "Insert 4 at index 2" << std::endl;
  list.insert(2, 4);
  list.display(); // 1 -> 5 -> 4 -> 3 -> nullptr

  std::cout << "Remove at index 1" << std::endl;
  list.remove(1);
  list.display(); // 1 -> 4 -> 3 -> nullptr

  return 0;
}
