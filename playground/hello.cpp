#include <bits/stdc++.h>
using namespace std;

void my_insert(int *arr, int &len, int pos, int val) {
  for (int i = len; i > pos; i--)
    arr[i] = arr[i - 1];
  arr[pos] = val;
  len++;
}

void my_delete(int *arr, int &len, int pos) {
  len--;
  for (int i = pos; i < len; i++)
    arr[i] = arr[i + 1];
}

void my_append(int *arr, int &len, int val) { arr[len++] = val; }

void my_prepend(int *arr, int &len, int val) { my_insert(arr, len, 0, val); }

int my_pop_front(int *arr, int &len) {
  int ret = arr[0];
  my_delete(arr, len, 0);
  return ret;
}

int my_pop_back(int *arr, int &len) {
  int ret = arr[len - 1];
  len--;
  return ret;
}

void print_array(int *arr, int len) {
  for (int i = 0; i < len; i++)
    cout << arr[i] << ' ';
  cout << "\n";
}

int main() {
  int arr[100] = {10, 20, 30};
  int len = 3;
  my_insert(arr, len, 1, 40); // 10 40 20 30
  print_array(arr, len);

  my_delete(arr, len, 2); // 10 40 30
  print_array(arr, len);

  my_append(arr, len, 50); // 10 40 30 50
  print_array(arr, len);

  my_prepend(arr, len, 60); // 60 10 40 30 50
  print_array(arr, len);

  cout << my_pop_front(arr, len) << endl; // 60
  print_array(arr, len);                  // 10 40 30 50

  cout << my_pop_back(arr, len) << endl; // 50
  print_array(arr, len);                 // 10 40 30
}